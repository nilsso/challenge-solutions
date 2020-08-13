#![feature(bool_to_option)]

use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    const PYTHAGORAS_CONSTANT: f32 = 1.41421356237;
    const F: f32 = 1.0 - 1.0 / PYTHAGORAS_CONSTANT;

    (1..(F * n as f32).ceil() as u32)
        .filter_map(|a| {
            let b = n * (n - 2 * a) / (n - a) / 2;
            let c = n - a - b;
            (a < b && a * a + b * b == c * c).then_some([a, b, c])
        })
        .collect()
}

// Mathematically...
//
// 1. We have three variables ($n$ is fixed)
// 2. We have two constraints
//     - $a^2 + b^2 = c^2$
//     - $a + b + c = n$
//     - (actually also $a < b < c$)
//
// Thus we should be able to fix two of the variables dependent on a single free variable.
// We can solve for $c$ in the second and substitute into the first to reduce the first
// to an equation of two variables, and solving for $b$ allows $a$ to be free:
//
// \begin{gather*}
//   c = n - a - b \\
//   \begin{aligned}
//     a^2 + b^2
//     &= (n - a - b)^2 \\
//     2nb - 2ab &= n^2 - 2na \\
//     2b(n - a) &= n(n - 2a) \Rightarrow
//     b = \frac{n(n - 2a)}{2(n - a)}.
//   \end{aligned}
// \end{gather*}
//
// A narrow upperbound for $a$ can be found considering $a < b = \frac{n(n - 2a)}{2(n - a)}$.
// Solving for solutions for $a = b$:
//
// \begin{gather*}
//   \begin{aligned}
//     0
//     &= -a + \frac{n(n - 2a)}{2(n - a)} \\
//     &= \frac{2a^2 - 4na + n^2}{2(n-a)} \\
//     &= a^2 - 2na + n^2 + \frac{n^2}{2} - n^2 \\
//     &= (a - n)^2 - \frac{n^2}{2}. \\
//   \end{aligned} \\
//   (a - n)^2 = \frac{n^2}{2} \Rightarrow
//   a - n = \pm\frac{n}{\sqrt 2} \Rightarrow
//   a = n \pm \frac{n}{\sqrt 2} = n(1 \pm \frac{1}{\sqrt 2}).
// \end{gather*}
//
// Since necessarily $a < n$, we take the solution $a < n(1 \pm \frac{1}{\sqrt 2})$
// as our upper bound.
