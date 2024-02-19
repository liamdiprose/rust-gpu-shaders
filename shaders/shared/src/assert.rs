#[macro_export]
macro_rules! assert_similar {
    ($x:expr, $y:expr, $e:expr) => {
        let delta = ($x - $y).abs();
        if $x.is_nan() || $y.is_nan() || delta > $e {
            panic!(
                "assertion `left ~= right` failed
         left: {}
        right: {}
allowed error: {}
 actual error: {}",
                $x,
                $y,
                $e,
                (delta - $e).abs()
            )
        }
    };
    ($x:expr, $y:expr) => {
        let delta = ($x - $y).abs();
        if $x.is_nan() || $y.is_nan() || delta > f32::EPSILON {
            panic!(
                "assertion `(left - right).abs() < ` failed
         left: {}
        right: {}
allowed error: {}
 actual error: {}",
                $x,
                $y,
                e,
                (delta - e).abs()
            )
        }
    };
}
