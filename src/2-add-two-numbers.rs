// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn get_list_value (l: &Option<Box<ListNode>>) -> String {
  match l {
    Some(b) => {
      let mut v = get_list_value(&b.next);
      v.push_str(b.val.to_string().as_str());
      return v
    },
    None => {
      return String::new()
    }
  }
}


fn addition(num1: String, num2: String) -> String {
  let mut additive_string = String::new();
  let mut overflow = 0;
  let mut number1: Vec<char> = num1.chars().collect();
  let mut number2: Vec<char> = num2.chars().collect();

  // Get the length of the longest number.
  let longest_len = number1.len().max(number2.len());

  // Do simple primary school addition with each value in the number.
  for _ in 0..longest_len {
    // Convert the first number to u8.
    let first: u8 = match number1.pop() {
      Some(character) => character.to_string().parse().unwrap(),
      None => 0,
    };

    // Convert the second number to u8.
    let second: u8 = match number2.pop() {
      Some(character) => character.to_string().parse().unwrap(),
      None => 0,
    };
    
    // Add the two numbers together and turn it into a string.
    let mut sum: String = (first + second + overflow)
      .to_string()
      .chars()
      .rev()
      .collect();
    
    // If the string has more than 1 number in it, it means we have to pass
    // the overflow value onto the next round of addition.
    overflow = if sum.len() > 1 {
      sum.pop().unwrap().to_string().parse().unwrap()
    } else {
      // Set the overflow back to 0 just in case.
      0
    };
    
    // Add the sum of the two values to the string.
    additive_string.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
  }     
  
  // Finally if there was a leftover overflow value, add it to the string.
  if overflow > 0 {
    additive_string.push(
      *overflow
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap(),
    );
  }
    
  // Reverse the order of the string and return it.
  additive_string.chars().rev().collect::<String>()
}


struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let number1 = get_list_value(&l1);
    let number2 = get_list_value(&l2);

    let sum = addition( number1 , number2);

    let mut list: Option<Box<ListNode>> = None;

    for i in sum.to_string().chars() {
      if list.is_none() {
        list =  Some(Box::new( ListNode::new(i.to_digit(10).unwrap_or(0) as i32)  ));
        continue;
      } 
        
      list = Some(Box::new( ListNode{ next: list, val: i.to_digit(10).unwrap_or(0) as i32}  ));
    }

    list
  }
}// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn get_list_value (l: &Option<Box<ListNode>>) -> String {
  match l {
    Some(b) => {
      let mut v = get_list_value(&b.next);
      v.push_str(b.val.to_string().as_str());
      return v
    },
    None => {
      return String::new()
    }
  }
}


fn addition(num1: String, num2: String) -> String {
  let mut additive_string = String::new();
  let mut overflow = 0;
  let mut number1: Vec<char> = num1.chars().collect();
  let mut number2: Vec<char> = num2.chars().collect();

  // Get the length of the longest number.
  let longest_len = number1.len().max(number2.len());

  // Do simple primary school addition with each value in the number.
  for _ in 0..longest_len {
    // Convert the first number to u8.
    let first: u8 = match number1.pop() {
      Some(character) => character.to_string().parse().unwrap(),
      None => 0,
    };

    // Convert the second number to u8.
    let second: u8 = match number2.pop() {
      Some(character) => character.to_string().parse().unwrap(),
      None => 0,
    };
    
    // Add the two numbers together and turn it into a string.
    let mut sum: String = (first + second + overflow)
      .to_string()
      .chars()
      .rev()
      .collect();
    
    // If the string has more than 1 number in it, it means we have to pass
    // the overflow value onto the next round of addition.
    overflow = if sum.len() > 1 {
      sum.pop().unwrap().to_string().parse().unwrap()
    } else {
      // Set the overflow back to 0 just in case.
      0
    };
    
    // Add the sum of the two values to the string.
    additive_string.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
  }     
  
  // Finally if there was a leftover overflow value, add it to the string.
  if overflow > 0 {
    additive_string.push(
      *overflow
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap(),
    );
  }
    
  // Reverse the order of the string and return it.
  additive_string.chars().rev().collect::<String>()
}


struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let number1 = get_list_value(&l1);
    let number2 = get_list_value(&l2);

    let sum = addition( number1 , number2);

    let mut list: Option<Box<ListNode>> = None;

    for i in sum.to_string().chars() {
      if list.is_none() {
        list =  Some(Box::new( ListNode::new(i.to_digit(10).unwrap_or(0) as i32)  ));
        continue;
      } 
        
      list = Some(Box::new( ListNode{ next: list, val: i.to_digit(10).unwrap_or(0) as i32}  ));
    }

    list
  }
}