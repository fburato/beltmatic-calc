# Beltmatic-calc

Beltmatic-calc is a Rust application defined to calculate the combinations for [Beltmatic](https://store.steampowered.com/app/2674590/Beltmatic/). Beltmatic is a factory building game where number are extracted from mining place and are combined with operator factories to generate other numbers. Results of extraction and calculation are delivered to the game central hub to unlock new mining places and upgrades.

Identifying the "best" way to combine numbers is the key aspect of the game. This application uses a brute force approach to calculate the combinations given the starting parameters, i.e. maximum value of number to cobine, maximum numbers to combine, and operators to use.

For each number of elements to combine:

1. Enumerate the number of potential parenthesisations for the size selected,
2. Enumerate all values to allocate for the parenthesisation operands,
3. Enumerate all values to allocate for the parenthesisation operators,
4. For each allocation, calculate the value `n` of the combinations. If there is no solution to calculate `n` or if another solution with the same size exist, save the solution.

Once the maximums are calculated, print all solutions in the format:

```
n -> (size) [<solutions>]
```

For example:

```
1 -> (1) ["1"]
2 -> (1) ["2"]
3 -> (1) ["3"]
4 -> (1) ["4"]
5 -> (1) ["5"]
6 -> (1) ["6"]
7 -> (1) ["7"]
8 -> (1) ["8"]
9 -> (1) ["9"]
10 -> (1) ["10"]
11 -> (1) ["11"]
12 -> (2) ["(11+1)", "(10+2)", "(9+3)", "(8+4)", "(7+5)", "(6+6)", "(5+7)", "(4+8)", "(3+9)", "(2+10)", "(1+11)", "(6*2)", "(4*3)", "(3*4)", "(2*6)"]
13 -> (2) ["(11+2)", "(10+3)", "(9+4)", "(8+5)", "(7+6)", "(6+7)", "(5+8)", "(4+9)", "(3+10)", "(2+11)"]
```

The combinatorial explosion starts relatively quickly, with `--max-number 11 --max-size 5 --operations '+,*,-,/'` generating 82MB worth of text.

There is space to reduce the combinatorial explosion by using dynamic programming and arithmetic simplification, but in general the solutions generated with size 5 are sufficiently detailed to allow to resolve an high amount of solutions in the games.

## Build

The project has been developed with rust 1.79.0 and has only [`clap`](https://docs.rs/clap/latest/clap/) to process command line argument parsing.

Compile with `cargo --release build` to generate the release version.

## Usage 

```
Usage: beltmatic-calc [OPTIONS] --max-number <MAX_NUMBER> --max-size <MAX_SIZE>

Options:
      --max-number <MAX_NUMBER>  
      --max-size <MAX_SIZE>      
      --operations <OPERATIONS>  
  -h, --help                     Print help
```

The operations argument is a CSV list of the 4 base arithmetic operators '+,-,*,/'.
