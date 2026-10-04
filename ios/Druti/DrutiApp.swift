import SwiftUI

/// The app that carries the Druti keyboard. iOS installs a keyboard only as
/// part of an app; this one explains how to turn the keyboard on and gives a
/// field to try it in (ios-keyboard spec, "Container app").
@main
struct DrutiApp: App {
    var body: some Scene {
        WindowGroup {
            SetupView()
        }
    }
}
