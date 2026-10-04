import Foundation

/// A key on the iOS keyboard (ios-keyboard spec, "Keyboard layout").
public enum KeyboardKey: Hashable, Sendable {
    /// Types this character, already in the case Shift gives it.
    case character(String)
    /// Shift: one tap for the next letter, a double tap for Caps Lock.
    case shift
    /// Backspace; holding it repeats.
    case backspace
    /// Space.
    case space
    /// Return.
    case returnKey
    /// The globe key, which switches to the next keyboard. Shown only when
    /// iOS asks for it (`needsInputModeSwitchKey`).
    case nextKeyboard
    /// Shows another layer of keys.
    case layer(KeyboardLayer)
    /// Opens the options panel (the output toggles).
    case options
}

/// A layer of keys, like the system keyboard's ABC, 123 and #+= pages.
public enum KeyboardLayer: Hashable, Sendable {
    /// Roman letters, which Druti types as Bengali.
    case letters
    /// Digits and common punctuation.
    case numbers
    /// More punctuation and symbols.
    case symbols
}

/// Whether letters are typed in upper case. Case matters in the phonetic
/// rules (`t` is ত, `T` is ট), so Shift never turns itself on.
public enum ShiftState: Hashable, Sendable {
    /// Lower case.
    case off
    /// Upper case for the next letter only.
    case once
    /// Upper case until Shift is tapped again (Caps Lock).
    case locked
}

/// Which keys the keyboard shows, and how they change as keys are pressed.
/// Free of UIKit so the rules can be unit-tested.
public struct KeyboardState: Hashable, Sendable {
    /// The layer on screen.
    public private(set) var layer = KeyboardLayer.letters
    /// The Shift state; always off outside the letters layer.
    public private(set) var shift = ShiftState.off

    /// The letters layer with Shift off.
    public init() {}

    /// The rows of keys, top to bottom. `showsNextKeyboard` adds the globe key.
    public func rows(showsNextKeyboard: Bool) -> [[KeyboardKey]] {
        let characterRows: [String]
        let thirdRowStart: KeyboardKey
        switch layer {
        case .letters:
            let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
            characterRows = shift == .off ? rows : rows.map { $0.uppercased() }
            thirdRowStart = .shift
        case .numbers:
            characterRows = ["1234567890", "-/:;()$&@\"", ".,?!'"]
            thirdRowStart = .layer(.symbols)
        case .symbols:
            characterRows = ["[]{}#%^*+=", "_\\|~<>`\u{09F3}\u{2022}\u{2026}", ".,?!'"]
            thirdRowStart = .layer(.numbers)
        }
        let characters = characterRows.map { row in row.map { KeyboardKey.character(String($0)) } }

        var bottom: [KeyboardKey] = [.layer(layer == .letters ? .numbers : .letters)]
        if showsNextKeyboard {
            bottom.append(.nextKeyboard)
        }
        bottom += [.options, .space, .returnKey]

        return [
            characters[0],
            characters[1],
            [thirdRowStart] + characters[2] + [.backspace],
            bottom,
        ]
    }

    /// Shift was tapped. A tap within the double-tap interval of the previous
    /// one turns on Caps Lock.
    public mutating func tapShift(isDoubleTap: Bool) {
        guard layer == .letters else {
            return
        }
        switch shift {
        case .off:
            shift = .once
        case .once:
            shift = isDoubleTap ? .locked : .off
        case .locked:
            shift = .off
        }
    }

    /// Shows `layer`, with Shift off.
    public mutating func show(_ layer: KeyboardLayer) {
        self.layer = layer
        shift = .off
    }

    /// A key was typed: a one-tap Shift ends after the letter, and Space on
    /// the numbers or symbols layer returns to the letters, as on the system
    /// keyboard.
    public mutating func didType(_ key: KeyboardKey) {
        switch key {
        case .character where shift == .once:
            shift = .off
        case .space where layer != .letters:
            show(.letters)
        default:
            break
        }
    }
}
