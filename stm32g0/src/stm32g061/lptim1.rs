#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    lptim_isr: LPTIM_ISR,
    lptim_icr: LPTIM_ICR,
    lptim_ier: LPTIM_IER,
    lptim_cfgr: LPTIM_CFGR,
    lptim_cr: LPTIM_CR,
    lptim_cmp: LPTIM_CMP,
    lptim_arr: LPTIM_ARR,
    lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 0x04],
    lptim_cfgr2: LPTIM_CFGR2,
}
impl RegisterBlock {
    ///0x00 - Interrupt and Status Register
    #[inline(always)]
    pub const fn lptim_isr(&self) -> &LPTIM_ISR {
        &self.lptim_isr
    }
    ///0x04 - Interrupt Clear Register
    #[inline(always)]
    pub const fn lptim_icr(&self) -> &LPTIM_ICR {
        &self.lptim_icr
    }
    ///0x08 - Interrupt Enable Register
    #[inline(always)]
    pub const fn lptim_ier(&self) -> &LPTIM_IER {
        &self.lptim_ier
    }
    ///0x0c - Configuration Register
    #[inline(always)]
    pub const fn lptim_cfgr(&self) -> &LPTIM_CFGR {
        &self.lptim_cfgr
    }
    ///0x10 - Control Register
    #[inline(always)]
    pub const fn lptim_cr(&self) -> &LPTIM_CR {
        &self.lptim_cr
    }
    ///0x14 - Compare Register
    #[inline(always)]
    pub const fn lptim_cmp(&self) -> &LPTIM_CMP {
        &self.lptim_cmp
    }
    ///0x18 - Autoreload Register
    #[inline(always)]
    pub const fn lptim_arr(&self) -> &LPTIM_ARR {
        &self.lptim_arr
    }
    ///0x1c - Counter Register
    #[inline(always)]
    pub const fn lptim_cnt(&self) -> &LPTIM_CNT {
        &self.lptim_cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn lptim_cfgr2(&self) -> &LPTIM_CFGR2 {
        &self.lptim_cfgr2
    }
}
/**LPTIM_ISR (r) register accessor: Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`lptim_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_ISR)

For information about available fields see [`mod@lptim_isr`]
module*/
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISRrs>;
///Interrupt and Status Register
pub mod lptim_isr;
/**LPTIM_ICR (w) register accessor: Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_ICR)

For information about available fields see [`mod@lptim_icr`]
module*/
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICRrs>;
///Interrupt Clear Register
pub mod lptim_icr;
/**LPTIM_IER (rw) register accessor: Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`lptim_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_IER)

For information about available fields see [`mod@lptim_ier`]
module*/
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IERrs>;
///Interrupt Enable Register
pub mod lptim_ier;
/**LPTIM_CFGR (rw) register accessor: Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lptim_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_CFGR)

For information about available fields see [`mod@lptim_cfgr`]
module*/
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGRrs>;
///Configuration Register
pub mod lptim_cfgr;
/**LPTIM_CR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`lptim_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_CR)

For information about available fields see [`mod@lptim_cr`]
module*/
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CRrs>;
///Control Register
pub mod lptim_cr;
/**LPTIM_CMP (rw) register accessor: Compare Register

You can [`read`](crate::Reg::read) this register and get [`lptim_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_CMP)

For information about available fields see [`mod@lptim_cmp`]
module*/
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMPrs>;
///Compare Register
pub mod lptim_cmp;
/**LPTIM_ARR (rw) register accessor: Autoreload Register

You can [`read`](crate::Reg::read) this register and get [`lptim_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_ARR)

For information about available fields see [`mod@lptim_arr`]
module*/
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARRrs>;
///Autoreload Register
pub mod lptim_arr;
/**LPTIM_CNT (r) register accessor: Counter Register

You can [`read`](crate::Reg::read) this register and get [`lptim_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_CNT)

For information about available fields see [`mod@lptim_cnt`]
module*/
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNTrs>;
///Counter Register
pub mod lptim_cnt;
/**LPTIM_CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#LPTIM1:LPTIM_CFGR2)

For information about available fields see [`mod@lptim_cfgr2`]
module*/
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2rs>;
///LPTIM configuration register 2
pub mod lptim_cfgr2;
