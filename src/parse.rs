use super::t::*;

static PREFIX: &'static str = "(";
static SUFFIX: &'static str = ")";

pub fn parse(exp: &'static str) -> Result<List, String> {
    Ok(parse0(&exp))
}

fn parse0(exp: &'static str) -> List {
    let mut stack = Vec::new();
    let mut next = true;
    let mut exp = exp.clone();
    while next {
        exp = exp.trim();
        let is_push = exp.starts_with(PREFIX);
        let next_exp = exp[1..].trim();
        let to_index = get_to_index(next_exp);
        next = next_exp.find(SUFFIX)!=None;
        let sub_exp = &next_exp[..to_index];
        // println!("sub_exp: {}", sub_exp);
        if (is_push) {
            let mut expr = List::new();
            expr.push_all(parse_list(sub_exp));
            stack.push(expr);
        } else {
            let brother = stack.pop().unwrap();
            if(stack.is_empty()){
                stack.push( brother);
            }else{
                let mut parent = stack.pop().unwrap();
                parent.push(LispType::Expr(brother));
                parent.push_all(parse_list(sub_exp));
                stack.push(parent);
            }
        }

        // println!("stack: {}", stack.len());
        // println!("-----");
        exp = exp[to_index + 1..].trim();
    }
    stack.pop().unwrap()
}

fn get_to_index(next_exp: &str)-> usize {
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

fn parse_list(exp: &'static str) -> Vec< LispType> {
    exp.split_whitespace()
        .map(|s| parse_atom(s).unwrap())
        .collect()
}

fn parse_atom(s: &str) -> Result<LispType, String> {
    let t: LispType = match s {
        "nil" => LispType::Nil,
        "#t" => LispType::Boolean(true),
        "#f" => LispType::Boolean(false),
        _ => {
            if s.starts_with("\'") && s.ends_with("\'") && s.len() > 2 {
                LispType::Char(s.chars().nth(1).unwrap())
            } else if s.parse::<i32>().is_ok() {
                LispType::Number(s.parse::<i32>().unwrap())
            }else {
                LispType::Symbol(s.to_string())
            }
        }
    };
    Ok(t)
}