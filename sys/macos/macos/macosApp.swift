//
//  macosApp.swift
//  macos
//
//  Created by Beau Butner on 4/4/23.
//

import SwiftUI
import AppKit

func cgEventCallback(proxy: CGEventTapProxy, type: CGEventType, event: CGEvent, refcon: UnsafeMutableRawPointer?) -> Unmanaged<CGEvent>? {
    let result = try_update_mouse_location(Int32(event.location.x), Int32(event.location.y));
    
    if !result.updated {
        print("returning nil for event \(type)")
        return nil;
    }
    
    return Unmanaged.passUnretained(event);
}

@main
struct macosApp: App {
    @State var currentNumber: String = "9";
    @State var currentTap: CFMachPort? = nil;
    
    var body: some Scene {
        MenuBarExtra(currentNumber, systemImage: "\(currentNumber).square") {
            Button("Init Fence") {
                let result = init_fence();
                
                print(result);
                
                let eventMask =
                (1 << CGEventType.mouseMoved.rawValue)
                | (1 << CGEventType.leftMouseDown.rawValue)
                | (1 << CGEventType.leftMouseUp.rawValue)
                | (1 << CGEventType.leftMouseDragged.rawValue)
                | (1 << CGEventType.rightMouseDown.rawValue)
                | (1 << CGEventType.rightMouseUp.rawValue)
                | (1 << CGEventType.rightMouseDragged.rawValue)
                | (1 << CGEventType.scrollWheel.rawValue)
                
                currentTap = CGEvent.tapCreate(
                    tap: .cghidEventTap,
                    place: .headInsertEventTap,
                    options: .defaultTap,
                    eventsOfInterest: CGEventMask(eventMask),
                    callback: cgEventCallback,
                    userInfo: nil
                );
                
                if (currentTap == nil) {
                    print("Failed to initialize Tap...");
                    return;
                }
                                
                let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, currentTap, 0);
                CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes);
                
                currentNumber = result ? "1" : "0";
            }
        }
    }
}
