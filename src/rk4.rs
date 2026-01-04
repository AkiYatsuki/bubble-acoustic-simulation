// rk4.rs

use crate::ode::ODESystem;

/// 四阶 Runge–Kutta 积分器
pub struct RK4 {
    k1: Vec<f64>,
    k2: Vec<f64>,
    k3: Vec<f64>,
    k4: Vec<f64>,
    tmp: Vec<f64>,
}

impl RK4 {
    /// 创建 RK4 积分器
    pub fn new(dim: usize) -> Self {
        Self {
            k1: vec![0.0; dim],
            k2: vec![0.0; dim],
            k3: vec![0.0; dim],
            k4: vec![0.0; dim],
            tmp: vec![0.0; dim],
        }
    }

    /// 单步积分
    ///
    /// x(t + dt) = RK4(x(t))
    pub fn step<S: ODESystem>(
        &mut self,
        system: &S,
        t: f64,
        dt: f64,
        x: &mut [f64],
    ) {
        let n = x.len();
        debug_assert_eq!(n, system.dim());

        // k1 = f(t, x)
        system.eval(t, x, &mut self.k1);

        // k2 = f(t + dt/2, x + dt/2 * k1)
        for i in 0..n {
            self.tmp[i] = x[i] + 0.5 * dt * self.k1[i];
        }
        system.eval(t + 0.5 * dt, &self.tmp, &mut self.k2);

        // k3 = f(t + dt/2, x + dt/2 * k2)
        for i in 0..n {
            self.tmp[i] = x[i] + 0.5 * dt * self.k2[i];
        }
        system.eval(t + 0.5 * dt, &self.tmp, &mut self.k3);

        // k4 = f(t + dt, x + dt * k3)
        for i in 0..n {
            self.tmp[i] = x[i] + dt * self.k3[i];
        }
        system.eval(t + dt, &self.tmp, &mut self.k4);

        // x += dt / 6 * (k1 + 2k2 + 2k3 + k4)
        for i in 0..n {
            x[i] += dt / 6.0
                * (self.k1[i]
                    + 2.0 * self.k2[i]
                    + 2.0 * self.k3[i]
                    + self.k4[i]);
        }
    }
}