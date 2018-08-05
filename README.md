# struct_gen
[![Build Status](https://travis-ci.org/robertDurst/struct_gen.svg?branch=master)](https://travis-ci.org/robertDurst/struct_gen)
[![Current Version](https://meritbadge.herokuapp.com/struct_gen)](https://crates.io/crates/struct_gen)

A macro based tool for automagically generating structs.

## Goals
Outlined in the milestone/issues [section](https://github.com/robertDurst/struct_gen/issues).

## Changelog
* 0.1.1 - August 5, 2018
    * Support for lifetimes
    * Support for 10 of 11 primitive types (tuples will come with next release)  including:
        * Signed integers
        * Unsigned integers
        * Boolean
        * Char
        * Vectors (`std::vec::Vec`)
        * Arrays
        * Slices
        * Floats
        * str
        * Strings
    * Added an internal procedural macro to generate array implementations for `Zero` trait (a bit hackish at the moment)
* 0.1.0 - July 30, 2018
    * Minimum viable crate
    * Ability to generate struct's with constructor for most basic primitives
    * Began implementing `struct_gen!` and `impl_zero!` macros
    * Created `Zero` trait for definition of *zero-or-override* method on types