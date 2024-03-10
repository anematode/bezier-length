
#[repr(C)]
pub struct BezierPoints {
  a: [f64; 2]
  b: [f64; 2]
  c: [f64; 2]
  d: [f64; 2]
}

#[repr(C)]
pub struct CanonicalizedBezierPoints {
  b: [f64; 2]
  c: [f64; 2]
};

pub struct CanonicalizedBezierInfo {
  pts: CanonicalizedBezierPoints
  scale: f64
}

#[inline(always)]
pub fn canonicalize_bezier(bez: &BezierPoints) -> CanonicalizedBezierInfo { 

}

pub fn decastiljau(bez: &CanonicalizedBezierPoints, split_t: f64) -> (CanonicalizedBezierInfo, CanonicalizedBezierInfo) {
  debug_assert!(split_t >= 0.0 && split_t <= 1.0);


}

/// Computes the roots in a numerically stable manner
pub fn quadratic_roots(a: f64, b: f64, c: f64, r1: &mut f64, r2: &mut f64) -> bool {

}

/// Returns t values at the extrema of kinks along with (bx(t), by(t)) as computed there
pub fn kinky_t(bez: &BezierPoints) -> [(f64, f64); 2] {

}

pub fn kinky_t_canonical(bez: &CanonicalizedBezierPoints) {

}
