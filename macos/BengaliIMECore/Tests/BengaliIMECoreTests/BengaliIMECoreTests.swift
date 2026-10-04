import BengaliIMECore
import Foundation
import Testing

@Suite("Composer bindings")
struct ComposerBindingTests {
    private func type(_ keys: String, into composer: Composer) -> String {
        var committed = ""
        for key in keys {
            let update = composer.key(key: String(key), textBeforeCaret: nil)
            #expect(update.handled, "key \(key)")
            committed += update.commit
        }
        return committed
    }

    // Task 4.3: k, h, u, b, space through the bindings commits খুব.
    @Test func typingKhubCommitsTheWord() {
        let composer = Composer(config: defaultConfig())
        #expect(type("khub ", into: composer) == "খুব ")
        #expect(composer.pending() == "")
    }

    @Test func pendingThenLetterBackspace() {
        let composer = Composer(config: defaultConfig())
        #expect(composer.key(key: "k", textBeforeCaret: nil).pending == "ক")
        #expect(composer.key(key: "h", textBeforeCaret: nil).pending == "খ")
        let update = composer.backspace()
        #expect(update.handled)
        #expect(update.pending == "")
        #expect(!composer.backspace().handled)
    }

    // whole-word-pending design D1: the word being typed stays pending, so
    // Backspace edits it in every app.
    @Test func podmoThenBackspaceRemovesTheLastConsonant() {
        let composer = Composer(config: defaultConfig())
        _ = type("podmo", into: composer)
        #expect(composer.pending() == "পদ্ম")
        let update = composer.backspace()
        #expect(update.handled)
        #expect(update.commit == "")
        #expect(update.pending == "পদ")
    }

    // Design D2: committed text is the app's; Backspace there is not handled.
    @Test func backspaceWithNothingPendingIsLeftToTheApp() {
        let composer = Composer(config: defaultConfig())
        #expect(type("khub ", into: composer) == "খুব ")
        #expect(!composer.backspace().handled)
    }

    // Design D3: a consonant after Backspace starts a new letter.
    @Test func consonantAfterBackspaceStartsANewLetter() {
        let composer = Composer(config: defaultConfig())
        _ = type("ka", into: composer)
        #expect(composer.backspace().pending == "ক")
        #expect(composer.key(key: "k", textBeforeCaret: nil).pending == "কক")
    }

    // whole-word-pending design D4: a caret the app reports late is not a
    // caret move when the text before it still ends like what the composer typed.
    @Test func textBeforeCaretIsCheckedAgainstTheComposer() {
        let composer = Composer(config: defaultConfig())
        _ = type("po ", into: composer)
        #expect(composer.matchesTextBeforeCaret(textBeforeCaret: "প "))
        #expect(!composer.matchesTextBeforeCaret(textBeforeCaret: "আমি "))
    }

    @Test func karAttachesToDocumentText() {
        let composer = Composer(config: defaultConfig())
        #expect(composer.key(key: "i", textBeforeCaret: "ক").pending == "\u{09BF}")
        #expect(
            Composer(config: defaultConfig()).key(key: "i", textBeforeCaret: nil).pending == "ই")
    }

    // Design D2: a rule never rewrites committed text; the `-` is typed as is.
    @Test func hyphenInDocumentIsNotRewritten() {
        let update = Composer(config: defaultConfig()).key(key: "-", textBeforeCaret: "a-")
        #expect(update.commit == "")
        #expect(update.pending == "-")
    }

    @Test func configToggles() {
        let composer = Composer(
            config: Config(
                bengaliDigits: false, dariForPeriod: true, smartQuotes: true, autocorrect: false))
        #expect(type("2", into: composer) == "2")
    }

    // Autocorrect spec: off by default; on, a word is corrected when it ends
    // and Backspace undoes it.
    @Test func autocorrectCorrectsAtWordEndAndBackspaceUndoes() {
        #expect(!defaultConfig().autocorrect)
        var config = defaultConfig()
        config.autocorrect = true
        let composer = Composer(config: config)
        #expect(type("amra ", into: composer) == "")
        #expect(composer.pending() == "আমরা ")
        #expect(composer.backspace().pending == "আম্রা")
        #expect(
            transpileRomanDocument(document: "amra", preserveLineBreaks: true, config: config)
                == "আমরা")
    }

    // A consonant starts a new letter after committed text, so it never reads
    // the document (whole-word-pending design D3).
    @Test func keyReadsDocumentOnlyForContextKeys() {
        #expect(keyReadsDocument(key: "i"))
        #expect(keyReadsDocument(key: "\""))
        #expect(!keyReadsDocument(key: "k"))
        #expect(!keyReadsDocument(key: "1"))
    }

    @Test func transpileSelection() {
        // U+09DF is য় (written escaped so editors can't decompose it).
        let converted = transpileRomanDocument(
            document: "ami banglay gan gai", preserveLineBreaks: true, config: defaultConfig())
        #expect(converted == "আমি বাংলা\u{09DF} গান গাই")
    }
}

