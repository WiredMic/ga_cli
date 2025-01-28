#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub enum Multiplication {
    Positive,
    Negative,
    Zero,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Einheit {
    dimension: u32,
    mul: Multiplication,
}

impl Einheit {
    pub fn new(dimension: u32, mul: Multiplication) -> Self {
        Einheit { dimension, mul }
    }
}

impl Mul for Einheit {
    type Output = Element;
    fn mul(self, rhs: Self) -> Element {
        if self.dimension == rhs.dimension {
            let scalar: f64 = match self.mul {
                Multiplication::Positive => 1.0,
                Multiplication::Negative => -1.0,
                Multiplication::Zero => 0.0,
            };
            return Element::new(scalar, None);
        }
        let unit = vec![self, rhs];
        return Element::new(1.0, Some(unit));
    }
}

impl Mul<f64> for Einheit {
    type Output = Element;

    fn mul(self, rhs: f64) -> Element {
        Element::new(rhs, Some(vec![self]))
    }
}

impl Mul<Einheit> for f64 {
    type Output = Element;

    fn mul(self, rhs: Einheit) -> Element {
        Element::new(self, Some(vec![rhs]))
    }
}

#[cfg(test)]
mod mul_einheit {
    use super::*;
    use Multiplication::{Negative, Positive, Zero};

    #[test]
    fn test_positive() {
        let e1 = Einheit::new(1, Positive);
        let res = Element::new(1.0, None);
        assert_eq!(e1.clone() * e1, res);
    }

    #[test]
    fn test_negative() {
        let e1 = Einheit::new(1, Negative);
        let res = Element::new(-1.0, None);
        assert_eq!(e1.clone() * e1, res);
    }

    #[test]
    fn test_zero() {
        let e1 = Einheit::new(1, Zero);
        let res = Element::new(0.0, None);
        assert_eq!(e1.clone() * e1, res);
    }

    #[test]
    fn test_different_dimensions() {
        let e1 = Einheit::new(1, Positive);
        let e2 = Einheit::new(2, Positive);
        let res = Element::new(1.0, Some(vec![e1.clone(), e2.clone()]));
        assert_eq!(e1 * e2, res);
    }

    #[test]
    fn scalar_einheit() {
        let e1 = Einheit::new(1, Positive);
        let res = Element::new(3.6, Some(vec![e1.clone()]));
        assert_eq!(3.6 * e1.clone(), res);
        assert_eq!(e1 * 3.6, res);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    scalar: f64,
    unit: Option<Vec<Einheit>>,
}

impl Element {
    pub fn new(scalar: f64, unit: Option<Vec<Einheit>>) -> Self {
        Element { scalar, unit }
    }
}

impl Mul<f64> for Element {
    type Output = Element;

    fn mul(self, rhs: f64) -> Element {
        Element::new(rhs * self.scalar, self.unit)
    }
}

impl Mul<Element> for f64 {
    type Output = Element;

    fn mul(self, rhs: Element) -> Element {
        Element::new(self * rhs.scalar, rhs.unit)
    }
}

impl Mul<Einheit> for Element {
    type Output = Element;

    fn mul(self, rhs: Einheit) -> Element {
        match self.unit {
            Some(mut unit) => {
                // Check for matching dimensions
                for (index, einheit) in unit.clone().iter().rev().enumerate() {
                    if rhs.dimension == einheit.dimension {
                        let scalar = match rhs.mul {
                            Multiplication::Positive => 1.0,
                            Multiplication::Negative => -1.0,
                            Multiplication::Zero => 0.0,
                        };
                        // test if unit vector are next to each other
                        // Other wise switch places
                        // \[ \mathrm{e}_i \mathrm{e}_j = -\mathrm{e}_j \mathrm{e}_i\]
                        let a = match index % 2 {
                            0 => 1.0 * scalar,
                            1 => -1.0 * scalar,
                            _ => 1.0,
                        };
                        unit.reverse();
                        unit.remove(index);
                        unit.reverse();

                        if unit.is_empty() || a == 0.0 {
                            return Element::new(a, None);
                        }
                        return Element::new(a, Some(unit));
                    }
                }
                // No matching dimension found, just push
                unit.push(rhs);
                Element::new(self.scalar, Some(unit))
            }
            None => Element::new(self.scalar, Some(vec![rhs])),
        }
    }
}

impl Mul<Element> for Einheit {
    type Output = Element;

