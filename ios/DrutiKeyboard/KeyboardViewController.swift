import BengaliIMECore
import UIKit

/// The Druti keyboard. iOS creates one each time the keyboard appears in a text
/// field. All Bengali logic lives in the Rust `Composer`, through
/// `KeyboardSession`; this class connects the key views to the session and the
/// session to the app's text field.
///
/// The class name must match NSExtensionPrincipalClass in Info.plist.
final class KeyboardViewController: UIInputViewController {
    private let session = KeyboardSession(config: Settings.config)
    private var state = KeyboardState()
    private lazy var keyboardView = KeyboardView(controller: self)

    /// The app's text field, as `KeyboardSession` uses it.
    private var document: ProxyDocument { ProxyDocument(proxy: textDocumentProxy) }

    override func viewDidLoad() {
        super.viewDidLoad()
        keyboardView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(keyboardView)
        NSLayoutConstraint.activate([
            keyboardView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            keyboardView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            keyboardView.topAnchor.constraint(equalTo: view.topAnchor),
            keyboardView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])
        keyboardView.onKey = { [weak self] key in self?.press(key) }
        keyboardView.onOptionChange = { [weak self] option, isOn in
            Settings.set(option, to: isOn)
            self?.session.config = Settings.config
        }
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        session.config = Settings.config
        state = KeyboardState()
        redraw()
    }

    /// Apple's guidance is to read `needsInputModeSwitchKey` here, once the
    /// keyboard is in its window.
    override func viewWillLayoutSubviews() {
        super.viewWillLayoutSubviews()
        redraw()
    }

    /// The keyboard is going away: what is pending becomes final, like
    /// switching input sources on macOS.
    override func viewWillDisappear(_ animated: Bool) {
        session.commit(in: document)
        super.viewWillDisappear(animated)
    }

    /// The app changed the text or moved the caret (design D3).
    override func textDidChange(_ textInput: UITextInput?) {
        super.textDidChange(textInput)
        session.documentDidChange(document)
        redraw()
    }

    private func press(_ key: KeyboardKey) {
        switch key {
        case .character(let text):
            session.type(text, in: document)
        case .space:
            session.type(" ", in: document)
        case .returnKey:
            session.returnKey(in: document)
        case .backspace:
            session.backspace(in: document)
        case .shift:
            state.tapShift(isDoubleTap: keyboardView.isShiftDoubleTap())
        case .layer(let layer):
            state.show(layer)
        case .options:
            keyboardView.showOptions()
            return
        case .nextKeyboard:
            // The globe key is wired to handleInputModeList(from:with:) instead.
            return
        }
        state.didType(key)
        redraw()
    }

    private func redraw() {
        keyboardView.show(
            state, showsNextKeyboard: needsInputModeSwitchKey,
            isDark: textDocumentProxy.keyboardAppearance == .dark)
    }
}

/// `UITextDocumentProxy` as a `TextDocument`.
@MainActor
private struct ProxyDocument: TextDocument {
    let proxy: any UITextDocumentProxy

    var textBeforeCaret: String? { proxy.documentContextBeforeInput }

    func setMarkedText(_ text: String) {
        proxy.setMarkedText(text, selectedRange: NSRange(location: text.utf16.count, length: 0))
    }

    func unmarkText() {
        proxy.unmarkText()
    }

    func insertText(_ text: String) {
        proxy.insertText(text)
    }

    func deleteBackward() {
        proxy.deleteBackward()
    }
}
