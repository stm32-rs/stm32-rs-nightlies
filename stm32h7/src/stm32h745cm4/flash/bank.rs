#[repr(C)]
#[derive(Debug)]
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub struct BANK {
    keyr: KEYR,
    _reserved1: [u8; 0x04],
    cr: CR,
    sr: SR,
    ccr: CCR,
    _reserved4: [u8; 0x10],
    prar_cur: PRAR_CUR,
    prar_prg: PRAR_PRG,
    scar_cur: SCAR_CUR,
    scar_prg: SCAR_PRG,
    wpsn_curr: WPSN_CURR,
    wpsn_prgr: WPSN_PRGR,
    _reserved10: [u8; 0x10],
    crccr: CRCCR,
    crcsaddr: CRCSADDR,
    crceaddr: CRCEADDR,
    _reserved13: [u8; 0x04],
    far: FAR,
    _reserved_end: [u8; 0xa0],
}
impl BANK {
    ///0x00 - FLASH key register for bank 1
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x08 - FLASH control register for bank 1
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - FLASH status register for bank 1
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x10 - FLASH clear control register for bank 1
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x24 - FLASH protection address for bank 1
    #[inline(always)]
    pub const fn prar_cur(&self) -> &PRAR_CUR {
        &self.prar_cur
    }
    ///0x28 - FLASH protection address for bank 1
    #[inline(always)]
    pub const fn prar_prg(&self) -> &PRAR_PRG {
        &self.prar_prg
    }
    ///0x2c - FLASH secure address for bank 1
    #[inline(always)]
    pub const fn scar_cur(&self) -> &SCAR_CUR {
        &self.scar_cur
    }
    ///0x30 - FLASH secure address for bank 1
    #[inline(always)]
    pub const fn scar_prg(&self) -> &SCAR_PRG {
        &self.scar_prg
    }
    ///0x34 - FLASH write sector protection for bank 1
    #[inline(always)]
    pub const fn wpsn_curr(&self) -> &WPSN_CURR {
        &self.wpsn_curr
    }
    ///0x38 - FLASH write sector protection for bank 1
    #[inline(always)]
    pub const fn wpsn_prgr(&self) -> &WPSN_PRGR {
        &self.wpsn_prgr
    }
    ///0x4c - FLASH CRC control register for bank 1
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    ///0x50 - FLASH CRC start address register for bank 1
    #[inline(always)]
    pub const fn crcsaddr(&self) -> &CRCSADDR {
        &self.crcsaddr
    }
    ///0x54 - FLASH CRC end address register for bank 1
    #[inline(always)]
    pub const fn crceaddr(&self) -> &CRCEADDR {
        &self.crceaddr
    }
    ///0x5c - FLASH ECC fail address for bank 1
    #[inline(always)]
    pub const fn far(&self) -> &FAR {
        &self.far
    }
}
/**KEYR (w) register accessor: FLASH key register for bank 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///FLASH key register for bank 1
pub mod keyr;
/**CR (rw) register accessor: FLASH control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///FLASH control register for bank 1
pub mod cr;
/**SR (rw) register accessor: FLASH status register for bank 1

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FLASH status register for bank 1
pub mod sr;
/**CCR (rw) register accessor: FLASH clear control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///FLASH clear control register for bank 1
pub mod ccr;
/**PRAR_CUR (rw) register accessor: FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prar_cur`] module*/
pub type PRAR_CUR = crate::Reg<prar_cur::PRAR_CURrs>;
///FLASH protection address for bank 1
pub mod prar_cur;
/**PRAR_PRG (rw) register accessor: FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prar_prg`] module*/
pub type PRAR_PRG = crate::Reg<prar_prg::PRAR_PRGrs>;
///FLASH protection address for bank 1
pub mod prar_prg;
/**SCAR_CUR (rw) register accessor: FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scar_cur`] module*/
pub type SCAR_CUR = crate::Reg<scar_cur::SCAR_CURrs>;
///FLASH secure address for bank 1
pub mod scar_cur;
/**SCAR_PRG (rw) register accessor: FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scar_prg`] module*/
pub type SCAR_PRG = crate::Reg<scar_prg::SCAR_PRGrs>;
///FLASH secure address for bank 1
pub mod scar_prg;
/**WPSN_CURR (rw) register accessor: FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_curr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_curr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wpsn_curr`] module*/
pub type WPSN_CURR = crate::Reg<wpsn_curr::WPSN_CURRrs>;
///FLASH write sector protection for bank 1
pub mod wpsn_curr;
/**WPSN_PRGR (rw) register accessor: FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_prgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wpsn_prgr`] module*/
pub type WPSN_PRGR = crate::Reg<wpsn_prgr::WPSN_PRGRrs>;
///FLASH write sector protection for bank 1
pub mod wpsn_prgr;
/**CRCCR (rw) register accessor: FLASH CRC control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crccr`] module*/
pub type CRCCR = crate::Reg<crccr::CRCCRrs>;
///FLASH CRC control register for bank 1
pub mod crccr;
/**CRCSADDR (rw) register accessor: FLASH CRC start address register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crcsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcsaddr`] module*/
pub type CRCSADDR = crate::Reg<crcsaddr::CRCSADDRrs>;
///FLASH CRC start address register for bank 1
pub mod crcsaddr;
/**CRCEADDR (rw) register accessor: FLASH CRC end address register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crceaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crceaddr`] module*/
pub type CRCEADDR = crate::Reg<crceaddr::CRCEADDRrs>;
///FLASH CRC end address register for bank 1
pub mod crceaddr;
/**FAR (rw) register accessor: FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@far`] module*/
pub type FAR = crate::Reg<far::FARrs>;
///FLASH ECC fail address for bank 1
pub mod far;
