use std::mem::swap;

use cast::usize;

type Point = (f32, f32);

/// An implementation of [Xiaolin Wu's line algorithm].
///
/// This algorithm works based on floating-points and returns an extra variable for how much a
/// a point is covered, which is useful for anti-aliasing.
///
/// Note that due to the implementation, the returned line will always go from left to right.
///
/// [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub struct XiaolinWu {
    steep: bool,
    gradient: f32,
    x: usize,
    y: f32,
    end_x: usize,
    lower: bool,
}

impl XiaolinWu {
    #[inline]
    pub fn new(mut start: Point, mut end: Point) -> Self {
        let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

        if steep {
            start = (start.1, start.0);
            end = (end.1, end.0);
        }

        if start.0 > end.0 {
            swap(&mut start, &mut end);
        }

        let dx = end.0 - start.0;
        let gradient = if 0. == dx { 1. } else { (end.1 - start.1) / dx };

        Self {
            steep,
            gradient,
            x: usize(start.0.round()).unwrap(),
            y: start.1,
            end_x: usize(end.0.round()).unwrap(),
            lower: false,
        }
    }
}

impl Iterator for XiaolinWu {
    type Item = ((usize, usize), f32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end_x {
            // get the fractional part of y
            let fpart = self.y - self.y.floor();

            // Calculate the integer value of y
            let mut y = usize(self.y).unwrap();
            if self.lower {
                y += 1;
            }

            // Get the point
            let point = if self.steep { (y, self.x) } else { (self.x, y) };

            if self.lower {
                // Return the lower point
                self.lower = false;
                self.x += 1;
                self.y += self.gradient;
                Some((point, fpart))
            } else {
                if fpart > 0. {
                    // Set to return the lower point if the fractional part is > 0
                    self.lower = true;
                } else {
                    // Otherwise move on
                    self.x += 1;
                    self.y += self.gradient;
                }

                // Return the remainder of the fractional part
                Some((point, 1. - fpart))
            }
        } else {
            None
        }
    }
}

#[test]
fn tests() {
    let xiaolin_wu = |a, b| XiaolinWu::new(a, b).collect::<Vec<_>>();

    assert_eq!(
        xiaolin_wu((0.0, 0.0), (6.0, 3.0)),
        [
            ((0, 0), 1.0),
            ((1, 0), 0.5),
            ((1, 1), 0.5),
            ((2, 1), 1.0),
            ((3, 1), 0.5),
            ((3, 2), 0.5),
            ((4, 2), 1.0),
            ((5, 2), 0.5),
            ((5, 3), 0.5),
            ((6, 3), 1.0)
        ]
    );

    assert_eq!(
        xiaolin_wu((4.0, 2.0), (4.0, 6.0)),
        [
            ((4, 2), 1.0),
            ((4, 3), 1.0),
            ((4, 4), 1.0),
            ((4, 5), 1.0),
            ((4, 6), 1.0),
        ]
    );

    assert_eq!(
        xiaolin_wu((2.0, 4.0), (6.0, 4.0)),
        [
            ((2, 4), 1.0),
            ((3, 4), 1.0),
            ((4, 4), 1.0),
            ((5, 4), 1.0),
            ((6, 4), 1.0),
        ]
    );

    // The algorithm reorders the points to be left-to-right

    assert_eq!(
        xiaolin_wu((340.5, 290.77), (110.0, 170.0)),
        xiaolin_wu((110.0, 170.0), (340.5, 290.77))
    );
}
