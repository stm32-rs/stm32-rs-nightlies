#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    csr1: CSR1,
    cr2: CR2,
    cr3: CR3,
    mpucr: MPUCR,
    mcucr: MCUCR,
    _reserved6: [u8; 0x08],
    wkupcr: WKUPCR,
    wkupfr: WKUPFR,
    mpuwkupenr: MPUWKUPENR,
    mcuwkupenr: MCUWKUPENR,
    _reserved10: [u8; 0x03c4],
    ver: VER,
    id: ID,
    sid: SID,
}
impl RegisterBlock {
    ///0x00 - Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - Reset on any system reset.
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    ///0x08 - Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x10 - See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn mpucr(&self) -> &MPUCR {
        &self.mpucr
    }
    ///0x14 - See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn mcucr(&self) -> &MCUCR {
        &self.mcucr
    }
    ///0x20 - Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\] bits and WKUPPUPD\[6:1\] bit pairs are discarded when the corresponding WKUPEN\[6:1\] bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn wkupcr(&self) -> &WKUPCR {
        &self.wkupcr
    }
    ///0x24 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)
    #[inline(always)]
    pub const fn wkupfr(&self) -> &WKUPFR {
        &self.wkupfr
    }
    ///0x28 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn mpuwkupenr(&self) -> &MPUWKUPENR {
        &self.mpuwkupenr
    }
    ///0x2c - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.
    #[inline(always)]
    pub const fn mcuwkupenr(&self) -> &MCUWKUPENR {
        &self.mcuwkupenr
    }
    ///0x3f4 - PWR IP version register
    #[inline(always)]
    pub const fn ver(&self) -> &VER {
        &self.ver
    }
    ///0x3f8 - PWR IP identification register
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    ///0x3fc - PWR size ID register
    #[inline(always)]
    pub const fn sid(&self) -> &SID {
        &self.sid
    }
}
/**CR1 (rw) register accessor: Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.
pub mod cr1;
/**CSR1 (r) register accessor: Reset on any system reset.

You can [`read`](crate::Reg::read) this register and get [`csr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:CSR1)

For information about available fields see [`mod@csr1`] module*/
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
///Reset on any system reset.
pub mod csr1;
/**CR2 (rw) register accessor: Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod cr2;
/**CR3 (rw) register accessor: Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod cr3;
/**MPUCR (rw) register accessor: See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mpucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:MPUCR)

For information about available fields see [`mod@mpucr`] module*/
pub type MPUCR = crate::Reg<mpucr::MPUCRrs>;
///See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod mpucr;
/**MCUCR (rw) register accessor: See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:MCUCR)

For information about available fields see [`mod@mcucr`] module*/
pub type MCUCR = crate::Reg<mcucr::MCUCRrs>;
///See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod mcucr;
/**WKUPCR (rw) register accessor: Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\] bits and WKUPPUPD\[6:1\] bit pairs are discarded when the corresponding WKUPEN\[6:1\] bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:WKUPCR)

For information about available fields see [`mod@wkupcr`] module*/
pub type WKUPCR = crate::Reg<wkupcr::WKUPCRrs>;
///Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\] bits and WKUPPUPD\[6:1\] bit pairs are discarded when the corresponding WKUPEN\[6:1\] bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod wkupcr;
/**WKUPFR (r) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:WKUPFR)

For information about available fields see [`mod@wkupfr`] module*/
pub type WKUPFR = crate::Reg<wkupfr::WKUPFRrs>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)
pub mod wkupfr;
/**MPUWKUPENR (rw) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mpuwkupenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuwkupenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:MPUWKUPENR)

For information about available fields see [`mod@mpuwkupenr`] module*/
pub type MPUWKUPENR = crate::Reg<mpuwkupenr::MPUWKUPENRrs>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod mpuwkupenr;
/**MCUWKUPENR (rw) register accessor: Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mcuwkupenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuwkupenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:MCUWKUPENR)

For information about available fields see [`mod@mcuwkupenr`] module*/
pub type MCUWKUPENR = crate::Reg<mcuwkupenr::MCUWKUPENRrs>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod mcuwkupenr;
/**VER (r) register accessor: PWR IP version register

You can [`read`](crate::Reg::read) this register and get [`ver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:VER)

For information about available fields see [`mod@ver`] module*/
pub type VER = crate::Reg<ver::VERrs>;
///PWR IP version register
pub mod ver;
/**ID (r) register accessor: PWR IP identification register

You can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:ID)

For information about available fields see [`mod@id`] module*/
pub type ID = crate::Reg<id::IDrs>;
///PWR IP identification register
pub mod id;
/**SID (r) register accessor: PWR size ID register

You can [`read`](crate::Reg::read) this register and get [`sid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:SID)

For information about available fields see [`mod@sid`] module*/
pub type SID = crate::Reg<sid::SIDrs>;
///PWR size ID register
pub mod sid;
