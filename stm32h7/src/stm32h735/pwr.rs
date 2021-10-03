#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - PWR control status register 1"]
    pub csr1: crate::Reg<csr1::CSR1_SPEC>,
    #[doc = "0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x10 - This register allows controlling CPU1 power."]
    pub cpucr: crate::Reg<cpucr::CPUCR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    pub d3cr: crate::Reg<d3cr::D3CR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    pub wkupcr: crate::Reg<wkupcr::WKUPCR_SPEC>,
    #[doc = "0x24 - reset only by system reset, not reset by wakeup from Standby mode"]
    pub wkupfr: crate::Reg<wkupfr::WKUPFR_SPEC>,
    #[doc = "0x28 - Reset only by system reset, not reset by wakeup from Standby mode"]
    pub wkupepr: crate::Reg<wkupepr::WKUPEPR_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "PWR control register 1"]
pub mod cr1;
#[doc = "CSR1 register accessor: an alias for `Reg<CSR1_SPEC>`"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "PWR control status register 1"]
pub mod csr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
pub mod cr3;
#[doc = "CPUCR register accessor: an alias for `Reg<CPUCR_SPEC>`"]
pub type CPUCR = crate::Reg<cpucr::CPUCR_SPEC>;
#[doc = "This register allows controlling CPU1 power."]
pub mod cpucr;
#[doc = "D3CR register accessor: an alias for `Reg<D3CR_SPEC>`"]
pub type D3CR = crate::Reg<d3cr::D3CR_SPEC>;
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
pub mod d3cr;
#[doc = "WKUPCR register accessor: an alias for `Reg<WKUPCR_SPEC>`"]
pub type WKUPCR = crate::Reg<wkupcr::WKUPCR_SPEC>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
pub mod wkupcr;
#[doc = "WKUPFR register accessor: an alias for `Reg<WKUPFR_SPEC>`"]
pub type WKUPFR = crate::Reg<wkupfr::WKUPFR_SPEC>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupfr;
#[doc = "WKUPEPR register accessor: an alias for `Reg<WKUPEPR_SPEC>`"]
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPR_SPEC>;
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupepr;
