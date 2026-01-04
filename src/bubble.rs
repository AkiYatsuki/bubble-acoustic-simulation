use crate::ode::ODESystem; 

pub struct Bubble {
    //liquid density(kg/m^3)
    pub rho: f64, 
    //surface tension(N/m)
    pub sigma: f64, 
    //
    pub mu: f64, 

    pub p_inf: f64, 

    pub r0: f64, 

    pub p0: f64, 

    pub gamma: f64, 

}

impl Bubble {
    #[inline]
    fn gas_pressure(&self, r: f64) -> f64{
        //P_g(R) = P0 * (R0 / R)^(3gamma)
        self.p0 * (self.r0 / r).powf(3.0 * self.gamma)
    }
}

impl ODESystem for Bubble {
    fn dim(&self) -> usize {
        2
    }

    fn eval(&self, _t: f64, x: &[f64], dx: &mut [f64]) {
        let r = x[0]; 
        let r_dot = x[1]; 
        //data hazard: 
        let r = r.max(1e-8); 

        let p_g = self.gas_pressure(r); 

        let rhs = (p_g - self.p_inf
                    - 2.0 * self.sigma / r
                    - 4.0 * self.mu* r_dot /r) / self.rho; 
        //dx
        dx[0] = r_dot; 
        //ODE -> ddx
        dx[1] = (rhs - 1.5 * r_dot * r_dot) / r
    }
}