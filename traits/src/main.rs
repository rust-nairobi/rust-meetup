#![allow(dead_code)]
#![allow(unused_variables)]

trait Animal
{
  // static: called as Animal::create()
  // returns the type of the implementor
  fn create(name: &'static str) -> Self;
  
  fn name(&self) -> &'static str;

  fn talk(&self)
  {
    println!("{} cannot talk", self.name());
  }
}

struct Human
{
  name: &'static str
}

impl Animal for Human
{
  fn create(name:&'static str) -> Human
  {
    Human{name: name}
  }

  fn name(&self) -> &'static str
  { 
    self.name
  }

  fn talk(&self)
  {
    println!("hello, my name is {}", self.name());
  }
}

struct Cat
{
  name: &'static str
}

impl Animal for Cat
{
  fn create(name:&'static str) -> Cat
  {
    Cat{name: name}
  }

  fn name(&self) -> &'static str
  { 
    self.name
  }

  fn talk(&self)
  {
    println!("{} says meow", self.name());
  }
}

fn traits()
{
  // println some custom type
  let h = Human{name:"John"};
  //let h:Human = Animal::create("John"); // type annotation mandatory!
  h.talk();

  //let m = Cat{name:"Misty"};
  let m:Cat = Animal::create("Misty");
  m.talk();

  // todo: derive, op overloading, and other stuff from abc
}

fn main() 
{
  traits();
 
}