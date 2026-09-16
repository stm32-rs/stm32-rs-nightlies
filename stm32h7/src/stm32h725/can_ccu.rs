#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crel: CREL,
    ccfg: CCFG,
    cstat: CSTAT,
    cwd: CWD,
    ir: IR,
    ie: IE,
}
impl RegisterBlock {
    ///0x00 - Clock Calibration Unit Core Release Register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    ///0x04 - Calibration Configuration Register
    #[inline(always)]
    pub const fn ccfg(&self) -> &CCFG {
        &self.ccfg
    }
    ///0x08 - Calibration Status Register
    #[inline(always)]
    pub const fn cstat(&self) -> &CSTAT {
        &self.cstat
    }
    ///0x0c - Calibration Watchdog Register
    #[inline(always)]
    pub const fn cwd(&self) -> &CWD {
        &self.cwd
    }
    ///0x10 - Clock Calibration Unit Interrupt Register
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x14 - Clock Calibration Unit Interrupt Enable Register
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
}
/**CREL (rw) register accessor: Clock Calibration Unit Core Release Register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///Clock Calibration Unit Core Release Register
pub mod crel;
/**CCFG (rw) register accessor: Calibration Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:CCFG)

For information about available fields see [`mod@ccfg`] module*/
pub type CCFG = crate::Reg<ccfg::CCFGrs>;
///Calibration Configuration Register
pub mod ccfg;
/**CSTAT (rw) register accessor: Calibration Status Register

You can [`read`](crate::Reg::read) this register and get [`cstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:CSTAT)

For information about available fields see [`mod@cstat`] module*/
pub type CSTAT = crate::Reg<cstat::CSTATrs>;
///Calibration Status Register
pub mod cstat;
/**CWD (rw) register accessor: Calibration Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`cwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:CWD)

For information about available fields see [`mod@cwd`] module*/
pub type CWD = crate::Reg<cwd::CWDrs>;
///Calibration Watchdog Register
pub mod cwd;
/**IR (rw) register accessor: Clock Calibration Unit Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///Clock Calibration Unit Interrupt Register
pub mod ir;
/**IE (rw) register accessor: Clock Calibration Unit Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///Clock Calibration Unit Interrupt Enable Register
pub mod ie;
