use elliptic_curves::helper::{hash160, hash256};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

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

pub fn op_over(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let len = stack.len();
    if let Some(last_two_el) = stack.get(len - 2) {
        stack.push(*last_two_el);
    }

    true
}

pub fn op_pick(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(last_el) = stack.pop() {
        let n = decode_num(&mut last_el.to_be_bytes().to_vec());

        if stack.len() < n as usize + 1 {
            return false;
        }

        let index = stack.len() - (n as usize) - 1;
        let element = stack[index];
        stack.push(element);
    }

    true
}

pub fn op_roll(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(last_el) = stack.pop() {
        let n = decode_num(&mut last_el.to_be_bytes().to_vec());

        if stack.len() < n as usize + 1 {
            return false;
        }

        let index = stack.len() - (n as usize) - 1;
        let element = stack.remove(index);
        stack.push(element);
    }

    true
}

pub fn op_rot(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 3 {
        return false;
    }

    let len = stack.len();
    let element = stack.remove(len - 3);
    stack.push(element);

    true
}

pub fn op_swap(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let len = stack.len();
    let element = stack.remove(len - 2);
    stack.push(element);

    true
}

pub fn op_tuck(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let len = stack.len();
    if let Some(element) = stack.get(len - 1) {
        stack.insert(len - 2, *element);
    }

    true
}

// pub fn op_size(stack: &mut Vec<u8>) -> bool {
//     if stack.is_empty() {
//         return false;
//     }
//
//     let size = stack.last().unwrap();
// }

pub fn op_equal(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let element1 = stack.pop().unwrap();
    let element2 = stack.pop().unwrap();

    if element1 == element2 {
        stack.append(&mut encode_num(1));
    } else {
        stack.append(&mut encode_num(0));
    }

    true
}

pub fn op_equalverify(stack: &mut Vec<u8>) -> bool {
    op_equal(stack) && op_verify(stack)
}

pub fn op_1add(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(last_el) = stack.pop() {
        let element = decode_num(&mut last_el.to_be_bytes().to_vec());
        stack.append(&mut encode_num(element + 1));
    }

    true
}

pub fn op_1sub(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(last_el) = stack.pop() {
        let element = decode_num(&mut last_el.to_be_bytes().to_vec());
        stack.append(&mut encode_num(element - 1));
    }
    true
}

pub fn op_negate(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element = decode_num(&mut last_el.to_be_bytes().to_vec());
    stack.append(&mut encode_num(-element));

    true
}

pub fn op_abs(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element = decode_num(&mut last_el.to_be_bytes().to_vec());
    if element < 0 {
        stack.append(&mut encode_num(-element));
    } else {
        stack.append(&mut encode_num(element));
    }

    true
}

pub fn op_not(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
        stack.append(&mut encode_num(1));
    } else {
        stack.append(&mut encode_num(0));
    }

    true
}

pub fn op_0notequal(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&mut element.to_be_bytes().to_vec()) == 0 {
        stack.append(&mut encode_num(0));
    } else {
        stack.append(&mut encode_num(1));
    }

    true
}

pub fn op_add(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    stack.append(&mut encode_num(element1 + element2));

    true
}

pub fn op_sub(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    stack.append(&mut encode_num(element2 - element1));

    true
}

