use std::fmt::Display;

struct Calculator {
    number_1: i32,
    number_2: i32
}

trait AdditiveOperations {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
}

trait MultiplicativeOperations {
    fn mul(&self) -> i32;
    fn div(&self) -> Option<i32>;
}

trait BinaryOperations {
    fn and(&self) -> i32;
    fn or(&self) -> i32;
    fn xor(&self) -> i32;
}

impl AdditiveOperations for Calculator {
    fn add(&self) -> i32 {
        self.number_1 + self.number_2
    }

    fn sub(&self) -> i32 {
        self.number_1 - self.number_2
    }
}

impl MultiplicativeOperations for Calculator {
    fn mul(&self) -> i32 {
        self.number_1 * self.number_2
    }

    fn div(&self) -> Option<i32> {
        if self.number_2 == 0 {
            None
        } else {
            Some(self.number_1 / self.number_2)
        }
    }
}

impl BinaryOperations for Calculator {
    fn and(&self) -> i32 {
        self.number_1 & self.number_2
    }

    fn or(&self) -> i32 {
        self.number_1 | self.number_2
    }

    fn xor(&self) -> i32 {
        self.number_1 ^ self.number_2
    }
}

impl Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {//TODO idiomatic
        write!(f, "- Addition {}
\n- Subtraction {}
\n- Multiplication {}
\n- Division {}
\n- AND {}
\n- OR {}
\n- XOR {}", self.add(), self.sub(), self.mul(), self.div().unwrap_or(0), self.and(), self.or(), self.xor())
    }
}

fn main() {
   let calculator = Calculator { number_1: 2, number_2: 1 };
    println!("Calculator: \n{}", calculator);
}