@Suite("Key routing")
struct KeyRoutingTests {
    @Test(arguments: [
        (UInt16(40), "k"), (40, "K"), (49, " "), (44, "?"),
    ])
    func printableKeysAreTyped(keyCode: UInt16, characters: String) {
        #expect(
            KeyRouting.route(KeyPress(keyCode: keyCode, characters: characters))
                == .type(characters))
    }

    @Test func backspace() {
        #expect(KeyRouting.route(KeyPress(keyCode: 51, characters: "\u{7F}")) == .backspace)
        #expect(
            KeyRouting.route(KeyPress(keyCode: 51, characters: "\u{7F}", option: true))
                == .commitAndPass)
    }

    @Test(arguments: [36, 76, 48, 53, 117, 115, 119, 116, 121, 123, 124, 125, 126] as [UInt16])
    func navigationAndReturnCommitAndPass(keyCode: UInt16) {
        #expect(KeyRouting.route(KeyPress(keyCode: keyCode, characters: "x")) == .commitAndPass)
    }

    @Test(arguments: [
        KeyPress(keyCode: 1, characters: "s", command: true),
        KeyPress(keyCode: 8, characters: "c", control: true),
        KeyPress(keyCode: 0, characters: "å", option: true),
    ])
    func shortcutsCommitAndPass(press: KeyPress) {
        #expect(KeyRouting.route(press) == .commitAndPass)
    }

    @Test(arguments: ["\u{F704}", "\u{1B}", nil, ""] as [String?])
    func functionAndControlCharactersCommitAndPass(characters: String?) {
        #expect(KeyRouting.route(KeyPress(keyCode: 0, characters: characters)) == .commitAndPass)
    }
}

@Suite("Document text")
struct DocumentTextTests {
    @Test func rangeIsBounded() {
        #expect(DocumentText.rangeBeforeCaret(10) == NSRange(location: 0, length: 10))
        #expect(DocumentText.rangeBeforeCaret(5000) == NSRange(location: 3976, length: 1024))
        #expect(DocumentText.rangeBeforeCaret(0) == NSRange(location: 0, length: 0))
    }

    @Test func clipsAtParagraphStart() {
        #expect(DocumentText.clipToParagraph("“আমি\nক") == "ক")
        #expect(DocumentText.clipToParagraph("আমি\r\n") == "")
        #expect(DocumentText.clipToParagraph("a\u{2029}b") == "b")
        #expect(DocumentText.clipToParagraph("কখ") == "কখ")
    }
}

@Suite("Install location")
struct InstallLocationTests {
    private let home = URL(fileURLWithPath: "/Users/someone", isDirectory: true)

    @Test(arguments: [
        "/Users/someone/Library/Input Methods/Druti.app",
        "/Library/Input Methods/Druti.app",
        "/Users/someone/Library/Input Methods/../Input Methods/Druti.app",
    ])
    func installedCopiesRunAsTheInputMethod(path: String) {
        #expect(InstallLocation.isInstalled(bundle: URL(fileURLWithPath: path), home: home))
    }

    @Test(arguments: [
        "/Volumes/Druti 1.0.0/Druti.app",
        "/Users/someone/Downloads/Druti.app",
        "/private/var/folders/xy/T/AppTranslocation/1234/d/Druti.app",
        "/Users/someone/Library/Input Methods/Old/Druti.app",
        "/Users/other/Library/Input Methods/Druti.app",
    ])
    func copiesElsewhereBecomeTheInstaller(path: String) {
        #expect(!InstallLocation.isInstalled(bundle: URL(fileURLWithPath: path), home: home))
    }
}
