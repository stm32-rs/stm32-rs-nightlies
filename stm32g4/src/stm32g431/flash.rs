#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    pdkeyr: PDKEYR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    cr: CR,
    eccr: ECCR,
    _reserved7: [u8; 0x04],
    optr: OPTR,
    pcrop1sr: PCROP1SR,
    pcrop1er: PCROP1ER,
    wrp1ar: WRP1AR,
    wrp1br: WRP1BR,
    _reserved12: [u8; 0x3c],
    sec1r: SEC1R,
}
impl RegisterBlock {
    ///0x00 - Flash access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - Flash Power-down key register
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    ///0x08 - Flash key register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x0c - Flash option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x10 - Flash status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - Flash control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x18 - Flash ECC register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x20 -
    #[inline(always)]
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x24 - Flash PCROP1 Start address register
    #[inline(always)]
    pub const fn pcrop1sr(&self) -> &PCROP1SR {
        &self.pcrop1sr
    }
    ///0x28 - Flash PCROP1 End address register
    #[inline(always)]
    pub const fn pcrop1er(&self) -> &PCROP1ER {
        &self.pcrop1er
    }
    ///0x2c - Flash WRP area A address register
    #[inline(always)]
    pub const fn wrp1ar(&self) -> &WRP1AR {
        &self.wrp1ar
    }
    ///0x30 - Flash WRP area B address register
    #[inline(always)]
    pub const fn wrp1br(&self) -> &WRP1BR {
        &self.wrp1br
    }
    ///0x70 - Flash Securable area register
    #[inline(always)]
    pub const fn sec1r(&self) -> &SEC1R {
        &self.sec1r
    }
}
/**ACR (rw) register accessor: Flash access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Flash access control register
pub mod acr;
/**PDKEYR (w) register accessor: Flash Power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:PDKEYR)

For information about available fields see [`mod@pdkeyr`] module*/
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
///Flash Power-down key register
pub mod pdkeyr;
/**KEYR (w) register accessor: Flash key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///Flash key register
pub mod keyr;
/**OPTKEYR (w) register accessor: Flash option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///Flash option key register
pub mod optkeyr;
/**SR (rw) register accessor: Flash status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Flash status register
pub mod sr;
/**CR (rw) register accessor: Flash control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Flash control register
pub mod cr;
/**ECCR (rw) register accessor: Flash ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///Flash ECC register
pub mod eccr;
/**OPTR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///
pub mod optr;
/**PCROP1SR (rw) register accessor: Flash PCROP1 Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:PCROP1SR)

For information about available fields see [`mod@pcrop1sr`] module*/
pub type PCROP1SR = crate::Reg<pcrop1sr::PCROP1SRrs>;
///Flash PCROP1 Start address register
pub mod pcrop1sr;
/**PCROP1ER (rw) register accessor: Flash PCROP1 End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:PCROP1ER)

For information about available fields see [`mod@pcrop1er`] module*/
pub type PCROP1ER = crate::Reg<pcrop1er::PCROP1ERrs>;
///Flash PCROP1 End address register
pub mod pcrop1er;
/**WRP1AR (rw) register accessor: Flash WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:WRP1AR)

For information about available fields see [`mod@wrp1ar`] module*/
pub type WRP1AR = crate::Reg<wrp1ar::WRP1ARrs>;
///Flash WRP area A address register
pub mod wrp1ar;
/**WRP1BR (rw) register accessor: Flash WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:WRP1BR)

For information about available fields see [`mod@wrp1br`] module*/
pub type WRP1BR = crate::Reg<wrp1br::WRP1BRrs>;
///Flash WRP area B address register
pub mod wrp1br;
/**SEC1R (rw) register accessor: Flash Securable area register

You can [`read`](crate::Reg::read) this register and get [`sec1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FLASH:SEC1R)

For information about available fields see [`mod@sec1r`] module*/
pub type SEC1R = crate::Reg<sec1r::SEC1Rrs>;
///Flash Securable area register
pub mod sec1r;
