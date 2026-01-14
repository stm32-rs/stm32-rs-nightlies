#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    gtpr: GTPR,
    rtor: RTOR,
    rqr: RQR,
    isr: ISR,
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - Control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
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
    ///0x10 - Guard time and prescaler register
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    ///0x14 - Receiver timeout register
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    ///0x18 - Request register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - Interrupt & status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
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
    ///0x2c - USART prescaler register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
/**CR1 (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///Control register 1
pub mod cr1;
/**CR2 (rw) register accessor: Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///Control register 2
pub mod cr2;
/**CR3 (rw) register accessor: Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///Control register 3
pub mod cr3;
pub use crate::stm32g441::usart1::brr;
pub use crate::stm32g441::usart1::BRR;
/**GTPR (rw) register accessor: Guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:GTPR)

For information about available fields see [`mod@gtpr`] module*/
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
///Guard time and prescaler register
pub mod gtpr;
pub use crate::stm32g441::usart1::rqr;
pub use crate::stm32g441::usart1::rtor;
pub use crate::stm32g441::usart1::RQR;
pub use crate::stm32g441::usart1::RTOR;
/**ISR (r) register accessor: Interrupt & status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Interrupt & status register
pub mod isr;
/**ICR (w) register accessor: Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#UART4:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///Interrupt flag clear register
pub mod icr;
pub use crate::stm32g441::usart1::presc;
pub use crate::stm32g441::usart1::rdr;
pub use crate::stm32g441::usart1::tdr;
pub use crate::stm32g441::usart1::PRESC;
pub use crate::stm32g441::usart1::RDR;
pub use crate::stm32g441::usart1::TDR;
