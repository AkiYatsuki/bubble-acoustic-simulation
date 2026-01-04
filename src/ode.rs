// ode.rs

/// 通用 ODE 系统接口
///
/// x' = f(t, x)
///
/// 设计目标：
/// - 不分配内存
/// - 允许小维度状态（2–几十）
pub trait ODESystem {
    /// 状态维度
    fn dim(&self) -> usize;

    /// 计算导数 dx/dt
    ///
    /// - t: 当前时间
    /// - x: 当前状态 (length = dim)
    /// - dx: 输出导数 (length = dim)
    fn eval(&self, t: f64, x: &[f64], dx: &mut [f64]);
}