#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gicv_ctlr: GICV_CTLR,
    gicv_pmr: GICV_PMR,
    gicv_bpr: GICV_BPR,
    gicv_iar: GICV_IAR,
    gicv_eoir: GICV_EOIR,
    gicv_rpr: GICV_RPR,
    gicv_hppir: GICV_HPPIR,
    gicv_abpr: GICV_ABPR,
    gicv_aiar: GICV_AIAR,
    gicv_aeoir: GICV_AEOIR,
    gicv_ahppir: GICV_AHPPIR,
    _reserved11: [u8; 0xa4],
    gicv_apr0: GICV_APR0,
    _reserved12: [u8; 0x28],
    gicv_iidr: GICV_IIDR,
    _reserved13: [u8; 0x0f00],
    gicv_dir: GICV_DIR,
}
impl RegisterBlock {
    ///0x00 - GICV virtual machine control register
    #[inline(always)]
    pub const fn gicv_ctlr(&self) -> &GICV_CTLR {
        &self.gicv_ctlr
    }
    ///0x04 - GICV VM priority mask register
    #[inline(always)]
    pub const fn gicv_pmr(&self) -> &GICV_PMR {
        &self.gicv_pmr
    }
    ///0x08 - GICV VM binary point register
    #[inline(always)]
    pub const fn gicv_bpr(&self) -> &GICV_BPR {
        &self.gicv_bpr
    }
    ///0x0c - GICV VM interrupt acknowledge register
    #[inline(always)]
    pub const fn gicv_iar(&self) -> &GICV_IAR {
        &self.gicv_iar
    }
    ///0x10 - GICV VM end of interrupt register
    #[inline(always)]
    pub const fn gicv_eoir(&self) -> &GICV_EOIR {
        &self.gicv_eoir
    }
    ///0x14 - GICV VM running priority register
    #[inline(always)]
    pub const fn gicv_rpr(&self) -> &GICV_RPR {
        &self.gicv_rpr
    }
    ///0x18 - GICV VM highest priority pending interrupt register
    #[inline(always)]
    pub const fn gicv_hppir(&self) -> &GICV_HPPIR {
        &self.gicv_hppir
    }
    ///0x1c - GICV VM aliased binary point register
    #[inline(always)]
    pub const fn gicv_abpr(&self) -> &GICV_ABPR {
        &self.gicv_abpr
    }
    ///0x20 - GICV VM aliased interrupt register
    #[inline(always)]
    pub const fn gicv_aiar(&self) -> &GICV_AIAR {
        &self.gicv_aiar
    }
    ///0x24 - GICV VM aliased end of interrupt register
    #[inline(always)]
    pub const fn gicv_aeoir(&self) -> &GICV_AEOIR {
        &self.gicv_aeoir
    }
    ///0x28 - GICV VM aliased highest priority pending interrupt register
    #[inline(always)]
    pub const fn gicv_ahppir(&self) -> &GICV_AHPPIR {
        &self.gicv_ahppir
    }
    ///0xd0 - The GICV_APR0 is an alias of GICH_APR.
    #[inline(always)]
    pub const fn gicv_apr0(&self) -> &GICV_APR0 {
        &self.gicv_apr0
    }
    ///0xfc - The GICV_IIDR is an alias of GICC_IIDR.
    #[inline(always)]
    pub const fn gicv_iidr(&self) -> &GICV_IIDR {
        &self.gicv_iidr
    }
    ///0x1000 - GICV VM deactivate interrupt register
    #[inline(always)]
    pub const fn gicv_dir(&self) -> &GICV_DIR {
        &self.gicv_dir
    }
}
/**GICV_CTLR (rw) register accessor: GICV virtual machine control register

You can [`read`](crate::Reg::read) this register and get [`gicv_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_CTLR)

For information about available fields see [`mod@gicv_ctlr`]
module*/
pub type GICV_CTLR = crate::Reg<gicv_ctlr::GICV_CTLRrs>;
///GICV virtual machine control register
pub mod gicv_ctlr;
/**GICV_PMR (rw) register accessor: GICV VM priority mask register

You can [`read`](crate::Reg::read) this register and get [`gicv_pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_PMR)

For information about available fields see [`mod@gicv_pmr`]
module*/
pub type GICV_PMR = crate::Reg<gicv_pmr::GICV_PMRrs>;
///GICV VM priority mask register
pub mod gicv_pmr;
/**GICV_BPR (rw) register accessor: GICV VM binary point register

You can [`read`](crate::Reg::read) this register and get [`gicv_bpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_bpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_BPR)

For information about available fields see [`mod@gicv_bpr`]
module*/
pub type GICV_BPR = crate::Reg<gicv_bpr::GICV_BPRrs>;
///GICV VM binary point register
pub mod gicv_bpr;
/**GICV_IAR (r) register accessor: GICV VM interrupt acknowledge register

You can [`read`](crate::Reg::read) this register and get [`gicv_iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_IAR)

For information about available fields see [`mod@gicv_iar`]
module*/
pub type GICV_IAR = crate::Reg<gicv_iar::GICV_IARrs>;
///GICV VM interrupt acknowledge register
pub mod gicv_iar;
/**GICV_EOIR (w) register accessor: GICV VM end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_EOIR)

For information about available fields see [`mod@gicv_eoir`]
module*/
pub type GICV_EOIR = crate::Reg<gicv_eoir::GICV_EOIRrs>;
///GICV VM end of interrupt register
pub mod gicv_eoir;
/**GICV_RPR (r) register accessor: GICV VM running priority register

You can [`read`](crate::Reg::read) this register and get [`gicv_rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_RPR)

For information about available fields see [`mod@gicv_rpr`]
module*/
pub type GICV_RPR = crate::Reg<gicv_rpr::GICV_RPRrs>;
///GICV VM running priority register
pub mod gicv_rpr;
/**GICV_HPPIR (r) register accessor: GICV VM highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`gicv_hppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_HPPIR)

For information about available fields see [`mod@gicv_hppir`]
module*/
pub type GICV_HPPIR = crate::Reg<gicv_hppir::GICV_HPPIRrs>;
///GICV VM highest priority pending interrupt register
pub mod gicv_hppir;
/**GICV_ABPR (rw) register accessor: GICV VM aliased binary point register

You can [`read`](crate::Reg::read) this register and get [`gicv_abpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_abpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_ABPR)

For information about available fields see [`mod@gicv_abpr`]
module*/
pub type GICV_ABPR = crate::Reg<gicv_abpr::GICV_ABPRrs>;
///GICV VM aliased binary point register
pub mod gicv_abpr;
/**GICV_AIAR (r) register accessor: GICV VM aliased interrupt register

You can [`read`](crate::Reg::read) this register and get [`gicv_aiar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_AIAR)

For information about available fields see [`mod@gicv_aiar`]
module*/
pub type GICV_AIAR = crate::Reg<gicv_aiar::GICV_AIARrs>;
///GICV VM aliased interrupt register
pub mod gicv_aiar;
/**GICV_AEOIR (w) register accessor: GICV VM aliased end of interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_AEOIR)

For information about available fields see [`mod@gicv_aeoir`]
module*/
pub type GICV_AEOIR = crate::Reg<gicv_aeoir::GICV_AEOIRrs>;
///GICV VM aliased end of interrupt register
pub mod gicv_aeoir;
/**GICV_AHPPIR (r) register accessor: GICV VM aliased highest priority pending interrupt register

You can [`read`](crate::Reg::read) this register and get [`gicv_ahppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_AHPPIR)

For information about available fields see [`mod@gicv_ahppir`]
module*/
pub type GICV_AHPPIR = crate::Reg<gicv_ahppir::GICV_AHPPIRrs>;
///GICV VM aliased highest priority pending interrupt register
pub mod gicv_ahppir;
/**GICV_APR0 (rw) register accessor: The GICV_APR0 is an alias of GICH_APR.

You can [`read`](crate::Reg::read) this register and get [`gicv_apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_APR0)

For information about available fields see [`mod@gicv_apr0`]
module*/
pub type GICV_APR0 = crate::Reg<gicv_apr0::GICV_APR0rs>;
///The GICV_APR0 is an alias of GICH_APR.
pub mod gicv_apr0;
/**GICV_IIDR (r) register accessor: The GICV_IIDR is an alias of GICC_IIDR.

You can [`read`](crate::Reg::read) this register and get [`gicv_iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_IIDR)

For information about available fields see [`mod@gicv_iidr`]
module*/
pub type GICV_IIDR = crate::Reg<gicv_iidr::GICV_IIDRrs>;
///The GICV_IIDR is an alias of GICC_IIDR.
pub mod gicv_iidr;
/**GICV_DIR (w) register accessor: GICV VM deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_DIR)

For information about available fields see [`mod@gicv_dir`]
module*/
pub type GICV_DIR = crate::Reg<gicv_dir::GICV_DIRrs>;
///GICV VM deactivate interrupt register
pub mod gicv_dir;