    fn mul(self, rhs: Element) -> Element {
        match rhs.unit {
            Some(mut unit) => {
                for (index, einheit) in unit.clone().iter().enumerate() {
                    if self.dimension == einheit.dimension {
                        let scalar: f64 = match self.mul {
                            Multiplication::Positive => 1.0,
                            Multiplication::Negative => -1.0,
                            Multiplication::Zero => 0.0,
                        };
                        // test if unit vector are next to each other
                        // Other wise switch places
                        // \[ \mathrm{e}_i \mathrm{e}_j = -\mathrm{e}_j \mathrm{e}_i\]
                        let a = match index % 2 {
                            0 => 1.0 * scalar,
                            1 => -1.0 * scalar,
                            _ => 1.0,
                        };
                        unit.remove(index);

                        if unit.is_empty() || a == 0.0 {
                            return Element::new(a, None);
                        }

                        return Element::new(scalar, Some(unit));
                    }
                }
                let mut tmp = vec![self];
                tmp.append(&mut unit);
                return Element::new(rhs.scalar, Some(tmp));
            }

            None => Element::new(rhs.scalar, Some(vec![self])),
        }
    }
}

#[cfg(test)]
mod mul_element {
    use super::*;
    use Multiplication::{Negative, Positive, Zero};

    #[test]
    fn scalar_element_mul() {
        let test = Element::new(1.0, None);
        let res = Element::new(6.0, None);
        assert_eq!(test.clone() * 6.0, res);
        assert_eq!(6.0 * test, res);
    }

    #[test]
    fn einheit_element_empty_mul() {
        // one unit vector and no unt vector
        let e1 = Einheit::new(2, Positive);
        let element = Element::new(1.0, None);
        let res = Element::new(1.0, Some(vec![e1.clone()]));
        assert_eq!(e1.clone() * element.clone(), res);
        assert_eq!(element.clone() * e1.clone(), res);
    }

    #[test]
    fn einheit_element_diffenet_mul() {
        // two different unit vectors
        let e1 = Einheit::new(1, Positive);
        let e2 = Einheit::new(2, Positive);
        let element = Element::new(1.0, Some(vec![e2.clone()]));
        let res1 = Element::new(1.0, Some(vec![e1.clone(), e2.clone()]));
        let res2 = Element::new(1.0, Some(vec![e2.clone(), e1.clone()]));
        assert_eq!(e1.clone() * element.clone(), res1);
        assert_eq!(element * e1.clone(), res2);
    }

    #[test]
    fn einheit_element_same_mul() {
        let e1 = Einheit::new(1, Positive);
        let element = Element::new(1.0, Some(vec![e1.clone()]));
        let res = Element::new(1.0, None);
        assert_eq!(e1.clone() * element.clone(), res);
        assert_eq!(element.clone() * e1.clone(), res);
    }

    #[test]
    fn einheit_element_dual() {
        let e1 = Einheit::new(1, Positive);
        let e2 = Einheit::new(2, Positive);
        let e3 = Einheit::new(3, Positive);
        let e4 = Einheit::new(4, Positive);
        let dual = Element::new(
            1.0,
            Some(vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()]),
        );
        let res = Element::new(1.0, Some(vec![e2.clone(), e3.clone(), e4.clone()]));
        let res1 = Element::new(-1.0, Some(vec![e2, e3, e4]));
        assert_eq!(e1.clone() * dual.clone(), res);
        assert_eq!(dual.clone() * e1.clone(), res1);
    }

    #[test]
    fn einheit_element_zero() {
        let e1 = Einheit::new(1, Positive);
        let e2 = Einheit::new(2, Zero);
        let element = Element::new(1.0, Some(vec![e1.clone(), e2.clone()]));
        let res = Element::new(0.0, None);

        assert_eq!(e2.clone() * element.clone(), res);
        assert_eq!(element.clone() * e2.clone(), res);
    }
}

pub struct Blade {
    element: Vec<Element>,
}

impl Blade {
    pub fn new(element: Vec<Element>) -> Self {
        Blade { element }
    }
}

pub struct Multivector {
    blade: Vec<Blade>,
}

impl Multivector {
    pub fn new(blade: Vec<Blade>) -> Self {
        Multivector { blade }
    }
}
