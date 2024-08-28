# F15 Key Simulator

F15 is a program that simulates the press of the F15 key on a Windows keyboard every 60 seconds.

*The F15 key code corresponds to the "Verr Num" (Num Lock) key on the keyboard.*


## Description

Welcome to **F15**! 😊 This project is designed to simulate keypresses of the Num Lock key at regular intervals. It is open source and available under the MIT license.
**Note: This program is designed for Windows only.**


## Use Cases

1. **Preventing Screen Lock**:
    - Use **F15** to simulate keypresses and prevent your computer from locking the screen due to inactivity.

2. **Presentation Mode**:
    - Use **F15** during presentations to prevent the screen from going to sleep or the screensaver from activating.

3. **Long-running Processes**:
    - Keep long-running processes active by simulating keypresses, ensuring they are not interrupted by system inactivity.

4. **Monitoring Dashboards**:
    - Use **F15** to keep monitoring dashboards (e.g., Grafana, Kibana) active and prevent them from timing out due to inactivity.

5. **Remote Session**:
    - Keep your remote desktop session active by simulating keypresses, ensuring you stay connected without interruptions.


## Features

- **Easy to use**: Simple and straightforward to set up.
- **Low Resource Usage**: Very low CPU and memory consumption.
- **High Performance**: Optimized for maximum performance.
- **Safe**: Open source code ensures transparency and security.
- **Extensible**: Easy to customize and extend.
- **Run at Startup**: Configure Windows to run **F15** at startup to ensure it is always active when your computer boots.


## Installation

To install this program, follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/nbigot/f15.git
    cd f15
    ```

2. Build the project:

    1. **Standard Build**:
        ```sh
        cargo build --release
        ```
        Using the default compilation settings, the **F15** program will not print anything to the console when running and the keypress frequency is 60 seconds.

    2. **Build for release but change the frequency to 10 seconds**:
        ```sh
        export SLEEP_DURATION=10
        cargo build --release
        ```
        Set the keypress frequency is 10 seconds.

    3. **Verbose Release Build**:
        ```sh
        cargo build --release --features "verbose"
        ```
        The program will output a message each time a simulated keystroke occurs.

4. Run the program:
    ```sh
    cd target/release
    ./f15
    ```

## Usage

Here is how to use **F15**:

1. Execute `f15.exe`
2. The program will simulate keypresses of the Num Lock key twice every 60 seconds.
3. Press Ctrl+C when you want to stop the program.


## License

This project is licensed under the MIT License.
