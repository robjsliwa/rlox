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
  print a;
}

test();