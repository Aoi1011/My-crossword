pub fn encode_num(num: i32) -> Vec<u8> {
    if num == 0 {
        return Vec::new();
    }

    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = Vec::new();

    let mut abs_num_copy = abs_num;
    while abs_num_copy > 0 {
        result.push((abs_num_copy & 0xff) as u8);
        abs_num_copy >>= 8;
    }

    if let Some(last_el) = result.last_mut() {
        if *last_el & 0x80 > 0 {
            if negative {
                result.push(0x80);
            } else {
                result.push(0);
            }
        } else if negative {
            *last_el |= 0x80;
        }
    }

    result
}

pub fn decode_num(elements: &mut Vec<u8>) -> i32 {
    if elements.is_empty() {
        return 0;
    }

    // reverse for big endian
    elements.reverse();

    // top bit being 1 means it's negative
    let negative = elements[0] & 0x80 != 0;

    let mut result = (elements[0] & 0x7f) as i32;

    for &c in &elements[1..] {
        result <<= 8;
        result += c as i32;
    }

    if negative {
        -result
    } else {
        result
    }
}

pub fn op_0(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(0));
    true
}

pub fn op_1negate(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(-1));
    true
}

pub fn op_1(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(1));
    true
}

pub fn op_2(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(2));
    true
}

pub fn op_3(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(3));
    true
}

pub fn op_4(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(4));
    true
}

pub fn op_5(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(5));
    true
}

pub fn op_6(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(6));
    true
}

pub fn op_7(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(7));
    true
}

pub fn op_8(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(8));
    true
}

pub fn op_9(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(9));
    true
}

pub fn op_10(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(10));
    true
}

pub fn op_11(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(11));
    true
}

pub fn op_12(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(12));
    true
}

pub fn op_13(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(13));
    true
}

pub fn op_14(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(14));
    true
}

pub fn op_15(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(15));
    true
}

pub fn op_16(stack: &mut Vec<u8>) -> bool {
    stack.append(&mut encode_num(16));
    true
}

pub fn op_nop(_stack: &mut Vec<u8>) -> bool {
    true
}

pub fn op_if(stack: &mut Vec<u8>, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    // go through and re-make the items array based on the top stack element
    let mut true_branch = Vec::new();
    let mut false_branch: Vec<u8> = Vec::new();
    let mut current_array = &mut true_branch;
    let mut found = false;
    let mut num_endifs_needed = 1;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_branch;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }

    if !found {
        return false;
    }

    if let Some(element) = stack.pop() {
        if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
            false_branch.append(items);
            *items = false_branch;
        } else {
            true_branch.append(items);
            *items = true_branch;
        }
    }

    true
}

pub fn op_notif(stack: &mut Vec<u8>, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    // go through and re-make the items array based on the top stack element
    let mut true_branch = Vec::new();
    let mut false_branch = Vec::new();
    let mut current_array = &mut true_branch;
    let mut found = false;
    let mut num_endifs_needed = 1;
    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_branch;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }
    if !found {
        return false;
    }

    if let Some(element) = stack.pop() {
        if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
            true_branch.append(items);
            *items = true_branch;
        } else {
            false_branch.append(items);
            *items = false_branch;
        }
    }

    true
}

pub fn op_verify(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(element) = stack.pop() {
        if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
            return false;
        }
    }

    true
}

pub fn op_return(_stack: &mut Vec<u8>) -> bool {
    false
}

pub fn op_toaltstack(stack: &mut Vec<u8>, altstack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(elem) = stack.pop() {
        altstack.push(elem);
    }

    true
}

pub fn op_formalstack(stack: &mut Vec<u8>, altstack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(elem) = altstack.pop() {
        stack.push(elem);
    }

    true
}

pub fn op_2drop(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    stack.pop();
    stack.pop();

    true
}

pub fn op_2dup(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let len = stack.len();
    let last_two = stack[len - 2..].to_vec();
    stack.extend_from_slice(&last_two);
    true
}

pub fn op_3dup(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 3 {
        return false;
    }

    let len = stack.len();
    let last_three = stack[len - 3..].to_vec();
    stack.extend_from_slice(&last_three);
    true
}

pub fn op_2over(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 4 {
        return false;
    }

    let len = stack.len();
    let slice = stack[len - 4..len - 2].to_vec();
    stack.extend_from_slice(&slice);
    true
}

pub fn op_2rot(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 6 {
        return false;
    }

    let len = stack.len();
    let slice = stack[len - 6..len - 4].to_vec();
    stack.extend_from_slice(&slice);
    true
}

pub fn op_2swap(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 4 {
        return false;
    }

    let len = stack.len();
    let last_two = &stack[len - 2..];
    let before_last_two = &stack[len - 4..len - 2];

    let mut new_stack = Vec::new();
    new_stack.extend_from_slice(last_two);
    new_stack.extend_from_slice(before_last_two);

    stack.splice(len - 4..len, new_stack);

    true
}

pub fn op_ifdup(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return true;
    }

    if let Some(last_elem) = stack.last() {
        if decode_num(&mut last_elem.to_be_bytes().to_vec()) != 0 {
            stack.push(*last_elem);
        }
    }

    true
}

pub fn op_depth(stack: &mut Vec<u8>) -> bool {
    let len = stack.len();
    stack.append(&mut encode_num(len as i32));
    true
}

pub fn op_drop(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    stack.pop();

    true
}

pub fn op_dup(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(last_elem) = stack.last() {
        stack.push(*last_elem);
    }

    true
}

pub fn op_nip(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let len = stack.len();
    stack.remove(len - 2);

    true
}
