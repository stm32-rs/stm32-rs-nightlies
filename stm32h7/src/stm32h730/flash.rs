#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    optkeyr: OPTKEYR,
    cr: CR,
    sr: SR,
    ccr: CCR,
    optcr: OPTCR,
    optsr_cur: OPTSR_CUR,
    optsr_prg: OPTSR_PRG,
    optccr: OPTCCR,
    prar_cur: PRAR_CUR,
    prar_prg: PRAR_PRG,
    scar_cur: SCAR_CUR,
    scar_prg: SCAR_PRG,
    wpsn_cur: WPSN_CUR,
    wpsn_prg: WPSN_PRG,
    boot_cur: BOOT_CUR,
    boot_prg: BOOT_PRG,
    _reserved18: [u8; 0x08],
    crccr: CRCCR,
    crcsaddr: CRCSADDR,
    crceaddr: CRCEADDR,
    crcdatar: CRCDATAR,
    ecc_far: ECC_FAR,
    _reserved23: [u8; 0x0c],
    optsr2_cur: OPTSR2_CUR,
    optsr2_prg: OPTSR2_PRG,
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - FLASH key register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x08 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x0c - FLASH control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - FLASH status register for bank 1
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - FLASH clear control register for bank 1
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x18 - FLASH option control register
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        &self.optcr
    }
    ///0x1c - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        &self.optsr_cur
    }
    ///0x20 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        &self.optsr_prg
    }
    ///0x24 - FLASH option clear control register
    #[inline(always)]
    pub const fn optccr(&self) -> &OPTCCR {
        &self.optccr
    }
    ///0x28 - FLASH protection address for bank 1
    #[inline(always)]
    pub const fn prar_cur(&self) -> &PRAR_CUR {
        &self.prar_cur
    }
    ///0x2c - FLASH protection address for bank 1
    #[inline(always)]
    pub const fn prar_prg(&self) -> &PRAR_PRG {
        &self.prar_prg
    }
    ///0x30 - FLASH secure address for bank 1
    #[inline(always)]
    pub const fn scar_cur(&self) -> &SCAR_CUR {
        &self.scar_cur
    }
    ///0x34 - FLASH secure address for bank 1
    #[inline(always)]
    pub const fn scar_prg(&self) -> &SCAR_PRG {
        &self.scar_prg
    }
    ///0x38 - FLASH write sector protection for bank 1
    #[inline(always)]
    pub const fn wpsn_cur(&self) -> &WPSN_CUR {
        &self.wpsn_cur
    }
    ///0x3c - FLASH write sector protection for bank 1
    #[inline(always)]
    pub const fn wpsn_prg(&self) -> &WPSN_PRG {
        &self.wpsn_prg
    }
    ///0x40 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_cur(&self) -> &BOOT_CUR {
        &self.boot_cur
    }
    ///0x44 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_prg(&self) -> &BOOT_PRG {
        &self.boot_prg
    }
    ///0x50 - FLASH CRC control register for bank 1
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    ///0x54 - FLASH CRC start address register for bank 1
    #[inline(always)]
    pub const fn crcsaddr(&self) -> &CRCSADDR {
        &self.crcsaddr
    }
    ///0x58 - FLASH CRC end address register for bank 1
    #[inline(always)]
    pub const fn crceaddr(&self) -> &CRCEADDR {
        &self.crceaddr
    }
    ///0x5c - FLASH CRC data register
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        &self.crcdatar
    }
    ///0x60 - FLASH ECC fail address for bank 1
    #[inline(always)]
    pub const fn ecc_far(&self) -> &ECC_FAR {
        &self.ecc_far
    }
    ///0x70 - FLASH ECC fail address for bank 1
    #[inline(always)]
    pub const fn optsr2_cur(&self) -> &OPTSR2_CUR {
        &self.optsr2_cur
    }
    ///0x74 - FLASH ECC fail address for bank 1
    #[inline(always)]
    pub const fn optsr2_prg(&self) -> &OPTSR2_PRG {
        &self.optsr2_prg
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**KEYR (rw) register accessor: FLASH key register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///FLASH key register
pub mod keyr;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**CR (rw) register accessor: FLASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///FLASH control register
pub mod cr;
/**SR (rw) register accessor: FLASH status register for bank 1

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FLASH status register for bank 1
pub mod sr;
/**CCR (rw) register accessor: FLASH clear control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///FLASH clear control register for bank 1
pub mod ccr;
/**OPTCR (rw) register accessor: FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTCR)

For information about available fields see [`mod@optcr`] module*/
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
///FLASH option control register
pub mod optcr;
/**OPTSR_CUR (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTSR_CUR)

For information about available fields see [`mod@optsr_cur`] module*/
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
///FLASH option status register
pub mod optsr_cur;
/**OPTSR_PRG (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTSR_PRG)

For information about available fields see [`mod@optsr_prg`] module*/
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
///FLASH option status register
pub mod optsr_prg;
/**OPTCCR (w) register accessor: FLASH option clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTCCR)

For information about available fields see [`mod@optccr`] module*/
pub type OPTCCR = crate::Reg<optccr::OPTCCRrs>;
///FLASH option clear control register
pub mod optccr;
/**PRAR_CUR (r) register accessor: FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:PRAR_CUR)

For information about available fields see [`mod@prar_cur`] module*/
pub type PRAR_CUR = crate::Reg<prar_cur::PRAR_CURrs>;
///FLASH protection address for bank 1
pub mod prar_cur;
/**PRAR_PRG (rw) register accessor: FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:PRAR_PRG)

For information about available fields see [`mod@prar_prg`] module*/
pub type PRAR_PRG = crate::Reg<prar_prg::PRAR_PRGrs>;
///FLASH protection address for bank 1
pub mod prar_prg;
/**SCAR_CUR (rw) register accessor: FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:SCAR_CUR)

For information about available fields see [`mod@scar_cur`] module*/
pub type SCAR_CUR = crate::Reg<scar_cur::SCAR_CURrs>;
///FLASH secure address for bank 1
pub mod scar_cur;
/**SCAR_PRG (rw) register accessor: FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:SCAR_PRG)

For information about available fields see [`mod@scar_prg`] module*/
pub type SCAR_PRG = crate::Reg<scar_prg::SCAR_PRGrs>;
///FLASH secure address for bank 1
pub mod scar_prg;
/**WPSN_CUR (r) register accessor: FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:WPSN_CUR)

For information about available fields see [`mod@wpsn_cur`] module*/
pub type WPSN_CUR = crate::Reg<wpsn_cur::WPSN_CURrs>;
///FLASH write sector protection for bank 1
pub mod wpsn_cur;
/**WPSN_PRG (rw) register accessor: FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:WPSN_PRG)

For information about available fields see [`mod@wpsn_prg`] module*/
pub type WPSN_PRG = crate::Reg<wpsn_prg::WPSN_PRGrs>;
///FLASH write sector protection for bank 1
pub mod wpsn_prg;
/**BOOT_CUR (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:BOOT_CUR)

For information about available fields see [`mod@boot_cur`] module*/
pub type BOOT_CUR = crate::Reg<boot_cur::BOOT_CURrs>;
///FLASH register with boot address
pub mod boot_cur;
/**BOOT_PRG (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:BOOT_PRG)

For information about available fields see [`mod@boot_prg`] module*/
pub type BOOT_PRG = crate::Reg<boot_prg::BOOT_PRGrs>;
///FLASH register with boot address
pub mod boot_prg;
/**CRCCR (rw) register accessor: FLASH CRC control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CRCCR)

For information about available fields see [`mod@crccr`] module*/
pub type CRCCR = crate::Reg<crccr::CRCCRrs>;
///FLASH CRC control register for bank 1
pub mod crccr;
/**CRCSADDR (rw) register accessor: FLASH CRC start address register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crcsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CRCSADDR)

For information about available fields see [`mod@crcsaddr`] module*/
pub type CRCSADDR = crate::Reg<crcsaddr::CRCSADDRrs>;
///FLASH CRC start address register for bank 1
pub mod crcsaddr;
/**CRCEADDR (rw) register accessor: FLASH CRC end address register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crceaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CRCEADDR)

For information about available fields see [`mod@crceaddr`] module*/
pub type CRCEADDR = crate::Reg<crceaddr::CRCEADDRrs>;
///FLASH CRC end address register for bank 1
pub mod crceaddr;
/**CRCDATAR (rw) register accessor: FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:CRCDATAR)

For information about available fields see [`mod@crcdatar`] module*/
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATARrs>;
///FLASH CRC data register
pub mod crcdatar;
/**ECC_FAR (r) register accessor: FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`ecc_far::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:ECC_FAR)

For information about available fields see [`mod@ecc_far`] module*/
pub type ECC_FAR = crate::Reg<ecc_far::ECC_FARrs>;
///FLASH ECC fail address for bank 1
pub mod ecc_far;
/**OPTSR2_CUR (r) register accessor: FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTSR2_CUR)

For information about available fields see [`mod@optsr2_cur`] module*/
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CURrs>;
///FLASH ECC fail address for bank 1
pub mod optsr2_cur;
/**OPTSR2_PRG (rw) register accessor: FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTSR2_PRG)

For information about available fields see [`mod@optsr2_prg`] module*/
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRGrs>;
///FLASH ECC fail address for bank 1
pub mod optsr2_prg;
