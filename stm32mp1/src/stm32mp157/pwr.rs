#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
    pub pwr_cr1: PWR_CR1,
    #[doc = "0x04 - Reset on any system reset."]
    pub pwr_csr1: PWR_CSR1,
    #[doc = "0x08 - Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_cr2: PWR_CR2,
    #[doc = "0x0c - Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_cr3: PWR_CR3,
    #[doc = "0x10 - See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mpucr: PWR_MPUCR,
    #[doc = "0x14 - See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mcucr: PWR_MCUCR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_wkupcr: PWR_WKUPCR,
    #[doc = "0x24 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
    pub pwr_wkupfr: PWR_WKUPFR,
    #[doc = "0x28 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mpuwkupenr: PWR_MPUWKUPENR,
    #[doc = "0x2c - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mcuwkupenr: PWR_MCUWKUPENR,
    _reserved10: [u8; 964usize],
    #[doc = "0x3f4 - PWR IP version register"]
    pub pwr_ver: PWR_VER,
    #[doc = "0x3f8 - PWR IP identification register"]
    pub pwr_id: PWR_ID,
    #[doc = "0x3fc - PWR size ID register"]
    pub pwr_sid: PWR_SID,
}
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr1](pwr_cr1) module"]
pub type PWR_CR1 = crate::Reg<u32, _PWR_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CR1;
#[doc = "`read()` method returns [pwr_cr1::R](pwr_cr1::R) reader structure"]
impl crate::Readable for PWR_CR1 {}
#[doc = "`write(|w| ..)` method takes [pwr_cr1::W](pwr_cr1::W) writer structure"]
impl crate::Writable for PWR_CR1 {}
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
pub mod pwr_cr1;
#[doc = "Reset on any system reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_csr1](pwr_csr1) module"]
pub type PWR_CSR1 = crate::Reg<u32, _PWR_CSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CSR1;
#[doc = "`read()` method returns [pwr_csr1::R](pwr_csr1::R) reader structure"]
impl crate::Readable for PWR_CSR1 {}
#[doc = "Reset on any system reset."]
pub mod pwr_csr1;
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr2](pwr_cr2) module"]
pub type PWR_CR2 = crate::Reg<u32, _PWR_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CR2;
#[doc = "`read()` method returns [pwr_cr2::R](pwr_cr2::R) reader structure"]
impl crate::Readable for PWR_CR2 {}
#[doc = "`write(|w| ..)` method takes [pwr_cr2::W](pwr_cr2::W) writer structure"]
impl crate::Writable for PWR_CR2 {}
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr2;
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr3](pwr_cr3) module"]
pub type PWR_CR3 = crate::Reg<u32, _PWR_CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CR3;
#[doc = "`read()` method returns [pwr_cr3::R](pwr_cr3::R) reader structure"]
impl crate::Readable for PWR_CR3 {}
#[doc = "`write(|w| ..)` method takes [pwr_cr3::W](pwr_cr3::W) writer structure"]
impl crate::Writable for PWR_CR3 {}
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr3;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mpucr](pwr_mpucr) module"]
pub type PWR_MPUCR = crate::Reg<u32, _PWR_MPUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_MPUCR;
#[doc = "`read()` method returns [pwr_mpucr::R](pwr_mpucr::R) reader structure"]
impl crate::Readable for PWR_MPUCR {}
#[doc = "`write(|w| ..)` method takes [pwr_mpucr::W](pwr_mpucr::W) writer structure"]
impl crate::Writable for PWR_MPUCR {}
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpucr;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mcucr](pwr_mcucr) module"]
pub type PWR_MCUCR = crate::Reg<u32, _PWR_MCUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_MCUCR;
#[doc = "`read()` method returns [pwr_mcucr::R](pwr_mcucr::R) reader structure"]
impl crate::Readable for PWR_MCUCR {}
#[doc = "`write(|w| ..)` method takes [pwr_mcucr::W](pwr_mcucr::W) writer structure"]
impl crate::Writable for PWR_MCUCR {}
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcucr;
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_wkupcr](pwr_wkupcr) module"]
pub type PWR_WKUPCR = crate::Reg<u32, _PWR_WKUPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_WKUPCR;
#[doc = "`read()` method returns [pwr_wkupcr::R](pwr_wkupcr::R) reader structure"]
impl crate::Readable for PWR_WKUPCR {}
#[doc = "`write(|w| ..)` method takes [pwr_wkupcr::W](pwr_wkupcr::W) writer structure"]
impl crate::Writable for PWR_WKUPCR {}
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_wkupcr;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_wkupfr](pwr_wkupfr) module"]
pub type PWR_WKUPFR = crate::Reg<u32, _PWR_WKUPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_WKUPFR;
#[doc = "`read()` method returns [pwr_wkupfr::R](pwr_wkupfr::R) reader structure"]
impl crate::Readable for PWR_WKUPFR {}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
pub mod pwr_wkupfr;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mpuwkupenr](pwr_mpuwkupenr) module"]
pub type PWR_MPUWKUPENR = crate::Reg<u32, _PWR_MPUWKUPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_MPUWKUPENR;
#[doc = "`read()` method returns [pwr_mpuwkupenr::R](pwr_mpuwkupenr::R) reader structure"]
impl crate::Readable for PWR_MPUWKUPENR {}
#[doc = "`write(|w| ..)` method takes [pwr_mpuwkupenr::W](pwr_mpuwkupenr::W) writer structure"]
impl crate::Writable for PWR_MPUWKUPENR {}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpuwkupenr;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mcuwkupenr](pwr_mcuwkupenr) module"]
pub type PWR_MCUWKUPENR = crate::Reg<u32, _PWR_MCUWKUPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_MCUWKUPENR;
#[doc = "`read()` method returns [pwr_mcuwkupenr::R](pwr_mcuwkupenr::R) reader structure"]
impl crate::Readable for PWR_MCUWKUPENR {}
#[doc = "`write(|w| ..)` method takes [pwr_mcuwkupenr::W](pwr_mcuwkupenr::W) writer structure"]
impl crate::Writable for PWR_MCUWKUPENR {}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcuwkupenr;
#[doc = "PWR IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ver](pwr_ver) module"]
pub type PWR_VER = crate::Reg<u32, _PWR_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_VER;
#[doc = "`read()` method returns [pwr_ver::R](pwr_ver::R) reader structure"]
impl crate::Readable for PWR_VER {}
#[doc = "PWR IP version register"]
pub mod pwr_ver;
#[doc = "PWR IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_id](pwr_id) module"]
pub type PWR_ID = crate::Reg<u32, _PWR_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_ID;
#[doc = "`read()` method returns [pwr_id::R](pwr_id::R) reader structure"]
impl crate::Readable for PWR_ID {}
#[doc = "PWR IP identification register"]
pub mod pwr_id;
#[doc = "PWR size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_sid](pwr_sid) module"]
pub type PWR_SID = crate::Reg<u32, _PWR_SID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_SID;
#[doc = "`read()` method returns [pwr_sid::R](pwr_sid::R) reader structure"]
impl crate::Readable for PWR_SID {}
#[doc = "PWR size ID register"]
pub mod pwr_sid;
