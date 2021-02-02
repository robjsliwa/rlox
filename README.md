# Rlox

Rlox is simple language designed purely to learn how build interpreters.

Rlox is a clone of lox langauge from https://craftinginterpreters.com/ book. Lox was written in Java and C and Rlox is a clone written in Rust just for fun.

*NOTE* - This is work in progress.  Branch `tree-walk` contains tree walking interpreter.

# Running REPL

Currently REPL it is very basic but you can execute all of the commands from it.  To run REPL just `cargo run`

# Running Rlox code from file

`cargo run -- -i path/filename.rl`

# Run tests

`cargo t`

# Some example code

## Compute Fibonacci 

```
fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 20; i = i + 1) {
  print fib(i);
}
```

## Closures

```
var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}

fun test() {
  print a;
  a=a+"!";
  return a;
}

print test();
```

## Classes and Inheritance

```
class Doughnut {
  cook() {
    return "Fry until golden brown.";
  }

  cookAnother() {
    return "Fry until golden.";
  }
}

class BostonCream < Doughnut {
  cookAnother() {
    var orig = super.cookAnother();
    return orig + " Then pipe full of custard and coat with chocolate.";
  }
}

var first = BostonCream().cook();
print first;

var second = BostonCream().cookAnother();
print second;
```
