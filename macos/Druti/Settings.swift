import BengaliIMECore
import Foundation

/// The input menu toggles, stored in UserDefaults (the input method's own
/// preferences domain), so they survive restarts and upgrades. The output
/// options are on by default, Autocorrect is off.
///
/// UserDefaults is the only copy: nothing is cached here, so a toggle made in
/// one text field reaches every other input controller on its next key.
enum Settings {
    /// One toggle; the raw value is its UserDefaults key.
    enum Option: String {
        case bengaliDigits
        case dariForPeriod
        case smartQuotes
        case autocorrect

        /// The value until the toggle is first used. Autocorrect is off, also
        /// after an upgrade from a version without it (autocorrect design D6).
        var defaultValue: Bool { self != .autocorrect }
    }

    private static var defaults: UserDefaults { .standard }

    static func isOn(_ option: Option) -> Bool {
        defaults.object(forKey: option.rawValue) as? Bool ?? option.defaultValue
    }

    static func toggle(_ option: Option) {
        defaults.set(!isOn(option), forKey: option.rawValue)
    }

    static var config: Config {
        Config(
            bengaliDigits: isOn(.bengaliDigits),
            dariForPeriod: isOn(.dariForPeriod),
            smartQuotes: isOn(.smartQuotes),
            autocorrect: isOn(.autocorrect))
    }
}
