// One

class DevonshireCream {
  serveOn() {
    return "Scones";
  }
}

print DevonshireCream; // Prints "DevonshireCream".

// Two

class Bagel {}
var bagel = Bagel();
print bagel;

bagel.description = "Good";
print bagel.description;

// Three

class Bacon {
    eat() {
        print "Crunch crunch crunch!";
    }
}

Bacon().eat();

// Four

class Cake {
  taste() {
    var adjective = "delicious";
    print "The " + this.flavor + " cake is " + adjective + "!";
  }
}

var cake = Cake();
cake.flavor = "German chocolate";
cake.taste(); // Prints "The German chocolate cake is delicious!".

// Five

class Foo {
  init() {
    print this;
  }
}

var foo = Foo();
print foo.init();

// Six

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
