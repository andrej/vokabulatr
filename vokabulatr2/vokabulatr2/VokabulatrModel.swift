//
//  VokabulatrModel.swift
//  vokabulatr2
//
//  Created by André Rösti on 4/2/25.
//

import Foundation

class VokabulatrModel : ObservableObject {
    @Published var front: String = "front";
    @Published var back: String = "back";
}
