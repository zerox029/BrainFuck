use std::io::stdin;

fn main() {
    exec("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
}

fn exec(code: &str) {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;

    let code_arr = code_to_char_array(code);

    let mut i = 0;
    while i < code_arr.len() {
        let current = code_arr[i];

        match current {
            '>' => ptr_right(&mut pointer),
            '<' => ptr_left(&mut pointer),
            '+' => inc_at_pointer(&pointer, &mut memory),
            '-' => dec_at_pointer(&pointer, &mut memory),
            '.' => output(&pointer, &memory),
            ',' => input(&pointer, &mut memory),
            '[' => jmp_past(&pointer, &mut memory, &code_arr, &mut i),
            ']' => jmp_back(&pointer, &mut memory, &code_arr, &mut i),
            _ => (),
        }
        
        i += 1;
    }
}

fn code_to_char_array(code: &str) -> Vec<char> {
    code.chars().collect()
}

fn ptr_right(ptr: &mut usize) {
    *ptr += 1;
}

fn ptr_left(ptr: &mut usize) {
    *ptr -= 1;
}

fn inc_at_pointer(ptr: &usize, mem: &mut [u8]) {
    mem[*ptr] += 1;
}

fn dec_at_pointer(ptr: &usize, mem: &mut [u8]) {
    mem[*ptr] -= 1;
}

fn output(ptr: &usize, mem: &[u8]) {
    print!("{}", mem[*ptr] as char);
}

fn input(ptr: &usize, mem: &mut [u8]) {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Incorrect input");

    let input = input.chars().nth(0).unwrap();
    mem[*ptr] = input as u8;
}

fn jmp_past(ptr: &usize, mem: &[u8], arr: &Vec<char>, i: &mut usize) {
    if mem[*ptr] == 0 {
        let mut non_closing_braces = 0;
        for j in *i + 1..arr.len() - 1 {
            let current = arr[j];

            match current {
                '[' => non_closing_braces += 1,
                ']' => {
                    if non_closing_braces > 0 {
                        non_closing_braces -= 1;
                    } else {
                        *i = j;
                        break;
                    }
                }
                _ => (),
            }
        }
    }
}

fn jmp_back(ptr: &usize, mem: &[u8], arr: &Vec<char>, i: &mut usize) {
    if mem[*ptr] != 0 {
        let mut non_opening_braces = 0;
        for j in (0..*i - 1).rev() {
            let current = arr[j];

            match current {
                ']' => non_opening_braces += 1,
                '[' => {
                    if non_opening_braces > 0 {
                        non_opening_braces -= 1;
                    } else {
                        *i = j;
                        break;
                    }
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_to_char_array() {
        let code = "++<.";
        let expected = vec!['+', '+', '<', '.'];

        assert_eq!(expected, code_to_char_array(code));
    }

    #[test]
    fn test_ptr_right() {
        let mut pointer: usize = 0;
        ptr_right(&mut pointer);

        assert_eq!(1, pointer);
    }

    #[test]
    fn test_ptr_left() {
        let mut pointer: usize = 1;
        ptr_left(&mut pointer);

        assert_eq!(0, pointer);
    }

    #[test]
    fn test_inc_at_pointer() {
        let mut pointer: usize = 0;
        let mut memory: [u8; 30000] = [0; 30000];

        inc_at_pointer(&mut pointer, &mut memory);

        assert_eq!(1, memory[0]);
    }

    #[test]
    fn test_dec_at_pointer() {
        let mut pointer: usize = 0;
        let mut memory: [u8; 30000] = [1; 30000];

        dec_at_pointer(&mut pointer, &mut memory);

        assert_eq!(0, memory[0]);
    }
}
