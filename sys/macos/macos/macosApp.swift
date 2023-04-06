//
//  macosApp.swift
//  macos
//
//  Created by Beau Butner on 4/4/23.
//

import SwiftUI
import AppKit

@main
struct macosApp: App {
    @State var currentNumber: String = "9"
    
    init() {
        
    }
    
    func test_handler(event: NSEvent!) {
        let location = NSEvent.mouseLocation;
        let screen = activeScreen();
        if (screen != nil) {
            let offsetY = screen!.frame.height - location.y;
            update_mouse_location(Int32(location.x), Int32(offsetY));
        }
    }
    
    func activeScreen() -> NSScreen? {
        let mouseLocation = NSEvent.mouseLocation
        let screens = NSScreen.screens
        let screenWithMouse = (screens.first { NSMouseInRect(mouseLocation, $0.frame, false) })

        return screenWithMouse
    }
    
    var body: some Scene {
//        WindowGroup {
//            ContentView()
//        }
        MenuBarExtra(currentNumber, systemImage: "\(currentNumber).square") {
            Button("Init Server") {
                let result = init_fence();
                
                print(result);
                
                currentNumber = result ? "1" : "0";
                
                NSEvent.addGlobalMonitorForEvents(matching: NSEvent.EventTypeMask.mouseMoved, handler: test_handler);
            }
            Button("Test Rust") {
                currentNumber = "2"
                
            }
            Button("Three") {
                currentNumber = "3"
                
            }
        }
    }
}
