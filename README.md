# My-Pety

Hello! This is My-Pety, a lightweight and cute AI-powered desktop pet and system assistant built using Rust and Tauri. 

I made this project for my Hack Club submission because I wanted to learn how to make desktop apps and practice my coding skills in Rust.

Here is the logo of the project:
![image](https://cdn.hackclub.com/019e6f1a-5fb4-74a6-917d-9da4ef5e643b/logo.png)

---

## What it does

My-Pety is not just a simple animation on your screen, it is a smart helper that sits in the corner of your screen as a transparent, borderless, and draggable window. It has a lot of cool features:

- **AI Chat Buddy**: You can chat with it about anything. It uses Hack Club's AI proxy with the `nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free` model. It responds like a friendly pet!
- **Screenshot Vision**: Tell the pet to "take screenshot" or "analyze screen". It will minimize its own window, wait a split second for the animation, take a screenshot of your screen, and explain what is happening on your screen using the AI vision model!
- **System Control (Windows)**: You can change your computer volume or brightness by typing `volume 50` or `brightness 80`. It uses low-level Windows integrations to make this happen.
- **Battery Info**: Type `battery` to see your current battery status and power state.
- **Memory System**: The pet can remember things about you! If you say `remember favorite language = rust` or `remember hobby = gaming`, it will save this to a local `memory.json` file and never forget.
- **Reminders**: Ask it to remind you something like `remind me 60 study rust` and it will trigger a reminder.
- **Open websites & apps**: Quickly open websites like `open youtube` or view your running apps by typing `open apps`.

---

## How to Setup & Run

### Prerequisites
To run this project, you need to have:
1. **Rust** installed on your computer.
2. **Windows** (since many of the low-level system integrations like volume and brightness are made with PowerShell and C# COM interfaces). but you can use other os

### Running the App
Since this app is built purely with Rust and Tauri (without complex Node.js dependencies), you can run it directly using Cargo!

1. Clone this repository.
2. Open terminal and navigate to the project folder.
3. Open `src-tauri` folder and run the cargo command:
   ```powershell
   cd src-tauri
   cargo run
   ```


or 
download the bin from github release and install it by your windows own setup application. [demo_app_download](https://github.com/cyberworrier8088/My-Pety/releases/download/Demo/web_app.exe)

4. The first time you open the app, it will show a setup screen where you can customize your pet's name, choose your pet asset (Ferris, dog, or cat), paste your Hack Club AI key, and set a password.

*Note: Please check the [m.md](file:///c:/Users/Muhammad_Nabhan_nk/Downloads/Project/My-Pety/m.md) file to read my detailed journal, design struggles, failures, and rabbit holes!*
