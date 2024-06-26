#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gicc_ctlr: GICC_CTLR,
    gicc_pmr: GICC_PMR,
    gicc_bpr: GICC_BPR,
    gicc_iar: GICC_IAR,
    gicc_eoir: GICC_EOIR,
    gicc_rpr: GICC_RPR,
    gicc_hppir: GICC_HPPIR,
    gicc_abpr: GICC_ABPR,
    gicc_aiar: GICC_AIAR,
    gicc_aeoir: GICC_AEOIR,
    gicc_ahppir: GICC_AHPPIR,
    _reserved11: [u8; 0xa4],
    gicc_apr0: GICC_APR0,
    _reserved12: [u8; 0x0c],
    gicc_nsapr0: GICC_NSAPR0,
    _reserved13: [u8; 0x18],
    gicc_iidr: GICC_IIDR,
    _reserved14: [u8; 0x0f00],
    gicc_dir: GICC_DIR,
}
impl RegisterBlock {
    ///0x00 - GICC control register
    #[inline(always)]
    pub const fn gicc_ctlr(&self) -> &GICC_CTLR {
        &self.gicc_ctlr
    }
    ///0x04 - GICC input priority mask register
    #[inline(always)]
    pub const fn gicc_pmr(&self) -> &GICC_PMR {
        &self.gicc_pmr
    }
    ///0x08 - GICC binary point register
    #[inline(always)]
    pub const fn gicc_bpr(&self) -> &GICC_BPR {
        &self.gicc_bpr
    }
    ///0x0c - GICC interrupt acknowledge register
    #[inline(always)]
    pub const fn gicc_iar(&self) -> &GICC_IAR {
        &self.gicc_iar
    }
    ///0x10 - GICC end of interrupt register
    #[inline(always)]
    pub const fn gicc_eoir(&self) -> &GICC_EOIR {
        &self.gicc_eoir
    }
    ///0x14 - GICC running priority register
    #[inline(always)]
    pub const fn gicc_rpr(&self) -> &GICC_RPR {
        &self.gicc_rpr
    }
    ///0x18 - GICC highest priority pending interrupt register
    #[inline(always)]
    pub const fn gicc_hppir(&self) -> &GICC_HPPIR {
        &self.gicc_hppir
    }
    ///0x1c - GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
    #[inline(always)]
    pub const fn gicc_abpr(&self) -> &GICC_ABPR {
        &self.gicc_abpr
    }
    ///0x20 - GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
    #[inline(always)]
    pub const fn gicc_aiar(&self) -> &GICC_AIAR {
        &self.gicc_aiar
    }
    ///0x24 - GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
    #[inline(always)]
    pub const fn gicc_aeoir(&self) -> &GICC_AEOIR {
        &self.gicc_aeoir
    }
    ///0x28 - ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
    #[inline(always)]
    pub const fn gicc_ahppir(&self) -> &GICC_AHPPIR {
        &self.gicc_ahppir
    }
    ///0xd0 - GICC active priority register
    #[inline(always)]
    pub const fn gicc_apr0(&self) -> &GICC_APR0 {
        &self.gicc_apr0
    }
    ///0xe0 - GICC non-secure active priority register
    #[inline(always)]
    pub const fn gicc_nsapr0(&self) -> &GICC_NSAPR0 {
        &self.gicc_nsapr0
    }
    ///0xfc - GICC interface identification register
    #[inline(always)]
    pub const fn gicc_iidr(&self) -> &GICC_IIDR {
        &self.gicc_iidr
    }
    ///0x1000 - GICC deactivate interrupt register
    #[inline(always)]
    pub const fn gicc_dir(&self) -> &GICC_DIR {
        &self.gicc_dir
    }
}
/**GICC_CTLR (rw) register accessor: GICC control register

You can [`read`](crate::Reg::read) this register and get [`gicc_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_CTLR)

For information about available fields see [`mod@gicc_ctlr`]
module*/
pub type GICC_CTLR = crate::Reg<gicc_ctlr::GICC_CTLRrs>;
///GICC control register
pub mod gicc_ctlr;
/**GICC_PMR (rw) register accessor: GICC input priority mask register

You can [`read`](crate::Reg::read) this register and get [`gicc_pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_PMR)

For information about available fields see [`mod@gicc_pmr`]
module*/
pub type GICC_PMR = crate::Reg<gicc_pmr::GICC_PMRrs>;
///GICC input priority mask register
pub mod gicc_pmr;
/**GICC_BPR (rw) register accessor: GICC binary point register

You can [`read`](crate::Reg::read) this register and get [`gicc_bpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_bpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_BPR)

For information about available fields see [`mod@gicc_bpr`]
module*/
pub type GICC_BPR = crate::Reg<gicc_bpr::GICC_BPRrs>;
///GICC binary point register
pub mod gicc_bpr;
/**GICC_IAR (r) register accessor: GICC interrupt acknowledge register

You can [`read`](crate::Reg::read) this register and get [`gicc_iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_IAR)

For information about available fields see [`mod@gicc_iar`]
module*/
pub type GICC_IAR = crate::Reg<gicc_iar::GICC_IARrs>;
///GICC interrupt acknowledge register
pub mod gicc_iar;
/**GICC_EOIR (w) register accessor: GICC end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_EOIR)

For information about available fields see [`mod@gicc_eoir`]
module*/
pub type GICC_EOIR = crate::Reg<gicc_eoir::GICC_EOIRrs>;
///GICC end of interrupt register
pub mod gicc_eoir;
/**GICC_RPR (r) register accessor: GICC running priority register

You can [`read`](crate::Reg::read) this register and get [`gicc_rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_RPR)

For information about available fields see [`mod@gicc_rpr`]
module*/
pub type GICC_RPR = crate::Reg<gicc_rpr::GICC_RPRrs>;
///GICC running priority register
pub mod gicc_rpr;
/**GICC_HPPIR (r) register accessor: GICC highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`gicc_hppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_HPPIR)

For information about available fields see [`mod@gicc_hppir`]
module*/
pub type GICC_HPPIR = crate::Reg<gicc_hppir::GICC_HPPIRrs>;
///GICC highest priority pending interrupt register
pub mod gicc_hppir;
/**GICC_ABPR (rw) register accessor: GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.

You can [`read`](crate::Reg::read) this register and get [`gicc_abpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_abpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_ABPR)

For information about available fields see [`mod@gicc_abpr`]
module*/
pub type GICC_ABPR = crate::Reg<gicc_abpr::GICC_ABPRrs>;
///GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
pub mod gicc_abpr;
/**GICC_AIAR (r) register accessor: GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.

You can [`read`](crate::Reg::read) this register and get [`gicc_aiar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_AIAR)

For information about available fields see [`mod@gicc_aiar`]
module*/
pub type GICC_AIAR = crate::Reg<gicc_aiar::GICC_AIARrs>;
///GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
pub mod gicc_aiar;
/**GICC_AEOIR (w) register accessor: GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_AEOIR)

For information about available fields see [`mod@gicc_aeoir`]
module*/
pub type GICC_AEOIR = crate::Reg<gicc_aeoir::GICC_AEOIRrs>;
///GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
pub mod gicc_aeoir;
/**GICC_AHPPIR (r) register accessor: ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.

You can [`read`](crate::Reg::read) this register and get [`gicc_ahppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_AHPPIR)

For information about available fields see [`mod@gicc_ahppir`]
module*/
pub type GICC_AHPPIR = crate::Reg<gicc_ahppir::GICC_AHPPIRrs>;
///ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
pub mod gicc_ahppir;
/**GICC_APR0 (rw) register accessor: GICC active priority register

You can [`read`](crate::Reg::read) this register and get [`gicc_apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_APR0)

For information about available fields see [`mod@gicc_apr0`]
module*/
pub type GICC_APR0 = crate::Reg<gicc_apr0::GICC_APR0rs>;
///GICC active priority register
pub mod gicc_apr0;
/**GICC_NSAPR0 (rw) register accessor: GICC non-secure active priority register

You can [`read`](crate::Reg::read) this register and get [`gicc_nsapr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_nsapr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_NSAPR0)

For information about available fields see [`mod@gicc_nsapr0`]
module*/
pub type GICC_NSAPR0 = crate::Reg<gicc_nsapr0::GICC_NSAPR0rs>;
///GICC non-secure active priority register
pub mod gicc_nsapr0;
/**GICC_IIDR (r) register accessor: GICC interface identification register

You can [`read`](crate::Reg::read) this register and get [`gicc_iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_IIDR)

For information about available fields see [`mod@gicc_iidr`]
module*/
pub type GICC_IIDR = crate::Reg<gicc_iidr::GICC_IIDRrs>;
///GICC interface identification register
pub mod gicc_iidr;
/**GICC_DIR (w) register accessor: GICC deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_DIR)

For information about available fields see [`mod@gicc_dir`]
module*/
pub type GICC_DIR = crate::Reg<gicc_dir::GICC_DIRrs>;
///GICC deactivate interrupt register
pub mod gicc_dir;
