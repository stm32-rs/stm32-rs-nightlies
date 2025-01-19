#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_lpuart_cr: [u8; 0x04],
    lpuart_cr2: LPUART_CR2,
    lpuart_cr3: LPUART_CR3,
    lpuart_brr: LPUART_BRR,
    _reserved4: [u8; 0x08],
    lpuart_rqr: LPUART_RQR,
    _reserved_5_lpuart_: [u8; 0x04],
    lpuart_icr: LPUART_ICR,
    lpuart_rdr: LPUART_RDR,
    lpuart_tdr: LPUART_TDR,
    lpuart_presc: LPUART_PRESC,
}
impl RegisterBlock {
    ///0x00 - LPUART control register 1
    #[inline(always)]
    pub const fn lpuart_cr1_alternate(&self) -> &LPUART_CR1_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPUART control register 1
    #[inline(always)]
    pub const fn lpuart_cr1(&self) -> &LPUART_CR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPUART control register 2
    #[inline(always)]
    pub const fn lpuart_cr2(&self) -> &LPUART_CR2 {
        &self.lpuart_cr2
    }
    ///0x08 - LPUART control register 3
    #[inline(always)]
    pub const fn lpuart_cr3(&self) -> &LPUART_CR3 {
        &self.lpuart_cr3
    }
    ///0x0c - LPUART baud rate register
    #[inline(always)]
    pub const fn lpuart_brr(&self) -> &LPUART_BRR {
        &self.lpuart_brr
    }
    ///0x18 - LPUART request register
    #[inline(always)]
    pub const fn lpuart_rqr(&self) -> &LPUART_RQR {
        &self.lpuart_rqr
    }
    ///0x1c - LPUART interrupt and status register
    #[inline(always)]
    pub const fn lpuart_isr_alternate(&self) -> &LPUART_ISR_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - LPUART interrupt and status register
    #[inline(always)]
    pub const fn lpuart_isr(&self) -> &LPUART_ISR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - LPUART interrupt flag clear register
    #[inline(always)]
    pub const fn lpuart_icr(&self) -> &LPUART_ICR {
        &self.lpuart_icr
    }
    ///0x24 - LPUART receive data register
    #[inline(always)]
    pub const fn lpuart_rdr(&self) -> &LPUART_RDR {
        &self.lpuart_rdr
    }
    ///0x28 - LPUART transmit data register
    #[inline(always)]
    pub const fn lpuart_tdr(&self) -> &LPUART_TDR {
        &self.lpuart_tdr
    }
    ///0x2c - LPUART prescaler register
    #[inline(always)]
    pub const fn lpuart_presc(&self) -> &LPUART_PRESC {
        &self.lpuart_presc
    }
}
/**LPUART_CR1 (rw) register accessor: LPUART control register 1

You can [`read`](crate::Reg::read) this register and get [`lpuart_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_CR1)

For information about available fields see [`mod@lpuart_cr1`]
module*/
pub type LPUART_CR1 = crate::Reg<lpuart_cr1::LPUART_CR1rs>;
///LPUART control register 1
pub mod lpuart_cr1;
/**LPUART_CR1_ALTERNATE (rw) register accessor: LPUART control register 1

You can [`read`](crate::Reg::read) this register and get [`lpuart_cr1_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_cr1_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_CR1_ALTERNATE)

For information about available fields see [`mod@lpuart_cr1_alternate`]
module*/
pub type LPUART_CR1_ALTERNATE = crate::Reg<lpuart_cr1_alternate::LPUART_CR1_ALTERNATErs>;
///LPUART control register 1
pub mod lpuart_cr1_alternate;
/**LPUART_CR2 (rw) register accessor: LPUART control register 2

You can [`read`](crate::Reg::read) this register and get [`lpuart_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_CR2)

For information about available fields see [`mod@lpuart_cr2`]
module*/
pub type LPUART_CR2 = crate::Reg<lpuart_cr2::LPUART_CR2rs>;
///LPUART control register 2
pub mod lpuart_cr2;
/**LPUART_CR3 (rw) register accessor: LPUART control register 3

You can [`read`](crate::Reg::read) this register and get [`lpuart_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_CR3)

For information about available fields see [`mod@lpuart_cr3`]
module*/
pub type LPUART_CR3 = crate::Reg<lpuart_cr3::LPUART_CR3rs>;
///LPUART control register 3
pub mod lpuart_cr3;
/**LPUART_BRR (rw) register accessor: LPUART baud rate register

You can [`read`](crate::Reg::read) this register and get [`lpuart_brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_BRR)

For information about available fields see [`mod@lpuart_brr`]
module*/
pub type LPUART_BRR = crate::Reg<lpuart_brr::LPUART_BRRrs>;
///LPUART baud rate register
pub mod lpuart_brr;
/**LPUART_RQR (w) register accessor: LPUART request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_RQR)

For information about available fields see [`mod@lpuart_rqr`]
module*/
pub type LPUART_RQR = crate::Reg<lpuart_rqr::LPUART_RQRrs>;
///LPUART request register
pub mod lpuart_rqr;
/**LPUART_ISR (r) register accessor: LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`lpuart_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_ISR)

For information about available fields see [`mod@lpuart_isr`]
module*/
pub type LPUART_ISR = crate::Reg<lpuart_isr::LPUART_ISRrs>;
///LPUART interrupt and status register
pub mod lpuart_isr;
/**LPUART_ISR_ALTERNATE (r) register accessor: LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`lpuart_isr_alternate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_ISR_ALTERNATE)

For information about available fields see [`mod@lpuart_isr_alternate`]
module*/
pub type LPUART_ISR_ALTERNATE = crate::Reg<lpuart_isr_alternate::LPUART_ISR_ALTERNATErs>;
///LPUART interrupt and status register
pub mod lpuart_isr_alternate;
/**LPUART_ICR (w) register accessor: LPUART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_ICR)

For information about available fields see [`mod@lpuart_icr`]
module*/
pub type LPUART_ICR = crate::Reg<lpuart_icr::LPUART_ICRrs>;
///LPUART interrupt flag clear register
pub mod lpuart_icr;
/**LPUART_RDR (r) register accessor: LPUART receive data register

You can [`read`](crate::Reg::read) this register and get [`lpuart_rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_RDR)

For information about available fields see [`mod@lpuart_rdr`]
module*/
pub type LPUART_RDR = crate::Reg<lpuart_rdr::LPUART_RDRrs>;
///LPUART receive data register
pub mod lpuart_rdr;
/**LPUART_TDR (rw) register accessor: LPUART transmit data register

You can [`read`](crate::Reg::read) this register and get [`lpuart_tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_TDR)

For information about available fields see [`mod@lpuart_tdr`]
module*/
pub type LPUART_TDR = crate::Reg<lpuart_tdr::LPUART_TDRrs>;
///LPUART transmit data register
pub mod lpuart_tdr;
/**LPUART_PRESC (rw) register accessor: LPUART prescaler register

You can [`read`](crate::Reg::read) this register and get [`lpuart_presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_PRESC)

For information about available fields see [`mod@lpuart_presc`]
module*/
pub type LPUART_PRESC = crate::Reg<lpuart_presc::LPUART_PRESCrs>;
///LPUART prescaler register
pub mod lpuart_presc;
