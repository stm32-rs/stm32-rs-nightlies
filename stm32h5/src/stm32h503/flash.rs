#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: ACR,
    nskeyr: NSKEYR,
    _reserved2: [u8; 0x04],
    optkeyr: OPTKEYR,
    _reserved3: [u8; 0x08],
    opsr: OPSR,
    optcr: OPTCR,
    nssr: NSSR,
    secsr: SECSR,
    nscr: NSCR,
    _reserved8: [u8; 0x04],
    nsccr: NSCCR,
    _reserved9: [u8; 0x08],
    privcfgr: PRIVCFGR,
    _reserved10: [u8; 0x08],
    hdpextr: HDPEXTR,
    _reserved11: [u8; 0x04],
    optsr_cur: OPTSR_CUR,
    optsr_prg: OPTSR_PRG,
    _reserved13: [u8; 0x18],
    optsr2_cur: OPTSR2_CUR,
    optsr2_prg: OPTSR2_PRG,
    _reserved15: [u8; 0x08],
    nsbootr_cur: NSBOOTR_CUR,
    nsbootr_prg: NSBOOTR_PRG,
    _reserved17: [u8; 0x08],
    otpblr_cur: OTPBLR_CUR,
    otpblr_prg: OTPBLR_PRG,
    _reserved19: [u8; 0x28],
    privbb1r: PRIVBB1R,
    _reserved20: [u8; 0x24],
    wrpsgn1r_cur: WRPSGN1R_CUR,
    wrpsgn1r_prg: WRPSGN1R_PRG,
    _reserved22: [u8; 0x08],
    hdp1r_cur: HDP1R_CUR,
    hdp1r_prg: HDP1R_PRG,
    ecccorr: ECCCORR,
    eccdetr: ECCDETR,
    eccdr: ECCDR,
    _reserved27: [u8; 0xdc],
    wrpsgn2r_cur: WRPSGN2R_CUR,
    wrpsgn2r_prg: WRPSGN2R_PRG,
    _reserved29: [u8; 0x08],
    hdp2r_cur: HDP2R_CUR,
    hdp2r_prg: HDP2R_PRG,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0x04 - FLASH key register"]
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    #[doc = "0x0c - FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    #[doc = "0x18 - FLASH operation status register"]
    #[inline(always)]
    pub const fn opsr(&self) -> &OPSR {
        &self.opsr
    }
    #[doc = "0x1c - FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        &self.optcr
    }
    #[doc = "0x20 - FLASH non-secure status register"]
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    #[doc = "0x24 - FLASH secure status register"]
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    #[doc = "0x28 - FLASH Non Secure control register"]
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    #[doc = "0x30 - FLASH non-secure clear control register"]
    #[inline(always)]
    pub const fn nsccr(&self) -> &NSCCR {
        &self.nsccr
    }
    #[doc = "0x3c - FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    #[doc = "0x48 - FLASH HDP extension register"]
    #[inline(always)]
    pub const fn hdpextr(&self) -> &HDPEXTR {
        &self.hdpextr
    }
    #[doc = "0x50 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        &self.optsr_cur
    }
    #[doc = "0x54 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        &self.optsr_prg
    }
    #[doc = "0x70 - FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_cur(&self) -> &OPTSR2_CUR {
        &self.optsr2_cur
    }
    #[doc = "0x74 - FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_prg(&self) -> &OPTSR2_PRG {
        &self.optsr2_prg
    }
    #[doc = "0x80 - FLASH non-secure unique boot entry register"]
    #[inline(always)]
    pub const fn nsbootr_cur(&self) -> &NSBOOTR_CUR {
        &self.nsbootr_cur
    }
    #[doc = "0x84 - FLASH non-secure unique boot entry address"]
    #[inline(always)]
    pub const fn nsbootr_prg(&self) -> &NSBOOTR_PRG {
        &self.nsbootr_prg
    }
    #[doc = "0x90 - FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_cur(&self) -> &OTPBLR_CUR {
        &self.otpblr_cur
    }
    #[doc = "0x94 - FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_prg(&self) -> &OTPBLR_PRG {
        &self.otpblr_prg
    }
    #[doc = "0xc0 - FLASH privilege register for bank 1"]
    #[inline(always)]
    pub const fn privbb1r(&self) -> &PRIVBB1R {
        &self.privbb1r
    }
    #[doc = "0xe8 - FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn wrpsgn1r_cur(&self) -> &WRPSGN1R_CUR {
        &self.wrpsgn1r_cur
    }
    #[doc = "0xec - FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn wrpsgn1r_prg(&self) -> &WRPSGN1R_PRG {
        &self.wrpsgn1r_prg
    }
    #[doc = "0xf8 - FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn hdp1r_cur(&self) -> &HDP1R_CUR {
        &self.hdp1r_cur
    }
    #[doc = "0xfc - FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn hdp1r_prg(&self) -> &HDP1R_PRG {
        &self.hdp1r_prg
    }
    #[doc = "0x100 - FLASH Flash ECC correction register"]
    #[inline(always)]
    pub const fn ecccorr(&self) -> &ECCCORR {
        &self.ecccorr
    }
    #[doc = "0x104 - FLASH ECC detection register"]
    #[inline(always)]
    pub const fn eccdetr(&self) -> &ECCDETR {
        &self.eccdetr
    }
    #[doc = "0x108 - FLASH ECC data"]
    #[inline(always)]
    pub const fn eccdr(&self) -> &ECCDR {
        &self.eccdr
    }
    #[doc = "0x1e8 - FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn wrpsgn2r_cur(&self) -> &WRPSGN2R_CUR {
        &self.wrpsgn2r_cur
    }
    #[doc = "0x1ec - FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn wrpsgn2r_prg(&self) -> &WRPSGN2R_PRG {
        &self.wrpsgn2r_prg
    }
    #[doc = "0x1f8 - FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn hdp2r_cur(&self) -> &HDP2R_CUR {
        &self.hdp2r_cur
    }
    #[doc = "0x1fc - FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn hdp2r_prg(&self) -> &HDP2R_PRG {
        &self.hdp2r_prg
    }
}
#[doc = "ACR (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACRrs>;
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "NSKEYR (w) register accessor: FLASH key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nskeyr`]
module"]
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
#[doc = "FLASH key register"]
pub mod nskeyr;
#[doc = "OPTKEYR (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`]
module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPSR (r) register accessor: FLASH operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opsr`]
module"]
pub type OPSR = crate::Reg<opsr::OPSRrs>;
#[doc = "FLASH operation status register"]
pub mod opsr;
#[doc = "OPTCR (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr`]
module"]
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "NSSR (r) register accessor: FLASH non-secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nssr`]
module"]
pub type NSSR = crate::Reg<nssr::NSSRrs>;
#[doc = "FLASH non-secure status register"]
pub mod nssr;
#[doc = "SECSR (r) register accessor: FLASH secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secsr`]
module"]
pub type SECSR = crate::Reg<secsr::SECSRrs>;
#[doc = "FLASH secure status register"]
pub mod secsr;
#[doc = "NSCR (rw) register accessor: FLASH Non Secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nscr`]
module"]
pub type NSCR = crate::Reg<nscr::NSCRrs>;
#[doc = "FLASH Non Secure control register"]
pub mod nscr;
#[doc = "NSCCR (w) register accessor: FLASH non-secure clear control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsccr`]
module"]
pub type NSCCR = crate::Reg<nsccr::NSCCRrs>;
#[doc = "FLASH non-secure clear control register"]
pub mod nsccr;
#[doc = "PRIVCFGR (w) register accessor: FLASH privilege configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`]
module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
#[doc = "FLASH privilege configuration register"]
pub mod privcfgr;
#[doc = "HDPEXTR (rw) register accessor: FLASH HDP extension register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdpextr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdpextr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdpextr`]
module"]
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTRrs>;
#[doc = "FLASH HDP extension register"]
pub mod hdpextr;
#[doc = "OPTSR_CUR (r) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur`]
module"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg`]
module"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR2_CUR (r) register accessor: FLASH option status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr2_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr2_cur`]
module"]
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CURrs>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_cur;
#[doc = "OPTSR2_PRG (rw) register accessor: FLASH option status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr2_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr2_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr2_prg`]
module"]
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRGrs>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_prg;
#[doc = "NSBOOTR_CUR (r) register accessor: FLASH non-secure unique boot entry register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsbootr_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootr_cur`]
module"]
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CURrs>;
#[doc = "FLASH non-secure unique boot entry register"]
pub mod nsbootr_cur;
#[doc = "NSBOOTR_PRG (rw) register accessor: FLASH non-secure unique boot entry address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsbootr_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootr_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootr_prg`]
module"]
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRGrs>;
#[doc = "FLASH non-secure unique boot entry address"]
pub mod nsbootr_prg;
#[doc = "OTPBLR_CUR (r) register accessor: FLASH non-secure OTP block lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otpblr_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpblr_cur`]
module"]
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CURrs>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_cur;
#[doc = "OTPBLR_PRG (rw) register accessor: FLASH non-secure OTP block lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otpblr_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otpblr_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpblr_prg`]
module"]
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRGrs>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_prg;
#[doc = "PRIVBB1R (rw) register accessor: FLASH privilege register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privbb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privbb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privbb1r`]
module"]
pub type PRIVBB1R = crate::Reg<privbb1r::PRIVBB1Rrs>;
#[doc = "FLASH privilege register for bank 1"]
pub mod privbb1r;
#[doc = "WRPSGN1R_CUR (r) register accessor: FLASH write sector protection for Bank1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn1r_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn1r_cur`]
module"]
pub type WRPSGN1R_CUR = crate::Reg<wrpsgn1r_cur::WRPSGN1R_CURrs>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_cur;
#[doc = "WRPSGN1R_PRG (rw) register accessor: FLASH write sector protection for Bank1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn1r_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn1r_prg`]
module"]
pub type WRPSGN1R_PRG = crate::Reg<wrpsgn1r_prg::WRPSGN1R_PRGrs>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_prg;
#[doc = "HDP1R_CUR (r) register accessor: FLASH HDP Bank1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp1r_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp1r_cur`]
module"]
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CURrs>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_cur;
#[doc = "HDP1R_PRG (r) register accessor: FLASH HDP Bank1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp1r_prg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp1r_prg`]
module"]
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRGrs>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_prg;
#[doc = "ECCCORR (rw) register accessor: FLASH Flash ECC correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecccorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecccorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecccorr`]
module"]
pub type ECCCORR = crate::Reg<ecccorr::ECCCORRrs>;
#[doc = "FLASH Flash ECC correction register"]
pub mod ecccorr;
#[doc = "ECCDETR (rw) register accessor: FLASH ECC detection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccdetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccdetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdetr`]
module"]
pub type ECCDETR = crate::Reg<eccdetr::ECCDETRrs>;
#[doc = "FLASH ECC detection register"]
pub mod eccdetr;
#[doc = "ECCDR (r) register accessor: FLASH ECC data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdr`]
module"]
pub type ECCDR = crate::Reg<eccdr::ECCDRrs>;
#[doc = "FLASH ECC data"]
pub mod eccdr;
#[doc = "WRPSGN2R_CUR (r) register accessor: FLASH write sector protection for Bank2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn2r_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn2r_cur`]
module"]
pub type WRPSGN2R_CUR = crate::Reg<wrpsgn2r_cur::WRPSGN2R_CURrs>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_cur;
#[doc = "WRPSGN2R_PRG (rw) register accessor: FLASH write sector protection for Bank2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn2r_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpsgn2r_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn2r_prg`]
module"]
pub type WRPSGN2R_PRG = crate::Reg<wrpsgn2r_prg::WRPSGN2R_PRGrs>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_prg;
#[doc = "HDP2R_CUR (r) register accessor: FLASH HDP Bank2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp2r_cur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp2r_cur`]
module"]
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CURrs>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_cur;
#[doc = "HDP2R_PRG (rw) register accessor: FLASH HDP Bank2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp2r_prg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp2r_prg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp2r_prg`]
module"]
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRGrs>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_prg;
