#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    csr1: CSR1,
    cr2: CR2,
    cr3: CR3,
    cpucr: CPUCR,
    _reserved5: [u8; 0x04],
    d3cr: D3CR,
    _reserved6: [u8; 0x04],
    wkupcr: WKUPCR,
    wkupfr: WKUPFR,
    wkupepr: WKUPEPR,
}
impl RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - PWR control status register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    #[doc = "0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    #[doc = "0x10 - This register allows controlling CPU1 power."]
    #[inline(always)]
    pub const fn cpucr(&self) -> &CPUCR {
        &self.cpucr
    }
    #[doc = "0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    #[inline(always)]
    pub const fn d3cr(&self) -> &D3CR {
        &self.d3cr
    }
    #[doc = "0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    #[inline(always)]
    pub const fn wkupcr(&self) -> &WKUPCR {
        &self.wkupcr
    }
    #[doc = "0x24 - reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupfr(&self) -> &WKUPFR {
        &self.wkupfr
    }
    #[doc = "0x28 - Reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupepr(&self) -> &WKUPEPR {
        &self.wkupepr
    }
}
#[doc = "CR1 (rw) register accessor: PWR control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "PWR control register 1"]
pub mod cr1;
#[doc = "CSR1 (r) register accessor: PWR control status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
#[doc = "PWR control status register 1"]
pub mod csr1;
#[doc = "CR2 (rw) register accessor: This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
pub mod cr3;
#[doc = "CPUCR (rw) register accessor: This register allows controlling CPU1 power.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucr`]
module"]
pub type CPUCR = crate::Reg<cpucr::CPUCRrs>;
#[doc = "This register allows controlling CPU1 power."]
pub mod cpucr;
#[doc = "D3CR (rw) register accessor: This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3cr`]
module"]
pub type D3CR = crate::Reg<d3cr::D3CRrs>;
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
pub mod d3cr;
#[doc = "WKUPCR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupcr`]
module"]
pub type WKUPCR = crate::Reg<wkupcr::WKUPCRrs>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
pub mod wkupcr;
#[doc = "WKUPFR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupfr`]
module"]
pub type WKUPFR = crate::Reg<wkupfr::WKUPFRrs>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupfr;
#[doc = "WKUPEPR (rw) register accessor: Reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupepr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupepr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupepr`]
module"]
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPRrs>;
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupepr;
