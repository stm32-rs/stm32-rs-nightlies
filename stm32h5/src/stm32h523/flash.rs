#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    nskeyr: NSKEYR,
    seckeyr: SECKEYR,
    optkeyr: OPTKEYR,
    nsobkkeyr: NSOBKKEYR,
    secobkkeyr: SECOBKKEYR,
    opsr: OPSR,
    optcr: OPTCR,
    nssr: NSSR,
    secsr: SECSR,
    nscr: NSCR,
    seccr: SECCR,
    nsccr: NSCCR,
    secccr: SECCCR,
    _reserved14: [u8; 0x04],
    privcfgr: PRIVCFGR,
    nsobkcfgr: NSOBKCFGR,
    secobkcfgr: SECOBKCFGR,
    hdpextr: HDPEXTR,
    _reserved18: [u8; 0x04],
    optsr_cur: OPTSR_CUR,
    optsr_prg: OPTSR_PRG,
    _reserved20: [u8; 0x08],
    nsepochr_cur: NSEPOCHR_CUR,
    _reserved21: [u8; 0x04],
    secepochr_cur: SECEPOCHR_CUR,
    _reserved22: [u8; 0x04],
    optsr2_cur: OPTSR2_CUR,
    optsr2_prg: OPTSR2_PRG,
    _reserved24: [u8; 0x08],
    nsbootr_cur: NSBOOTR_CUR,
    nsbootr_prg: NSBOOTR_PRG,
    secbootr_cur: SECBOOTR_CUR,
    bootr_prg: BOOTR_PRG,
    otpblr_cur: OTPBLR_CUR,
    otpblr_prg: OTPBLR_PRG,
    _reserved30: [u8; 0x08],
    secbb1r1: SECBB1R1,
    secbb1r2: SECBB1R2,
    secbb1r3: SECBB1R3,
    secbb1r4: SECBB1R4,
    _reserved34: [u8; 0x10],
    privbb1r1: PRIVBB1R1,
    privbb1r2: PRIVBB1R2,
    privbb1r3: PRIVBB1R3,
    privbb1r4: PRIVBB1R4,
    _reserved38: [u8; 0x10],
    secwm1r_cur: SECWM1R_CUR,
    secwm1r_prg: SECWM1R_PRG,
    wrp1r_cur: WRP1R_CUR,
    wrp1r_prg: WRP1R_PRG,
    edata1r_cur: EDATA1R_CUR,
    edata1r_prg: EDATA1R_PRG,
    hdp1r_cur: HDP1R_CUR,
    hdp1r_prg: HDP1R_PRG,
    ecccorr: ECCCORR,
    eccdetr: ECCDETR,
    eccdr: ECCDR,
    _reserved49: [u8; 0x94],
    secbb2r1: SECBB2R1,
    secbb2r2: SECBB2R2,
    secbb2r3: SECBB2R3,
    secbb2r4: SECBB2R4,
    _reserved53: [u8; 0x10],
    privbb2r1: PRIVBB2R1,
    privbb2r2: PRIVBB2R2,
    privbb2r3: PRIVBB2R3,
    privbb2r4: PRIVBB2R4,
    _reserved57: [u8; 0x10],
    secwm2r_cur: SECWM2R_CUR,
    secwm2r_prg: SECWM2R_PRG,
    wrp2r_cur: WRP2R_CUR,
    wrp2r_prg: WRP2R_PRG,
    edata2r_cur: EDATA2R_CUR,
    edata2r_prg: EDATA2R_PRG,
    hdp2r_cur: HDP2R_CUR,
    hdp2r_prg: HDP2R_PRG,
}
impl RegisterBlock {
    ///0x00 - FLASH access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - FLASH non-secure key register
    #[inline(always)]
    pub const fn nskeyr(&self) -> &NSKEYR {
        &self.nskeyr
    }
    ///0x08 - FLASH secure key register
    #[inline(always)]
    pub const fn seckeyr(&self) -> &SECKEYR {
        &self.seckeyr
    }
    ///0x0c - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x10 - FLASH non-secure OBK key register
    #[inline(always)]
    pub const fn nsobkkeyr(&self) -> &NSOBKKEYR {
        &self.nsobkkeyr
    }
    ///0x14 - FLASH secure OBK key register
    #[inline(always)]
    pub const fn secobkkeyr(&self) -> &SECOBKKEYR {
        &self.secobkkeyr
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
    ///0x28 - FLASH non-secure control register
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    ///0x2c - FLASH secure control register
    #[inline(always)]
    pub const fn seccr(&self) -> &SECCR {
        &self.seccr
    }
    ///0x30 - FLASH non-secure clear control register
    #[inline(always)]
    pub const fn nsccr(&self) -> &NSCCR {
        &self.nsccr
    }
    ///0x34 - FLASH secure clear control register
    #[inline(always)]
    pub const fn secccr(&self) -> &SECCCR {
        &self.secccr
    }
    ///0x3c - FLASH privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x40 - FLASH non-secure OBK configuration register
    #[inline(always)]
    pub const fn nsobkcfgr(&self) -> &NSOBKCFGR {
        &self.nsobkcfgr
    }
    ///0x44 - FLASH secure OBK configuration register
    #[inline(always)]
    pub const fn secobkcfgr(&self) -> &SECOBKCFGR {
        &self.secobkcfgr
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
    ///0x60 - FLASH non-secure EPOCH register
    #[inline(always)]
    pub const fn nsepochr_cur(&self) -> &NSEPOCHR_CUR {
        &self.nsepochr_cur
    }
    ///0x68 - FLASH secure EPOCH register
    #[inline(always)]
    pub const fn secepochr_cur(&self) -> &SECEPOCHR_CUR {
        &self.secepochr_cur
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
    ///0x80 - FLASH non-secure boot register
    #[inline(always)]
    pub const fn nsbootr_cur(&self) -> &NSBOOTR_CUR {
        &self.nsbootr_cur
    }
    ///0x84 - FLASH non-secure boot register
    #[inline(always)]
    pub const fn nsbootr_prg(&self) -> &NSBOOTR_PRG {
        &self.nsbootr_prg
    }
    ///0x88 - FLASH secure boot register
    #[inline(always)]
    pub const fn secbootr_cur(&self) -> &SECBOOTR_CUR {
        &self.secbootr_cur
    }
    ///0x8c - FLASH secure boot register
    #[inline(always)]
    pub const fn bootr_prg(&self) -> &BOOTR_PRG {
        &self.bootr_prg
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
    ///0xa0 - FLASH secure block based register for Bank1
    #[inline(always)]
    pub const fn secbb1r1(&self) -> &SECBB1R1 {
        &self.secbb1r1
    }
    ///0xa4 - FLASH secure block based register for Bank1
    #[inline(always)]
    pub const fn secbb1r2(&self) -> &SECBB1R2 {
        &self.secbb1r2
    }
    ///0xa8 - FLASH secure block based register for Bank1
    #[inline(always)]
    pub const fn secbb1r3(&self) -> &SECBB1R3 {
        &self.secbb1r3
    }
    ///0xac - FLASH secure block based register for Bank1
    #[inline(always)]
    pub const fn secbb1r4(&self) -> &SECBB1R4 {
        &self.secbb1r4
    }
    ///0xc0 - FLASH privilege block based register for Bank1
    #[inline(always)]
    pub const fn privbb1r1(&self) -> &PRIVBB1R1 {
        &self.privbb1r1
    }
    ///0xc4 - FLASH privilege block based register for Bank1
    #[inline(always)]
    pub const fn privbb1r2(&self) -> &PRIVBB1R2 {
        &self.privbb1r2
    }
    ///0xc8 - FLASH privilege block based register for Bank1
    #[inline(always)]
    pub const fn privbb1r3(&self) -> &PRIVBB1R3 {
        &self.privbb1r3
    }
    ///0xcc - FLASH privilege block based register for Bank1
    #[inline(always)]
    pub const fn privbb1r4(&self) -> &PRIVBB1R4 {
        &self.privbb1r4
    }
    ///0xe0 - FLASH security watermark for Bank1
    #[inline(always)]
    pub const fn secwm1r_cur(&self) -> &SECWM1R_CUR {
        &self.secwm1r_cur
    }
    ///0xe4 - FLASH security watermark for Bank1
    #[inline(always)]
    pub const fn secwm1r_prg(&self) -> &SECWM1R_PRG {
        &self.secwm1r_prg
    }
    ///0xe8 - FLASH write sector group protection for Bank1
    #[inline(always)]
    pub const fn wrp1r_cur(&self) -> &WRP1R_CUR {
        &self.wrp1r_cur
    }
    ///0xec - FLASH write sector group protection for Bank1
    #[inline(always)]
    pub const fn wrp1r_prg(&self) -> &WRP1R_PRG {
        &self.wrp1r_prg
    }
    ///0xf0 - FLASH data sector configuration Bank1
    #[inline(always)]
    pub const fn edata1r_cur(&self) -> &EDATA1R_CUR {
        &self.edata1r_cur
    }
    ///0xf4 - FLASH data sector configuration Bank1
    #[inline(always)]
    pub const fn edata1r_prg(&self) -> &EDATA1R_PRG {
        &self.edata1r_prg
    }
    ///0xf8 - FLASH HDP Bank1 configuration
    #[inline(always)]
    pub const fn hdp1r_cur(&self) -> &HDP1R_CUR {
        &self.hdp1r_cur
    }
    ///0xfc - FLASH HDP Bank1 configuration
    #[inline(always)]
    pub const fn hdp1r_prg(&self) -> &HDP1R_PRG {
        &self.hdp1r_prg
    }
    ///0x100 - FLASH ECC correction register
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
    ///0x1a0 - FLASH secure block-based register for Bank2
    #[inline(always)]
    pub const fn secbb2r1(&self) -> &SECBB2R1 {
        &self.secbb2r1
    }
    ///0x1a4 - FLASH secure block-based register for Bank2
    #[inline(always)]
    pub const fn secbb2r2(&self) -> &SECBB2R2 {
        &self.secbb2r2
    }
    ///0x1a8 - FLASH secure block-based register for Bank2
    #[inline(always)]
    pub const fn secbb2r3(&self) -> &SECBB2R3 {
        &self.secbb2r3
    }
    ///0x1ac - FLASH secure block-based register for Bank2
    #[inline(always)]
    pub const fn secbb2r4(&self) -> &SECBB2R4 {
        &self.secbb2r4
    }
    ///0x1c0 - FLASH privilege block-based register for Bank2
    #[inline(always)]
    pub const fn privbb2r1(&self) -> &PRIVBB2R1 {
        &self.privbb2r1
    }
    ///0x1c4 - FLASH privilege block-based register for Bank2
    #[inline(always)]
    pub const fn privbb2r2(&self) -> &PRIVBB2R2 {
        &self.privbb2r2
    }
    ///0x1c8 - FLASH privilege block-based register for Bank2
    #[inline(always)]
    pub const fn privbb2r3(&self) -> &PRIVBB2R3 {
        &self.privbb2r3
    }
    ///0x1cc - FLASH privilege block-based register for Bank2
    #[inline(always)]
    pub const fn privbb2r4(&self) -> &PRIVBB2R4 {
        &self.privbb2r4
    }
    ///0x1e0 - FLASH security watermark for Bank2
    #[inline(always)]
    pub const fn secwm2r_cur(&self) -> &SECWM2R_CUR {
        &self.secwm2r_cur
    }
    ///0x1e4 - FLASH security watermark for Bank2
    #[inline(always)]
    pub const fn secwm2r_prg(&self) -> &SECWM2R_PRG {
        &self.secwm2r_prg
    }
    ///0x1e8 - FLASH write sector group protection for Bank2
    #[inline(always)]
    pub const fn wrp2r_cur(&self) -> &WRP2R_CUR {
        &self.wrp2r_cur
    }
    ///0x1ec - FLASH write sector group protection for Bank2
    #[inline(always)]
    pub const fn wrp2r_prg(&self) -> &WRP2R_PRG {
        &self.wrp2r_prg
    }
    ///0x1f0 - FLASH data sectors configuration Bank2
    #[inline(always)]
    pub const fn edata2r_cur(&self) -> &EDATA2R_CUR {
        &self.edata2r_cur
    }
    ///0x1f4 - FLASH data sector configuration Bank2
    #[inline(always)]
    pub const fn edata2r_prg(&self) -> &EDATA2R_PRG {
        &self.edata2r_prg
    }
    ///0x1f8 - FLASH HDP Bank2 configuration
    #[inline(always)]
    pub const fn hdp2r_cur(&self) -> &HDP2R_CUR {
        &self.hdp2r_cur
    }
    ///0x1fc - FLASH HDP Bank2 configuration
    #[inline(always)]
    pub const fn hdp2r_prg(&self) -> &HDP2R_PRG {
        &self.hdp2r_prg
    }
}
/**ACR (rw) register accessor: FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///FLASH access control register
pub mod acr;
/**NSKEYR (w) register accessor: FLASH non-secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSKEYR)

For information about available fields see [`mod@nskeyr`] module*/
pub type NSKEYR = crate::Reg<nskeyr::NSKEYRrs>;
///FLASH non-secure key register
pub mod nskeyr;
/**SECKEYR (w) register accessor: FLASH secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECKEYR)

For information about available fields see [`mod@seckeyr`] module*/
pub type SECKEYR = crate::Reg<seckeyr::SECKEYRrs>;
///FLASH secure key register
pub mod seckeyr;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**NSOBKKEYR (w) register accessor: FLASH non-secure OBK key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsobkkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSOBKKEYR)

For information about available fields see [`mod@nsobkkeyr`] module*/
pub type NSOBKKEYR = crate::Reg<nsobkkeyr::NSOBKKEYRrs>;
///FLASH non-secure OBK key register
pub mod nsobkkeyr;
/**SECOBKKEYR (w) register accessor: FLASH secure OBK key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secobkkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECOBKKEYR)

For information about available fields see [`mod@secobkkeyr`] module*/
pub type SECOBKKEYR = crate::Reg<secobkkeyr::SECOBKKEYRrs>;
///FLASH secure OBK key register
pub mod secobkkeyr;
/**OPSR (r) register accessor: FLASH operation status register

You can [`read`](crate::Reg::read) this register and get [`opsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPSR)

For information about available fields see [`mod@opsr`] module*/
pub type OPSR = crate::Reg<opsr::OPSRrs>;
///FLASH operation status register
pub mod opsr;
/**OPTCR (rw) register accessor: FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTCR)

For information about available fields see [`mod@optcr`] module*/
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
///FLASH option control register
pub mod optcr;
/**NSSR (r) register accessor: FLASH non-secure status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSSR)

For information about available fields see [`mod@nssr`] module*/
pub type NSSR = crate::Reg<nssr::NSSRrs>;
///FLASH non-secure status register
pub mod nssr;
/**SECSR (r) register accessor: FLASH secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///FLASH secure status register
pub mod secsr;
/**NSCR (rw) register accessor: FLASH non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSCR)

For information about available fields see [`mod@nscr`] module*/
pub type NSCR = crate::Reg<nscr::NSCRrs>;
///FLASH non-secure control register
pub mod nscr;
/**SECCR (rw) register accessor: FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECCR)

For information about available fields see [`mod@seccr`] module*/
pub type SECCR = crate::Reg<seccr::SECCRrs>;
///FLASH secure control register
pub mod seccr;
/**NSCCR (w) register accessor: FLASH non-secure clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSCCR)

For information about available fields see [`mod@nsccr`] module*/
pub type NSCCR = crate::Reg<nsccr::NSCCRrs>;
///FLASH non-secure clear control register
pub mod nsccr;
/**SECCCR (w) register accessor: FLASH secure clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECCCR)

For information about available fields see [`mod@secccr`] module*/
pub type SECCCR = crate::Reg<secccr::SECCCRrs>;
///FLASH secure clear control register
pub mod secccr;
/**PRIVCFGR (rw) register accessor: FLASH privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///FLASH privilege configuration register
pub mod privcfgr;
/**NSOBKCFGR (rw) register accessor: FLASH non-secure OBK configuration register

You can [`read`](crate::Reg::read) this register and get [`nsobkcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsobkcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSOBKCFGR)

For information about available fields see [`mod@nsobkcfgr`] module*/
pub type NSOBKCFGR = crate::Reg<nsobkcfgr::NSOBKCFGRrs>;
///FLASH non-secure OBK configuration register
pub mod nsobkcfgr;
/**SECOBKCFGR (rw) register accessor: FLASH secure OBK configuration register

You can [`read`](crate::Reg::read) this register and get [`secobkcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secobkcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECOBKCFGR)

For information about available fields see [`mod@secobkcfgr`] module*/
pub type SECOBKCFGR = crate::Reg<secobkcfgr::SECOBKCFGRrs>;
///FLASH secure OBK configuration register
pub mod secobkcfgr;
/**HDPEXTR (rw) register accessor: FLASH HDP extension register

You can [`read`](crate::Reg::read) this register and get [`hdpextr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpextr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:HDPEXTR)

For information about available fields see [`mod@hdpextr`] module*/
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTRrs>;
///FLASH HDP extension register
pub mod hdpextr;
/**OPTSR_CUR (r) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTSR_CUR)

For information about available fields see [`mod@optsr_cur`] module*/
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
///FLASH option status register
pub mod optsr_cur;
/**OPTSR_PRG (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTSR_PRG)

For information about available fields see [`mod@optsr_prg`] module*/
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
///FLASH option status register
pub mod optsr_prg;
/**NSEPOCHR_CUR (r) register accessor: FLASH non-secure EPOCH register

You can [`read`](crate::Reg::read) this register and get [`nsepochr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSEPOCHR_CUR)

For information about available fields see [`mod@nsepochr_cur`] module*/
pub type NSEPOCHR_CUR = crate::Reg<nsepochr_cur::NSEPOCHR_CURrs>;
///FLASH non-secure EPOCH register
pub mod nsepochr_cur;
/**SECEPOCHR_CUR (r) register accessor: FLASH secure EPOCH register

You can [`read`](crate::Reg::read) this register and get [`secepochr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECEPOCHR_CUR)

For information about available fields see [`mod@secepochr_cur`] module*/
pub type SECEPOCHR_CUR = crate::Reg<secepochr_cur::SECEPOCHR_CURrs>;
///FLASH secure EPOCH register
pub mod secepochr_cur;
/**OPTSR2_CUR (r) register accessor: FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTSR2_CUR)

For information about available fields see [`mod@optsr2_cur`] module*/
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CURrs>;
///FLASH option status register 2
pub mod optsr2_cur;
/**OPTSR2_PRG (rw) register accessor: FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTSR2_PRG)

For information about available fields see [`mod@optsr2_prg`] module*/
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRGrs>;
///FLASH option status register 2
pub mod optsr2_prg;
/**NSBOOTR_CUR (r) register accessor: FLASH non-secure boot register

You can [`read`](crate::Reg::read) this register and get [`nsbootr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSBOOTR_CUR)

For information about available fields see [`mod@nsbootr_cur`] module*/
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CURrs>;
///FLASH non-secure boot register
pub mod nsbootr_cur;
/**NSBOOTR_PRG (rw) register accessor: FLASH non-secure boot register

You can [`read`](crate::Reg::read) this register and get [`nsbootr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:NSBOOTR_PRG)

For information about available fields see [`mod@nsbootr_prg`] module*/
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRGrs>;
///FLASH non-secure boot register
pub mod nsbootr_prg;
/**SECBOOTR_CUR (r) register accessor: FLASH secure boot register

You can [`read`](crate::Reg::read) this register and get [`secbootr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBOOTR_CUR)

For information about available fields see [`mod@secbootr_cur`] module*/
pub type SECBOOTR_CUR = crate::Reg<secbootr_cur::SECBOOTR_CURrs>;
///FLASH secure boot register
pub mod secbootr_cur;
/**BOOTR_PRG (rw) register accessor: FLASH secure boot register

You can [`read`](crate::Reg::read) this register and get [`bootr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:BOOTR_PRG)

For information about available fields see [`mod@bootr_prg`] module*/
pub type BOOTR_PRG = crate::Reg<bootr_prg::BOOTR_PRGrs>;
///FLASH secure boot register
pub mod bootr_prg;
/**OTPBLR_CUR (r) register accessor: FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OTPBLR_CUR)

For information about available fields see [`mod@otpblr_cur`] module*/
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CURrs>;
///FLASH non-secure OTP block lock
pub mod otpblr_cur;
/**OTPBLR_PRG (rw) register accessor: FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpblr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OTPBLR_PRG)

For information about available fields see [`mod@otpblr_prg`] module*/
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRGrs>;
///FLASH non-secure OTP block lock
pub mod otpblr_prg;
/**SECBB1R1 (rw) register accessor: FLASH secure block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`secbb1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB1R1)

For information about available fields see [`mod@secbb1r1`] module*/
pub type SECBB1R1 = crate::Reg<secbb1r1::SECBB1R1rs>;
///FLASH secure block based register for Bank1
pub mod secbb1r1;
/**SECBB1R2 (rw) register accessor: FLASH secure block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`secbb1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB1R2)

For information about available fields see [`mod@secbb1r2`] module*/
pub type SECBB1R2 = crate::Reg<secbb1r2::SECBB1R2rs>;
///FLASH secure block based register for Bank1
pub mod secbb1r2;
/**SECBB1R3 (rw) register accessor: FLASH secure block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`secbb1r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB1R3)

For information about available fields see [`mod@secbb1r3`] module*/
pub type SECBB1R3 = crate::Reg<secbb1r3::SECBB1R3rs>;
///FLASH secure block based register for Bank1
pub mod secbb1r3;
/**SECBB1R4 (rw) register accessor: FLASH secure block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`secbb1r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB1R4)

For information about available fields see [`mod@secbb1r4`] module*/
pub type SECBB1R4 = crate::Reg<secbb1r4::SECBB1R4rs>;
///FLASH secure block based register for Bank1
pub mod secbb1r4;
/**PRIVBB1R1 (rw) register accessor: FLASH privilege block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`privbb1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB1R1)

For information about available fields see [`mod@privbb1r1`] module*/
pub type PRIVBB1R1 = crate::Reg<privbb1r1::PRIVBB1R1rs>;
///FLASH privilege block based register for Bank1
pub mod privbb1r1;
/**PRIVBB1R2 (rw) register accessor: FLASH privilege block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`privbb1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB1R2)

For information about available fields see [`mod@privbb1r2`] module*/
pub type PRIVBB1R2 = crate::Reg<privbb1r2::PRIVBB1R2rs>;
///FLASH privilege block based register for Bank1
pub mod privbb1r2;
/**PRIVBB1R3 (rw) register accessor: FLASH privilege block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`privbb1r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB1R3)

For information about available fields see [`mod@privbb1r3`] module*/
pub type PRIVBB1R3 = crate::Reg<privbb1r3::PRIVBB1R3rs>;
///FLASH privilege block based register for Bank1
pub mod privbb1r3;
/**PRIVBB1R4 (rw) register accessor: FLASH privilege block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`privbb1r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB1R4)

For information about available fields see [`mod@privbb1r4`] module*/
pub type PRIVBB1R4 = crate::Reg<privbb1r4::PRIVBB1R4rs>;
///FLASH privilege block based register for Bank1
pub mod privbb1r4;
/**SECWM1R_CUR (r) register accessor: FLASH security watermark for Bank1

You can [`read`](crate::Reg::read) this register and get [`secwm1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECWM1R_CUR)

For information about available fields see [`mod@secwm1r_cur`] module*/
pub type SECWM1R_CUR = crate::Reg<secwm1r_cur::SECWM1R_CURrs>;
///FLASH security watermark for Bank1
pub mod secwm1r_cur;
/**SECWM1R_PRG (rw) register accessor: FLASH security watermark for Bank1

You can [`read`](crate::Reg::read) this register and get [`secwm1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECWM1R_PRG)

For information about available fields see [`mod@secwm1r_prg`] module*/
pub type SECWM1R_PRG = crate::Reg<secwm1r_prg::SECWM1R_PRGrs>;
///FLASH security watermark for Bank1
pub mod secwm1r_prg;
/**WRP1R_CUR (r) register accessor: FLASH write sector group protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrp1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:WRP1R_CUR)

For information about available fields see [`mod@wrp1r_cur`] module*/
pub type WRP1R_CUR = crate::Reg<wrp1r_cur::WRP1R_CURrs>;
///FLASH write sector group protection for Bank1
pub mod wrp1r_cur;
/**WRP1R_PRG (rw) register accessor: FLASH write sector group protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrp1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:WRP1R_PRG)

For information about available fields see [`mod@wrp1r_prg`] module*/
pub type WRP1R_PRG = crate::Reg<wrp1r_prg::WRP1R_PRGrs>;
///FLASH write sector group protection for Bank1
pub mod wrp1r_prg;
/**EDATA1R_CUR (r) register accessor: FLASH data sector configuration Bank1

You can [`read`](crate::Reg::read) this register and get [`edata1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:EDATA1R_CUR)

For information about available fields see [`mod@edata1r_cur`] module*/
pub type EDATA1R_CUR = crate::Reg<edata1r_cur::EDATA1R_CURrs>;
///FLASH data sector configuration Bank1
pub mod edata1r_cur;
/**EDATA1R_PRG (rw) register accessor: FLASH data sector configuration Bank1

You can [`read`](crate::Reg::read) this register and get [`edata1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edata1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:EDATA1R_PRG)

For information about available fields see [`mod@edata1r_prg`] module*/
pub type EDATA1R_PRG = crate::Reg<edata1r_prg::EDATA1R_PRGrs>;
///FLASH data sector configuration Bank1
pub mod edata1r_prg;
/**HDP1R_CUR (r) register accessor: FLASH HDP Bank1 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:HDP1R_CUR)

For information about available fields see [`mod@hdp1r_cur`] module*/
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CURrs>;
///FLASH HDP Bank1 configuration
pub mod hdp1r_cur;
/**HDP1R_PRG (rw) register accessor: FLASH HDP Bank1 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:HDP1R_PRG)

For information about available fields see [`mod@hdp1r_prg`] module*/
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRGrs>;
///FLASH HDP Bank1 configuration
pub mod hdp1r_prg;
/**ECCCORR (rw) register accessor: FLASH ECC correction register

You can [`read`](crate::Reg::read) this register and get [`ecccorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecccorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:ECCCORR)

For information about available fields see [`mod@ecccorr`] module*/
pub type ECCCORR = crate::Reg<ecccorr::ECCCORRrs>;
///FLASH ECC correction register
pub mod ecccorr;
/**ECCDETR (rw) register accessor: FLASH ECC detection register

You can [`read`](crate::Reg::read) this register and get [`eccdetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:ECCDETR)

For information about available fields see [`mod@eccdetr`] module*/
pub type ECCDETR = crate::Reg<eccdetr::ECCDETRrs>;
///FLASH ECC detection register
pub mod eccdetr;
/**ECCDR (r) register accessor: FLASH ECC data

You can [`read`](crate::Reg::read) this register and get [`eccdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:ECCDR)

For information about available fields see [`mod@eccdr`] module*/
pub type ECCDR = crate::Reg<eccdr::ECCDRrs>;
///FLASH ECC data
pub mod eccdr;
/**SECBB2R1 (rw) register accessor: FLASH secure block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`secbb2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB2R1)

For information about available fields see [`mod@secbb2r1`] module*/
pub type SECBB2R1 = crate::Reg<secbb2r1::SECBB2R1rs>;
///FLASH secure block-based register for Bank2
pub mod secbb2r1;
/**SECBB2R2 (rw) register accessor: FLASH secure block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`secbb2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB2R2)

For information about available fields see [`mod@secbb2r2`] module*/
pub type SECBB2R2 = crate::Reg<secbb2r2::SECBB2R2rs>;
///FLASH secure block-based register for Bank2
pub mod secbb2r2;
/**SECBB2R3 (rw) register accessor: FLASH secure block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`secbb2r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB2R3)

For information about available fields see [`mod@secbb2r3`] module*/
pub type SECBB2R3 = crate::Reg<secbb2r3::SECBB2R3rs>;
///FLASH secure block-based register for Bank2
pub mod secbb2r3;
/**SECBB2R4 (rw) register accessor: FLASH secure block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`secbb2r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb2r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB2R4)

For information about available fields see [`mod@secbb2r4`] module*/
pub type SECBB2R4 = crate::Reg<secbb2r4::SECBB2R4rs>;
///FLASH secure block-based register for Bank2
pub mod secbb2r4;
/**PRIVBB2R1 (rw) register accessor: FLASH privilege block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`privbb2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB2R1)

For information about available fields see [`mod@privbb2r1`] module*/
pub type PRIVBB2R1 = crate::Reg<privbb2r1::PRIVBB2R1rs>;
///FLASH privilege block-based register for Bank2
pub mod privbb2r1;
/**PRIVBB2R2 (rw) register accessor: FLASH privilege block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`privbb2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB2R2)

For information about available fields see [`mod@privbb2r2`] module*/
pub type PRIVBB2R2 = crate::Reg<privbb2r2::PRIVBB2R2rs>;
///FLASH privilege block-based register for Bank2
pub mod privbb2r2;
/**PRIVBB2R3 (rw) register accessor: FLASH privilege block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`privbb2r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb2r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB2R3)

For information about available fields see [`mod@privbb2r3`] module*/
pub type PRIVBB2R3 = crate::Reg<privbb2r3::PRIVBB2R3rs>;
///FLASH privilege block-based register for Bank2
pub mod privbb2r3;
/**PRIVBB2R4 (rw) register accessor: FLASH privilege block-based register for Bank2

You can [`read`](crate::Reg::read) this register and get [`privbb2r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb2r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB2R4)

For information about available fields see [`mod@privbb2r4`] module*/
pub type PRIVBB2R4 = crate::Reg<privbb2r4::PRIVBB2R4rs>;
///FLASH privilege block-based register for Bank2
pub mod privbb2r4;
/**SECWM2R_CUR (r) register accessor: FLASH security watermark for Bank2

You can [`read`](crate::Reg::read) this register and get [`secwm2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECWM2R_CUR)

For information about available fields see [`mod@secwm2r_cur`] module*/
pub type SECWM2R_CUR = crate::Reg<secwm2r_cur::SECWM2R_CURrs>;
///FLASH security watermark for Bank2
pub mod secwm2r_cur;
/**SECWM2R_PRG (rw) register accessor: FLASH security watermark for Bank2

You can [`read`](crate::Reg::read) this register and get [`secwm2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECWM2R_PRG)

For information about available fields see [`mod@secwm2r_prg`] module*/
pub type SECWM2R_PRG = crate::Reg<secwm2r_prg::SECWM2R_PRGrs>;
///FLASH security watermark for Bank2
pub mod secwm2r_prg;
/**WRP2R_CUR (r) register accessor: FLASH write sector group protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrp2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:WRP2R_CUR)

For information about available fields see [`mod@wrp2r_cur`] module*/
pub type WRP2R_CUR = crate::Reg<wrp2r_cur::WRP2R_CURrs>;
///FLASH write sector group protection for Bank2
pub mod wrp2r_cur;
/**WRP2R_PRG (rw) register accessor: FLASH write sector group protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrp2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:WRP2R_PRG)

For information about available fields see [`mod@wrp2r_prg`] module*/
pub type WRP2R_PRG = crate::Reg<wrp2r_prg::WRP2R_PRGrs>;
///FLASH write sector group protection for Bank2
pub mod wrp2r_prg;
/**EDATA2R_CUR (r) register accessor: FLASH data sectors configuration Bank2

You can [`read`](crate::Reg::read) this register and get [`edata2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:EDATA2R_CUR)

For information about available fields see [`mod@edata2r_cur`] module*/
pub type EDATA2R_CUR = crate::Reg<edata2r_cur::EDATA2R_CURrs>;
///FLASH data sectors configuration Bank2
pub mod edata2r_cur;
/**EDATA2R_PRG (rw) register accessor: FLASH data sector configuration Bank2

You can [`read`](crate::Reg::read) this register and get [`edata2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edata2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:EDATA2R_PRG)

For information about available fields see [`mod@edata2r_prg`] module*/
pub type EDATA2R_PRG = crate::Reg<edata2r_prg::EDATA2R_PRGrs>;
///FLASH data sector configuration Bank2
pub mod edata2r_prg;
/**HDP2R_CUR (r) register accessor: FLASH HDP Bank2 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:HDP2R_CUR)

For information about available fields see [`mod@hdp2r_cur`] module*/
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CURrs>;
///FLASH HDP Bank2 configuration
pub mod hdp2r_cur;
/**HDP2R_PRG (rw) register accessor: FLASH HDP Bank2 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:HDP2R_PRG)

For information about available fields see [`mod@hdp2r_prg`] module*/
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRGrs>;
///FLASH HDP Bank2 configuration
pub mod hdp2r_prg;
