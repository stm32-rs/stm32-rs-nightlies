#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwr_cr1: PWR_CR1,
    pwr_csr1: PWR_CSR1,
    pwr_cr2: PWR_CR2,
    pwr_cr3: PWR_CR3,
    pwr_mpucr: PWR_MPUCR,
    pwr_mcucr: PWR_MCUCR,
    _reserved6: [u8; 0x08],
    pwr_wkupcr: PWR_WKUPCR,
    pwr_wkupfr: PWR_WKUPFR,
    pwr_mpuwkupenr: PWR_MPUWKUPENR,
    pwr_mcuwkupenr: PWR_MCUWKUPENR,
    _reserved10: [u8; 0x03c4],
    pwr_ver: PWR_VER,
    pwr_id: PWR_ID,
    pwr_sid: PWR_SID,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PWR_CR1 {
        &self.pwr_cr1
    }
    #[doc = "0x04 - Reset on any system reset."]
    #[inline(always)]
    pub const fn pwr_csr1(&self) -> &PWR_CSR1 {
        &self.pwr_csr1
    }
    #[doc = "0x08 - Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PWR_CR2 {
        &self.pwr_cr2
    }
    #[doc = "0x0c - Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PWR_CR3 {
        &self.pwr_cr3
    }
    #[doc = "0x10 - See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_mpucr(&self) -> &PWR_MPUCR {
        &self.pwr_mpucr
    }
    #[doc = "0x14 - See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_mcucr(&self) -> &PWR_MCUCR {
        &self.pwr_mcucr
    }
    #[doc = "0x20 - Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_wkupcr(&self) -> &PWR_WKUPCR {
        &self.pwr_wkupcr
    }
    #[doc = "0x24 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
    #[inline(always)]
    pub const fn pwr_wkupfr(&self) -> &PWR_WKUPFR {
        &self.pwr_wkupfr
    }
    #[doc = "0x28 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_mpuwkupenr(&self) -> &PWR_MPUWKUPENR {
        &self.pwr_mpuwkupenr
    }
    #[doc = "0x2c - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
    #[inline(always)]
    pub const fn pwr_mcuwkupenr(&self) -> &PWR_MCUWKUPENR {
        &self.pwr_mcuwkupenr
    }
    #[doc = "0x3f4 - PWR IP version register"]
    #[inline(always)]
    pub const fn pwr_ver(&self) -> &PWR_VER {
        &self.pwr_ver
    }
    #[doc = "0x3f8 - PWR IP identification register"]
    #[inline(always)]
    pub const fn pwr_id(&self) -> &PWR_ID {
        &self.pwr_id
    }
    #[doc = "0x3fc - PWR size ID register"]
    #[inline(always)]
    pub const fn pwr_sid(&self) -> &PWR_SID {
        &self.pwr_sid
    }
}
#[doc = "PWR_CR1 (rw) register accessor: Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr1`]
module"]
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1rs>;
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value."]
pub mod pwr_cr1;
#[doc = "PWR_CSR1 (r) register accessor: Reset on any system reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_csr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_csr1`]
module"]
pub type PWR_CSR1 = crate::Reg<pwr_csr1::PWR_CSR1rs>;
#[doc = "Reset on any system reset."]
pub mod pwr_csr1;
#[doc = "PWR_CR2 (rw) register accessor: Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr2`]
module"]
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2rs>;
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr2;
#[doc = "PWR_CR3 (rw) register accessor: Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr3`]
module"]
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3rs>;
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_cr3;
#[doc = "PWR_MPUCR (rw) register accessor: See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mpucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mpucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_mpucr`]
module"]
pub type PWR_MPUCR = crate::Reg<pwr_mpucr::PWR_MPUCRrs>;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpucr;
#[doc = "PWR_MCUCR (rw) register accessor: See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mcucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mcucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_mcucr`]
module"]
pub type PWR_MCUCR = crate::Reg<pwr_mcucr::PWR_MCUCRrs>;
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcucr;
#[doc = "PWR_WKUPCR (rw) register accessor: Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wkupcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wkupcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wkupcr`]
module"]
pub type PWR_WKUPCR = crate::Reg<pwr_wkupcr::PWR_WKUPCRrs>;
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_wkupcr;
#[doc = "PWR_WKUPFR (r) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wkupfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wkupfr`]
module"]
pub type PWR_WKUPFR = crate::Reg<pwr_wkupfr::PWR_WKUPFRrs>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)"]
pub mod pwr_wkupfr;
#[doc = "PWR_MPUWKUPENR (rw) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mpuwkupenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mpuwkupenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_mpuwkupenr`]
module"]
pub type PWR_MPUWKUPENR = crate::Reg<pwr_mpuwkupenr::PWR_MPUWKUPENRrs>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mpuwkupenr;
#[doc = "PWR_MCUWKUPENR (rw) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mcuwkupenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mcuwkupenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_mcuwkupenr`]
module"]
pub type PWR_MCUWKUPENR = crate::Reg<pwr_mcuwkupenr::PWR_MCUWKUPENRrs>;
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed."]
pub mod pwr_mcuwkupenr;
#[doc = "PWR_VER (r) register accessor: PWR IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ver`]
module"]
pub type PWR_VER = crate::Reg<pwr_ver::PWR_VERrs>;
#[doc = "PWR IP version register"]
pub mod pwr_ver;
#[doc = "PWR_ID (r) register accessor: PWR IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_id`]
module"]
pub type PWR_ID = crate::Reg<pwr_id::PWR_IDrs>;
#[doc = "PWR IP identification register"]
pub mod pwr_id;
#[doc = "PWR_SID (r) register accessor: PWR size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_sid`]
module"]
pub type PWR_SID = crate::Reg<pwr_sid::PWR_SIDrs>;
#[doc = "PWR size ID register"]
pub mod pwr_sid;
