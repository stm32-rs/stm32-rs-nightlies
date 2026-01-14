#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - FLASH access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - FLASH key register
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    ///0x0c - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x18 - FLASH operation status register
    #[inline(always)]
    pub const fn opsr(&self) -> &OPSR {
        &self.opsr
    }
    ///0x1c - FLASH option control register
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        &self.optcr
    }
    ///0x20 - FLASH non-secure status register
    #[inline(always)]
    pub const fn nssr(&self) -> &NSSR {
        &self.nssr
    }
    ///0x24 - FLASH secure status register
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    ///0x28 - FLASH Non Secure control register
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    ///0x30 - FLASH non-secure clear control register
    #[inline(always)]
    pub const fn nsccr(&self) -> &NSCCR {
        &self.nsccr
    }
    ///0x3c - FLASH privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x48 - FLASH HDP extension register
    #[inline(always)]
    pub const fn hdpextr(&self) -> &HDPEXTR {
        &self.hdpextr
    }
    ///0x50 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        &self.optsr_cur
    }
    ///0x54 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        &self.optsr_prg
    }
    ///0x70 - FLASH option status register 2
    #[inline(always)]
    pub const fn optsr2_cur(&self) -> &OPTSR2_CUR {
        &self.optsr2_cur
    }
    ///0x74 - FLASH option status register 2
    #[inline(always)]
    pub const fn optsr2_prg(&self) -> &OPTSR2_PRG {
        &self.optsr2_prg
    }
    ///0x80 - FLASH non-secure unique boot entry register
    #[inline(always)]
    pub const fn nsbootr_cur(&self) -> &NSBOOTR_CUR {
        &self.nsbootr_cur
    }
    ///0x84 - FLASH non-secure unique boot entry address
    #[inline(always)]
    pub const fn nsbootr_prg(&self) -> &NSBOOTR_PRG {
        &self.nsbootr_prg
    }
    ///0x90 - FLASH non-secure OTP block lock
    #[inline(always)]
    pub const fn otpblr_cur(&self) -> &OTPBLR_CUR {
        &self.otpblr_cur
    }
    ///0x94 - FLASH non-secure OTP block lock
    #[inline(always)]
    pub const fn otpblr_prg(&self) -> &OTPBLR_PRG {
        &self.otpblr_prg
    }
    ///0xc0 - FLASH privilege register for bank 1
    #[inline(always)]
    pub const fn privbb1r(&self) -> &PRIVBB1R {
        &self.privbb1r
    }
    ///0xe8 - FLASH write sector protection for Bank1
    #[inline(always)]
    pub const fn wrpsgn1r_cur(&self) -> &WRPSGN1R_CUR {
        &self.wrpsgn1r_cur
    }
    ///0xec - FLASH write sector protection for Bank1
    #[inline(always)]
    pub const fn wrpsgn1r_prg(&self) -> &WRPSGN1R_PRG {
        &self.wrpsgn1r_prg
    }
    ///0xf8 - FLASH HDP Bank1 register
    #[inline(always)]
    pub const fn hdp1r_cur(&self) -> &HDP1R_CUR {
        &self.hdp1r_cur
    }
    ///0xfc - FLASH HDP Bank1 register
    #[inline(always)]
    pub const fn hdp1r_prg(&self) -> &HDP1R_PRG {
        &self.hdp1r_prg
    }
    ///0x100 - FLASH Flash ECC correction register
    #[inline(always)]
    pub const fn ecccorr(&self) -> &ECCCORR {
        &self.ecccorr
    }
    ///0x104 - FLASH ECC detection register
    #[inline(always)]
    pub const fn eccdetr(&self) -> &ECCDETR {
        &self.eccdetr
    }
    ///0x108 - FLASH ECC data
    #[inline(always)]
    pub const fn eccdr(&self) -> &ECCDR {
        &self.eccdr
    }
    ///0x1e8 - FLASH write sector protection for Bank2
    #[inline(always)]
    pub const fn wrpsgn2r_cur(&self) -> &WRPSGN2R_CUR {
        &self.wrpsgn2r_cur
    }
    ///0x1ec - FLASH write sector protection for Bank2
    #[inline(always)]
    pub const fn wrpsgn2r_prg(&self) -> &WRPSGN2R_PRG {
        &self.wrpsgn2r_prg
    }
    ///0x1f8 - FLASH HDP Bank2 register
    #[inline(always)]
    pub const fn hdp2r_cur(&self) -> &HDP2R_CUR {
        &self.hdp2r_cur
    }
    ///0x1fc - FLASH HDP Bank2 register
    #[inline(always)]
    pub const fn hdp2r_prg(&self) -> &HDP2R_PRG {
        &self.hdp2r_prg
    }
}
/**ACR (rw) register accessor: FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///FLASH access control register
pub mod acr;
/**NSKEYR (w) register accessor: FLASH key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSKEYR)

For information about available fields see [`mod@nskeyr`] module*/
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
///FLASH key register
pub mod nskeyr;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**OPSR (r) register accessor: FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPSR)