pub fn op_booland(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element1 != 0 && element2 != 0 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_boolor(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element1 != 0 || element2 != 0 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_numequal(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element1 == element2 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_numequalverify(stack: &mut Vec<u8>) -> bool {
    op_numequal(stack) && op_verify(stack)
}

pub fn op_numnotequal(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element1 == element2 { 0 } else { 1 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_lessthan(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element2 < element1 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_greaterthan(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element2 > element1 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_lessthanorequal(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element2 <= element1 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_greaterthanorequal(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element2 >= element1 { 1 } else { 0 };

    stack.append(&mut encode_num(res));

    true
}

pub fn op_min(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    if element1 < element2 {
        stack.append(&mut encode_num(element1));
    } else {
        stack.append(&mut encode_num(element2));
    };

    true
}

pub fn op_max(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let element1 = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element2 = decode_num(&mut last_el.to_be_bytes().to_vec());

    if element1 > element2 {
        stack.append(&mut encode_num(element1));
    } else {
        stack.append(&mut encode_num(element2));
    };

    true
}

pub fn op_within(stack: &mut Vec<u8>) -> bool {
    if stack.len() < 3 {
        return false;
    }

    let last_el = stack.pop().unwrap();
    let maximum = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let minimum = decode_num(&mut last_el.to_be_bytes().to_vec());

    let last_el = stack.pop().unwrap();
    let element = decode_num(&mut last_el.to_be_bytes().to_vec());

    let res = if element >= minimum && element < maximum {
        1
    } else {
        0
    };
    stack.append(&mut encode_num(res));

    true
}

pub fn op_ripemd160(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    let mut hasher1 = Sha256::new();
    hasher1.update(element.to_be_bytes());
    let digest = hasher1.finalize();

    let res = Ripemd160::digest(digest);

    stack.append(&mut res.to_vec());

    true
}

// pub fn op_sha1(stack: &mut Vec<u8>) -> bool {
//     if stack.is_empty() {
//         return false;
//     }
//
//     let element = stack.pop().unwrap();
//     let mut hasher1 = Sha1::new();
//     hasher1.update(element.to_be_bytes());
//     let digest = hasher1.finalize();
//
//     stack.append(&mut digest.to_vec());
//
//     true
// }

pub fn op_sha256(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    let mut hasher1 = Sha256::new();
    hasher1.update(element.to_be_bytes());
    let digest = hasher1.finalize();

    stack.append(&mut digest.to_vec());

    true
}

pub fn op_hash160(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    if let Some(element) = stack.pop() {
        stack.append(&mut hash160(&element.to_be_bytes().to_vec()));
    };

    true
}

pub fn op_hash256(stack: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    stack.append(&mut hash256(&element.to_be_bytes()).to_vec());

    true
}

// pub fn op_checksig(stack: &mut Vec<u8>, z: u32) -> bool {}

// pub fn op_checksigverify(stack: &mut Vec<u8>, z: u32) -> bool {}

// pub fn op_checkmultisig(stack: &mut Vec<u8>, z: u32) -> bool {}

// pub fn op_checkmultisigverify(stack: &mut Vec<u8>, z: u32) -> bool {}

pub fn op_checklocktimeverify(stack: &Vec<i64>, locktime: i64, sequence: i64) -> bool {
    if sequence == 0xffffffff {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let element = stack.last().unwrap();
    if *element < 0 {
        return false;
    }
    if *element < 500000000 && locktime > 500000000 {
        return false;
    }
    if locktime < *element {
        return false;
    }
    true
}

pub fn op_checksequenceverify(stack: &Vec<i64>, version: i64, sequence: i64) -> bool {
    if sequence & (1 << 31) == (1 << 31) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let element = stack.last().unwrap();
    if *element < 0 {
        return false;
    }
    if element & (1 << 31) == (1 << 31) {
        if version < 2 {
            return false;
        } else if sequence & (1 << 31) == (1 << 31) {
            return false;
        } else if element & (1 << 22) != sequence & (1 << 22) {
            return false;
        } else if element & 0xffff > sequence & 0xffff {
            return false;
        }
    }
    true
}

pub enum OpCodeFunctions {
    OP0 = 0,
    OP1Negate = 79,
    OP1 = 81,
    OP2 = 82,
    OP3 = 83,
    OP4 = 84,
    OP5 = 85,
    OP6 = 86,
    OP7 = 87,
    OP8 = 88,
    OP9 = 89,
    OP10 = 90,
    OP11 = 91,
    OP12 = 92,
    OP13 = 93,
    OP14 = 94,
    OP15 = 95,
    OP16 = 96,
    OPNop = 97,
    OPIf = 99,
    OPNotIf = 100,
    OPVerify = 105,
    OPReturn = 106,
    OPTotalStack = 107,
    OP2Drop = 109,
    OP2dUp = 110,
    OP3dUp = 111,
    OP2Over = 112,
    OP2Rot = 113,
    OP2Swap = 114,
    OPIfdUp = 115,
    OPDepth = 116,
    OPDrop = 117,
    OPDup = 118,
    OPNip = 119,
    OPOver = 120,
    OPPick = 121,
    OPRoll = 122,
    OPRot = 123,
    OPSwap = 124,
    OPTuck = 125,
    OPSize = 130,
    OPEqual = 135,
    OPEqualVerify = 136,
    OP1Add = 139,
    OP1Sub = 140,
    OPNegate = 143,
    OPAbs = 144,
    OPNot = 145,
    OP0NotEqual = 146,
    OPAdd = 147,
    OPSub = 148,
    OPBoolAnd = 154,
    OPBoolOr = 155,
    OPNumEqual = 156,
    OPNumEqualVerify = 157,
    OPNumNotEqual = 158,
    OPLessThan = 159,
    OPGreaterThan = 160,
    OPlessThanOrEqual = 161,
    OPGreaterThanOrEqual = 162,
    OPMin = 163,
    OPMax = 164,
    OPWithin = 165,
    OPRipemd160 = 166,
    OPSha1 = 167,
    OPSha256 = 168,
    OPHash160 = 169,
    OPHash256 = 170,
    OPCheckSig = 172,
    OPCheckSigVerify = 173,
    OPCheckMultiSig = 174,
    OPCheckMultiSigVerify = 175,
    // OPNop = 176,
    OPCheckLockTimeVerify = 177,
    OPCheckSequenceVerify = 178,
    // OPNop = 177,
    // OPNop = 178,
    // OPNop = 179,
    // OPNop = 180,
    // OPNop = 181,
    // OPNop = 182,
    // OPNop = 183,
    // OPNop = 184,
    // OPNop = 185,
}

pub enum OPCodeNames {
    OP0 = 0,
    OPPushData1 = 76,
    OPPushData2 = 77,
    OPPushData4 = 78,
    OP1Negate = 79,
    OP1 = 81,
    OP2 = 82,
    OP3 = 83,
    OP4 = 84,
    OP5 = 85,
    OP6 = 86,
    OP7 = 87,
    OP8 = 88,
    OP9 = 89,
    OP10 = 90,
    OP11 = 91,
    OP12 = 92,
    OP13 = 93,
    OP14 = 94,
    OP15 = 95,
    OP16 = 96,
    OPNop = 97,
    OPIf = 99,
    OPNotIf = 100,
    OPElse = 103,
    OPEndIf = 104,
    OPVerify = 105,
    OPReturn = 106,
    OPTotalStack = 107,
    OPFromMaltStack = 108,
    OP2Drop = 109,
    OP2Dup = 110,
    OP3Dup = 111,
    OP2Over = 112,
    OP2Rot = 113,
    OP2Swap = 114,
    OPIfDup = 115,
    OPDepth = 116,
    OPDrop = 117,
    OPDup = 118,
    OPNip = 119,
    OPOver = 120,
    OPPick = 121,
    OPRoll = 122,
    OPRot = 123,
    OPSwap = 124,
    OPTuck = 125,
    OPSize = 130,
    OPEqual = 135,
    OPEqualVerify = 136,
    OP1Add = 139,
    OP1Sub = 140,
    OPNegate = 143,
    OPAbs = 144,
    OPNot = 145,
    OP0NotEqual = 146,
    OPAdd = 147,
    OPSub = 148,
    OPBoolAnd = 154,
    OPBoolOr = 155,
    OPNumEqual = 156,
    OPNumEqualVerify = 157,
    OPNumNotEqual = 158,
    OPLessThan = 159,
    OPGreaterThan = 160,
    OPLessThanOrEqual = 161,
    OPGreaterThanOrEqual = 162,
    OPMin = 163,
    OPMax = 164,
    OPWithin = 165,
    OPRipemd160 = 166,
    OPSha1 = 167,
    OPSha256 = 168,
    OPHash160 = 169,
    OPHash256 = 170,
    OpCodeSeparator = 171,
    OpCheckSig = 172,
    OPCheckSigVerify = 173,
    OPCheckMultiSig = 174,
    OPCheckMultiSigVerify = 175,
    OPNop1 = 176,
    OPCheckLockTimeVerify = 177,
    OPCheckSequenceVerify = 178,
    OPNop4 = 179,
    OPNop5 = 180,
    OPNo6 = 181,
    OpNo7 = 182,
    OpNo8 = 183,
    OpNo9 = 184,
    OpNo10 = 185,
}
