import AppKit
import BengaliIMECore
import InputMethodKit

/// IMK creates one controller per text-input session (roughly, per text field
/// that uses Druti). All Bengali logic lives in the Rust `Composer`;
/// this class routes keys to it and applies its updates to the app:
/// committed text becomes normal text, and pending text (the Bengali word being
/// typed, which later keys and Backspace may still change) is shown as marked
/// text.
///
/// The Objective-C name must match InputMethodServerControllerClass in Info.plist.
@objc(DrutiInputController)
final class InputController: IMKInputController {
    private let composer = Composer(config: Settings.config)

    /// The pending text currently shown as marked text in the app.
    private var shownPending = ""

    /// Where the caret should be before the next key. Recorded after each update
    /// that leaves nothing pending; nil when unknown (the check is skipped).
    private var expectedCaret: Int?

    private static let noReplacement = NSRange(location: NSNotFound, length: NSNotFound)

    // MARK: - Session

    override func recognizedEvents(_ sender: Any!) -> Int {
        Int(NSEvent.EventTypeMask.keyDown.rawValue)
    }

    override func activateServer(_ sender: Any!) {
        super.activateServer(sender)
        syncSettings()
        _ = composer.reset(textBeforeCaret: nil)
        shownPending = ""
        expectedCaret = nil
    }

    /// Switching to another input source or leaving the text field.
    override func deactivateServer(_ sender: Any!) {
        if let client = textInput(sender) {
            commitPending(client)
        }
        super.deactivateServer(sender)
    }

    /// The app asks for the composition to end (a click, a focus change, ...).
    override func commitComposition(_ sender: Any!) {
        if let client = textInput(sender) {
            commitPending(client)
        }
    }

    // MARK: - Keys

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event, event.type == .keyDown, let client = sender as? IMKTextInput else {
            return false
        }
        syncSettings()

        let flags = event.modifierFlags
        let press = KeyPress(
            keyCode: event.keyCode,
            characters: event.characters,
            command: flags.contains(.command),
            control: flags.contains(.control),
            option: flags.contains(.option))

