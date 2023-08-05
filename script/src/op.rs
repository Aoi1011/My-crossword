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

pub fn decode_num(element: &mut Vec<u8>) -> i32 {
    if element.is_empty() {
        return 0;
    }

    // reverse for big endian
    element.reverse();

    // top bit being 1 means it's negative
    let negative = element[0] & 0x80 != 0;

    let mut result = (element[0] & 0x7f) as i32;

    for &c in &element[1..] {
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
    if stack.len() < 1 {
        return false;
    }

    // go through and re-make the items array based on the top stack element
    let mut true_items = Vec::new();
    let mut false_items: Vec<u8> = Vec::new();
    let mut current_array = true_items.clone();
    let mut found = false;
    let mut num_endifs_needed = 1;

    while items.len() > 0 {
        let item = items.remove(0);
        if matches!(item, 99 | 100) {
            num_endifs_needed += 1;
            current_array.push(item);
        } else if num_endifs_needed == 1 && item == 103 {
            current_array = false_items.clone();
        } else if item == 104 {
            if num_endifs_needed == 1 {
                found = true;
                break;
            } else {
                num_endifs_needed -= 1;
                current_array.push(item);
            }
        } else {
            current_array.push(item);
        }
    }

    if !found {
        return false;
    }

    if let Some(element) = stack.pop() {
        if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
            false_items.append(items);
            *items = false_items;
        } else {
            true_items.append(items);
            *items = true_items;
        }
    }

    true
}
