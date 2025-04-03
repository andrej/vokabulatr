//
//  vokabulatr2App.swift
//  vokabulatr2
//
//  Created by André Rösti on 4/2/25.
//

import SwiftUI

@main
struct vokabulatr2App: App {
    private var model = VokabulatrModel()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(model)
        }
    }
}
