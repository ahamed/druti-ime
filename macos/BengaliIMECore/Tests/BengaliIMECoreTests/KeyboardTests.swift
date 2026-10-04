import BengaliIMECore
import Foundation
import Testing

/// A text field as a keyboard sees it: committed text before the caret, then
/// the marked text, with the caret at its end.
@MainActor
private final class FakeDocument: TextDocument {
    var committed: String
    var marked = ""
    /// False models apps that don't expose their text to keyboards.
    var exposesText = true

    init(_ committed: String = "") {
        self.committed = committed
    }

    var text: String { committed + marked }

    var textBeforeCaret: String? { exposesText ? text : nil }

    func setMarkedText(_ text: String) {
        marked = text
    }

    func unmarkText() {
        committed += marked
        marked = ""
    }

    func insertText(_ text: String) {
        #expect(marked.isEmpty, "insertText(\(text)) while \(marked) is marked")
        committed += text
    }

    func deleteBackward() {
        #expect(marked.isEmpty, "deleteBackward while \(marked) is marked")
        if !committed.unicodeScalars.isEmpty {
            committed.unicodeScalars.removeLast()
        }
    }

    /// What iOS does when the user taps elsewhere: the marked text stays as
    /// normal text, and the caret goes after `textBeforeCaret`.
    func tapElsewhere(_ textBeforeCaret: String) {
        unmarkText()
        committed = textBeforeCaret
    }
}

@Suite("Keyboard session")
@MainActor
struct KeyboardSessionTests {
    private func type(
        _ keys: String, into document: FakeDocument, session: KeyboardSession? = nil
    ) -> KeyboardSession {
        let session = session ?? KeyboardSession(config: defaultConfig())
        for key in keys {
            session.type(String(key), in: document)
        }
        return session
    }

    // ios-keyboard spec, "Typing in any app": each key shows its Bengali as
    // marked text, and Space commits the word.
    @Test func typingKhubShowsEachKeyAsMarkedText() {
        let document = FakeDocument()
        let session = KeyboardSession(config: defaultConfig())
        for (key, shown) in zip(["k", "h", "u", "b"], ["ক", "খ", "খু", "খুব"]) {
            session.type(key, in: document)
            #expect(document.marked == shown, "after \(key)")
            #expect(document.committed == "", "after \(key)")
        }
        session.type(" ", in: document)
        #expect(document.committed == "খুব ")
        #expect(document.marked == "")
        #expect(session.shownPending == "")
    }

    @Test func returnCommitsTheWordThenTypesALineBreak() {
        let document = FakeDocument()
        let session = type("ami", into: document)
        session.returnKey(in: document)
        #expect(document.text == "আমি\n")
        #expect(document.marked == "")
    }

    // whole-word-pending design D1: Backspace edits the pending word.
    @Test func backspaceRemovesOneLetterOfThePendingWord() {
        let document = FakeDocument()
        let session = type("podmo", into: document)
        #expect(document.marked == "পদ্ম")
        session.backspace(in: document)
        #expect(document.marked == "পদ")
        #expect(document.committed == "")
    }

    @Test func backspaceRemovingTheWholeWordLeavesNothingMarked() {
        let document = FakeDocument("আমি ")
        let session = type("k", into: document)
        session.backspace(in: document)
        #expect(document.text == "আমি ")
        #expect(session.shownPending == "")
    }

    // Design D2: committed text is the app's; Backspace there deletes as usual.
    @Test func backspaceWithNothingPendingDeletesCommittedText() {
        let document = FakeDocument()
        let session = type("ka ", into: document)
        session.backspace(in: document)
        #expect(document.text == "কা")
    }

    @Test func vowelAfterAnExistingConsonantBecomesAKar() {
        let document = FakeDocument("ক")
        _ = type("i", into: document)
        #expect(document.text == "কি")
    }

    @Test func appsThatHideTheirTextGetAnIndependentVowel() {
        let document = FakeDocument("ক")
        document.exposesText = false
        _ = type("i", into: document)
        #expect(document.text == "কই")
    }

    // apple-silicon-and-ios-keyboard design D3: with nothing pending, text
    // before the caret that doesn't continue what was typed is a caret move.
    @Test func typingAfterTheCaretMovedStartsAtTheNewPlace() {
        let document = FakeDocument()
        let session = type("ki ", into: document)
        document.tapElsewhere("ক")
        _ = type("i", into: document, session: session)
        #expect(document.text == "কি")
    }

