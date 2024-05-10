# Stat

Perform statistical analysis on a set of floating-point numbers provided as
input.

## Building

With `rustc` and `gcc` in PATH, run

```bash
rustc main.rs
```

## Usage

Once the program is running, you can start typing numbers separated by spaces or
newlines. For example: `1.2 3.4 5.6 7.8 9.0`

After entering all your numbers, you can signal the end of input with e.g.
CTRL+D on Linux and MacOS.

- Input: the program reads numbers from the standard input (stdin). You can
  provide input by typing numbers separated by spaces or newlines directly into
  the terminal where the program is running. Alternatively, you can redirect
  input from a file or another command.

- Output: the program calculates and prints various statistics about the input
  numbers, including:
  - The count of numbers (n)
  - The minimum value (min)
  - The maximum value (max)
  - The sum of all numbers (sum)
  - The mean (average) of the numbers (mean)
  - The median of the numbers (median)
  - The mode(s) of the numbers (modes)
  - The standard deviation of the numbers (stdev)
  - A histogram representing the distribution of the numbers (histogram)

### Example

```bash
$ echo "1.2 3.4 5.6 7.8 9.0" | ./main > output.txt
$ cat output.txt
n 5
min 1.2000
max 9.0000
sum 27.0000
mean 5.4000
median 5.6000
modes []
stdev 3.1780
1.2000-1.9800   1   **
1.9800-2.7600   0
2.7600-3.5400   1   **
3.5400-4.3200   0
4.3200-5.1000   0
5.1000-5.8800   1   **
5.8800-6.6600   0
6.6600-7.4400   0
7.4400-8.2200   1   **
8.2200-9.0000   0
```

```bash
$ echo "1.2 3.4 5.6 7.8 9.0 1.2" | ./main > output.txt
$ cat output.txt
n 6
min 1.2000
max 9.0000
sum 28.2000
mean 4.7000
median 4.5000
modes [1.2]
stdev 3.3196
1.2000-1.9800   2   **
1.9800-2.7600   0
2.7600-3.5400   1   **
3.5400-4.3200   0
4.3200-5.1000   0
5.1000-5.8800   1   **
5.8800-6.6600   0
6.6600-7.4400   0
7.4400-8.2200   1   **
8.2200-9.0000   0
```

## License

Licensed under [BSD 3-Clause License](LICENSE).