        switch KeyRouting.route(press) {
        case .commitAndPass:
            commitPending(client)
            return false

        case .backspace:
            // whole-word-pending design D2: Druti edits only its own pending
            // word. With nothing pending the app deletes committed text by its
            // own rules, and the composer has reset.
            let update = composer.backspace()
            guard update.handled else {
                expectedCaret = nil
                return false
            }
            apply(update, to: client)
            return true

        case .type(let key):
            resetIfCaretMoved(client)
            let context =
                shownPending.isEmpty && keyReadsDocument(key: key)
                ? ClientText.beforeCaret(of: client)
                : nil
            let update = composer.key(key: key, textBeforeCaret: context)
            apply(update, to: client)
            if !update.handled {
                commitPending(client)
            }
            return update.handled
        }
    }

    /// Applies a composer update: commit `commit` in place of the marked text,
    /// then show `pending` as marked text. Updates never change text committed
    /// earlier, so no replacement range is ever needed; apps that ignore one
    /// (Chromium-based apps, editors built on EditContext) behave like the rest
    /// (whole-word-pending design D2).
    private func apply(_ update: Update, to client: IMKTextInput) {
        if !update.commit.isEmpty {
            // Replaces the marked text with final text.
            client.insertText(update.commit, replacementRange: Self.noReplacement)
        } else if !shownPending.isEmpty && update.pending.isEmpty {
            // Backspace removed the whole pending word.
            client.setMarkedText(
                "", selectionRange: NSRange(location: 0, length: 0),
                replacementRange: Self.noReplacement)
        }

        if !update.pending.isEmpty {
            client.setMarkedText(
                markedText(update.pending),
                selectionRange: NSRange(location: update.pending.utf16.count, length: 0),
                replacementRange: Self.noReplacement)
        }

        shownPending = update.pending
        expectedCaret = update.pending.isEmpty ? ClientText.caret(of: client) : nil
    }

    /// Pending text styled to look like normal text. Apps may still draw their
    /// own marked-text underline; then the word being typed is underlined.
    private func markedText(_ text: String) -> NSAttributedString {
        NSAttributedString(
            string: text,
            attributes: [
                .underlineStyle: 0,
                .markedClauseSegment: 0,
            ])
    }

    /// Commits the pending text and starts over: the caret is about to move or
    /// the session is ending, so the composer's view of the document is stale.
    private func commitPending(_ client: IMKTextInput) {
        apply(composer.flush(), to: client)
        _ = composer.reset(textBeforeCaret: nil)
        expectedCaret = nil
    }

    /// Design D9: if the caret isn't where the last update left it (a click, a
    /// paste, an edit by the app), start a new cluster at the new position.
    private func resetIfCaretMoved(_ client: IMKTextInput) {
        guard shownPending.isEmpty, let expected = expectedCaret else {
            return
        }
        let selection = client.selectedRange()
        guard selection.location != NSNotFound else {
            return
        }
        if selection.location != expected || selection.length > 0 {
            // whole-word-pending design D4: Chromium-based apps report the
            // caret late or in their own coordinates. If the text before the
            // caret still ends like what the composer typed, keep typing.
            if let text = ClientText.beforeCaret(of: client),
                composer.matchesTextBeforeCaret(textBeforeCaret: text)
            {
                Log.input.debug(
                    "Caret at \(selection.location, privacy: .public), expected \(expected, privacy: .public); text matches, not resetting"
                )
                return
            }
            Log.input.debug(
                "Caret moved from \(expected, privacy: .public) to \(selection.location, privacy: .public)+\(selection.length, privacy: .public); resetting"
            )
            _ = composer.reset(textBeforeCaret: nil)
            expectedCaret = nil
        }
    }

    // MARK: - Settings and menu

    /// Pushes the input menu toggles to the composer if they changed, here or
    /// in another text field.
    private func syncSettings() {
        let config = Settings.config
        if composer.config() != config {
            composer.setConfig(config: config)
        }
    }

    /// The input menu (the Druti icon in the menu bar).
    override func menu() -> NSMenu! {
        let menu = NSMenu(title: "Druti")

        func addToggle(_ title: String, _ option: Settings.Option, _ action: Selector) {
            let item = NSMenuItem(title: title, action: action, keyEquivalent: "")
            item.state = Settings.isOn(option) ? .on : .off
            menu.addItem(item)
        }
        addToggle(
            "Bengali Digits (\u{09E7}\u{09E8}\u{09E9})", .bengaliDigits,
            #selector(toggleBengaliDigits(_:)))
        // দাঁড়ি, written with escapes so editors can't decompose ড়.
        addToggle(
            "\u{09A6}\u{09BE}\u{0981}\u{09DC}\u{09BF} (\u{0964}) for Full Stop", .dariForPeriod,
            #selector(toggleDari(_:)))
        addToggle(
            "Smart Quotes (\u{201C} \u{201D})", .smartQuotes, #selector(toggleSmartQuotes(_:)))
        addToggle("Autocorrect", .autocorrect, #selector(toggleAutocorrect(_:)))

        menu.addItem(.separator())
        menu.addItem(
            NSMenuItem(
                title: "Convert Selection to Bengali", action: #selector(convertSelection(_:)),
                keyEquivalent: ""))

        menu.addItem(.separator())
        menu.addItem(
            NSMenuItem(
                title: "Uninstall Druti\u{2026}", action: #selector(uninstall(_:)),
                keyEquivalent: ""))
        return menu
    }

    // IMK sends menu actions to the controller with a dictionary argument.
    @objc private func toggleBengaliDigits(_ sender: Any?) { toggle(.bengaliDigits) }
    @objc private func toggleDari(_ sender: Any?) { toggle(.dariForPeriod) }
    @objc private func toggleSmartQuotes(_ sender: Any?) { toggle(.smartQuotes) }
    @objc private func toggleAutocorrect(_ sender: Any?) { toggle(.autocorrect) }

    private func toggle(_ option: Settings.Option) {
        Settings.toggle(option)
        syncSettings()
    }

    /// Replaces the selected roman text with Bengali, keeping line breaks and
    /// applying the current toggles. Does nothing in apps that don't expose
    /// their selection.
    @objc private func convertSelection(_ sender: Any?) {
        guard let client = textInput(nil) else {
            return
        }
        commitPending(client)
        guard let selection = ClientText.selection(of: client) else {
            Log.input.info("Convert selection: no readable selection")
            return
        }
        let converted = transpileRomanDocument(
            document: selection.text, preserveLineBreaks: true, config: Settings.config)
        guard converted != selection.text else {
            return
        }
        client.insertText(converted, replacementRange: selection.range)
        expectedCaret = nil
    }

    /// Asks, then disables Druti, moves it to the Trash, deletes its settings
    /// and quits (add-macos-distribution design D5).
    @MainActor @objc private func uninstall(_ sender: Any?) {
        if let client = textInput(nil) {
            commitPending(client)
        }
        let app = NSApplication.shared
        InstallerUI.bringToFront(app)
        let alert = NSAlert()
        alert.messageText = "Uninstall Druti?"
        alert.informativeText =
            "Druti will be removed from your input sources and moved to the Trash, and its settings will be deleted."
        alert.addButton(withTitle: "Uninstall").hasDestructiveAction = true
        alert.addButton(withTitle: "Cancel")
        guard alert.runModal() == .alertFirstButtonReturn else {
            // Back to a background app, so focus returns to the text field.
            app.setActivationPolicy(.prohibited)
            app.hide(nil)
            return
        }
        do {
            try Installer.uninstall(moveToTrash: true)
            Log.input.info("Uninstalled")
            exit(0)
        } catch {
            let failure = NSAlert(error: error)
            failure.messageText = "Druti couldn’t be uninstalled"
            failure.informativeText = error.localizedDescription
            failure.runModal()
            app.setActivationPolicy(.prohibited)
            app.hide(nil)
        }
    }

    // MARK: - Helpers

    /// IMK passes the client as `sender` for most calls; fall back to the
    /// controller's current client.
    private func textInput(_ sender: Any?) -> IMKTextInput? {
        if let client = sender as? IMKTextInput {
            return client
        }
        return client()
    }
}