For information about available fields see [`mod@opsr`] module*/
pub type OPSR = crate::Reg<opsr::OPSRrs>;
///FLASH operation status register
pub mod opsr;
/**OPTCR (rw) register accessor: FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTCR)

For information about available fields see [`mod@optcr`] module*/
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
///FLASH option control register
pub mod optcr;
/**NSSR (r) register accessor: FLASH non-secure status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSSR)

For information about available fields see [`mod@nssr`] module*/
pub type NSSR = crate::Reg<nssr::NSSRrs>;
///FLASH non-secure status register
pub mod nssr;
/**SECSR (r) register accessor: FLASH secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///FLASH secure status register
pub mod secsr;
/**NSCR (rw) register accessor: FLASH Non Secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSCR)

For information about available fields see [`mod@nscr`] module*/
pub type NSCR = crate::Reg<nscr::NSCRrs>;
///FLASH Non Secure control register
pub mod nscr;
/**NSCCR (w) register accessor: FLASH non-secure clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSCCR)

For information about available fields see [`mod@nsccr`] module*/
pub type NSCCR = crate::Reg<nsccr::NSCCRrs>;
///FLASH non-secure clear control register
pub mod nsccr;
/**PRIVCFGR (w) register accessor: FLASH privilege configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///FLASH privilege configuration register
pub mod privcfgr;
/**HDPEXTR (rw) register accessor: FLASH HDP extension register

You can [`read`](crate::Reg::read) this register and get [`hdpextr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpextr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDPEXTR)

For information about available fields see [`mod@hdpextr`] module*/
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTRrs>;
///FLASH HDP extension register
pub mod hdpextr;
/**OPTSR_CUR (r) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTSR_CUR)

For information about available fields see [`mod@optsr_cur`] module*/
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
///FLASH option status register
pub mod optsr_cur;
/**OPTSR_PRG (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTSR_PRG)

For information about available fields see [`mod@optsr_prg`] module*/
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
///FLASH option status register
pub mod optsr_prg;
/**OPTSR2_CUR (r) register accessor: FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTSR2_CUR)

For information about available fields see [`mod@optsr2_cur`] module*/
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CURrs>;
///FLASH option status register 2
pub mod optsr2_cur;
/**OPTSR2_PRG (rw) register accessor: FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTSR2_PRG)

For information about available fields see [`mod@optsr2_prg`] module*/
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRGrs>;
///FLASH option status register 2
pub mod optsr2_prg;
/**NSBOOTR_CUR (r) register accessor: FLASH non-secure unique boot entry register

You can [`read`](crate::Reg::read) this register and get [`nsbootr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSBOOTR_CUR)

For information about available fields see [`mod@nsbootr_cur`] module*/
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CURrs>;
///FLASH non-secure unique boot entry register
pub mod nsbootr_cur;
/**NSBOOTR_PRG (rw) register accessor: FLASH non-secure unique boot entry address

You can [`read`](crate::Reg::read) this register and get [`nsbootr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:NSBOOTR_PRG)

For information about available fields see [`mod@nsbootr_prg`] module*/
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRGrs>;
///FLASH non-secure unique boot entry address
pub mod nsbootr_prg;
/**OTPBLR_CUR (r) register accessor: FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OTPBLR_CUR)

For information about available fields see [`mod@otpblr_cur`] module*/
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CURrs>;
///FLASH non-secure OTP block lock
pub mod otpblr_cur;
/**OTPBLR_PRG (rw) register accessor: FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpblr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OTPBLR_PRG)

For information about available fields see [`mod@otpblr_prg`] module*/
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRGrs>;
///FLASH non-secure OTP block lock
pub mod otpblr_prg;
/**PRIVBB1R (rw) register accessor: FLASH privilege register for bank 1

You can [`read`](crate::Reg::read) this register and get [`privbb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:PRIVBB1R)

For information about available fields see [`mod@privbb1r`] module*/
pub type PRIVBB1R = crate::Reg<privbb1r::PRIVBB1Rrs>;
///FLASH privilege register for bank 1
pub mod privbb1r;
/**WRPSGN1R_CUR (r) register accessor: FLASH write sector protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN1R_CUR)

For information about available fields see [`mod@wrpsgn1r_cur`] module*/
pub type WRPSGN1R_CUR = crate::Reg<wrpsgn1r_cur::WRPSGN1R_CURrs>;
///FLASH write sector protection for Bank1
pub mod wrpsgn1r_cur;
/**WRPSGN1R_PRG (rw) register accessor: FLASH write sector protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN1R_PRG)

For information about available fields see [`mod@wrpsgn1r_prg`] module*/
pub type WRPSGN1R_PRG = crate::Reg<wrpsgn1r_prg::WRPSGN1R_PRGrs>;
///FLASH write sector protection for Bank1
pub mod wrpsgn1r_prg;
/**HDP1R_CUR (r) register accessor: FLASH HDP Bank1 register

You can [`read`](crate::Reg::read) this register and get [`hdp1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDP1R_CUR)

For information about available fields see [`mod@hdp1r_cur`] module*/
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CURrs>;
///FLASH HDP Bank1 register
pub mod hdp1r_cur;
/**HDP1R_PRG (r) register accessor: FLASH HDP Bank1 register

You can [`read`](crate::Reg::read) this register and get [`hdp1r_prg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDP1R_PRG)

For information about available fields see [`mod@hdp1r_prg`] module*/
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRGrs>;
///FLASH HDP Bank1 register
pub mod hdp1r_prg;
/**ECCCORR (rw) register accessor: FLASH Flash ECC correction register

You can [`read`](crate::Reg::read) this register and get [`ecccorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecccorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:ECCCORR)

For information about available fields see [`mod@ecccorr`] module*/
pub type ECCCORR = crate::Reg<ecccorr::ECCCORRrs>;
///FLASH Flash ECC correction register
pub mod ecccorr;
/**ECCDETR (rw) register accessor: FLASH ECC detection register

You can [`read`](crate::Reg::read) this register and get [`eccdetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:ECCDETR)

For information about available fields see [`mod@eccdetr`] module*/
pub type ECCDETR = crate::Reg<eccdetr::ECCDETRrs>;
///FLASH ECC detection register
pub mod eccdetr;
/**ECCDR (r) register accessor: FLASH ECC data

You can [`read`](crate::Reg::read) this register and get [`eccdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:ECCDR)

For information about available fields see [`mod@eccdr`] module*/
pub type ECCDR = crate::Reg<eccdr::ECCDRrs>;
///FLASH ECC data
pub mod eccdr;
/**WRPSGN2R_CUR (r) register accessor: FLASH write sector protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN2R_CUR)

For information about available fields see [`mod@wrpsgn2r_cur`] module*/
pub type WRPSGN2R_CUR = crate::Reg<wrpsgn2r_cur::WRPSGN2R_CURrs>;
///FLASH write sector protection for Bank2
pub mod wrpsgn2r_cur;
/**WRPSGN2R_PRG (rw) register accessor: FLASH write sector protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN2R_PRG)

For information about available fields see [`mod@wrpsgn2r_prg`] module*/
pub type WRPSGN2R_PRG = crate::Reg<wrpsgn2r_prg::WRPSGN2R_PRGrs>;
///FLASH write sector protection for Bank2
pub mod wrpsgn2r_prg;
/**HDP2R_CUR (r) register accessor: FLASH HDP Bank2 register

You can [`read`](crate::Reg::read) this register and get [`hdp2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDP2R_CUR)

For information about available fields see [`mod@hdp2r_cur`] module*/
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CURrs>;
///FLASH HDP Bank2 register
pub mod hdp2r_cur;
/**HDP2R_PRG (rw) register accessor: FLASH HDP Bank2 register

You can [`read`](crate::Reg::read) this register and get [`hdp2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDP2R_PRG)

For information about available fields see [`mod@hdp2r_prg`] module*/
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRGrs>;
///FLASH HDP Bank2 register
pub mod hdp2r_prg;
