#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    icr: ICR,
    ier: IER,
    cfgr: CFGR,
    cr: CR,
    cmp: CMP,
    arr: ARR,
    cnt: CNT,
    _reserved8: [u8; 0x04],
    cfgr2: CFGR2,
    _reserved9: [u8; 0x03c8],
    lptim1_hwcfgr: LPTIM1_HWCFGR,
    verr: VERR,
    pidr: PIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - LPTIM interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - LPTIM interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x08 - LPTIM interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x14 - LPTIM compare register
    #[inline(always)]
    pub const fn cmp(&self) -> &CMP {
        &self.cmp
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x3f0 - LPTIM 1 peripheral hardware configuration register
    #[inline(always)]
    pub const fn lptim1_hwcfgr(&self) -> &LPTIM1_HWCFGR {
        &self.lptim1_hwcfgr
    }
    ///0x3f4 - LPTIM peripheral version identification register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - LPTIM peripheral type identification register
    #[inline(always)]
    pub const fn pidr(&self) -> &PIDR {
        &self.pidr
    }
    ///0x3fc - LPTIM registers map size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**ISR (r) register accessor: LPTIM interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LPTIM interrupt and status register
pub mod isr;
/**ICR (w) register accessor: LPTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LPTIM interrupt clear register
pub mod icr;
/**IER (rw) register accessor: LPTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///LPTIM interrupt enable register
pub mod ier;
/**CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///LPTIM configuration register
pub mod cfgr;
/**CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///LPTIM control register
pub mod cr;
/**CMP (rw) register accessor: LPTIM compare register

You can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:CMP)

For information about available fields see [`mod@cmp`] module*/
pub type CMP = crate::Reg<cmp::CMPrs>;
///LPTIM compare register
pub mod cmp;
/**ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///LPTIM autoreload register
pub mod arr;
/**CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///LPTIM counter register
pub mod cnt;
/**CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///LPTIM configuration register 2
pub mod cfgr2;
/**LPTIM1_HWCFGR (r) register accessor: LPTIM 1 peripheral hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim1_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM1_HWCFGR)

For information about available fields see [`mod@lptim1_hwcfgr`] module*/
pub type LPTIM1_HWCFGR = crate::Reg<lptim1_hwcfgr::LPTIM1_HWCFGRrs>;
///LPTIM 1 peripheral hardware configuration register
pub mod lptim1_hwcfgr;
/**VERR (r) register accessor: LPTIM peripheral version identification register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///LPTIM peripheral version identification register
pub mod verr;
/**PIDR (r) register accessor: LPTIM peripheral type identification register

You can [`read`](crate::Reg::read) this register and get [`pidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:PIDR)

For information about available fields see [`mod@pidr`] module*/
pub type PIDR = crate::Reg<pidr::PIDRrs>;
///LPTIM peripheral type identification register
pub mod pidr;
/**SIDR (r) register accessor: LPTIM registers map size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///LPTIM registers map size identification register
pub mod sidr;
