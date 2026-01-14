#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
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
}
impl RegisterBlock {
    ///0x04 - AConfiguration register 1
    #[inline(always)]
    pub const fn acr1(&self) -> &ACR1 {
        &self.acr1
    }
    ///0x08 - AConfiguration register 2
    #[inline(always)]
    pub const fn acr2(&self) -> &ACR2 {
        &self.acr2
    }
    ///0x0c - AFRCR
    #[inline(always)]
    pub const fn afrcr(&self) -> &AFRCR {
        &self.afrcr
    }
    ///0x10 - ASlot register
    #[inline(always)]
    pub const fn aslotr(&self) -> &ASLOTR {
        &self.aslotr
    }
    ///0x14 - AInterrupt mask register2
    #[inline(always)]
    pub const fn aim(&self) -> &AIM {
        &self.aim
    }
    ///0x18 - AStatus register
    #[inline(always)]
    pub const fn asr(&self) -> &ASR {
        &self.asr
    }
    ///0x1c - AClear flag register
    #[inline(always)]
    pub const fn aclrfr(&self) -> &ACLRFR {
        &self.aclrfr
    }
    ///0x20 - AData register
    #[inline(always)]
    pub const fn adr(&self) -> &ADR {
        &self.adr
    }
    ///0x24 - BConfiguration register 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x28 - BConfiguration register 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    ///0x2c - BFRCR
    #[inline(always)]
    pub const fn bfrcr(&self) -> &BFRCR {
        &self.bfrcr
    }
    ///0x30 - BSlot register
    #[inline(always)]
    pub const fn bslotr(&self) -> &BSLOTR {
        &self.bslotr
    }
    ///0x34 - BInterrupt mask register2
    #[inline(always)]
    pub const fn bim(&self) -> &BIM {
        &self.bim
    }
    ///0x38 - BStatus register
    #[inline(always)]
    pub const fn bsr(&self) -> &BSR {
        &self.bsr
    }
    ///0x3c - BClear flag register
    #[inline(always)]
    pub const fn bclrfr(&self) -> &BCLRFR {
        &self.bclrfr
    }
    ///0x40 - BData register
    #[inline(always)]
    pub const fn bdr(&self) -> &BDR {
        &self.bdr
    }
}
/**BCR1 (rw) register accessor: BConfiguration register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///BConfiguration register 1
pub mod bcr1;
/**BCR2 (rw) register accessor: BConfiguration register 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BCR2)

For information about available fields see [`mod@bcr2`] module*/
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
///BConfiguration register 2
pub mod bcr2;
/**BFRCR (rw) register accessor: BFRCR

You can [`read`](crate::Reg::read) this register and get [`bfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BFRCR)

For information about available fields see [`mod@bfrcr`] module*/
pub type BFRCR = crate::Reg<bfrcr::BFRCRrs>;
///BFRCR
pub mod bfrcr;
/**BSLOTR (rw) register accessor: BSlot register

You can [`read`](crate::Reg::read) this register and get [`bslotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bslotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BSLOTR)

For information about available fields see [`mod@bslotr`] module*/
pub type BSLOTR = crate::Reg<bslotr::BSLOTRrs>;
///BSlot register
pub mod bslotr;
/**BIM (rw) register accessor: BInterrupt mask register2

You can [`read`](crate::Reg::read) this register and get [`bim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BIM)

For information about available fields see [`mod@bim`] module*/
pub type BIM = crate::Reg<bim::BIMrs>;
///BInterrupt mask register2
pub mod bim;
/**BSR (r) register accessor: BStatus register

You can [`read`](crate::Reg::read) this register and get [`bsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BSR)

For information about available fields see [`mod@bsr`] module*/
pub type BSR = crate::Reg<bsr::BSRrs>;
///BStatus register
pub mod bsr;
/**BCLRFR (w) register accessor: BClear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BCLRFR)

For information about available fields see [`mod@bclrfr`] module*/
pub type BCLRFR = crate::Reg<bclrfr::BCLRFRrs>;
///BClear flag register
pub mod bclrfr;
/**BDR (rw) register accessor: BData register

You can [`read`](crate::Reg::read) this register and get [`bdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BDR)

For information about available fields see [`mod@bdr`] module*/
pub type BDR = crate::Reg<bdr::BDRrs>;
///BData register
pub mod bdr;
/**ACR1 (rw) register accessor: AConfiguration register 1

You can [`read`](crate::Reg::read) this register and get [`acr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ACR1)

For information about available fields see [`mod@acr1`] module*/
pub type ACR1 = crate::Reg<acr1::ACR1rs>;
///AConfiguration register 1
pub mod acr1;
/**ACR2 (rw) register accessor: AConfiguration register 2

You can [`read`](crate::Reg::read) this register and get [`acr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ACR2)

For information about available fields see [`mod@acr2`] module*/
pub type ACR2 = crate::Reg<acr2::ACR2rs>;
///AConfiguration register 2
pub mod acr2;
/**AFRCR (rw) register accessor: AFRCR

You can [`read`](crate::Reg::read) this register and get [`afrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:AFRCR)

For information about available fields see [`mod@afrcr`] module*/
pub type AFRCR = crate::Reg<afrcr::AFRCRrs>;
///AFRCR
pub mod afrcr;
/**ASLOTR (rw) register accessor: ASlot register

You can [`read`](crate::Reg::read) this register and get [`aslotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aslotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ASLOTR)

For information about available fields see [`mod@aslotr`] module*/
pub type ASLOTR = crate::Reg<aslotr::ASLOTRrs>;
///ASlot register
pub mod aslotr;
/**AIM (rw) register accessor: AInterrupt mask register2

You can [`read`](crate::Reg::read) this register and get [`aim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:AIM)

For information about available fields see [`mod@aim`] module*/
pub type AIM = crate::Reg<aim::AIMrs>;
///AInterrupt mask register2
pub mod aim;
/**ASR (rw) register accessor: AStatus register

You can [`read`](crate::Reg::read) this register and get [`asr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ASR)

For information about available fields see [`mod@asr`] module*/
pub type ASR = crate::Reg<asr::ASRrs>;
///AStatus register
pub mod asr;
/**ACLRFR (rw) register accessor: AClear flag register

You can [`read`](crate::Reg::read) this register and get [`aclrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ACLRFR)

For information about available fields see [`mod@aclrfr`] module*/
pub type ACLRFR = crate::Reg<aclrfr::ACLRFRrs>;
///AClear flag register
pub mod aclrfr;
/**ADR (rw) register accessor: AData register

You can [`read`](crate::Reg::read) this register and get [`adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:ADR)

For information about available fields see [`mod@adr`] module*/
pub type ADR = crate::Reg<adr::ADRrs>;
///AData register
pub mod adr;
