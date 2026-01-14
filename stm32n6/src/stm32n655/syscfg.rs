#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bootcr: BOOTCR,
    cm55cr: CM55CR,
    cm55tcmcr: CM55TCMCR,
    cm55rwmcr: CM55RWMCR,
    initsvtorcr: INITSVTORCR,
    initnsvtorcr: INITNSVTORCR,
    cm55rstcr: CM55RSTCR,
    cm55pahbwpr: CM55PAHBWPR,
    vencramcr: VENCRAMCR,
    pottamprstcr: POTTAMPRSTCR,
    _reserved10: [u8; 0x0c],
    icnewrcr: ICNEWRCR,
    icncgcr: ICNCGCR,
    icnbwrcr: ICNBWRCR,
    iocr: IOCR,
    vddio1cccr: VDDIO1CCCR,
    vddio1ccsr: VDDIO1CCSR,
    vddio2cccr: VDDIO2CCCR,
    vddio2ccsr: VDDIO2CCSR,
    vddio3cccr: VDDIO3CCCR,
    vddio3ccsr: VDDIO3CCSR,
    vddio4cccr: VDDIO4CCCR,
    vddio4ccsr: VDDIO4CCSR,
    vddiocccr: VDDIOCCCR,
    vddioccsr: VDDIOCCSR,
    cbr: CBR,
    sec_aidcr: SEC_AIDCR,
    fmc_retimecr: FMC_RETIMECR,
    npu_icncr: NPU_ICNCR,
    _reserved28: [u8; 0x84],
    bootsr: BOOTSR,
    ahbwp_error_sr: AHBWP_ERROR_SR,
    _reserved30: [u8; 0x02f8],
    smpshdpcr: SMPSHDPCR,
    _reserved31: [u8; 0x03fc],
    nonsec_aidcr: NONSEC_AIDCR,
}
impl RegisterBlock {
    ///0x00 - SYSCFG boot pin control register
    #[inline(always)]
    pub const fn bootcr(&self) -> &BOOTCR {
        &self.bootcr
    }
    ///0x04 - SYSCFG Cortex-M55 control register
    #[inline(always)]
    pub const fn cm55cr(&self) -> &CM55CR {
        &self.cm55cr
    }
    ///0x08 - SYSCFG Cortex-M55 TCM control register
    #[inline(always)]
    pub const fn cm55tcmcr(&self) -> &CM55TCMCR {
        &self.cm55tcmcr
    }
    ///0x0c - SYSCFG Cortex-CM55 memory RW margin register
    #[inline(always)]
    pub const fn cm55rwmcr(&self) -> &CM55RWMCR {
        &self.cm55rwmcr
    }
    ///0x10 - SYSCFG Cortex-M55 SVTOR control register
    #[inline(always)]
    pub const fn initsvtorcr(&self) -> &INITSVTORCR {
        &self.initsvtorcr
    }
    ///0x14 - SYSCFG Cortex-M55 NSVTOR control register
    #[inline(always)]
    pub const fn initnsvtorcr(&self) -> &INITNSVTORCR {
        &self.initnsvtorcr
    }
    ///0x18 - SYSCFG Cortex-M55 reset type control register
    #[inline(always)]
    pub const fn cm55rstcr(&self) -> &CM55RSTCR {
        &self.cm55rstcr
    }
    ///0x1c - SYSCFG Cortex-M55 P-AHB write posting control register
    #[inline(always)]
    pub const fn cm55pahbwpr(&self) -> &CM55PAHBWPR {
        &self.cm55pahbwpr
    }
    ///0x20 - SYSCFG VENCRAM control register
    #[inline(always)]
    pub const fn vencramcr(&self) -> &VENCRAMCR {
        &self.vencramcr
    }
    ///0x24 - SYSCFG potential tamper reset register
    #[inline(always)]
    pub const fn pottamprstcr(&self) -> &POTTAMPRSTCR {
        &self.pottamprstcr
    }
    ///0x34 - SYSCFG AHB-AXI bridge early write response control register
    #[inline(always)]
    pub const fn icnewrcr(&self) -> &ICNEWRCR {
        &self.icnewrcr
    }
    ///0x38 - SYSCFG ICN clock gating control register
    #[inline(always)]
    pub const fn icncgcr(&self) -> &ICNCGCR {
        &self.icncgcr
    }
    ///0x3c - SYSCFG ICN bandwidth regulator control register
    #[inline(always)]
    pub const fn icnbwrcr(&self) -> &ICNBWRCR {
        &self.icnbwrcr
    }
    ///0x40 - SYSCFG /O control register
    #[inline(always)]
    pub const fn iocr(&self) -> &IOCR {
        &self.iocr
    }
    ///0x44 - SYSCFG VDDIO1 compensation cell control register
    #[inline(always)]
    pub const fn vddio1cccr(&self) -> &VDDIO1CCCR {
        &self.vddio1cccr
    }
    ///0x48 - SYSCFG VDDIO1 compensation cell status register
    #[inline(always)]
    pub const fn vddio1ccsr(&self) -> &VDDIO1CCSR {
        &self.vddio1ccsr
    }
    ///0x4c - SYSCFG VDDIO2 compensation cell control register
    #[inline(always)]
    pub const fn vddio2cccr(&self) -> &VDDIO2CCCR {
        &self.vddio2cccr
    }
    ///0x50 - SYSCFG VDDIO2 compensation cell status register
    #[inline(always)]
    pub const fn vddio2ccsr(&self) -> &VDDIO2CCSR {
        &self.vddio2ccsr
    }
    ///0x54 - SYSCFG VDDIO3 compensation cell control register
    #[inline(always)]
    pub const fn vddio3cccr(&self) -> &VDDIO3CCCR {
        &self.vddio3cccr
    }
    ///0x58 - SYSCFG VDDIO3 compensation cell status register
    #[inline(always)]
    pub const fn vddio3ccsr(&self) -> &VDDIO3CCSR {
        &self.vddio3ccsr
    }
    ///0x5c - SYSCFG VDDIO4 compensation cell control register
    #[inline(always)]
    pub const fn vddio4cccr(&self) -> &VDDIO4CCCR {
        &self.vddio4cccr
    }
    ///0x60 - SYSCFG VDDIO4 compensation cell status register
    #[inline(always)]
    pub const fn vddio4ccsr(&self) -> &VDDIO4CCSR {
        &self.vddio4ccsr
    }
    ///0x64 - SYSCFG VDDIO compensation cell control register
    #[inline(always)]
    pub const fn vddiocccr(&self) -> &VDDIOCCCR {
        &self.vddiocccr
    }
    ///0x68 - SYSCFG VDDIO compensation cell status register
    #[inline(always)]
    pub const fn vddioccsr(&self) -> &VDDIOCCSR {
        &self.vddioccsr
    }
    ///0x6c - SYSCFG control timer break register
    #[inline(always)]
    pub const fn cbr(&self) -> &CBR {
        &self.cbr
    }
    ///0x70 - SYSCFG DMA CID secure control register
    #[inline(always)]
    pub const fn sec_aidcr(&self) -> &SEC_AIDCR {
        &self.sec_aidcr
    }
    ///0x74 - SYSCFG FMC retiming logic control register
    #[inline(always)]
    pub const fn fmc_retimecr(&self) -> &FMC_RETIMECR {
        &self.fmc_retimecr
    }
    ///0x78 - SYSCFG NPU RAM interleaving control register
    #[inline(always)]
    pub const fn npu_icncr(&self) -> &NPU_ICNCR {
        &self.npu_icncr
    }
    ///0x100 - SYSCFG boot pin status register
    #[inline(always)]
    pub const fn bootsr(&self) -> &BOOTSR {
        &self.bootsr
    }
    ///0x104 - SYSCFG AHB write posting address error register
    #[inline(always)]
    pub const fn ahbwp_error_sr(&self) -> &AHBWP_ERROR_SR {
        &self.ahbwp_error_sr
    }
    ///0x400 - SYSCFG SMPS observable signals through HDP selection configuration register
    #[inline(always)]
    pub const fn smpshdpcr(&self) -> &SMPSHDPCR {
        &self.smpshdpcr
    }
    ///0x800 - SYSCFG DMA CID non-secure control register
    #[inline(always)]
    pub const fn nonsec_aidcr(&self) -> &NONSEC_AIDCR {
        &self.nonsec_aidcr
    }
}
/**BOOTCR (rw) register accessor: SYSCFG boot pin control register

You can [`read`](crate::Reg::read) this register and get [`bootcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:BOOTCR)

For information about available fields see [`mod@bootcr`] module*/
pub type BOOTCR = crate::Reg<bootcr::BOOTCRrs>;
///SYSCFG boot pin control register
pub mod bootcr;
/**CM55CR (rw) register accessor: SYSCFG Cortex-M55 control register

You can [`read`](crate::Reg::read) this register and get [`cm55cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55CR)

For information about available fields see [`mod@cm55cr`] module*/
pub type CM55CR = crate::Reg<cm55cr::CM55CRrs>;
///SYSCFG Cortex-M55 control register
pub mod cm55cr;
/**CM55TCMCR (rw) register accessor: SYSCFG Cortex-M55 TCM control register

You can [`read`](crate::Reg::read) this register and get [`cm55tcmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55tcmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55TCMCR)

For information about available fields see [`mod@cm55tcmcr`] module*/
pub type CM55TCMCR = crate::Reg<cm55tcmcr::CM55TCMCRrs>;
///SYSCFG Cortex-M55 TCM control register
pub mod cm55tcmcr;
/**CM55RWMCR (rw) register accessor: SYSCFG Cortex-CM55 memory RW margin register

You can [`read`](crate::Reg::read) this register and get [`cm55rwmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55rwmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55RWMCR)

For information about available fields see [`mod@cm55rwmcr`] module*/
pub type CM55RWMCR = crate::Reg<cm55rwmcr::CM55RWMCRrs>;
///SYSCFG Cortex-CM55 memory RW margin register
pub mod cm55rwmcr;
/**INITSVTORCR (rw) register accessor: SYSCFG Cortex-M55 SVTOR control register

You can [`read`](crate::Reg::read) this register and get [`initsvtorcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvtorcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:INITSVTORCR)

For information about available fields see [`mod@initsvtorcr`] module*/
pub type INITSVTORCR = crate::Reg<initsvtorcr::INITSVTORCRrs>;
///SYSCFG Cortex-M55 SVTOR control register
pub mod initsvtorcr;
/**INITNSVTORCR (rw) register accessor: SYSCFG Cortex-M55 NSVTOR control register

You can [`read`](crate::Reg::read) this register and get [`initnsvtorcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initnsvtorcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:INITNSVTORCR)

For information about available fields see [`mod@initnsvtorcr`] module*/
pub type INITNSVTORCR = crate::Reg<initnsvtorcr::INITNSVTORCRrs>;
///SYSCFG Cortex-M55 NSVTOR control register
pub mod initnsvtorcr;
/**CM55RSTCR (rw) register accessor: SYSCFG Cortex-M55 reset type control register

You can [`read`](crate::Reg::read) this register and get [`cm55rstcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55rstcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55RSTCR)

For information about available fields see [`mod@cm55rstcr`] module*/
pub type CM55RSTCR = crate::Reg<cm55rstcr::CM55RSTCRrs>;
///SYSCFG Cortex-M55 reset type control register
pub mod cm55rstcr;
/**CM55PAHBWPR (rw) register accessor: SYSCFG Cortex-M55 P-AHB write posting control register

You can [`read`](crate::Reg::read) this register and get [`cm55pahbwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55pahbwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55PAHBWPR)

For information about available fields see [`mod@cm55pahbwpr`] module*/
pub type CM55PAHBWPR = crate::Reg<cm55pahbwpr::CM55PAHBWPRrs>;
///SYSCFG Cortex-M55 P-AHB write posting control register
pub mod cm55pahbwpr;
/**VENCRAMCR (rw) register accessor: SYSCFG VENCRAM control register

You can [`read`](crate::Reg::read) this register and get [`vencramcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vencramcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VENCRAMCR)

For information about available fields see [`mod@vencramcr`] module*/
pub type VENCRAMCR = crate::Reg<vencramcr::VENCRAMCRrs>;
///SYSCFG VENCRAM control register
pub mod vencramcr;
/**POTTAMPRSTCR (rw) register accessor: SYSCFG potential tamper reset register

You can [`read`](crate::Reg::read) this register and get [`pottamprstcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pottamprstcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:POTTAMPRSTCR)

For information about available fields see [`mod@pottamprstcr`] module*/
pub type POTTAMPRSTCR = crate::Reg<pottamprstcr::POTTAMPRSTCRrs>;
///SYSCFG potential tamper reset register
pub mod pottamprstcr;
/**ICNEWRCR (rw) register accessor: SYSCFG AHB-AXI bridge early write response control register

You can [`read`](crate::Reg::read) this register and get [`icnewrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnewrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:ICNEWRCR)

For information about available fields see [`mod@icnewrcr`] module*/
pub type ICNEWRCR = crate::Reg<icnewrcr::ICNEWRCRrs>;
///SYSCFG AHB-AXI bridge early write response control register
pub mod icnewrcr;
/**ICNCGCR (rw) register accessor: SYSCFG ICN clock gating control register

You can [`read`](crate::Reg::read) this register and get [`icncgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icncgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:ICNCGCR)

For information about available fields see [`mod@icncgcr`] module*/
pub type ICNCGCR = crate::Reg<icncgcr::ICNCGCRrs>;
///SYSCFG ICN clock gating control register
pub mod icncgcr;
/**ICNBWRCR (rw) register accessor: SYSCFG ICN bandwidth regulator control register

You can [`read`](crate::Reg::read) this register and get [`icnbwrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnbwrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:ICNBWRCR)

For information about available fields see [`mod@icnbwrcr`] module*/
pub type ICNBWRCR = crate::Reg<icnbwrcr::ICNBWRCRrs>;
///SYSCFG ICN bandwidth regulator control register
pub mod icnbwrcr;
/**IOCR (rw) register accessor: SYSCFG /O control register

You can [`read`](crate::Reg::read) this register and get [`iocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:IOCR)

For information about available fields see [`mod@iocr`] module*/
pub type IOCR = crate::Reg<iocr::IOCRrs>;
///SYSCFG /O control register
pub mod iocr;
/**VDDIO1CCCR (rw) register accessor: SYSCFG VDDIO1 compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddio1cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddio1cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO1CCCR)

For information about available fields see [`mod@vddio1cccr`] module*/
pub type VDDIO1CCCR = crate::Reg<vddio1cccr::VDDIO1CCCRrs>;
///SYSCFG VDDIO1 compensation cell control register
pub mod vddio1cccr;
/**VDDIO1CCSR (r) register accessor: SYSCFG VDDIO1 compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddio1ccsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO1CCSR)

For information about available fields see [`mod@vddio1ccsr`] module*/
pub type VDDIO1CCSR = crate::Reg<vddio1ccsr::VDDIO1CCSRrs>;
///SYSCFG VDDIO1 compensation cell status register
pub mod vddio1ccsr;
/**VDDIO2CCCR (rw) register accessor: SYSCFG VDDIO2 compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddio2cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddio2cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO2CCCR)

For information about available fields see [`mod@vddio2cccr`] module*/
pub type VDDIO2CCCR = crate::Reg<vddio2cccr::VDDIO2CCCRrs>;
///SYSCFG VDDIO2 compensation cell control register
pub mod vddio2cccr;
/**VDDIO2CCSR (r) register accessor: SYSCFG VDDIO2 compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddio2ccsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO2CCSR)

For information about available fields see [`mod@vddio2ccsr`] module*/
pub type VDDIO2CCSR = crate::Reg<vddio2ccsr::VDDIO2CCSRrs>;
///SYSCFG VDDIO2 compensation cell status register
pub mod vddio2ccsr;
/**VDDIO3CCCR (rw) register accessor: SYSCFG VDDIO3 compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddio3cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddio3cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO3CCCR)

For information about available fields see [`mod@vddio3cccr`] module*/
pub type VDDIO3CCCR = crate::Reg<vddio3cccr::VDDIO3CCCRrs>;
///SYSCFG VDDIO3 compensation cell control register
pub mod vddio3cccr;
/**VDDIO3CCSR (r) register accessor: SYSCFG VDDIO3 compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddio3ccsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO3CCSR)

For information about available fields see [`mod@vddio3ccsr`] module*/
pub type VDDIO3CCSR = crate::Reg<vddio3ccsr::VDDIO3CCSRrs>;
///SYSCFG VDDIO3 compensation cell status register
pub mod vddio3ccsr;
/**VDDIO4CCCR (rw) register accessor: SYSCFG VDDIO4 compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddio4cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddio4cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO4CCCR)

For information about available fields see [`mod@vddio4cccr`] module*/
pub type VDDIO4CCCR = crate::Reg<vddio4cccr::VDDIO4CCCRrs>;
///SYSCFG VDDIO4 compensation cell control register
pub mod vddio4cccr;
/**VDDIO4CCSR (r) register accessor: SYSCFG VDDIO4 compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddio4ccsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIO4CCSR)

For information about available fields see [`mod@vddio4ccsr`] module*/
pub type VDDIO4CCSR = crate::Reg<vddio4ccsr::VDDIO4CCSRrs>;
///SYSCFG VDDIO4 compensation cell status register
pub mod vddio4ccsr;
/**VDDIOCCCR (rw) register accessor: SYSCFG VDDIO compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`vddiocccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddiocccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIOCCCR)

For information about available fields see [`mod@vddiocccr`] module*/
pub type VDDIOCCCR = crate::Reg<vddiocccr::VDDIOCCCRrs>;
///SYSCFG VDDIO compensation cell control register
pub mod vddiocccr;
/**VDDIOCCSR (r) register accessor: SYSCFG VDDIO compensation cell status register

You can [`read`](crate::Reg::read) this register and get [`vddioccsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:VDDIOCCSR)

For information about available fields see [`mod@vddioccsr`] module*/
pub type VDDIOCCSR = crate::Reg<vddioccsr::VDDIOCCSRrs>;
///SYSCFG VDDIO compensation cell status register
pub mod vddioccsr;
/**CBR (rw) register accessor: SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`cbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CBR)

For information about available fields see [`mod@cbr`] module*/
pub type CBR = crate::Reg<cbr::CBRrs>;
///SYSCFG control timer break register
pub mod cbr;
/**SEC_AIDCR (rw) register accessor: SYSCFG DMA CID secure control register

You can [`read`](crate::Reg::read) this register and get [`sec_aidcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_aidcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:SEC_AIDCR)

For information about available fields see [`mod@sec_aidcr`] module*/
pub type SEC_AIDCR = crate::Reg<sec_aidcr::SEC_AIDCRrs>;
///SYSCFG DMA CID secure control register
pub mod sec_aidcr;
/**FMC_RETIMECR (rw) register accessor: SYSCFG FMC retiming logic control register

You can [`read`](crate::Reg::read) this register and get [`fmc_retimecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_retimecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:FMC_RETIMECR)

For information about available fields see [`mod@fmc_retimecr`] module*/
pub type FMC_RETIMECR = crate::Reg<fmc_retimecr::FMC_RETIMECRrs>;
///SYSCFG FMC retiming logic control register
pub mod fmc_retimecr;
/**NPU_ICNCR (rw) register accessor: SYSCFG NPU RAM interleaving control register

You can [`read`](crate::Reg::read) this register and get [`npu_icncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`npu_icncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:NPU_ICNCR)

For information about available fields see [`mod@npu_icncr`] module*/
pub type NPU_ICNCR = crate::Reg<npu_icncr::NPU_ICNCRrs>;
///SYSCFG NPU RAM interleaving control register
pub mod npu_icncr;
/**BOOTSR (r) register accessor: SYSCFG boot pin status register

You can [`read`](crate::Reg::read) this register and get [`bootsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:BOOTSR)

For information about available fields see [`mod@bootsr`] module*/
pub type BOOTSR = crate::Reg<bootsr::BOOTSRrs>;
///SYSCFG boot pin status register
pub mod bootsr;
/**AHBWP_ERROR_SR (r) register accessor: SYSCFG AHB write posting address error register

You can [`read`](crate::Reg::read) this register and get [`ahbwp_error_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:AHBWP_ERROR_SR)

For information about available fields see [`mod@ahbwp_error_sr`] module*/
pub type AHBWP_ERROR_SR = crate::Reg<ahbwp_error_sr::AHBWP_ERROR_SRrs>;
///SYSCFG AHB write posting address error register
pub mod ahbwp_error_sr;
/**SMPSHDPCR (rw) register accessor: SYSCFG SMPS observable signals through HDP selection configuration register

You can [`read`](crate::Reg::read) this register and get [`smpshdpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpshdpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:SMPSHDPCR)

For information about available fields see [`mod@smpshdpcr`] module*/
pub type SMPSHDPCR = crate::Reg<smpshdpcr::SMPSHDPCRrs>;
///SYSCFG SMPS observable signals through HDP selection configuration register
pub mod smpshdpcr;
/**NONSEC_AIDCR (rw) register accessor: SYSCFG DMA CID non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nonsec_aidcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsec_aidcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:NONSEC_AIDCR)

For information about available fields see [`mod@nonsec_aidcr`] module*/
pub type NONSEC_AIDCR = crate::Reg<nonsec_aidcr::NONSEC_AIDCRrs>;
///SYSCFG DMA CID non-secure control register
pub mod nonsec_aidcr;
