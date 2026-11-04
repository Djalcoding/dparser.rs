A Rust library for parsing configuration files 
<div align="left">

```text
!! This is documentation.
    !! This is a number
number1: 12.643 
number2: -12 !! This is an inline comment
    !! This is a color as an rgb string
color1: (0,255,0)
    !! This is a color as an hexadecimal color
color2: #FFFFFF
    !! This is a color represented with a name
color3: red
    !! This is text
text: "Lorem Ipsum"
    !! Those is are booleans
bool1: yes
bool2: y
bool2: true
bool3: 1 !! Note that this doesn't work for every number, only 1
bool1: no
bool2: n
bool2: false
bool3: 0
!! Raw data can be used for enums and such
raw_data: EnumVariant
!! This is an array
!! Arrays are typed
string array: "Hello, ", "World"
number array: 1, 1.618, 2, 2.718, 3, 3.142 
boolean array: true, false, yes, no 
color array: (0,255,0), transparent, #ffffff, red
raw array: I, Will, Survive

## Usage
### Retrieving data
This crate provides a HashMap wrapper to interpret data in different output type : **ParsedData**
Data can be interpreted as **4** different datatypes :
  - bool
  - String
  - f64 <br>
  - `Color`, a struct provided by the library that represents an rgb color and does hexadecimal conversion for you.
Arrays are also supported
Parsing failure has to be explicitly handled
