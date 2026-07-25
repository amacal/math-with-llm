fn main() {
    println!("root of x^3 - x at 5.0 = {:?}", find_root(|x| x * x * x - x, 0.01, 8.0, 1e-6, 100));
}

type Function = fn(f64) -> f64;

fn find_root(f0: Function, x0: f64, x1: f64, eps: f64, n: usize) -> Option<f64> {
    let mut x0 = x0;
    let mut x2 = x1;

    if x2 == x0 {
        return None;
    }

    let mut y0 = f0(x0);
    let mut y2 = f0(x2);

    if y0 == 0.0 {
        return Some(x0);
    }

    if y2 == 0.0 {
        return Some(x2);
    }

    if (y0 > 0.0) == (y2 > 0.0) {
        return None;
    }

    if x0 > x2 {
        (x0, x2) = (x2, x0);
        (y0, y2) = (y2, y0);
    }

    let y_diff = y2 - y0;
    let x_diff = x2 - x0;

    if y_diff == 0.0 {
        return None;
    }

    let mut x1 = x2 - y2 * x_diff / y_diff;
    let mut y1 = f0(x1);

    let mut distance = (x2 - x0).abs();
    let mut guess = x1;

    for _ in 0..n {
        let mut reset = false;

        let d0 = (y0 - y1) * (y0 - y2);
        let d1 = (y1 - y0) * (y1 - y2);
        let d2 = (y2 - y0) * (y2 - y1);

        // check for zero denominators
        if d0 == 0.0 || d1 == 0.0 || d2 == 0.0 {
            reset = true;
        }

        if reset == false {
            let l0 = x0 * y1 * y2 / d0;
            let l1 = x1 * y0 * y2 / d1;
            let l2 = x2 * y0 * y1 / d2;

            // check if the new guess is too far from the previous guess
            let x = l0 + l1 + l2;
            if (x - x1).abs() >= 0.5 * distance {
                reset = true;
            } else {
                distance = (x - guess).abs();
                guess = x;
            }

            x1 = x;
            y1 = f0(x1);

            if y1 == 0.0 {
                return Some(x1);
            }
        }

        // ensure that x1 is between x0 and x2
        if reset || x1 < x0 || x1 > x2 {
            x1 = (x0 + x2) / 2.0;
            y1 = f0(x1);
        }

        // ensure y0 and y2 have opposite signs
        if (y1 > 0.0) == (y0 > 0.0) {
            (x0, y0, x1, y1) = (x1, y1, x0, y0);
        } else if (y1 > 0.0) == (y2 > 0.0) {
            (x2, y2, x1, y1) = (x1, y1, x2, y2);
        }

        if (x0 - x2).abs() < eps {
            return Some((x2 - x0) / 2.0 + x0);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn root_of_x_cube_plus_x_between_10_and_10() {
        let f0 = |x: f64| x * x * x + x;

        match super::find_root(f0, -10.0, 10.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_2_between_10_and_10() {
        let f0 = |x: f64| x * x * x - 2.0;

        match super::find_root(f0, -10.0, 10.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 2.0f64.powf(1.0 / 3.0)).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_square_minus_2_between_0_and_10() {
        let f0 = |x: f64| x * x - 2.0;

        match super::find_root(f0, 0.0, 10.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 2.0f64.sqrt()).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_square_minus_2_between_10_and_00() {
        let f0 = |x: f64| x * x - 2.0;

        match super::find_root(f0, -10.0, 0.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root + 2.0f64.sqrt()).abs() < 1e-5),
        };
    }

    #[test]
    fn edge_wrong_brackets() {
        let f0 = |x: f64| x * x - 2.0;

        match super::find_root(f0, 2.0, 3.0, 1e-6, 100) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }

    #[test]
    fn edge_extreme_roots_right() {
        let f0 = |x: f64| (x - 0.0000001) * (x - 10000000.0);

        match super::find_root(f0, 0.0000002, 20000000.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 10000000.0).abs() < 1e-5),
        };
    }

    #[test]
    fn edge_extreme_roots_left() {
        let f0 = |x: f64| (x - 0.0000001) * (x - 10000000.0);

        match super::find_root(f0, -20000000.0, 0.0000002, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0000001).abs() < 1e-5),
        };
    }
}
