use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, RemAssign, Sub, SubAssign},
};

use bevy::ecs::system::Resource;

#[derive(Resource, Clone, PartialEq, Eq)]
pub(crate) struct Money {
    dollars: Dollar,
    cents: Cent,
}
impl Money {
    pub(crate) const fn new(dollars: u64, cents: u8) -> Self {
        Self {
            dollars: Dollar(dollars),
            cents: Cent(cents),
        }
    }
}
impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.cents;
        *self += rhs.dollars;
    }
}
impl AddAssign<&Self> for Money {
    fn add_assign(&mut self, rhs: &Self) {
        *self += &rhs.cents;
        *self += &rhs.dollars;
    }
}
impl AddAssign<Cent> for Money {
    fn add_assign(&mut self, rhs: Cent) {
        self.cents += rhs;
        if self.cents > Cent(100) {
            self.dollars += Dollar::from(&self.cents);
            self.cents %= Cent(100);
        }
    }
}
impl AddAssign<&Cent> for Money {
    fn add_assign(&mut self, rhs: &Cent) {
        self.cents += rhs;
        if self.cents > Cent(100) {
            self.dollars += Dollar::from(&self.cents);
            self.cents %= Cent(100);
        }
    }
}
impl AddAssign<Dollar> for Money {
    fn add_assign(&mut self, rhs: Dollar) {
        self.dollars += rhs;
    }
}
impl AddAssign<&Dollar> for Money {
    fn add_assign(&mut self, rhs: &Dollar) {
        self.dollars += rhs;
    }
}
impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        if rhs.ge(self) {
            self.dollars = Dollar(0);
            self.cents = Cent(0);
        } else if rhs.cents > self.cents {
            self.dollars -= Dollar(1) + rhs.dollars;
            self.cents += Cent(100) - rhs.cents;
        } else {
            self.dollars -= rhs.dollars;
            self.cents -= rhs.cents;
        }
    }
}
impl SubAssign<&Self> for Money {
    fn sub_assign(&mut self, rhs: &Self) {
        if rhs.ge(self) {
            self.dollars = Dollar(0);
            self.cents = Cent(0);
        } else if rhs.cents > self.cents {
            self.dollars -= Dollar(1) + &rhs.dollars;
            self.cents += Cent(100) - &rhs.cents;
        } else {
            self.dollars -= &rhs.dollars;
            self.cents -= &rhs.cents;
        }
    }
}
impl PartialOrd for Money {
    fn lt(&self, other: &Self) -> bool {
        if self.dollars.lt(&other.dollars) {
            true
        } else if self.dollars.eq(&other.dollars) {
            self.cents.lt(&other.cents)
        } else {
            false
        }
    }

    fn le(&self, other: &Self) -> bool {
        if self.dollars.lt(&other.dollars) {
            true
        } else if self.dollars.eq(&other.dollars) {
            self.cents.le(&other.cents)
        } else {
            false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        if self.dollars.gt(&other.dollars) {
            true
        } else if self.dollars.eq(&other.dollars) {
            self.cents.gt(&other.cents)
        } else {
            false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        if self.dollars.gt(&other.dollars) {
            true
        } else if self.dollars.eq(&other.dollars) {
            self.cents.ge(&other.cents)
        } else {
            false
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) {
            Some(Ordering::Less)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl Default for Money {
    fn default() -> Self {
        Self {
            dollars: Dollar(90),
            cents: Cent(0),
        }
    }
}
impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.dollars,
            if self.cents < Cent(10) {
                format!("0{}", self.cents)
            } else {
                self.cents.to_string()
            }
        )
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Dollar(u64);
impl AddAssign for Dollar {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}
impl AddAssign<&Self> for Dollar {
    fn add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0);
    }
}
impl Add for Dollar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}
impl Add<&Self> for Dollar {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}
impl SubAssign for Dollar {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0);
    }
}
impl SubAssign<&Self> for Dollar {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0);
    }
}
impl From<&Cent> for Dollar {
    fn from(value: &Cent) -> Self {
        Self(u64::from(value.0 / 100))
    }
}
impl Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cent(u8);
impl AddAssign for Cent {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}
impl AddAssign<&Self> for Cent {
    fn add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0);
    }
}
impl SubAssign for Cent {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0);
    }
}
impl SubAssign<&Self> for Cent {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0);
    }
}
impl Sub for Cent {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}
impl Sub<&Self> for Cent {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}
impl Div for Cent {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}
impl RemAssign for Cent {
    fn rem_assign(&mut self, rhs: Self) {
        self.0.rem_assign(rhs.0);
    }
}
impl Display for Cent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
