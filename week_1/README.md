
Week 1 - Exercises
    Exercise 1 - Ownership and Borrowing (DONE)
        Copy/paste the following code to a Rust main.rs file
        Explain why this code snippet does not work.
        Give at least 2 ways to fix the issue (there are more than 2).
        ```
        fn function_1(var: String) {

            println!("In function_1, variable is: {}", var);
        }

        fn main() {
            let variable = String::from("Welcome to RustSkills");

            function_1(variable);

            println!("In main, variable is: {}", variable);
        }
        ​```
        Now, replace the String variable with a scalar variable (u32, i32, u64, i64, …) and retest the same code snippet.
        Why does it work?

    Exercise 2 - Code analysis
        For each provided code snippet, analyze and document what it should do.
        - Snippet 1
            - Without testing it, what is wrong with this code snippet? a is not mutable
            - How can it be fixed? make a mutable
            
            ```rust
            fn main() {
                let a = vec![1,2,3,4];
                a.push(27);
            }
            ```
        
        - Snippet 2
            - Without testing it, what is wrong with this code snippet? a is not mutable, and a+ pretends to store the sum result in a.
            - How can it be fixed? make the a parameter mutable
            
            ```rust
            fn my_operation(a: u64, b: u64) -> u64 {
                a += b;
                a
            }
            
            fn main() {
                let num1 = 1234;
                let num2 = 1122;
                println!("My result is {}!", my_operation(num1, num2));
            }
            ```
            
        - Snippet 3
            
            Without executing the code, what is the printed value of `x`? I think 4, but its 3
            
            Test it and explain why `x` has this value: The third scope assigns the value 3 to the x from the previous scope.
            
            ```rust
            fn main() {
                let x = 1;
            
                {
                    let mut x = 2;
            
                    x = x ^ 2;
            
                    {
                        x = 3;
                        let x = 12;
                    }
                    println!("x is: {}", x);
                }
            }
            ```
            
        - Snippet 4
            
            The following Solidity and Rust snippets shows the (Key ⇒ Value) functionality. Solidity provides this through a mapping while Rust provides it through an Hashmap.
            
            - What is the main difference between the two languages about non-initialized data?
            - Snippets
                
                ```solidity
                // SPDX-License-Identifier: GPL-3.0
                
                pragma solidity ^0.8.0;
                
                contract TestMapping {
                
                    mapping(string => uint256) values;
                
                    function add(string calldata input, uint256 value) external {
                        values[input] = value;
                    }
                
                    function read(string calldata input) external view returns (uint256) {
                        return values[input];
                    }
                }
                ```
                
                ```rust
                use std::collections::HashMap;
                
                fn main() {
                    let mut values: HashMap<String, u64> = HashMap::new();
                
                    values.insert(String::from("test"), 12345);
                    println!("\"test\" associated value is {}", values.get(&String::from("test")).unwrap());
                }
                ```
                
            - Hint
                - https://docs.soliditylang.org/en/latest/control-structures.html#scoping-and-declarations
                - https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map

    Exercise 3 - Security analysis
        Analyze the following piece of code with a security mindset and answer to the following:
        Explain what the function does.
        What could go wrong?
        How to fix it?
        Note: This code is taken from a Solana program written with Anchor, but it has been heavily simplified.
        Note: DECIMALS_SCALAR value is not given. It is an u128 which may have any value except 0.
        Note: exchange_rate.deposit_rate is an u64.
        ```
            pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
                let rate = exchange_rate.deposit_rate as u128;
                let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; 

                        // transfer(token, from, to, amount)
                token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

                        // mint_to(token, to, amount)
                token::mint_to(shares_token, ctx.caller, amt)?;

                Ok(())
            }
        ```