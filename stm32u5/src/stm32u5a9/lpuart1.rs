#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    _reserved4: [u8; 0x08],
    rqr: RQR,
    _reserved_5_isr: [u8; 0x04],
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
    autocr: AUTOCR,
}
impl RegisterBlock {
    ///0x00 - Control register 1
    #[inline(always)]
    pub const fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    ///0x00 - Control register 1
    #[inline(always)]
    pub const fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    ///0x04 - Control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - Baud rate register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x18 - Request register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - Interrupt and status register
    #[inline(always)]
    pub const fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - Interrupt and status register
    #[inline(always)]
    pub const fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - Interrupt flag clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x24 - Receive data register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x28 - Transmit data register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x2c - prescaler register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
    ///0x30 - Autonomous mode control register
    #[inline(always)]
    pub const fn autocr(&self) -> &AUTOCR {
        &self.autocr
    }
}
/**CR1_enabled (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1_enabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_enabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:CR1_enabled)

For information about available fields see [`mod@cr1_enabled`]
module*/
#[doc(alias = "CR1_enabled")]
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLEDrs>;
///Control register 1
pub mod cr1_enabled;
/**CR1_disabled (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:CR1_disabled)

For information about available fields see [`mod@cr1_disabled`]
module*/
#[doc(alias = "CR1_disabled")]
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLEDrs>;
///Control register 1
pub mod cr1_disabled;
/**CR2 (rw) register accessor: Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:CR2)

For information about available fields see [`mod@cr2`]
module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Control register 2
pub mod cr2;
/**CR3 (rw) register accessor: Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:CR3)

For information about available fields see [`mod@cr3`]
module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///Control register 3
pub mod cr3;
/**BRR (rw) register accessor: Baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:BRR)

For information about available fields see [`mod@brr`]
module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///Baud rate register
pub mod brr;
/**RQR (w) register accessor: Request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:RQR)

For information about available fields see [`mod@rqr`]
module*/
pub type RQR = crate::Reg<rqr::RQRrs>;
///Request register
pub mod rqr;
/**ISR_enabled (r) register accessor: Interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_enabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:ISR_enabled)

For information about available fields see [`mod@isr_enabled`]
module*/
#[doc(alias = "ISR_enabled")]
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLEDrs>;
///Interrupt and status register
pub mod isr_enabled;
/**ISR_disabled (r) register accessor: Interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_disabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:ISR_disabled)

For information about available fields see [`mod@isr_disabled`]
module*/
#[doc(alias = "ISR_disabled")]
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLEDrs>;
///Interrupt and status register
pub mod isr_disabled;
/**ICR (w) register accessor: Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:ICR)

For information about available fields see [`mod@icr`]
module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Interrupt flag clear register
pub mod icr;
/**RDR (r) register accessor: Receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:RDR)

For information about available fields see [`mod@rdr`]
module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///Receive data register
pub mod rdr;
/**TDR (rw) register accessor: Transmit data register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:TDR)

For information about available fields see [`mod@tdr`]
module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///Transmit data register
pub mod tdr;
/**PRESC (rw) register accessor: prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:PRESC)

For information about available fields see [`mod@presc`]
module*/
pub type PRESC = crate::Reg<presc::PRESCrs>;
///prescaler register
pub mod presc;
/**AUTOCR (rw) register accessor: Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LPUART1:AUTOCR)

For information about available fields see [`mod@autocr`]
module*/
pub type AUTOCR = crate::Reg<autocr::AUTOCRrs>;
///Autonomous mode control register
pub mod autocr;
