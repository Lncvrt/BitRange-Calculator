# BitRange Calculator

Simple command-line tool written in Rust that calculates and formats the maximum, signed, and unsigned ranges of integers based on a given bit size. It helps to understand the range of values that can be represented in binary for a specified number of bits.

## Features

-   Calculate the maximum value for an unsigned integer based on the number of bits.
-   Calculate the signed integer range (both positive and negative) based on the number of bits.
-   Format large numbers with commas for easy readability.
-   Supports flexible bit sizes (positive integers only).

## Download

You can download pre-compiled executables here: [Click to go to the Releases Tab](https://github.com/Lncvrt/BitRange-Calculator/releases/latest)

## Usage

The program prompts you to enter the number of bits. Based on the input, it calculates the following:

-   **Maximum value for an unsigned integer**.
-   **Formatted maximum value for an unsigned integer** with commas for readability.
-   **Signed integer range** showing the minimum and maximum values.
-   **Formatted signed range** with commas for readability.

Examples:
Note: These are outputs from the app.

16-Bit Calculation:

```
Max value for a 16-bit unsigned integer: 65535 (65,535)
Signed range for a 16-bit integer: -32768 to 32767 (-32,768 to 32,767)
```

32-Bit Calculation:

```
Max value for a 32-bit unsigned integer: 4294967295 (4,294,967,295)
Signed range for a 32-bit integer: -2147483648 to 2147483647 (-2,147,483,648 to 2,147,483,647)
```

64-Bit Calculation:

```
Max value for a 64-bit unsigned integer: 18446744073709551615 (18,446,744,073,709,551,615)
Signed range for a 64-bit integer: -9223372036854775808 to 9223372036854775807 (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
```
