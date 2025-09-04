#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    acr1: ACR1,
    acr2: ACR2,
    afrcr: AFRCR,
    aslotr: ASLOTR,
    aim: AIM,
    asr: ASR,
    aclrfr: ACLRFR,
    adr: ADR,
    bcr1: BCR1,
    bcr2: BCR2,
    bfrcr: BFRCR,
    bslotr: BSLOTR,
    bim: BIM,
    bsr: BSR,
    bclrfr: BCLRFR,
    bdr: BDR,
    pdmcr: PDMCR,
    pdmdly: PDMDLY,
}
impl RegisterBlock {
    ///0x00 - SAI global configuration register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x04 - SAI configuration register 1
    #[inline(always)]
    pub const fn acr1(&self) -> &ACR1 {
        &self.acr1
    }
    ///0x08 - SAI configuration register 2
    #[inline(always)]
    pub const fn acr2(&self) -> &ACR2 {
        &self.acr2
    }
    ///0x0c - SAI frame configuration register
    #[inline(always)]
    pub const fn afrcr(&self) -> &AFRCR {
        &self.afrcr
    }
    ///0x10 - SAI slot register
    #[inline(always)]
    pub const fn aslotr(&self) -> &ASLOTR {
        &self.aslotr
    }
    ///0x14 - SAI interrupt mask register
    #[inline(always)]
    pub const fn aim(&self) -> &AIM {
        &self.aim
    }
    ///0x18 - SAI status register
    #[inline(always)]
    pub const fn asr(&self) -> &ASR {
        &self.asr
    }
    ///0x1c - SAI clear flag register
    #[inline(always)]
    pub const fn aclrfr(&self) -> &ACLRFR {
        &self.aclrfr
    }
    ///0x20 - SAI data register
    #[inline(always)]
    pub const fn adr(&self) -> &ADR {
        &self.adr
    }
    ///0x24 - SAI configuration register 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x28 - SAI configuration register 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    ///0x2c - SAI frame configuration register
    #[inline(always)]
    pub const fn bfrcr(&self) -> &BFRCR {
        &self.bfrcr
    }
    ///0x30 - SAI slot register
    #[inline(always)]
    pub const fn bslotr(&self) -> &BSLOTR {
        &self.bslotr
    }
    ///0x34 - SAI interrupt mask register
    #[inline(always)]
    pub const fn bim(&self) -> &BIM {
        &self.bim
    }
    ///0x38 - SAI status register
    #[inline(always)]
    pub const fn bsr(&self) -> &BSR {
        &self.bsr
    }
    ///0x3c - SAI clear flag register
    #[inline(always)]
    pub const fn bclrfr(&self) -> &BCLRFR {
        &self.bclrfr
    }
    ///0x40 - SAI data register
    #[inline(always)]
    pub const fn bdr(&self) -> &BDR {
        &self.bdr
    }
    ///0x44 - SAI PDM control register
    #[inline(always)]
    pub const fn pdmcr(&self) -> &PDMCR {
        &self.pdmcr
    }
    ///0x48 - SAI PDM delay register
    #[inline(always)]
    pub const fn pdmdly(&self) -> &PDMDLY {
        &self.pdmdly
    }
}
/**GCR (rw) register accessor: SAI global configuration register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///SAI global configuration register
pub mod gcr;
/**ACR1 (rw) register accessor: SAI configuration register 1

You can [`read`](crate::Reg::read) this register and get [`acr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ACR1)

For information about available fields see [`mod@acr1`] module*/
pub type ACR1 = crate::Reg<acr1::ACR1rs>;
///SAI configuration register 1
pub mod acr1;
/**ACR2 (rw) register accessor: SAI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`acr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ACR2)

For information about available fields see [`mod@acr2`] module*/
pub type ACR2 = crate::Reg<acr2::ACR2rs>;
///SAI configuration register 2
pub mod acr2;
/**AFRCR (rw) register accessor: SAI frame configuration register

You can [`read`](crate::Reg::read) this register and get [`afrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:AFRCR)

For information about available fields see [`mod@afrcr`] module*/
pub type AFRCR = crate::Reg<afrcr::AFRCRrs>;
///SAI frame configuration register
pub mod afrcr;
/**ASLOTR (rw) register accessor: SAI slot register

You can [`read`](crate::Reg::read) this register and get [`aslotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aslotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ASLOTR)

For information about available fields see [`mod@aslotr`] module*/
pub type ASLOTR = crate::Reg<aslotr::ASLOTRrs>;
///SAI slot register
pub mod aslotr;
/**AIM (rw) register accessor: SAI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`aim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:AIM)

For information about available fields see [`mod@aim`] module*/
pub type AIM = crate::Reg<aim::AIMrs>;
///SAI interrupt mask register
pub mod aim;
/**ASR (r) register accessor: SAI status register

You can [`read`](crate::Reg::read) this register and get [`asr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ASR)

For information about available fields see [`mod@asr`] module*/
pub type ASR = crate::Reg<asr::ASRrs>;
///SAI status register
pub mod asr;
/**ACLRFR (w) register accessor: SAI clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ACLRFR)

For information about available fields see [`mod@aclrfr`] module*/
pub type ACLRFR = crate::Reg<aclrfr::ACLRFRrs>;
///SAI clear flag register
pub mod aclrfr;
/**ADR (rw) register accessor: SAI data register

You can [`read`](crate::Reg::read) this register and get [`adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:ADR)

For information about available fields see [`mod@adr`] module*/
pub type ADR = crate::Reg<adr::ADRrs>;
///SAI data register
pub mod adr;
/**BCR1 (rw) register accessor: SAI configuration register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SAI configuration register 1
pub mod bcr1;
/**BCR2 (rw) register accessor: SAI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BCR2)

For information about available fields see [`mod@bcr2`] module*/
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
///SAI configuration register 2
pub mod bcr2;
/**BFRCR (rw) register accessor: SAI frame configuration register

You can [`read`](crate::Reg::read) this register and get [`bfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BFRCR)

For information about available fields see [`mod@bfrcr`] module*/
pub type BFRCR = crate::Reg<bfrcr::BFRCRrs>;
///SAI frame configuration register
pub mod bfrcr;
/**BSLOTR (rw) register accessor: SAI slot register

You can [`read`](crate::Reg::read) this register and get [`bslotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bslotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BSLOTR)

For information about available fields see [`mod@bslotr`] module*/
pub type BSLOTR = crate::Reg<bslotr::BSLOTRrs>;
///SAI slot register
pub mod bslotr;
/**BIM (rw) register accessor: SAI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`bim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BIM)

For information about available fields see [`mod@bim`] module*/
pub type BIM = crate::Reg<bim::BIMrs>;
///SAI interrupt mask register
pub mod bim;
/**BSR (r) register accessor: SAI status register

You can [`read`](crate::Reg::read) this register and get [`bsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BSR)

For information about available fields see [`mod@bsr`] module*/
pub type BSR = crate::Reg<bsr::BSRrs>;
///SAI status register
pub mod bsr;
/**BCLRFR (w) register accessor: SAI clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BCLRFR)

For information about available fields see [`mod@bclrfr`] module*/
pub type BCLRFR = crate::Reg<bclrfr::BCLRFRrs>;
///SAI clear flag register
pub mod bclrfr;
/**BDR (rw) register accessor: SAI data register

You can [`read`](crate::Reg::read) this register and get [`bdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:BDR)

For information about available fields see [`mod@bdr`] module*/
pub type BDR = crate::Reg<bdr::BDRrs>;
///SAI data register
pub mod bdr;
/**PDMCR (rw) register accessor: SAI PDM control register

You can [`read`](crate::Reg::read) this register and get [`pdmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:PDMCR)

For information about available fields see [`mod@pdmcr`] module*/
pub type PDMCR = crate::Reg<pdmcr::PDMCRrs>;
///SAI PDM control register
pub mod pdmcr;
/**PDMDLY (rw) register accessor: SAI PDM delay register

You can [`read`](crate::Reg::read) this register and get [`pdmdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAI1:PDMDLY)

For information about available fields see [`mod@pdmdly`] module*/
pub type PDMDLY = crate::Reg<pdmdly::PDMDLYrs>;
///SAI PDM delay register
pub mod pdmdly;
