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