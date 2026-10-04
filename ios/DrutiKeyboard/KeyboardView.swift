import BengaliIMECore
import UIKit

/// The keys, drawn like the system keyboard, and the options panel. It only
/// draws `KeyboardState` and reports presses; what a key does is decided by
/// `KeyboardViewController` and the Rust engine.
final class KeyboardView: UIView {
    /// A key was pressed (or repeated, for a held Backspace).
    var onKey: ((KeyboardKey) -> Void)?
    /// An output toggle was switched in the options panel.
    var onOptionChange: ((Settings.Option, Bool) -> Void)?

    /// Receives the globe key's touches, as Apple requires for the
    /// keyboard-switching menu.
    private weak var controller: UIInputViewController?
    private let keys = UIStackView()
    private let options = OptionsView()
    private var shown: Appearance?
    private var lastShiftTap = Date.distantPast
    private var backspaceRepeat: Task<Void, Never>?

    private static let keyHeight: CGFloat = 42
    private static let rowSpacing: CGFloat = 11
    private static let keySpacing: CGFloat = 6
    /// A second Shift tap within this interval turns on Caps Lock.
    private static let doubleTapInterval: TimeInterval = 0.35

    init(controller: UIInputViewController) {
        self.controller = controller
        super.init(frame: .zero)

        keys.axis = .vertical
        keys.spacing = Self.rowSpacing
        keys.distribution = .fillEqually
        options.isHidden = true
        options.onChange = { [weak self] option, isOn in self?.onOptionChange?(option, isOn) }
        options.onDone = { [weak self] in self?.hideOptions() }

        for content in [keys, options] as [UIView] {
            content.translatesAutoresizingMaskIntoConstraints = false
            addSubview(content)
            NSLayoutConstraint.activate([
                content.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 3),
                content.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -3),
                content.topAnchor.constraint(equalTo: topAnchor, constant: 8),
                content.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -4),
            ])
        }
        // Four rows of keys set the keyboard's height; the options panel uses
        // the same space.
        keys.heightAnchor.constraint(
            equalToConstant: 4 * Self.keyHeight + 3 * Self.rowSpacing
        ).isActive = true
    }

    @available(*, unavailable, message: "Created in code only")
    required init?(coder: NSCoder) {
        return nil
    }

    /// Draws `state`. Does nothing when nothing changed, so a held Backspace
    /// keeps its button (and its touch) while it repeats.
    func show(_ state: KeyboardState, showsNextKeyboard: Bool, isDark: Bool) {
        let appearance = Appearance(
            state: state, showsNextKeyboard: showsNextKeyboard, isDark: isDark)
        guard appearance != shown else {
            return
        }
        shown = appearance
        stopBackspaceRepeat()
        overrideUserInterfaceStyle = isDark ? .dark : .unspecified

        for row in keys.arrangedSubviews {
            row.removeFromSuperview()
        }
        let rows = state.rows(showsNextKeyboard: showsNextKeyboard)
        for (index, row) in rows.enumerated() {
            keys.addArrangedSubview(rowView(row, index: index, of: rows.count, state: state))
        }
    }

    /// Whether this Shift tap follows the previous one closely enough to be a
    /// double tap.
    func isShiftDoubleTap() -> Bool {
        let now = Date()
        defer { lastShiftTap = now }
        return now.timeIntervalSince(lastShiftTap) < Self.doubleTapInterval
    }

    /// Replaces the keys with the options panel.
    func showOptions() {
        options.refresh()
        keys.isHidden = true
        options.isHidden = false
    }

    private func hideOptions() {
        options.isHidden = true
        keys.isHidden = false
    }

    // MARK: - Rows and keys

    /// One row. Widths are in key units, ten to a row, like the system
    /// keyboard: the second letter row is inset by half a key on each side,
    /// and Space takes what the bottom row leaves.
    private func rowView(_ row: [KeyboardKey], index: Int, of count: Int, state: KeyboardState)
        -> UIStackView
    {
        let stack = UIStackView()
        stack.axis = .horizontal
        stack.spacing = Self.keySpacing
        stack.alignment = .fill

        let inset = state.layer == .letters && index == 1
        if inset {
            stack.addArrangedSubview(spacer())
        }
        let isBottom = index == count - 1
        let isPunctuation = index == 2 && state.layer != .letters
        for key in row {
            let button = keyButton(key, state: state)
            stack.addArrangedSubview(button)
            if let units = width(of: key, isBottom: isBottom, isPunctuation: isPunctuation) {
                let width = button.widthAnchor.constraint(
                    equalTo: stack.widthAnchor, multiplier: units / 10, constant: -Self.keySpacing)
                // Just below required, so rounding never makes the row
                // unsatisfiable, but above the buttons' own size.
                width.priority = UILayoutPriority(999)
                width.isActive = true
            } else {
                button.setContentHuggingPriority(.defaultLow, for: .horizontal)
            }
        }
        if inset {
            stack.addArrangedSubview(spacer())
        }
        if let first = stack.arrangedSubviews.first, inset {
            first.widthAnchor.constraint(equalTo: stack.widthAnchor, multiplier: 0.05)
                .isActive = true
            stack.arrangedSubviews.last?.widthAnchor.constraint(equalTo: first.widthAnchor)
                .isActive = true
        }
        return stack
    }

    /// A key's width in key units, or nil for Space, which fills the rest.
    private func width(of key: KeyboardKey, isBottom: Bool, isPunctuation: Bool) -> CGFloat? {
        switch key {
        case .space:
            return nil
        case .returnKey:
            return 2.2
        case .layer, .nextKeyboard, .options:
            return isBottom ? 1.25 : 1.4
        case .shift, .backspace:
            return 1.4
        case .character:
            // The punctuation row of the numbers and symbols layers has five
            // wider keys, as on the system keyboard.
            return isPunctuation ? 1.4 : 1
        }
    }

    private func spacer() -> UIView {
        let view = UIView()
        view.isUserInteractionEnabled = false
        return view
    }

    private func keyButton(_ key: KeyboardKey, state: KeyboardState) -> KeyButton {
        let button = KeyButton(key: key, isFunctionKey: Self.isFunctionKey(key))
        switch key {
        case .character(let text):
            button.setTitle(text, for: .normal)
            button.titleLabel?.font = .systemFont(ofSize: 24)
        case .space:
            button.setTitle("space", for: .normal)
        case .returnKey:
            button.setTitle("return", for: .normal)
        case .layer(let layer):
            button.setTitle(Self.title(of: layer), for: .normal)
        case .shift:
            let symbol =
                switch state.shift {
                case .off: "shift"
                case .once: "shift.fill"
                case .locked: "capslock.fill"
                }
            button.setImage(UIImage(systemName: symbol), for: .normal)
            button.accessibilityLabel = "Shift"
        case .backspace:
            button.setImage(UIImage(systemName: "delete.left"), for: .normal)
            button.accessibilityLabel = "Delete"
        case .nextKeyboard:
            button.setImage(UIImage(systemName: "globe"), for: .normal)
            button.accessibilityLabel = "Next keyboard"
        case .options:
            button.setImage(UIImage(systemName: "gearshape"), for: .normal)
            button.accessibilityLabel = "Druti options"
        }

        switch key {
        case .nextKeyboard:
            if let controller {
                button.addTarget(
                    controller,
                    action: #selector(UIInputViewController.handleInputModeList(from:with:)),
                    for: .allTouchEvents)
            }
        case .backspace:
            button.addTarget(self, action: #selector(backspaceDown), for: .touchDown)
            button.addTarget(
                self, action: #selector(backspaceUp),
                for: [.touchUpInside, .touchUpOutside, .touchCancel])
        default:
            button.addTarget(self, action: #selector(keyUp(_:)), for: .touchUpInside)
        }
        return button
    }

    private static func isFunctionKey(_ key: KeyboardKey) -> Bool {
        if case .character = key {
            return false
        }
        return key != .space
    }

    private static func title(of layer: KeyboardLayer) -> String {
        switch layer {
        case .letters: "ABC"
        case .numbers: "123"
        case .symbols: "#+="
        }
    }

    @objc private func keyUp(_ sender: KeyButton) {
        onKey?(sender.key)
    }

    /// Backspace deletes once on touch down, then repeats while held.
    @objc private func backspaceDown() {
        onKey?(.backspace)
        stopBackspaceRepeat()
        backspaceRepeat = Task { [weak self] in
            var delay = Duration.milliseconds(500)
            // Task.sleep throws only when the task is cancelled (the key was
            // released), which ends the loop.
            while (try? await Task.sleep(for: delay)) != nil {
                self?.onKey?(.backspace)
                delay = .milliseconds(100)
            }
        }
    }

    @objc private func backspaceUp() {
        stopBackspaceRepeat()
    }

    private func stopBackspaceRepeat() {
        backspaceRepeat?.cancel()
        backspaceRepeat = nil
    }
}

/// What `KeyboardView.show` last drew.
private struct Appearance: Equatable {
    var state: KeyboardState
    var showsNextKeyboard: Bool
    var isDark: Bool
}

/// One key: a rounded button in the system keyboard's colours, darker for
/// function keys, highlighted while pressed.
private final class KeyButton: UIButton {
    let key: KeyboardKey
    private let isFunctionKey: Bool

    init(key: KeyboardKey, isFunctionKey: Bool) {
        self.key = key
        self.isFunctionKey = isFunctionKey
        super.init(frame: .zero)
        layer.cornerRadius = 5
        layer.shadowColor = UIColor.black.cgColor
        layer.shadowOpacity = 0.3
        layer.shadowRadius = 0
        layer.shadowOffset = CGSize(width: 0, height: 1)
        titleLabel?.font = .systemFont(ofSize: 16)
        titleLabel?.adjustsFontSizeToFitWidth = true
        setTitleColor(.label, for: .normal)
        tintColor = .label
        updateBackground()
    }

    @available(*, unavailable, message: "Created in code only")
    required init?(coder: NSCoder) {
        return nil
    }

    override var isHighlighted: Bool {
        didSet { updateBackground() }
    }

    private func updateBackground() {
        backgroundColor = isFunctionKey != isHighlighted ? Self.functionColor : Self.characterColor
    }

    private static let characterColor = UIColor { traits in
        traits.userInterfaceStyle == .dark ? UIColor(white: 0.42, alpha: 1) : .white
    }

    private static let functionColor = UIColor { traits in
        traits.userInterfaceStyle == .dark
            ? UIColor(white: 0.27, alpha: 1)
            : UIColor(red: 0.67, green: 0.69, blue: 0.73, alpha: 1)
    }
}

/// The toggles, shown in place of the keys (design D5). They are the same four
/// as in the macOS input menu.
private final class OptionsView: UIView {
    var onChange: ((Settings.Option, Bool) -> Void)?
    var onDone: (() -> Void)?

    private var switches: [(option: Settings.Option, control: UISwitch)] = []

    override init(frame: CGRect) {
        super.init(frame: frame)
        let stack = UIStackView()
        stack.axis = .vertical
        stack.spacing = 8
        stack.translatesAutoresizingMaskIntoConstraints = false
        addSubview(stack)
        NSLayoutConstraint.activate([
            stack.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 12),
            stack.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -12),
            stack.centerYAnchor.constraint(equalTo: centerYAnchor),
        ])

        for option in Settings.Option.allCases {
            let label = UILabel()
            label.text = Self.title(of: option)
            label.adjustsFontSizeToFitWidth = true
            let control = UISwitch()
            control.addTarget(self, action: #selector(switched(_:)), for: .valueChanged)
            switches.append((option, control))
            let row = UIStackView(arrangedSubviews: [label, control])
            row.spacing = 8
            stack.addArrangedSubview(row)
        }

        let done = UIButton(type: .system)
        done.setTitle("Done", for: .normal)
        done.titleLabel?.font = .boldSystemFont(ofSize: 17)
        done.addTarget(self, action: #selector(doneTapped), for: .touchUpInside)
        stack.addArrangedSubview(done)
    }

    @available(*, unavailable, message: "Created in code only")
    required init?(coder: NSCoder) {
        return nil
    }

    /// Shows the stored toggles.
    func refresh() {
        for (option, control) in switches {
            control.isOn = Settings.isOn(option)
        }
    }

    private static func title(of option: Settings.Option) -> String {
        switch option {
        case .bengaliDigits:
            "Bengali Digits (\u{09E7}\u{09E8}\u{09E9})"
        case .dariForPeriod:
            // দাঁড়ি, written with escapes so editors can't decompose ড়.
            "\u{09A6}\u{09BE}\u{0981}\u{09DC}\u{09BF} (\u{0964}) for Full Stop"
        case .smartQuotes:
            "Smart Quotes (\u{201C} \u{201D})"
        case .autocorrect:
            "Autocorrect"
        }
    }

    @objc private func switched(_ sender: UISwitch) {
        guard let option = switches.first(where: { $0.control === sender })?.option else {
            return
        }
        onChange?(option, sender.isOn)
    }

    @objc private func doneTapped() {
        onDone?()
    }
}
