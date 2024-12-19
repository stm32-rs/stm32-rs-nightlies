#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    flash_acr: FLASH_ACR,
    _reserved1: [u8; 0x04],
    flash_keyr: FLASH_KEYR,
    flash_optkeyr: FLASH_OPTKEYR,
    flash_sr: FLASH_SR,
    flash_cr: FLASH_CR,
    flash_eccr: FLASH_ECCR,
    _reserved6: [u8; 0x04],
    flash_optr: FLASH_OPTR,
    _reserved7: [u8; 0x08],
    flash_wrp1ar: FLASH_WRP1AR,
    flash_wrp1br: FLASH_WRP1BR,
    _reserved9: [u8; 0x4c],
    flash_secr: FLASH_SECR,
}
impl RegisterBlock {
    ///0x00 - FLASH access control register
    #[inline(always)]
    pub const fn flash_acr(&self) -> &FLASH_ACR {
        &self.flash_acr
    }
    ///0x08 - FLASH key register
    #[inline(always)]
    pub const fn flash_keyr(&self) -> &FLASH_KEYR {
        &self.flash_keyr
    }
    ///0x0c - FLASH option key register
    #[inline(always)]
    pub const fn flash_optkeyr(&self) -> &FLASH_OPTKEYR {
        &self.flash_optkeyr
    }
    ///0x10 - FLASH status register
    #[inline(always)]
    pub const fn flash_sr(&self) -> &FLASH_SR {
        &self.flash_sr
    }
    ///0x14 - FLASH control register
    #[inline(always)]
    pub const fn flash_cr(&self) -> &FLASH_CR {
        &self.flash_cr
    }
    ///0x18 - FLASH ECC register
    #[inline(always)]
    pub const fn flash_eccr(&self) -> &FLASH_ECCR {
        &self.flash_eccr
    }
    ///0x20 - FLASH option register
    #[inline(always)]
    pub const fn flash_optr(&self) -> &FLASH_OPTR {
        &self.flash_optr
    }
    ///0x2c - FLASH WRP area A address register
    #[inline(always)]
    pub const fn flash_wrp1ar(&self) -> &FLASH_WRP1AR {
        &self.flash_wrp1ar
    }
    ///0x30 - FLASH WRP area B address register
    #[inline(always)]
    pub const fn flash_wrp1br(&self) -> &FLASH_WRP1BR {
        &self.flash_wrp1br
    }
    ///0x80 - FLASH security register
    #[inline(always)]
    pub const fn flash_secr(&self) -> &FLASH_SECR {
        &self.flash_secr
    }
}
/**FLASH_ACR (rw) register accessor: FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`flash_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_ACR)

For information about available fields see [`mod@flash_acr`]
module*/
pub type FLASH_ACR = crate::Reg<flash_acr::FLASH_ACRrs>;
///FLASH access control register
pub mod flash_acr;
/**FLASH_KEYR (w) register accessor: FLASH key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_KEYR)

For information about available fields see [`mod@flash_keyr`]
module*/
pub type FLASH_KEYR = crate::Reg<flash_keyr::FLASH_KEYRrs>;
///FLASH key register
pub mod flash_keyr;
/**FLASH_OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_OPTKEYR)

For information about available fields see [`mod@flash_optkeyr`]
module*/
pub type FLASH_OPTKEYR = crate::Reg<flash_optkeyr::FLASH_OPTKEYRrs>;
///FLASH option key register
pub mod flash_optkeyr;
/**FLASH_SR (rw) register accessor: FLASH status register

You can [`read`](crate::Reg::read) this register and get [`flash_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_SR)

For information about available fields see [`mod@flash_sr`]
module*/
pub type FLASH_SR = crate::Reg<flash_sr::FLASH_SRrs>;
///FLASH status register
pub mod flash_sr;
/**FLASH_CR (rw) register accessor: FLASH control register

You can [`read`](crate::Reg::read) this register and get [`flash_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_CR)

For information about available fields see [`mod@flash_cr`]
module*/
pub type FLASH_CR = crate::Reg<flash_cr::FLASH_CRrs>;
///FLASH control register
pub mod flash_cr;
/**FLASH_ECCR (rw) register accessor: FLASH ECC register

You can [`read`](crate::Reg::read) this register and get [`flash_eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_ECCR)

For information about available fields see [`mod@flash_eccr`]
module*/
pub type FLASH_ECCR = crate::Reg<flash_eccr::FLASH_ECCRrs>;
///FLASH ECC register
pub mod flash_eccr;
/**FLASH_OPTR (rw) register accessor: FLASH option register

You can [`read`](crate::Reg::read) this register and get [`flash_optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_OPTR)

For information about available fields see [`mod@flash_optr`]
module*/
pub type FLASH_OPTR = crate::Reg<flash_optr::FLASH_OPTRrs>;
///FLASH option register
pub mod flash_optr;
/**FLASH_WRP1AR (rw) register accessor: FLASH WRP area A address register

You can [`read`](crate::Reg::read) this register and get [`flash_wrp1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_wrp1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_WRP1AR)

For information about available fields see [`mod@flash_wrp1ar`]
module*/
pub type FLASH_WRP1AR = crate::Reg<flash_wrp1ar::FLASH_WRP1ARrs>;
///FLASH WRP area A address register
pub mod flash_wrp1ar;
/**FLASH_WRP1BR (rw) register accessor: FLASH WRP area B address register

You can [`read`](crate::Reg::read) this register and get [`flash_wrp1br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_wrp1br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_WRP1BR)

For information about available fields see [`mod@flash_wrp1br`]
module*/
pub type FLASH_WRP1BR = crate::Reg<flash_wrp1br::FLASH_WRP1BRrs>;
///FLASH WRP area B address register
pub mod flash_wrp1br;
/**FLASH_SECR (rw) register accessor: FLASH security register

You can [`read`](crate::Reg::read) this register and get [`flash_secr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_secr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:FLASH_SECR)

For information about available fields see [`mod@flash_secr`]
module*/
pub type FLASH_SECR = crate::Reg<flash_secr::FLASH_SECRrs>;
///FLASH security register
pub mod flash_secr;
