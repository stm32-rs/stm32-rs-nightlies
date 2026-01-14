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
    _reserved12: [u8; 0x0c],
    nsapr0: NSAPR0,
    _reserved13: [u8; 0x18],
    iidr: IIDR,
    _reserved14: [u8; 0x0f00],
    dir: DIR,
}
impl RegisterBlock {
    ///0x00 - GICC control register
    #[inline(always)]
    pub const fn ctlr(&self) -> &CTLR {
        &self.ctlr
    }
    ///0x04 - GICC input priority mask register
    #[inline(always)]
    pub const fn pmr(&self) -> &PMR {
        &self.pmr
    }
    ///0x08 - GICC binary point register
    #[inline(always)]
    pub const fn bpr(&self) -> &BPR {
        &self.bpr
    }
    ///0x0c - GICC interrupt acknowledge register
    #[inline(always)]
    pub const fn iar(&self) -> &IAR {
        &self.iar
    }
    ///0x10 - GICC end of interrupt register
    #[inline(always)]
    pub const fn eoir(&self) -> &EOIR {
        &self.eoir
    }
    ///0x14 - GICC running priority register
    #[inline(always)]
    pub const fn rpr(&self) -> &RPR {
        &self.rpr
    }
    ///0x18 - GICC highest priority pending interrupt register
    #[inline(always)]
    pub const fn hppir(&self) -> &HPPIR {
        &self.hppir
    }
    ///0x1c - GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
    #[inline(always)]
    pub const fn abpr(&self) -> &ABPR {
        &self.abpr
    }
    ///0x20 - GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
    #[inline(always)]
    pub const fn aiar(&self) -> &AIAR {
        &self.aiar
    }
    ///0x24 - GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
    #[inline(always)]
    pub const fn aeoir(&self) -> &AEOIR {
        &self.aeoir
    }
    ///0x28 - ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
    #[inline(always)]
    pub const fn ahppir(&self) -> &AHPPIR {
        &self.ahppir
    }
    ///0xd0 - GICC active priority register
    #[inline(always)]
    pub const fn apr0(&self) -> &APR0 {
        &self.apr0
    }
    ///0xe0 - GICC non-secure active priority register
    #[inline(always)]
    pub const fn nsapr0(&self) -> &NSAPR0 {
        &self.nsapr0
    }
    ///0xfc - GICC interface identification register
    #[inline(always)]
    pub const fn iidr(&self) -> &IIDR {
        &self.iidr
    }
    ///0x1000 - GICC deactivate interrupt register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
}
/**CTLR (rw) register accessor: GICC control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:CTLR)

For information about available fields see [`mod@ctlr`] module*/
pub type CTLR = crate::Reg<ctlr::CTLRrs>;
///GICC control register
pub mod ctlr;
/**PMR (rw) register accessor: GICC input priority mask register

You can [`read`](crate::Reg::read) this register and get [`pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:PMR)

For information about available fields see [`mod@pmr`] module*/
pub type PMR = crate::Reg<pmr::PMRrs>;
///GICC input priority mask register
pub mod pmr;
/**BPR (rw) register accessor: GICC binary point register

You can [`read`](crate::Reg::read) this register and get [`bpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:BPR)

For information about available fields see [`mod@bpr`] module*/
pub type BPR = crate::Reg<bpr::BPRrs>;
///GICC binary point register
pub mod bpr;
/**IAR (r) register accessor: GICC interrupt acknowledge register

You can [`read`](crate::Reg::read) this register and get [`iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:IAR)

For information about available fields see [`mod@iar`] module*/
pub type IAR = crate::Reg<iar::IARrs>;
///GICC interrupt acknowledge register
pub mod iar;
/**EOIR (w) register accessor: GICC end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:EOIR)

For information about available fields see [`mod@eoir`] module*/
pub type EOIR = crate::Reg<eoir::EOIRrs>;
///GICC end of interrupt register
pub mod eoir;
/**RPR (r) register accessor: GICC running priority register

You can [`read`](crate::Reg::read) this register and get [`rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:RPR)

For information about available fields see [`mod@rpr`] module*/
pub type RPR = crate::Reg<rpr::RPRrs>;
///GICC running priority register
pub mod rpr;
/**HPPIR (r) register accessor: GICC highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`hppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:HPPIR)

For information about available fields see [`mod@hppir`] module*/
pub type HPPIR = crate::Reg<hppir::HPPIRrs>;
///GICC highest priority pending interrupt register
pub mod hppir;
/**ABPR (rw) register accessor: GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.

You can [`read`](crate::Reg::read) this register and get [`abpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:ABPR)

For information about available fields see [`mod@abpr`] module*/
pub type ABPR = crate::Reg<abpr::ABPRrs>;
///GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
pub mod abpr;
/**AIAR (r) register accessor: GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.

You can [`read`](crate::Reg::read) this register and get [`aiar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:AIAR)

For information about available fields see [`mod@aiar`] module*/
pub type AIAR = crate::Reg<aiar::AIARrs>;
///GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
pub mod aiar;
/**AEOIR (w) register accessor: GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:AEOIR)

For information about available fields see [`mod@aeoir`] module*/
pub type AEOIR = crate::Reg<aeoir::AEOIRrs>;
///GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
pub mod aeoir;
/**AHPPIR (r) register accessor: ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.

You can [`read`](crate::Reg::read) this register and get [`ahppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:AHPPIR)

For information about available fields see [`mod@ahppir`] module*/
pub type AHPPIR = crate::Reg<ahppir::AHPPIRrs>;
///ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
pub mod ahppir;
/**APR0 (rw) register accessor: GICC active priority register

You can [`read`](crate::Reg::read) this register and get [`apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:APR0)

For information about available fields see [`mod@apr0`] module*/
pub type APR0 = crate::Reg<apr0::APR0rs>;
///GICC active priority register
pub mod apr0;
/**NSAPR0 (rw) register accessor: GICC non-secure active priority register

You can [`read`](crate::Reg::read) this register and get [`nsapr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsapr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:NSAPR0)

For information about available fields see [`mod@nsapr0`] module*/
pub type NSAPR0 = crate::Reg<nsapr0::NSAPR0rs>;
///GICC non-secure active priority register
pub mod nsapr0;
/**IIDR (r) register accessor: GICC interface identification register

You can [`read`](crate::Reg::read) this register and get [`iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:IIDR)

For information about available fields see [`mod@iidr`] module*/
pub type IIDR = crate::Reg<iidr::IIDRrs>;
///GICC interface identification register
pub mod iidr;
/**DIR (w) register accessor: GICC deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///GICC deactivate interrupt register
pub mod dir;
