import SwiftUI
import UIKit

/// How to turn on the keyboard, and a field to try it in.
struct SetupView: View {
    @Environment(\.openURL) private var openURL
    @State private var trial = ""

    var body: some View {
        NavigationStack {
            Form {
                Section {
                    Text(
                        "Type roman letters and get Bengali: ami banglay gan gai becomes \u{0986}\u{09AE}\u{09BF} \u{09AC}\u{09BE}\u{0982}\u{09B2}\u{09BE}\u{09DF} \u{0997}\u{09BE}\u{09A8} \u{0997}\u{09BE}\u{0987}."
                    )
                }

                Section("Turn on the keyboard") {
                    Text("1. Open Settings \u{2192} Druti \u{2192} Keyboards and turn on Druti.")
                    Text(
                        "Or: Settings \u{2192} General \u{2192} Keyboard \u{2192} Keyboards \u{2192} Add New Keyboard\u{2026} \u{2192} Druti."
                    )
                    Text("2. In any app, touch and hold \u{1F310} and choose Druti.")
                    Button("Open Settings") {
                        // The app's own page in Settings, where its keyboards are listed.
                        if let url = URL(string: UIApplication.openSettingsURLString) {
                            openURL(url)
                        }
                    }
                }

                Section("Try it") {
                    TextField("Type here", text: $trial, axis: .vertical)
                        .lineLimit(3...8)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                }

                Section("Privacy") {
                    Text(
                        "Druti doesn\u{2019}t ask for Full Access. It works without the network, and what you type stays on your device."
                    )
                }

                Section("Options") {
                    Text(
                        "Tap the gear key on the keyboard to turn Bengali digits, \u{09A6}\u{09BE}\u{0981}\u{09DC}\u{09BF} (\u{0964}) for full stop, smart quotes and Autocorrect on or off."
                    )
                }

                // The word list's CC BY-SA 4.0 licence asks for this attribution
                // wherever it ships (autocorrect design D7).
                Section("About") {
                    Text(
                        "Druti is free software under the MIT license. The Autocorrect word list is derived from the Dakshina dataset (Google Research, CC BY-SA 4.0) and from FrequencyWords by Hermit Dave (built from OpenSubtitles, CC BY-SA 4.0). It is licensed CC BY-SA 4.0."
                    )
                }
            }
            .navigationTitle("Druti")
        }
    }
}
