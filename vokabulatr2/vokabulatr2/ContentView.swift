//
//  ContentView.swift
//  vokabulatr2
//
//  Created by André Rösti on 4/2/25.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var model: VokabulatrModel
    @State var text: String = ""
    @State var selectedOption = "opt"
    var body: some View {
        HStack {
            VStack {
                List {
                    Text("hi")
                    Text("bye")
                }
                Picker("Selection", selection: $selectedOption) {
                    Text("All")
                    Text("Hardest")
                    Text("Custom")
                }
                .pickerStyle(SegmentedPickerStyle())
            }.padding()
            VStack {
                Text(model.front)
                TextField("Answer", text: $text)
                Button("Submit") {
                    
                }
            }.padding()
        }
    }
}

#Preview {
    ContentView()
        .environmentObject(VokabulatrModel())
}
