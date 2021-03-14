#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - PWR control status register 1"]
    pub csr1: CSR1,
    #[doc = "0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    pub cr2: CR2,
    #[doc = "0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    pub cr3: CR3,
    #[doc = "0x10 - This register allows controlling CPU1 power."]
    pub cpucr: CPUCR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    pub d3cr: D3CR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    pub wkupcr: WKUPCR,
    #[doc = "0x24 - reset only by system reset, not reset by wakeup from Standby mode"]
    pub wkupfr: WKUPFR,
    #[doc = "0x28 - Reset only by system reset, not reset by wakeup from Standby mode"]
    pub wkupepr: WKUPEPR,
}
#[doc = "PWR control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "PWR control register 1"]
pub mod cr1;
#[doc = "PWR control status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr1](csr1) module"]
pub type CSR1 = crate::Reg<u32, _CSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR1;
#[doc = "`read()` method returns [csr1::R](csr1::R) reader structure"]
impl crate::Readable for CSR1 {}
#[doc = "PWR control status register 1"]
pub mod csr1;
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
pub mod cr2;
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](cr3) module"]
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
#[doc = "`read()` method returns [cr3::R](cr3::R) reader structure"]
impl crate::Readable for CR3 {}
#[doc = "`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure"]
impl crate::Writable for CR3 {}
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
pub mod cr3;
#[doc = "This register allows controlling CPU1 power.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpucr](cpucr) module"]
pub type CPUCR = crate::Reg<u32, _CPUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCR;
#[doc = "`read()` method returns [cpucr::R](cpucr::R) reader structure"]
impl crate::Readable for CPUCR {}
#[doc = "`write(|w| ..)` method takes [cpucr::W](cpucr::W) writer structure"]
impl crate::Writable for CPUCR {}
#[doc = "This register allows controlling CPU1 power."]
pub mod cpucr;
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3cr](d3cr) module"]
pub type D3CR = crate::Reg<u32, _D3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3CR;
#[doc = "`read()` method returns [d3cr::R](d3cr::R) reader structure"]
impl crate::Readable for D3CR {}
#[doc = "`write(|w| ..)` method takes [d3cr::W](d3cr::W) writer structure"]
impl crate::Writable for D3CR {}
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
pub mod d3cr;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupcr](wkupcr) module"]
pub type WKUPCR = crate::Reg<u32, _WKUPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WKUPCR;
#[doc = "`read()` method returns [wkupcr::R](wkupcr::R) reader structure"]
impl crate::Readable for WKUPCR {}
#[doc = "`write(|w| ..)` method takes [wkupcr::W](wkupcr::W) writer structure"]
impl crate::Writable for WKUPCR {}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
pub mod wkupcr;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupfr](wkupfr) module"]
pub type WKUPFR = crate::Reg<u32, _WKUPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WKUPFR;
#[doc = "`read()` method returns [wkupfr::R](wkupfr::R) reader structure"]
impl crate::Readable for WKUPFR {}
#[doc = "`write(|w| ..)` method takes [wkupfr::W](wkupfr::W) writer structure"]
impl crate::Writable for WKUPFR {}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupfr;
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupepr](wkupepr) module"]
pub type WKUPEPR = crate::Reg<u32, _WKUPEPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WKUPEPR;
#[doc = "`read()` method returns [wkupepr::R](wkupepr::R) reader structure"]
impl crate::Readable for WKUPEPR {}
#[doc = "`write(|w| ..)` method takes [wkupepr::W](wkupepr::W) writer structure"]
impl crate::Writable for WKUPEPR {}
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupepr;
