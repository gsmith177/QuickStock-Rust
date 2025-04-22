# QuickStock-Rust



This project is for CS4340 - Software Maintenance at UCCS. We chose to change an open source github project from Python to Rust (and some other things).


## Dependencies
Must have Rust downloaded, all other dependancies should be installed when running the program. To see all dependencies for this app, consult the cargo.toml file.

**Download Rust here** (either 32 or 64 bit versions should work): https://www.rust-lang.org/tools/install

Use the default installation for this app.

After installation, restart your IDE (we are using VScode). Then move on to the run instructions.


## Run Backend Instructions
To compose the proram on your machiene, go to the terminal and execute these commands:
```
cargo build
```
```
cargo run
```

To terminate the program close the window normally (the X button in the top corner or the window) or terminate the process in the terminal (ctrl + C).

## Run Frontend Instructions
Firstly, make sure you have Node.js installed. You can check this by executing the following command in your terminal:
```
node -v
```
If Node.js is not installed, you can install it at https://nodejs.org/en.

Execute the following commands to install dependencies:
```
npm install react-router-dom
```

Open a terminal and change the directory to the "Frontend" folder (The path should be C:\...\QuickStock-Rust\Frontend\frontend\new-frontendS) in the QuickStock-Rust project folder. Execute the following command:
```
npm start
```
If the window does not automatically navigate to your localhost, enter "http://localhost:3000/" into your browser of choice.

To terminate the program close the window normally (the X button in the top corner or the window) or terminate the process in the terminal (ctrl + C).

## Other Documentation
This is some other documentation that we found useful for this project:

**Rust documentation:** https://doc.rust-lang.org/stable/

**React documentation:** https://www.google.com/search?client=opera-gx&q=react+dosumentation&sourceid=opera&ie=UTF-8&oe=UTF-8

## Original Project
Here is the original project: https://github.com/Hamilton-Davis/Inventory-Intro-to-Software-Dev
