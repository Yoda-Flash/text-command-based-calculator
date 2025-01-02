This is a simple command line app that acts as a scientific calculator, operating through commands instead of buttons.

For now, only one operation may be performed at a time. However, this will be updated soon so that multiple operations can be performed through a single line!

## Commands
For detailed directions on the command line: ```cargo run calculator -h```
### Addition (--plus or -p)
```cargo run [base_number] --plus [addend]```
Eg. ```cargo run 10 --plus 10``` will result in 20.

### Subtraction (--subtract-by or -s)
```cargo run [base_number] --subtract-by [subtrahend]```
Eg. ```cargo run 10 --subtract-by 8``` will result in 2.

### Multiplication (--multiply-by or -m)
```cargo run [base_number] --multiply-by [multiplier]```
Eg. ```cargo run 10 --multiply-by 2``` will result in 20.

### Division (--divide-by or -d)
```cargo run [base_number] --divide-by [divisor]```
Eg. ```cargo run 10 --divide-by 5``` will result in 2.

### Exponentiation (--exponent or -x)
```cargo run [base_number] --exponent [exponent]```
Eg. ```cargo run 10 --exponent 2``` will result in 100.

### Radical Operation (--root or -r)
```cargo run [base_number] --root [root]```
Eg. ```cargo run 100 --root 2``` will result in 10.

### Logarithm (--log or -l)
```cargo run [base_number] --log [base]```
Eg. ```cargo run 100 --log 10``` will result in 2.

### Trigonometry
All base numbers must be in radians.

#### Sine (--sin)
```cargo run [base_number] --sin```
Eg. ```cargo run 3.14 --sin``` will result in 0.

#### Cosine (--cos)
```cargo run [base_number] --cos```
Eg. ```cargo run 3.14 --cos``` will result in -1.

#### Tangent (--tan)
```cargo run [base_number] --tan```
Eg. ```cargo run 3.14 --tan``` will result in 0.

#### Inverse Sine (--arcsin)
```cargo run [base_number] --arcsin```
Eg. ```cargo run 1 --arcsin``` will result in 1.57.

#### Inverse Cosine (--arccos)
```cargo run [base_number] --arccos```
Eg. ```cargo run 0 --arccos``` will result in 0.

#### Inverse Tangent (--arctan)
```cargo run [base_number] --arctan```
Eg. ```cargo run 1 --arctan``` will result in 0.79.

### Angle Conversions
#### Degrees to Radians (--degrees-to-radians)
```cargo run [base_number] --degrees-to-radians```
Eg. ```cargo run 180 --degrees-to-radians``` will result in 3.14.

#### Radians to Degrees (--radians-to-degrees)
```cargo run [base_number] --radians-to-degrees```
Eg. ```cargo run 3.14 --radians-to-degrees``` will result in 180.