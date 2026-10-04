import Foundation

/// The text field a custom keyboard types into, reduced to what a keyboard can
/// do with it (`UITextDocumentProxy` on iOS). Kept free of UIKit so
/// ``KeyboardSession`` can be unit-tested with a fake document.
@MainActor
public protocol TextDocument {
    /// The text before the caret, including any marked text, or nil when the
    /// app doesn't expose it.
    var textBeforeCaret: String? { get }

    /// Shows `text` as marked text with the caret at its end, replacing the
    /// current marked text or, with none, inserting it at the caret. An empty
    /// `text` removes the marked text.
    func setMarkedText(_ text: String)

    /// Makes the marked text final, leaving it in the document.
    func unmarkText()

    /// Inserts `text` at the caret. Called only while nothing is marked.
    func insertText(_ text: String)

    /// Deletes the character before the caret. Called only while nothing is
    /// marked.
    func deleteBackward()
}

/// One keyboard's typing session on iOS: the Rust `Composer` plus the pending
/// text it shows as marked text (ios-keyboard spec, "Typing in any app").
///
/// It follows the macOS input source: committed text is final and never
/// changed again, and the word being typed is marked text that later keys and
/// Backspace replace (whole-word-pending design D1 and D2). Unlike macOS, a
/// keyboard can't read the caret position, so it tells a caret move from its
/// own typing by the text before the caret alone
/// (apple-silicon-and-ios-keyboard design D3).
@MainActor
public final class KeyboardSession {
    private let composer: Composer

    /// The pending text currently shown as marked text.
    public private(set) var shownPending = ""

    /// A session with nothing pending.
    public init(config: Config) {
        composer = Composer(config: config)
    }

    /// The output options; a change applies from the next key.
    public var config: Config {
        get { composer.config() }
        set {
            if composer.config() != newValue {
                composer.setConfig(config: newValue)
            }
        }
    }

    /// Types one character (space included).
    public func type(_ key: String, in document: some TextDocument) {
        var context: String?
        if shownPending.isEmpty {
            let before = textBeforeCaret(in: document)
            resetIfCaretMoved(before)
            if keyReadsDocument(key: key) {
                context = before
            }
        }
        let update = composer.key(key: key, textBeforeCaret: context)
        apply(update, to: document)
        if !update.handled {
            // Only multi-character key names are left unhandled; a keyboard
            // has no app behind it to type them, so type them as they are.
            commit(in: document)
            document.insertText(key)
        }
    }

    /// Backspace: removes one letter of the pending word (`দ্ম` → `দ`); with
    /// nothing pending the app deletes one character of committed text.
    public func backspace(in document: some TextDocument) {
        let update = composer.backspace()
        guard update.handled else {
            document.deleteBackward()
            return
        }
        apply(update, to: document)
    }

    /// Return: commits the pending word, then types a line break.
    public func returnKey(in document: some TextDocument) {
        commit(in: document)
        document.insertText("\n")
    }

    /// Makes the pending word final and starts over: the keyboard is closing,
    /// or the next text doesn't continue this word.
    public func commit(in document: some TextDocument) {
        apply(composer.flush(), to: document)
        _ = composer.reset(textBeforeCaret: nil)
    }

    /// The app changed the text or the selection (iOS calls
    /// `textDidChange`). If the text before the caret no longer ends with the
    /// pending word, the app has ended the composition (a tap elsewhere keeps
    /// the marked text as normal text), so the session forgets it without
    /// writing anything (apple-silicon-and-ios-keyboard design D3).
    public func documentDidChange(_ document: some TextDocument) {
        guard !shownPending.isEmpty, let before = document.textBeforeCaret,
            !before.utf16.reversed().starts(with: shownPending.utf16.reversed())
        else {
            return
        }
        _ = composer.reset(textBeforeCaret: nil)
        shownPending = ""
    }

    /// Applies a composer update: `commit` replaces the marked text as final
    /// text, then `pending` becomes the new marked text.
    private func apply(_ update: Update, to document: some TextDocument) {
        if !update.commit.isEmpty {
            if shownPending.isEmpty {
                document.insertText(update.commit)
            } else {
                document.setMarkedText(update.commit)
                document.unmarkText()
            }
        } else if !shownPending.isEmpty && update.pending.isEmpty {
            // Backspace removed the whole pending word.
            document.setMarkedText("")
            document.unmarkText()
        }
        if !update.pending.isEmpty {
            document.setMarkedText(update.pending)
        }
        shownPending = update.pending
    }

    /// The text before the caret in the current paragraph, or nil when the app
    /// doesn't expose it.
    private func textBeforeCaret(in document: some TextDocument) -> String? {
        guard let text = document.textBeforeCaret, !text.isEmpty else {
            return nil
        }
        return DocumentText.clipToParagraph(text)
    }

    /// Design D3: with nothing pending, text before the caret that doesn't end
    /// like what the composer typed means the caret moved (or the app changed
    /// the text), so the next key starts a new word there.
    private func resetIfCaretMoved(_ before: String?) {
        guard let before, !composer.matchesTextBeforeCaret(textBeforeCaret: before) else {
            return
        }
        _ = composer.reset(textBeforeCaret: nil)
    }
}
