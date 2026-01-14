#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sr: SR,
    _reserved1: [u8; 0x02],
    dr: DR,
    _reserved2: [u8; 0x02],
    brr: BRR,
    _reserved3: [u8; 0x02],
    cr1: CR1,
    _reserved4: [u8; 0x02],
    cr2: CR2,
    _reserved5: [u8; 0x02],
    cr3: CR3,
    _reserved6: [u8; 0x02],
    gtpr: GTPR,
}
impl RegisterBlock {
    ///0x00 - UART4_SR
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x04 - UART4_DR
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x08 - UART4_BRR
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x0c - UART4_CR1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x10 - UART4_CR2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x14 - UART4_CR3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x18 - Guard Time and Prescaler Register
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
}
/**SR (rw) register accessor: UART4_SR

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#UART4:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///UART4_SR
pub mod sr;
pub use crate::stm32f101::usart1::brr;
pub use crate::stm32f101::usart1::cr1;
pub use crate::stm32f101::usart1::dr;
pub use crate::stm32f101::usart1::BRR;
pub use crate::stm32f101::usart1::CR1;
pub use crate::stm32f101::usart1::DR;
/**CR2 (rw) register accessor: UART4_CR2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#UART4:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///UART4_CR2
pub mod cr2;
/**CR3 (rw) register accessor: UART4_CR3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#UART4:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///UART4_CR3
pub mod cr3;
/**GTPR (rw) register accessor: Guard Time and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#UART4:GTPR)

For information about available fields see [`mod@gtpr`] module*/
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
///Guard Time and Prescaler Register
pub mod gtpr;