    @Test func tappingElsewhereEndsThePendingWord() {
        let document = FakeDocument()
        let session = type("kh", into: document)
        document.tapElsewhere("আমি ")
        session.documentDidChange(document)
        #expect(session.shownPending == "")
        _ = type("k", into: document, session: session)
        #expect(document.text == "আমি ক")
    }

    @Test func ownEditsAreNotMistakenForTheAppEndingTheWord() {
        let document = FakeDocument()
        let session = type("kh", into: document)
        session.documentDidChange(document)
        #expect(session.shownPending == "খ")
        _ = type("a", into: document, session: session)
        #expect(document.marked == "খা")
    }

    @Test func commitMakesThePendingWordFinal() {
        let document = FakeDocument()
        let session = type("kh", into: document)
        session.commit(in: document)
        #expect(document.committed == "খ")
        #expect(document.marked == "")
        #expect(session.shownPending == "")
    }

    @Test func configChangesApplyFromTheNextKey() {
        let document = FakeDocument()
        let session = type("1", into: document)
        session.config = Config(bengaliDigits: false, dariForPeriod: true, smartQuotes: true)
        _ = type("1 ", into: document, session: session)
        #expect(document.text == "১1 ")
    }
}

@Suite("Keyboard layout")
struct KeyboardLayoutTests {
    private func characters(_ row: [KeyboardKey]) -> String {
        row.compactMap { key -> String? in
            guard case .character(let text) = key else {
                return nil
            }
            return text
        }.joined()
    }

    @Test func lettersLayerIsQwerty() {
        let rows = KeyboardState().rows(showsNextKeyboard: true)
        #expect(rows.map(characters) == ["qwertyuiop", "asdfghjkl", "zxcvbnm", ""])
        #expect(rows[2].first == .shift)
        #expect(rows[2].last == .backspace)
        #expect(rows[3] == [.layer(.numbers), .nextKeyboard, .options, .space, .returnKey])
    }

    @Test func globeKeyOnlyWhenIOSAsksForIt() {
        let bottom = KeyboardState().rows(showsNextKeyboard: false)[3]
        #expect(!bottom.contains(.nextKeyboard))
    }

    @Test func shiftTypesOneCapitalThenTurnsOff() {
        var state = KeyboardState()
        state.tapShift(isDoubleTap: false)
        #expect(state.shift == .once)
        #expect(characters(state.rows(showsNextKeyboard: false)[0]) == "QWERTYUIOP")
        state.didType(.character("T"))
        #expect(state.shift == .off)
    }

    @Test func doubleTapOnShiftIsCapsLock() {
        var state = KeyboardState()
        state.tapShift(isDoubleTap: false)
        state.tapShift(isDoubleTap: true)
        #expect(state.shift == .locked)
        state.didType(.character("O"))
        #expect(state.shift == .locked)
        state.tapShift(isDoubleTap: false)
        #expect(state.shift == .off)
    }

    // The chandrabindu (`^`) and bisarga (`:`) keys must be reachable.
    @Test func numbersAndSymbolsHoldThePhoneticPunctuation() {
        var state = KeyboardState()
        state.show(.numbers)
        let numbers = state.rows(showsNextKeyboard: false).map(characters).joined()
        #expect(numbers.contains(":"))
        #expect(numbers.contains("1"))
        state.show(.symbols)
        #expect(state.rows(showsNextKeyboard: false).map(characters).joined().contains("^"))
    }

    @Test func spaceOnTheNumbersLayerReturnsToLetters() {
        var state = KeyboardState()
        state.show(.numbers)
        state.didType(.character("1"))
        #expect(state.layer == .numbers)
        state.didType(.space)
        #expect(state.layer == .letters)
    }

    @Test func everyRowFitsTheWidthOfTenKeys() {
        for layer in [KeyboardLayer.letters, .numbers, .symbols] {
            var state = KeyboardState()
            state.show(layer)
            for row in state.rows(showsNextKeyboard: true).dropLast() {
                #expect(row.count <= 10, "\(layer): \(row)")
            }
        }
    }
}
