fn main() {}

type Function = fn(f64) -> f64;

fn find_root(f0: Function, f1: Function, x0: f64, eps: f64, n: usize) -> Option<f64> {
    let mut x = x0;

    for _ in 0..n {
        let y0 = f0(x);
        let y1 = f1(x);

        if y1 == 0.0 {
            return None;
        }

        let x1 = x - y0 / y1;
        if (x1 - x).abs() < eps {
            return Some(x1);
        }

        x = x1;
    }

    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn root_of_x_cube_plus_x_at_plus_1() {
        let f0 = |x: f64| x * x * x + x;
        let f1 = |x: f64| 3.0 * x * x + 1.0;

        match super::find_root(f0, f1, 1.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_plus_x_at_minus_1() {
        let f0 = |x: f64| x * x * x + x;
        let f1 = |x: f64| 3.0 * x * x + 1.0;

        match super::find_root(f0, f1, -1.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_plus_x_at_plus_100() {
        let f0 = |x: f64| x * x * x + x;
        let f1 = |x: f64| 3.0 * x * x + 1.0;

        match super::find_root(f0, f1, 100.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_2() {
        let f0 = |x: f64| x * x * x - 2.0;
        let f1 = |x: f64| 3.0 * x * x;

        match super::find_root(f0, f1, 1.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 2.0f64.powf(1.0/3.0)).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_right() {
        let f0 = |x: f64| x * x * x - x;
        let f1 = |x: f64| 3.0 * x * x - 1.0;

        match super::find_root(f0, f1, 1.46, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 1.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_left() {
        let f0 = |x: f64| x * x * x - x;
        let f1 = |x: f64| 3.0 * x * x - 1.0;

        match super::find_root(f0, f1, -1.46, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root + 1.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_left_between() {
        let f0 = |x: f64| x * x * x - x;
        let f1 = |x: f64| 3.0 * x * x - 1.0;

        match super::find_root(f0, f1, -0.16, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_right_between() {
        let f0 = |x: f64| x * x * x - x;
        let f1 = |x: f64| 3.0 * x * x - 1.0;

        match super::find_root(f0, f1, 0.16, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-6),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_1_by_sqrt_5() {
        let f0 = |x: f64| x * x * x - x;
        let f1 = |x: f64| 3.0 * x * x - 1.0;

        match super::find_root(f0, f1, 1.0 / 5.0f64.sqrt(), 1e-6, 100) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }


    #[test]
    fn root_of_x_minus_1_square_at_1_0001() {
        let f0 = |x: f64| (x - 1.0) * (x - 1.0);
        let f1 = |x: f64| 2.0 * (x - 1.0);

        match super::find_root(f0, f1, 1.0001, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 1.0).abs() < 1e-6),
        };
    }
}
