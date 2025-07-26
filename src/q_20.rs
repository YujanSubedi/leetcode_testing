pub fn is_valid(s: String) -> bool {
    let mut temp_stack: Vec<char> = Vec::with_capacity(s.len());
    for char_val in s.chars() {
        match char_val {
            '[' | '{' | '(' => {
                temp_stack.push(char_val);
            }
            ']' => {
                if temp_stack.is_empty() || temp_stack[temp_stack.len() - 1] != '[' {
                    return false;
                }
                temp_stack.pop();
            }
            '}' => {
                if temp_stack.is_empty() || temp_stack[temp_stack.len() - 1] != '{' {
                    return false;
                }
                temp_stack.pop();
            }
            ')' => {
                if temp_stack.is_empty() || temp_stack[temp_stack.len() - 1] != '(' {
                    return false;
                }
                temp_stack.pop();
            }
            _ => {}
        }
    }
    temp_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let s = "()".to_string();
        let output_val = true;
        assert_eq!(is_valid(s), output_val);
    }

    #[test]
    fn test_02() {
        let s = "()[]{}".to_string();
        let output_val = true;
        assert_eq!(is_valid(s), output_val);
    }

    #[test]
    fn test_03() {
        let s = "(]".to_string();
        let output_val = false;
        assert_eq!(is_valid(s), output_val);
    }

    #[test]
    fn test_04() {
        let s = "([])".to_string();
        let output_val = true;
        assert_eq!(is_valid(s), output_val);
    }

    #[test]
    fn test_05() {
        let s = "([)]".to_string();
        let output_val = false;
        assert_eq!(is_valid(s), output_val);
    }
}
