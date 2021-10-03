#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
    pub pwr_cr1: crate::Reg<pwr_cr1::PWR_CR1_SPEC>,
    #[doc = "0x04 - Reset on any system reset."]
    pub pwr_csr1: crate::Reg<pwr_csr1::PWR_CSR1_SPEC>,
    #[doc = "0x08 - Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_cr2: crate::Reg<pwr_cr2::PWR_CR2_SPEC>,
    #[doc = "0x0c - Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_cr3: crate::Reg<pwr_cr3::PWR_CR3_SPEC>,
    #[doc = "0x10 - See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mpucr: crate::Reg<pwr_mpucr::PWR_MPUCR_SPEC>,
    #[doc = "0x14 - See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mcucr: crate::Reg<pwr_mcucr::PWR_MCUCR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_wkupcr: crate::Reg<pwr_wkupcr::PWR_WKUPCR_SPEC>,
    #[doc = "0x24 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
    pub pwr_wkupfr: crate::Reg<pwr_wkupfr::PWR_WKUPFR_SPEC>,
    #[doc = "0x28 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mpuwkupenr: crate::Reg<pwr_mpuwkupenr::PWR_MPUWKUPENR_SPEC>,
    #[doc = "0x2c - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    pub pwr_mcuwkupenr: crate::Reg<pwr_mcuwkupenr::PWR_MCUWKUPENR_SPEC>,
    _reserved10: [u8; 0x03c4],
    #[doc = "0x3f4 - PWR IP version register"]
    pub pwr_ver: crate::Reg<pwr_ver::PWR_VER_SPEC>,
    #[doc = "0x3f8 - PWR IP identification register"]
    pub pwr_id: crate::Reg<pwr_id::PWR_ID_SPEC>,
    #[doc = "0x3fc - PWR size ID register"]
    pub pwr_sid: crate::Reg<pwr_sid::PWR_SID_SPEC>,
}
#[doc = "PWR_CR1 register accessor: an alias for `Reg<PWR_CR1_SPEC>`"]
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1_SPEC>;
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
pub mod pwr_cr1;
#[doc = "PWR_CSR1 register accessor: an alias for `Reg<PWR_CSR1_SPEC>`"]
pub type PWR_CSR1 = crate::Reg<pwr_csr1::PWR_CSR1_SPEC>;
#[doc = "Reset on any system reset."]
pub mod pwr_csr1;
#[doc = "PWR_CR2 register accessor: an alias for `Reg<PWR_CR2_SPEC>`"]
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2_SPEC>;
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr2;
#[doc = "PWR_CR3 register accessor: an alias for `Reg<PWR_CR3_SPEC>`"]
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3_SPEC>;
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr3;
#[doc = "PWR_MPUCR register accessor: an alias for `Reg<PWR_MPUCR_SPEC>`"]
pub type PWR_MPUCR = crate::Reg<pwr_mpucr::PWR_MPUCR_SPEC>;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpucr;
#[doc = "PWR_MCUCR register accessor: an alias for `Reg<PWR_MCUCR_SPEC>`"]
pub type PWR_MCUCR = crate::Reg<pwr_mcucr::PWR_MCUCR_SPEC>;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcucr;
#[doc = "PWR_WKUPCR register accessor: an alias for `Reg<PWR_WKUPCR_SPEC>`"]
pub type PWR_WKUPCR = crate::Reg<pwr_wkupcr::PWR_WKUPCR_SPEC>;
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_wkupcr;
#[doc = "PWR_WKUPFR register accessor: an alias for `Reg<PWR_WKUPFR_SPEC>`"]
pub type PWR_WKUPFR = crate::Reg<pwr_wkupfr::PWR_WKUPFR_SPEC>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
pub mod pwr_wkupfr;
#[doc = "PWR_MPUWKUPENR register accessor: an alias for `Reg<PWR_MPUWKUPENR_SPEC>`"]
pub type PWR_MPUWKUPENR = crate::Reg<pwr_mpuwkupenr::PWR_MPUWKUPENR_SPEC>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpuwkupenr;
#[doc = "PWR_MCUWKUPENR register accessor: an alias for `Reg<PWR_MCUWKUPENR_SPEC>`"]
pub type PWR_MCUWKUPENR = crate::Reg<pwr_mcuwkupenr::PWR_MCUWKUPENR_SPEC>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcuwkupenr;
#[doc = "PWR_VER register accessor: an alias for `Reg<PWR_VER_SPEC>`"]
pub type PWR_VER = crate::Reg<pwr_ver::PWR_VER_SPEC>;
#[doc = "PWR IP version register"]
pub mod pwr_ver;
#[doc = "PWR_ID register accessor: an alias for `Reg<PWR_ID_SPEC>`"]
pub type PWR_ID = crate::Reg<pwr_id::PWR_ID_SPEC>;
#[doc = "PWR IP identification register"]
pub mod pwr_id;
#[doc = "PWR_SID register accessor: an alias for `Reg<PWR_SID_SPEC>`"]
pub type PWR_SID = crate::Reg<pwr_sid::PWR_SID_SPEC>;
#[doc = "PWR size ID register"]
pub mod pwr_sid;
