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
