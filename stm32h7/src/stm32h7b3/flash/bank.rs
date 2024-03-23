#[repr(C)]
#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
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
    #[doc = "0x00 - FLASH key register for bank 1"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x08 - FLASH control register for bank 1"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - FLASH status register for bank 1"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - FLASH clear control register for bank 1"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x24 - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_cur(&self) -> &PRAR_CUR {
        &self.prar_cur
    }
    #[doc = "0x28 - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_prg(&self) -> &PRAR_PRG {
        &self.prar_prg
    }
    #[doc = "0x2c - FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_cur(&self) -> &SCAR_CUR {
        &self.scar_cur
    }
    #[doc = "0x30 - FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_prg(&self) -> &SCAR_PRG {
        &self.scar_prg
    }
    #[doc = "0x34 - FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_curr(&self) -> &WPSN_CURR {
        &self.wpsn_curr
    }
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_prgr(&self) -> &WPSN_PRGR {
        &self.wpsn_prgr
    }
    #[doc = "0x4c - FLASH CRC control register for bank 1"]
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    #[doc = "0x50 - FLASH CRC start address register for bank 1"]
    #[inline(always)]
    pub const fn crcsaddr(&self) -> &CRCSADDR {
        &self.crcsaddr
    }
    #[doc = "0x54 - FLASH CRC end address register for bank 1"]
    #[inline(always)]
    pub const fn crceaddr(&self) -> &CRCEADDR {
        &self.crceaddr
    }
    #[doc = "0x5c - FLASH ECC fail address for bank 1"]
    #[inline(always)]
    pub const fn far(&self) -> &FAR {
        &self.far
    }
}
#[doc = "KEYR (rw) register accessor: FLASH key register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "FLASH key register for bank 1"]
pub mod keyr;
#[doc = "CR (rw) register accessor: FLASH control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "FLASH control register for bank 1"]
pub mod cr;
#[doc = "SR (rw) register accessor: FLASH status register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "FLASH status register for bank 1"]
pub mod sr;
#[doc = "CCR (rw) register accessor: FLASH clear control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "FLASH clear control register for bank 1"]
pub mod ccr;
#[doc = "PRAR_CUR (r) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_cur`]
module"]
pub type PRAR_CUR = crate::Reg<prar_cur::PRAR_CURrs>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur;
#[doc = "PRAR_PRG (rw) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_prg`]
module"]
pub type PRAR_PRG = crate::Reg<prar_prg::PRAR_PRGrs>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_prg;
#[doc = "SCAR_CUR (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_cur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_cur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_cur`]
module"]
pub type SCAR_CUR = crate::Reg<scar_cur::SCAR_CURrs>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_cur;
#[doc = "SCAR_PRG (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_prg`]
module"]
pub type SCAR_PRG = crate::Reg<scar_prg::SCAR_PRGrs>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_prg;
#[doc = "WPSN_CURR (r) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_curr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_curr`]
module"]
pub type WPSN_CURR = crate::Reg<wpsn_curr::WPSN_CURRrs>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_curr;
#[doc = "WPSN_PRGR (rw) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_prgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpsn_prgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_prgr`]
module"]
pub type WPSN_PRGR = crate::Reg<wpsn_prgr::WPSN_PRGRrs>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_prgr;
#[doc = "CRCCR (rw) register accessor: FLASH CRC control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr`]
module"]
pub type CRCCR = crate::Reg<crccr::CRCCRrs>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr;
#[doc = "CRCSADDR (rw) register accessor: FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsaddr`]
module"]
pub type CRCSADDR = crate::Reg<crcsaddr::CRCSADDRrs>;
#[doc = "FLASH CRC start address register for bank 1"]
pub mod crcsaddr;
#[doc = "CRCEADDR (rw) register accessor: FLASH CRC end address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crceaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crceaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crceaddr`]
module"]
pub type CRCEADDR = crate::Reg<crceaddr::CRCEADDRrs>;
#[doc = "FLASH CRC end address register for bank 1"]
pub mod crceaddr;
#[doc = "FAR (r) register accessor: FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`]
module"]
pub type FAR = crate::Reg<far::FARrs>;
#[doc = "FLASH ECC fail address for bank 1"]
pub mod far;
