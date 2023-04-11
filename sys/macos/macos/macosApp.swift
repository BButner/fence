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
    @State var lastDeltaX: Double = 0;
    @State var lastDeltaY: Double = 0;
    
    init() {
        
    }
    
    func test_handler(event: NSEvent!) {
        let location = NSEvent.mouseLocation;
        let screen = activeScreen();
        
        if (screen != nil) {
            let deltaX = event.deltaX - lastDeltaX;
            let deltaY = event.deltaY - lastDeltaY;
            
            let newX = location.x + deltaX;
            let flippedY = screen!.frame.size.height - location.y;
            let newY = flippedY + deltaY;
            
            print(newY)
            
//            try_update_mouse_location(Int32(location.x), Int32(newY));
            
            let result = try_update_mouse_location(Int32(newX), Int32(newY));
            
            if (!result.updated) {
                print(result)
                print("should be setting it back to a valid point");
                CGWarpMouseCursorPosition(CGPoint(x: Double(result.location.x), y: Double(result.location.y)));
            } else {
                CGWarpMouseCursorPosition(CGPoint(x: newX, y: newY));
            }
            
            lastDeltaX = newX - location.x;
            lastDeltaY = newY - flippedY;
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
                
//                print(result);
                
//                currentNumber = result ? "1" : "0";
                currentNumber = "7";
                
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
