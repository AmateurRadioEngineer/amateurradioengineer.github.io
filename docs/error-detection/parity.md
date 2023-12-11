# Understanding Parity Checks: The Basics of Error Detection

## Introduction

In the world of digital communications, ensuring the accuracy of transmitted data is paramount. One of the simplest and earliest methods developed for error detection is the parity check. This technique, though basic, lays the foundational principles for more advanced error detection and correction methods.

## What is a Parity Check?

A parity check is a method used to detect errors in digital data. It involves adding an extra bit, known as the parity bit, to a sequence of binary data. The value of this bit is set so that the total number of 1s in the data, including the parity bit, is even (for even parity) or odd (for odd parity).

## How Does it Work?

- Even Parity: If the number of 1s in the data is odd, the parity bit is set to 1, making the total count of 1s even. If it's already even, the parity bit is set to 0.
- Odd Parity: Conversely, if the number of 1s in the data is even, the parity bit is set to 1 to make the total count odd. If the count is already odd, the parity bit is set to 0.

## Parity Check in Action

Consider a simple binary sequence, `1011`. Under even parity, this sequence would have a parity bit of 1 (making `10111`), as the number of 1s (three) is odd. Under odd parity, the parity bit would be 0 (making `10110`), as it already has an odd number of 1s.

## Detecting Errors

When data is received, a parity check is performed by counting the number of 1s. If the count does not match the expected parity (even or odd), it indicates that an error has occurred during transmission.

## Limitations

While parity checks can detect single-bit errors, they cannot detect all types of errors (like when two bits are flipped) and do not offer a way to correct any detected errors.

## Real-World Usage

- RS-232 Communications: Widely used in older computer serial ports for external device communication.
- Microcontroller to Peripheral Communication: In embedded systems, where microcontrollers communicate with sensors or other devices over a serial interface.
- Industrial and Network Equipment: Many industrial machines and network devices use serial communication for configuration or data logging, with parity checks ensuring the reliability of these transmissions.

In these applications, parity checks provide a simple yet effective method for detecting transmission errors. While they are limited to detecting only certain types of errors (primarily single-bit errors), their simplicity makes them suitable for many serial communication scenarios where bandwidth is limited, and the overhead of more complex error detection and correction methods is not justified.

## Challenges

While the following challenges are trivial enough to perform without any tool, I would recommend you create an automated tool to perform this check. It will not be used for later, but it can be a good way to make sure you're setup for success.

### Challenge 1: Identifying Errors with Parity Checks

This challenge involves analyzing binary messages with their respective parity bits to determine their integrity. For each given message, assuming **even parity**, decide whether it is "Valid" or "Corrupt". This exercise tests your ability to apply parity check principles to detect transmission errors in binary data.

#### Challenge Data

The format is CSV with the first column being the binary data as a string and the second column is the parity check bit.

[Download challenge data](./parity-challenge-1.csv)

<details>
    <summary>Show challenge data</summary>

  ```csv
  data,parity_bit
  00111010,0
  10011010,0
  00100000,1
  00100100,1
  01111010,0
  00101000,0
  01000100,1
  10101101,1
  01001110,0
  01101011,1
  00010001,1
  11101000,0
  11110011,0
  01100101,1
  11011010,0
  00000101,1
  00110001,0
  01001010,1
  11001010,0
  01111111,1
  00110110,1
  10100001,0
  10000000,1
  01001101,1
  01001011,1
  00100100,0
  00111000,0
  10011000,0
  11111000,1
  00011001,1
  01001000,0
  01001010,1
  ```

</details>

#### Solution

The format is CSV with the first column being the binary data as a string, the second column is the parity check bit, and the third column is whether the parity check is valid or corrupt.

[Download solution](./parity-solution-1.csv)

<details>
  <summary>Show solution</summary>

  ```csv
  data,parity_bit,validity
  00111010,0,Valid
  10011010,0,Valid
  00100000,1,Valid
  00100100,1,Corrupt
  01111010,0,Corrupt
  00101000,0,Valid
  01000100,1,Corrupt
  10101101,1,Valid
  01001110,0,Valid
  01101011,1,Valid
  00010001,1,Corrupt
  11101000,0,Valid
  11110011,0,Valid
  01100101,1,Corrupt
  11011010,0,Corrupt
  00000101,1,Corrupt
  00110001,0,Corrupt
  01001010,1,Valid
  11001010,0,Valid
  01111111,1,Valid
  00110110,1,Corrupt
  10100001,0,Corrupt
  10000000,1,Valid
  01001101,1,Corrupt
  01001011,1,Corrupt
  00100100,0,Valid
  00111000,0,Corrupt
  10011000,0,Corrupt
  11111000,1,Valid
  00011001,1,Valid
  01001000,0,Valid
  01001010,1,Valid
  ```

</details>

### Challenge 2: Determining the Correct Parity Bit

In this challenge, you are provided with a series of binary data sequences. Your task is to calculate and append the appropriate parity bit to each sequence, assuming **odd parity**. This challenge assesses your understanding of how parity bits are used to maintain data integrity in digital communication systems.

#### Challenge Data

The format is CSV with the first column being the binary data as a string.

[Download challenge data](./parity-challenge-2.csv)

<details>
    <summary>Show challenge data</summary>

  ```csv
  data
  00011110
  11110100
  01101110
  00000010
  10001111
  00010011
  01110011
  00101111
  11000000
  10010100
  11101010
  10110110
  11011101
  11001110
  10011101
  01001010
  11000010
  10001000
  11010111
  10001110
  00010101
  00010101
  01000111
  01100010
  10010011
  00100010
  01011101
  10101110
  10110111
  11011010
  10101010
  10100010
  ```

</details>

#### Solution

The format is CSV with the first column being the binary data as a string and the second column is the parity check bit.

[Download solution](./parity-solution-2.csv)

<details>
  <summary>Show solution</summary>

  ```csv
  data,parity_bit
  00011110,1
  11110100,0
  01101110,0
  00000010,0
  10001111,0
  00010011,0
  01110011,0
  00101111,0
  11000000,1
  10010100,0
  11101010,0
  10110110,0
  11011101,1
  11001110,0
  10011101,0
  01001010,0
  11000010,0
  10001000,1
  11010111,1
  10001110,1
  00010101,0
  00010101,0
  01000111,1
  01100010,0
  10010011,1
  00100010,1
  01011101,0
  10101110,0
  10110111,1
  11011010,0
  10101010,1
  10100010,0
  ```

</details>
