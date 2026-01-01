A library for parsing .dconfig files 
<div align="left">

```dconfig
!! This is documentation. 
  !! This is a number
number1: 12.643
  !! This is also a number
number2: -12
  !! This is a color as an rgb string
color1: (0,255,0)
  !! This is a color as an hexadecimal color
color2: #FFFFFF
  !! This is a color represented with a name
color3: red
  !! This is text
text: "Lorem Ipsum"
  !! Those is are booleans
bool1 = yes
bool2 = y
bool2 = true
bool3 = 1 !! Note that this doesn't work for every number, only 1
bool1 = no
bool2 = n
bool2 = false
bool3 = 0
```
## Usage

### Retrieving data
Using dparser is quite simple. the library provides you a HashMap wrapper 
to retrieve the data present associated with a given key inside of the configuration file : **ParsedData**.

It also allows you to interpret the data as **4** different datatype :
  - bool
  - String
  - f64 <br>
  - `Color`, a struct provided by the library that represents an rgb color and does hexadecimal conversion for you.
### Error handling
  if the library fails to parse your file, or interpret a datatype, 
  you are provided with two new error types that give you access to the error message, the file where the error happened
  and (optionally) the line where the error happenned.
