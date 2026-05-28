# My-Pety Project Journal
### :)



---

## My Thoughts on My-Pety
I wanted to build My-Pety because I think thi good for learning and prove my skills. I am still a buggner in coding but I wanted to challenge myself to make a desktop app that is actually useful and also fun. My-Pety is an AI-powered desktop pet that lives on your screen. It can chat, remember things, search the web, and run system tools like volume, brightness, remind you things, and even take screenshots and analyze your screen. 

When I first started this project, I was very nervous because I never did something this big before. But I knew it would be a very good way to learn Rust and desktop development.

---

## Finding the Right Tools & finaly Chosing Tauri
At the start, I spent so much time just trying to find how to create a desktop window that is transparent, borderless, and draggable. I did not know anything about this. The first time lib finding was so time consumed. I searched for so many GUI libraries on the web for rust, but everything was so hard to setup or looked very old.

Finally, I chossed Tauri. Tauri is really cool because it lets me use simple HTML, CSS, and Javascript for the UI, and write the main fast backend in Rust. But learning how Tauri communicate between Javascript and Rust backend took me a lot of days to understand because I never used tauri before.

---

## Logo Designing Struggles
Here is the logo I designed for my project:

![image](https://cdn.hackclub.com/019e6f1a-5fb4-74a6-917d-9da4ef5e643b/logo.png)

this is logo but this simplae and scribe style but i am buggner and first time disgning logo that why i am reserch so much and instaolled inkscape but i am using time not understand how to use it, so much things i am swatch to scribe style and windows own paint application used draw, this bad but i will try my best to make it better :).

When I installed Inkscape, there were so many tools, paths, nodes and grids that my head started spinning. I spent hours watching tutorials but I still could not make anything look right. That is why I just decided to open MS Paint and draw it by hand with my mouse in a simple scribe style. It looks a bit messy, but it has a nice handmade feeling to it.

---

## Biggest Rabbit Holes

### 1. The Volume Powershell Command Finding
One of the absolute worst rabbit holes was the volume tool in `volumetool.rs`. I wanted the pet to change the computer volume when the user type `volume 50`. I thought there would be a simple crate in Rust for this, but there was not. 

So I started searching how to do it with powershell. This volume powershell finding in web was so many time consuming. I had to read so many github and stack overflow and etc posts and forums. In the end, simple command did not work, so I had to write a C# script inside the Rust code and load it using `Add-Type` in PowerShell. The script uses low-level Windows COM interfaces like `IAudioEndpointVolume` and `IMMDeviceEnumerator` to control the master volume scalar. 

Here is the code I had to write:
```rust
    let script = format!(
        r#"
$code = @"
using System;
using System.Runtime.InteropServices;

[Guid("5CDF2C82-841E-4546-9722-0CF74078229A"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IAudioEndpointVolume {{
    ...
}}
...
public static class VolumeControl {{
    public static void SetVolume(float level) {{
        ...
        Marshal.ThrowExceptionForHR(volume.SetMasterVolumeLevelScalar(level, Guid.Empty));
    }}
}}
"@

Add-Type -TypeDefinition $code -Language CSharp
[VolumeControl]::SetVolume({scalar})
"#
    );
```
Getting all the GUIDs and interface definitions correct in C# inside a Rust string wrapper took me so many hours of trial and error.

### 2. Screenshot Window Minimize & Restore
Another big rabbit hole was the screenshot tool in `screenshottool.rs`. I wanted the pet to capture the screen and use Hack Club's AI proxy with the `nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free` model to tell the user what is on their screen. 

But there was a big problem: when you call the screenshot tool, the My-Pety window itself is showing on the screen, so it blocks the view and the screenshot just captures the pet window. 

To fix this, I had to minimize the window first. But that was not enough because the minimize animation has a small delay. If you capture immediately, the pet window is still half-faded on the screen. So I had to search how to minimize and unminimize the window properly. Finding how to minimize the window, wait for the animation, take screenshot, and then unminimize and show it back took so much time consumed.

I had to use this code:
```rust
    window.minimize().map_err(|e| e.to_string())?; // minimizing window

    tokio::time::sleep(std::time::Duration::from_millis(500)).await; // waiting for window to minimize

    let path = take_screenshot()?;

    window.unminimize().map_err(|e| e.to_string())?; // unminimizing window
```
Adding the 500 milliseconds sleep was the magic fix, but finding out why the screenshot was capturing the pet window took me so many hours of debugging.

### 3. Memory System and Optimizing
Adding the memory feature in `memory.rs` was also a lot of work. The memory is so much reserch went to add. I wanted the pet to remember user things (like their favorite programming language or their hobbies) even when they close and reopen the app. I had to research how to read and write to `memory.json` locally, and how to use the Rust `HashMap` to store and load them on startup. 

Also, optimizing is so time consuming. I had to make sure the app does not consume too much RAM or CPU, since it is a desktop pet that should sit quietly in the corner of the screen. I spent a lot of time testing the performance and cleaning up unused variables and managing requests carefully.

---

## Failures and Limitations
Even though I worked very hard, there are many failures in this project:

1. **Only 3 Pets Added**: I wanted to add a lot of pets, but because I spent so much time on the backend tools, I dont added more pets only added 3 pets (Ferris, dog, and cat). 
2. **Not Customizable**: This app connot costmizble and pet not costimzble. The user cannot change the size, animations, or styles easily from the UI yet.
3. **OS Compatibility Issues**: Some tools not work for other os like brightness and battery and volume etc. Because I used Windows powershell commands and Windows C# COM interfaces, these awesome tools will only work on Windows OS. If you run it on macOS or Linux, these features will fail. I did not have time to write native code for other operating systems.

---

## Conclusion
This project was a very big challenge for me but it was the best learning experience. Even though I got stuck in so many rabbit holes, wrote low level C# code inside Rust, and drew a terrible logo in MS Paint, I am really proud that the app works and I can chat with my pet. I hope this proves my skills :D



Thank you for reading my project journal :)
