# :keyboard: ctrlfreak: control your Ctrl

Welcome to **ctrlfreak**, a lightweight utility tool engineered with the purpose of fetching the state of modifier keys like Alt and Ctrl on your computer. We've all been there. It's a common occurrence where your operating system or environment gets tricked into believing a modifier key is pressed, when in reality, it's not. As a result, you may find that the characters you type aren't what you expected, leading to a confusing user experience. This is an especially prevalent issue for users who leverage advanced features like sticky modifiers with keyboard firmware such as QMK. **ctrlfreak** can be run as a module for status bars like Polybar, providing a quick and convenient method to monitor and rectify your modifier state. The current implementation supports environments using an X server.

## :bulb: Understanding ctrlfreak

**ctrlfreak** operates by monitoring changes to the state of your keyboard's modifier keys, outputting a new line with each state change. The output line format matches the regular expression `^(a|A)(c|C)(s|S)(w|W)$`, where an uppercase letter corresponds to an active modifier, and a lowercase letter signifies an inactive modifier. The letters 'a', 'c', 's', and 'w' represent the Alt, Ctrl, Shift, and Win/Super keys respectively. This efficient mechanism enables **ctrlfreak** to provide real-time updates about your modifier key activity. The current implementation supports environments using an X server.
## :gear: Installation Process

**ctrlfreak** is built using the Rust programming language, so you'll need to have the Rust toolchain installed on your system. Once that's in place, **ctrlfreak** can be installed using the `cargo install` command. If you're unfamiliar with the Rust environment, don't worry! There are ample resources and tutorials available online to help you get started.

## :jigsaw: Integrating ctrlfreak with Polybar

Incorporating **ctrlfreak** into Polybar is straightforward. You'll need to add the following custom module definition to your Polybar configuration (ini) file:

```ini
[module/ctrlfreak]
type = custom/script
exec = ctrlfreak
tail = true
```

ctrlfreak can then be added to your bars. With this simple addition, Polybar will now have the capabilities of **ctrlfreak**. Your status bar will offer an accurate reflection of the modifier key states, eliminating confusion and enhancing your computing experience.

So give **ctrlfreak** a try! It's a tool designed to keep your modifier key status transparent and manageable, aiding in a more streamlined and enjoyable computing experience. Happy coding!
