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
    _reserved9: [u8; 0x03c8],
    lptim1_hwcfgr: LPTIM1_HWCFGR,
    lptim_verr: LPTIM_VERR,
    lptim_pidr: LPTIM_PIDR,
    lptim_sidr: LPTIM_SIDR,
}
impl RegisterBlock {
    ///0x00 - LPTIM interrupt and status register
    #[inline(always)]
    pub const fn lptim_isr(&self) -> &LPTIM_ISR {
        &self.lptim_isr
    }
    ///0x04 - LPTIM interrupt clear register
    #[inline(always)]
    pub const fn lptim_icr(&self) -> &LPTIM_ICR {
        &self.lptim_icr
    }
    ///0x08 - LPTIM interrupt enable register
    #[inline(always)]
    pub const fn lptim_ier(&self) -> &LPTIM_IER {
        &self.lptim_ier
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn lptim_cfgr(&self) -> &LPTIM_CFGR {
        &self.lptim_cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn lptim_cr(&self) -> &LPTIM_CR {
        &self.lptim_cr
    }
    ///0x14 - LPTIM compare register
    #[inline(always)]
    pub const fn lptim_cmp(&self) -> &LPTIM_CMP {
        &self.lptim_cmp
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn lptim_arr(&self) -> &LPTIM_ARR {
        &self.lptim_arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn lptim_cnt(&self) -> &LPTIM_CNT {
        &self.lptim_cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn lptim_cfgr2(&self) -> &LPTIM_CFGR2 {
        &self.lptim_cfgr2
    }
    ///0x3f0 - LPTIM 1 peripheral hardware configuration register
    #[inline(always)]
    pub const fn lptim1_hwcfgr(&self) -> &LPTIM1_HWCFGR {
        &self.lptim1_hwcfgr
    }
    ///0x3f4 - LPTIM peripheral version identification register
    #[inline(always)]
    pub const fn lptim_verr(&self) -> &LPTIM_VERR {
        &self.lptim_verr
    }
    ///0x3f8 - LPTIM peripheral type identification register
    #[inline(always)]
    pub const fn lptim_pidr(&self) -> &LPTIM_PIDR {
        &self.lptim_pidr
    }
    ///0x3fc - LPTIM registers map size identification register
    #[inline(always)]
    pub const fn lptim_sidr(&self) -> &LPTIM_SIDR {
        &self.lptim_sidr
    }
}
/**LPTIM_ISR (r) register accessor: LPTIM interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`lptim_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_ISR)

For information about available fields see [`mod@lptim_isr`]
module*/
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISRrs>;
///LPTIM interrupt and status register
pub mod lptim_isr;
/**LPTIM_ICR (w) register accessor: LPTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_ICR)

For information about available fields see [`mod@lptim_icr`]
module*/
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICRrs>;
///LPTIM interrupt clear register
pub mod lptim_icr;
/**LPTIM_IER (rw) register accessor: LPTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`lptim_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_IER)

For information about available fields see [`mod@lptim_ier`]
module*/
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IERrs>;
///LPTIM interrupt enable register
pub mod lptim_ier;
/**LPTIM_CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_CFGR)

For information about available fields see [`mod@lptim_cfgr`]
module*/
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGRrs>;
///LPTIM configuration register
pub mod lptim_cfgr;
/**LPTIM_CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`lptim_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_CR)

For information about available fields see [`mod@lptim_cr`]
module*/
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CRrs>;
///LPTIM control register
pub mod lptim_cr;
/**LPTIM_CMP (rw) register accessor: LPTIM compare register

You can [`read`](crate::Reg::read) this register and get [`lptim_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_CMP)

For information about available fields see [`mod@lptim_cmp`]
module*/
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMPrs>;
///LPTIM compare register
pub mod lptim_cmp;
/**LPTIM_ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`lptim_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_ARR)

For information about available fields see [`mod@lptim_arr`]
module*/
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARRrs>;
///LPTIM autoreload register
pub mod lptim_arr;
/**LPTIM_CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`lptim_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_CNT)

For information about available fields see [`mod@lptim_cnt`]
module*/
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNTrs>;
///LPTIM counter register
pub mod lptim_cnt;
/**LPTIM_CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_CFGR2)

For information about available fields see [`mod@lptim_cfgr2`]
module*/
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2rs>;
///LPTIM configuration register 2
pub mod lptim_cfgr2;
/**LPTIM1_HWCFGR (r) register accessor: LPTIM 1 peripheral hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim1_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM1_HWCFGR)

For information about available fields see [`mod@lptim1_hwcfgr`]
module*/
pub type LPTIM1_HWCFGR = crate::Reg<lptim1_hwcfgr::LPTIM1_HWCFGRrs>;
///LPTIM 1 peripheral hardware configuration register
pub mod lptim1_hwcfgr;
/**LPTIM_VERR (r) register accessor: LPTIM peripheral version identification register

You can [`read`](crate::Reg::read) this register and get [`lptim_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_VERR)

For information about available fields see [`mod@lptim_verr`]
module*/
pub type LPTIM_VERR = crate::Reg<lptim_verr::LPTIM_VERRrs>;
///LPTIM peripheral version identification register
pub mod lptim_verr;
/**LPTIM_PIDR (r) register accessor: LPTIM peripheral type identification register

You can [`read`](crate::Reg::read) this register and get [`lptim_pidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_PIDR)

For information about available fields see [`mod@lptim_pidr`]
module*/
pub type LPTIM_PIDR = crate::Reg<lptim_pidr::LPTIM_PIDRrs>;
///LPTIM peripheral type identification register
pub mod lptim_pidr;
/**LPTIM_SIDR (r) register accessor: LPTIM registers map size identification register

You can [`read`](crate::Reg::read) this register and get [`lptim_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_SIDR)

For information about available fields see [`mod@lptim_sidr`]
module*/
pub type LPTIM_SIDR = crate::Reg<lptim_sidr::LPTIM_SIDRrs>;
///LPTIM registers map size identification register
pub mod lptim_sidr;
