import Foundation

/// The toggles (Bengali digits, দাঁড়ি, smart quotes, Autocorrect), stored in
/// the process's own UserDefaults so they survive restarts and upgrades. All
/// on by default, Autocorrect included: the engine's `defaultConfig()` keeps
/// Autocorrect off for the fixtures and the playground, but the Mac and iOS
/// apps turn it on (autocorrect design D6, as amended).
///
/// The macOS input menu and the iOS keyboard's options panel both use this
/// (apple-silicon-and-ios-keyboard design D5). Each stores its toggles in its
/// own preferences domain: the iOS keyboard extension can't share the app's
/// without an App Group, which free Apple accounts can't use.
///
/// UserDefaults is the only copy: nothing is cached here, so a toggle made in
/// one text field reaches every other session on its next key.
public enum Settings {
    /// One toggle; the raw value is its UserDefaults key.
    public enum Option: String, CaseIterable, Sendable {
        /// `1` types `১`.
        case bengaliDigits
        /// `.` types `।`.
        case dariForPeriod
        /// `"` and `'` type typographic quotes.
        case smartQuotes
        /// A finished word is corrected from the Autocorrect list.
        case autocorrect

        /// The value until the toggle is first used, also after an upgrade
        /// from a version without it.
        public var defaultValue: Bool { true }
    }

    private static var defaults: UserDefaults { .standard }

    /// Whether `option` is on.
    public static func isOn(_ option: Option) -> Bool {
        defaults.object(forKey: option.rawValue) as? Bool ?? option.defaultValue
    }

    /// Turns `option` on or off.
    public static func set(_ option: Option, to isOn: Bool) {
        defaults.set(isOn, forKey: option.rawValue)
    }

    /// Flips `option`.
    public static func toggle(_ option: Option) {
        set(option, to: !isOn(option))
    }

    /// The toggles as the engine's options.
    public static var config: Config {
        Config(
            bengaliDigits: isOn(.bengaliDigits),
            dariForPeriod: isOn(.dariForPeriod),
            smartQuotes: isOn(.smartQuotes),
            autocorrect: isOn(.autocorrect))
    }
}
