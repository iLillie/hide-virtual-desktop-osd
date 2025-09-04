# Hide Virtual Desktop OSD

Hide Virtual Desktop OSD is a background running program that hides the on-screen display on Windows 11 that appears when you switch virtual desktops.

It also makes sure that you can still see volume, and brightness slider after it has finished the virtual desktop popup.

## How do I use this?

### Release

Download the exe file, and run it.
It will start as background process, and a tray icon should appear.
(I assume Windows might complain about unsafe code)

Then move it away from Downloads folder to store it somewhere else.
You need to run it after each restart unless you add it as a startup app. 

### Building from soure code

If you are not sure about the release .exe, you can build it yourself.

1. Follow [Cargo installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Run `cargo build --release`
3. Build file should be under `target/release/hide-virtual-desktop-osd.exe`


## Why I made this?

I have used MacOS for a while now, and recently got a windows laptop.
When I tried out Virtual Desktop on Windows it was quite annoying seeing a the on-screen display popup when you switch desktops.

I never found a solution to disable it, however I found [HideVolumeOSD](https://github.com/UnlimitedStack/HideVolumeOSD). It was a solution, but 
there wasn't any official releases, and I'm skeptical to a random setup file they have on a different website. And I believe it hides the volume OSD which I don't mind.

So I decided to make this, as I only wanted the Virtual Desktop OSD disabled.

## How does it work?

### Simple

The program is running in the background, waiting for when Virtual Desktop is switching desktops.
When a change happens, it will hide the on-screen display. It will be hidden until there have been 2.5 seconds since last change happened.
This is to make sure you can still see the volume, and brightness on-screen display.

**NOTE: The behaviour won't be flawless, but works good enough for myself**

### Technical

1. Finds XamlExplorerHostIslandWindow and the first child with window title "DesktopWindowXamlSource" using win32 API.
2. Update the window state to be visible (as it should be the default state)
3. I setup a listener for Virtual Desktop events in a new thread
4. When a event is triggered this will happen
  1. Update window state to be hidden
  2. Add event to Debouncer
5. After 2.5 seconds since last event in the debouncer it will update window state to be visible
6. "Event loop" to make sure the program does not close. 

## How can I help?

Would be amazing to get help on how to only hide the Virtual Desktop OSD without the debouncing.

If you are willing to help out then I can recommend looking into [XAML Islands](https://blogs.windows.com/windowsdeveloper/2018/11/02/xaml-islands-a-deep-dive-part-1/), or if you can find a way to get a property with win32 API that can uniquely identify the Virtual Desktop OSD.

# Showcase

## Before

https://github.com/user-attachments/assets/06e2672d-c8ed-4a92-b024-08fb71e93893

## After

https://github.com/user-attachments/assets/2c7cc86f-79ff-4369-9b8c-e1c06d052067




