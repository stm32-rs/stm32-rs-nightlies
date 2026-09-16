#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    csr1: CSR1,
    cr2: CR2,
    cr3: CR3,
    cpu1cr: CPU1CR,
    cpu2cr: CPU2CR,
    d3cr: D3CR,
    _reserved7: [u8; 0x04],
    wkupcr: WKUPCR,
    wkupfr: WKUPFR,
    wkupepr: WKUPEPR,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - PWR control status register 1
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    ///0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x10 - This register allows controlling CPU1 power.
    #[inline(always)]
    pub const fn cpu1cr(&self) -> &CPU1CR {
        &self.cpu1cr
    }
    ///0x14 - This register allows controlling CPU2 power
    #[inline(always)]
    pub const fn cpu2cr(&self) -> &CPU2CR {
        &self.cpu2cr
    }
    ///0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software
    #[inline(always)]
    pub const fn d3cr(&self) -> &D3CR {
        &self.d3cr
    }
    ///0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).
    #[inline(always)]
    pub const fn wkupcr(&self) -> &WKUPCR {
        &self.wkupcr
    }
    ///0x24 - reset only by system reset, not reset by wakeup from Standby mode
    #[inline(always)]
    pub const fn wkupfr(&self) -> &WKUPFR {
        &self.wkupfr
    }
    ///0x28 - Reset only by system reset, not reset by wakeup from Standby mode
    #[inline(always)]
    pub const fn wkupepr(&self) -> &WKUPEPR {
        &self.wkupepr
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CSR1 (r) register accessor: PWR control status register 1

You can [`read`](crate::Reg::read) this register and get [`csr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CSR1)

For information about available fields see [`mod@csr1`] module*/
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
///PWR control status register 1
pub mod csr1;
/**CR2 (rw) register accessor: This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.
pub mod cr2;
/**CR3 (rw) register accessor: Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.
pub mod cr3;
/**CPU1CR (rw) register accessor: This register allows controlling CPU1 power.

You can [`read`](crate::Reg::read) this register and get [`cpu1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CPU1CR)

For information about available fields see [`mod@cpu1cr`] module*/
pub type CPU1CR = crate::Reg<cpu1cr::CPU1CRrs>;
///This register allows controlling CPU1 power.
pub mod cpu1cr;
/**CPU2CR (rw) register accessor: This register allows controlling CPU2 power

You can [`read`](crate::Reg::read) this register and get [`cpu2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:CPU2CR)

For information about available fields see [`mod@cpu2cr`] module*/
pub type CPU2CR = crate::Reg<cpu2cr::CPU2CRrs>;
///This register allows controlling CPU2 power
pub mod cpu2cr;
/**D3CR (rw) register accessor: This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software

You can [`read`](crate::Reg::read) this register and get [`d3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:D3CR)

For information about available fields see [`mod@d3cr`] module*/
pub type D3CR = crate::Reg<d3cr::D3CRrs>;
///This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software
pub mod d3cr;
/**WKUPCR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:WKUPCR)

For information about available fields see [`mod@wkupcr`] module*/
pub type WKUPCR = crate::Reg<wkupcr::WKUPCRrs>;
///reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).
pub mod wkupcr;
/**WKUPFR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:WKUPFR)

For information about available fields see [`mod@wkupfr`] module*/
pub type WKUPFR = crate::Reg<wkupfr::WKUPFRrs>;
///reset only by system reset, not reset by wakeup from Standby mode
pub mod wkupfr;
/**WKUPEPR (rw) register accessor: Reset only by system reset, not reset by wakeup from Standby mode

You can [`read`](crate::Reg::read) this register and get [`wkupepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#PWR:WKUPEPR)

For information about available fields see [`mod@wkupepr`] module*/
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPRrs>;
///Reset only by system reset, not reset by wakeup from Standby mode
pub mod wkupepr;
