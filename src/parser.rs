use std::ops::Index;

use super::types::*;

static PREFIX: &'static str = "(";
static SUFFIX: &'static str = ")";

pub fn parser(exp: String) -> Result<List, String> {
    Ok(parse0(exp.replace("\"", "'")))
}

fn parse0(exp: String) -> List {
    let mut stack = Vec::new();
    let mut next = true;
    let mut exp = exp.as_str();
    while next {
        exp = exp.trim();
        let is_push = exp.starts_with(PREFIX);
        let next_exp = &exp[1..];
        let to_index = get_to_index(next_exp);
        next = next_exp.find(SUFFIX) != None;
        let sub_exp = &next_exp[..to_index];
        // print!("sub_exp:{} next_exp:{}" , sub_exp, next_exp);
        if (is_push) {
            let mut expr = List::new();
            expr.push_vec(parse_list(sub_exp));
            stack.push(expr);
        } else {
            let brother = stack.pop().unwrap();
            if (stack.is_empty()) {
                stack.push(brother);
            } else {
                let mut parent = stack.pop().unwrap();
                parent.push(LispType::Expr(brother));
                parent.push_vec(parse_list(sub_exp));
                stack.push(parent);
            }
        }

        // println!("stack: {}", stack.len());
        // println!("-----");
        // print!("old-exp:{}to_index:{}",exp, to_index);
        exp = exp[to_index + 1..].trim();
        // println!("exp:{}",exp)
    }
    stack.pop().unwrap()
}

fn get_to_index(next_exp: &str) -> usize {
    let pre = next_exp.find(PREFIX);
    let suf = next_exp.find(SUFFIX);
    if suf != None {
        let suf_index = suf.unwrap();
        if pre != None {
            let pre_index = pre.unwrap();
            if pre_index < suf_index {
                pre_index
            } else {
                suf_index
            }
        } else {
            suf_index
        }
    } else {
        0
    }
}

fn parse_list(exp: &str) -> Vec<LispType> {
    rep_str(exp.to_string())
        .trim()
        .split_whitespace()
        .map(|s| parse_atom(s).unwrap())
        .collect()
}

fn rep_str(str: String) -> String {
    if let Some(i) = str.find("'") {
        let sub = &str[i + 1..];
        if let Some(j) = sub.find("'") {
            let s = &sub[..j].replace(" ", "\\u0009");
            let mut all_str = String::new();
            let right_str = &str[..i + 1];
            let left_str = &sub[j..];
            all_str.push_str(right_str);
            all_str.push_str(s);
            // println!("right_str:[{}] s:[{}] left_str:[{}]", right_str, s, left_str);
            all_str.push_str(rep_str(left_str.to_string()).as_str());
            return all_str;
        }
        return str;
    } else {
        return str;
    }
}

fn parse_atom(s: &str) -> Result<LispType, String> {
    let t: LispType = match s {
        "nil" => LispType::Nil,
        "#t" => LispType::Boolean(true),
        "#f" => LispType::Boolean(false),
        _ => {
            if s.starts_with("'") && s.ends_with("'") && s.len() > 2 {
                LispType::Strings(s[1..s.len() - 1].replace("\\u0009", " ").replace("\\r", "\r").replace("\\n", "\n").to_string())
            } else if s.starts_with("\\#") && s.len() > 2 {
                LispType::Char(s.chars().nth(2).unwrap())
            } else if s.parse::<isize>().is_ok() {
                LispType::Number(s.parse::<isize>().unwrap())
            } else{
                peel_onions(s, vec![",@",",","'"])
            }
        }
    };
    Ok(t)
}


fn peel_onions(s:&str, keys:Vec<&str>)->LispType{
    for  key in keys{
        if s.starts_with(key)&&s.len()>key.len(){
            return LispType::expr_of(vec![LispType::Symbol(key.to_string()),LispType::Symbol(s[key.len()..].to_string())]);
        }
    }
    LispType::Symbol(s.to_string())
}