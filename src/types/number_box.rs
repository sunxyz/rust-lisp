use std::{ops::{Add, Sub, Mul, Div, Rem}, fmt::{Display, Formatter, Error, Debug}};

pub enum NumberBox {
    Integer(isize),
    Float(f64),
}
impl Add for NumberBox {
    type Output = NumberBox;
    fn add(self, other: NumberBox) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Integer(n + m);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n as f64 + m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Float(n + m as f64);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n + m);
                    }
                }
            }
        }
    }
}

impl Sub for NumberBox {
    type Output = NumberBox;
    fn sub(self, other: NumberBox) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Integer(n - m);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n as f64 - m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Float(n - m as f64);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n - m);
                    }
                }
            }
        }
    }
}

impl Mul for NumberBox {
    type Output = NumberBox;
    fn mul(self, other: NumberBox) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Integer(n * m);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n as f64 * m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Float(n * m as f64);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n * m);
                    }
                }
            }
        }
    }
}

impl Div for NumberBox {
    type Output = NumberBox;
    fn div(self, other: NumberBox) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Integer(n / m);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n as f64 / m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Float(n / m as f64);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n / m);
                    }
                }
            }
        }
    }
}
 

impl Rem for NumberBox {
    type Output = NumberBox;
    fn rem(self, other: NumberBox) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Integer(n % m);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n as f64 % m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return NumberBox::Float(n % m as f64);
                    }
                    NumberBox::Float(m) => {
                        return NumberBox::Float(n % m);
                    }
                }
            }
        }
    }
}

impl Clone for NumberBox {
    fn clone(&self) -> NumberBox {
        match self {
            NumberBox::Integer(n) => {
                return NumberBox::Integer(n.clone());
            }
            NumberBox::Float(n) => {
                return NumberBox::Float(n.clone());
            }
        }
    }
}

impl PartialEq for NumberBox {
    fn eq(&self, other: &NumberBox) -> bool {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return n == m;
                    }
                    NumberBox::Float(m) => {
                        return *n as f64 == *m;
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return *n == *m as f64;
                    }
                    NumberBox::Float(m) => {
                        return n == m;
                    }
                }
            }
        }
    }
}

impl PartialOrd for NumberBox {
    fn partial_cmp(&self, other: &NumberBox) -> Option<std::cmp::Ordering> {
        match self {
            NumberBox::Integer(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return n.partial_cmp(m);
                    }
                    NumberBox::Float(m) => {
                        return (*n as f64).partial_cmp(m);
                    }
                }
            }
            NumberBox::Float(n) => {
                match other {
                    NumberBox::Integer(m) => {
                        return n.partial_cmp(&(*m as f64));
                    }
                    NumberBox::Float(m) => {
                        return n.partial_cmp(m);
                    }
                }
            }
        }
    }
}

impl Display for NumberBox {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            NumberBox::Integer(n) => {
                return write!(f, "{}", n);
            }
            NumberBox::Float(n) => {
                return write!(f, "{}", n);
            }
        }
    }
}

impl Debug for NumberBox {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            NumberBox::Integer(n) => {
                return write!(f, "{}", n);
            }
            NumberBox::Float(n) => {
                return write!(f, "{}", n);
            }
        }
    }
}