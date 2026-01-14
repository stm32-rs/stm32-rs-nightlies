#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctlr: CTLR,
    pmr: PMR,
    bpr: BPR,
    iar: IAR,
    eoir: EOIR,
    rpr: RPR,
    hppir: HPPIR,
    abpr: ABPR,
    aiar: AIAR,
    aeoir: AEOIR,
    ahppir: AHPPIR,
    _reserved11: [u8; 0xa4],
    apr0: APR0,
    _reserved12: [u8; 0x28],
    iidr: IIDR,
    _reserved13: [u8; 0x0f00],
    dir: DIR,
}
impl RegisterBlock {
    ///0x00 - GICV virtual machine control register
    #[inline(always)]
    pub const fn ctlr(&self) -> &CTLR {
        &self.ctlr
    }
    ///0x04 - GICV VM priority mask register
    #[inline(always)]
    pub const fn pmr(&self) -> &PMR {
        &self.pmr
    }
    ///0x08 - GICV VM binary point register
    #[inline(always)]
    pub const fn bpr(&self) -> &BPR {
        &self.bpr
    }
    ///0x0c - GICV VM interrupt acknowledge register
    #[inline(always)]
    pub const fn iar(&self) -> &IAR {
        &self.iar
    }
    ///0x10 - GICV VM end of interrupt register
    #[inline(always)]
    pub const fn eoir(&self) -> &EOIR {
        &self.eoir
    }
    ///0x14 - GICV VM running priority register
    #[inline(always)]
    pub const fn rpr(&self) -> &RPR {
        &self.rpr
    }
    ///0x18 - GICV VM highest priority pending interrupt register
    #[inline(always)]
    pub const fn hppir(&self) -> &HPPIR {
        &self.hppir
    }
    ///0x1c - GICV VM aliased binary point register
    #[inline(always)]
    pub const fn abpr(&self) -> &ABPR {
        &self.abpr
    }
    ///0x20 - GICV VM aliased interrupt register
    #[inline(always)]
    pub const fn aiar(&self) -> &AIAR {
        &self.aiar
    }
    ///0x24 - GICV VM aliased end of interrupt register
    #[inline(always)]
    pub const fn aeoir(&self) -> &AEOIR {
        &self.aeoir
    }
    ///0x28 - GICV VM aliased highest priority pending interrupt register
    #[inline(always)]
    pub const fn ahppir(&self) -> &AHPPIR {
        &self.ahppir
    }
    ///0xd0 - The GICV_APR0 is an alias of GICH_APR.
    #[inline(always)]
    pub const fn apr0(&self) -> &APR0 {
        &self.apr0
    }
    ///0xfc - The GICV_IIDR is an alias of GICC_IIDR.
    #[inline(always)]
    pub const fn iidr(&self) -> &IIDR {
        &self.iidr
    }
    ///0x1000 - GICV VM deactivate interrupt register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
}
/**CTLR (rw) register accessor: GICV virtual machine control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:CTLR)

For information about available fields see [`mod@ctlr`] module*/
pub type CTLR = crate::Reg<ctlr::CTLRrs>;
///GICV virtual machine control register
pub mod ctlr;
/**PMR (rw) register accessor: GICV VM priority mask register

You can [`read`](crate::Reg::read) this register and get [`pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:PMR)

For information about available fields see [`mod@pmr`] module*/
pub type PMR = crate::Reg<pmr::PMRrs>;
///GICV VM priority mask register
pub mod pmr;
/**BPR (rw) register accessor: GICV VM binary point register

You can [`read`](crate::Reg::read) this register and get [`bpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:BPR)

For information about available fields see [`mod@bpr`] module*/
pub type BPR = crate::Reg<bpr::BPRrs>;
///GICV VM binary point register
pub mod bpr;
/**IAR (r) register accessor: GICV VM interrupt acknowledge register

You can [`read`](crate::Reg::read) this register and get [`iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:IAR)

For information about available fields see [`mod@iar`] module*/
pub type IAR = crate::Reg<iar::IARrs>;
///GICV VM interrupt acknowledge register
pub mod iar;
/**EOIR (w) register accessor: GICV VM end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:EOIR)

For information about available fields see [`mod@eoir`] module*/
pub type EOIR = crate::Reg<eoir::EOIRrs>;
///GICV VM end of interrupt register
pub mod eoir;
/**RPR (r) register accessor: GICV VM running priority register

You can [`read`](crate::Reg::read) this register and get [`rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:RPR)

For information about available fields see [`mod@rpr`] module*/
pub type RPR = crate::Reg<rpr::RPRrs>;
///GICV VM running priority register
pub mod rpr;
/**HPPIR (r) register accessor: GICV VM highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`hppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:HPPIR)

For information about available fields see [`mod@hppir`] module*/
pub type HPPIR = crate::Reg<hppir::HPPIRrs>;
///GICV VM highest priority pending interrupt register
pub mod hppir;
/**ABPR (rw) register accessor: GICV VM aliased binary point register

You can [`read`](crate::Reg::read) this register and get [`abpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:ABPR)

For information about available fields see [`mod@abpr`] module*/
pub type ABPR = crate::Reg<abpr::ABPRrs>;
///GICV VM aliased binary point register
pub mod abpr;
/**AIAR (r) register accessor: GICV VM aliased interrupt register

You can [`read`](crate::Reg::read) this register and get [`aiar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:AIAR)

For information about available fields see [`mod@aiar`] module*/
pub type AIAR = crate::Reg<aiar::AIARrs>;
///GICV VM aliased interrupt register
pub mod aiar;
/**AEOIR (w) register accessor: GICV VM aliased end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:AEOIR)

For information about available fields see [`mod@aeoir`] module*/
pub type AEOIR = crate::Reg<aeoir::AEOIRrs>;
///GICV VM aliased end of interrupt register
pub mod aeoir;
/**AHPPIR (r) register accessor: GICV VM aliased highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`ahppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:AHPPIR)

For information about available fields see [`mod@ahppir`] module*/
pub type AHPPIR = crate::Reg<ahppir::AHPPIRrs>;
///GICV VM aliased highest priority pending interrupt register
pub mod ahppir;
/**APR0 (rw) register accessor: The GICV_APR0 is an alias of GICH_APR.

You can [`read`](crate::Reg::read) this register and get [`apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:APR0)

For information about available fields see [`mod@apr0`] module*/
pub type APR0 = crate::Reg<apr0::APR0rs>;
///The GICV_APR0 is an alias of GICH_APR.
pub mod apr0;
/**IIDR (r) register accessor: The GICV_IIDR is an alias of GICC_IIDR.

You can [`read`](crate::Reg::read) this register and get [`iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:IIDR)

For information about available fields see [`mod@iidr`] module*/
pub type IIDR = crate::Reg<iidr::IIDRrs>;
///The GICV_IIDR is an alias of GICC_IIDR.
pub mod iidr;
/**DIR (w) register accessor: GICV VM deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///GICV VM deactivate interrupt register
pub mod dir;
