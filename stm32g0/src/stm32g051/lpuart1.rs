#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    brr: BRR,
    _reserved4: [u8; 0x08],
    rqr: RQR,
    isr: ISR,
    icr: ICR,
    rdr: RDR,
    tdr: TDR,
    presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - LPUART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - LPUART control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - LPUART control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - LPUART baud rate register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x18 - LPUART request register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - LPUART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x20 - LPUART interrupt flag clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x24 - LPUART receive data register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x28 - LPUART transmit data register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x2c - LPUART prescaler register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
/**CR1 (rw) register accessor: LPUART control register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///LPUART control register 1 \[alternate\]
pub mod cr1;
/**CR2 (rw) register accessor: LPUART control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///LPUART control register 2
pub mod cr2;
/**CR3 (rw) register accessor: LPUART control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///LPUART control register 3
pub mod cr3;
/**BRR (rw) register accessor: LPUART baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///LPUART baud rate register
pub mod brr;
/**RQR (w) register accessor: LPUART request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:RQR)

For information about available fields see [`mod@rqr`] module*/
pub type RQR = crate::Reg<rqr::RQRrs>;
///LPUART request register
pub mod rqr;
/**ISR (r) register accessor: LPUART interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LPUART interrupt and status register \[alternate\]
pub mod isr;
/**ICR (w) register accessor: LPUART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LPUART interrupt flag clear register
pub mod icr;
/**RDR (r) register accessor: LPUART receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///LPUART receive data register
pub mod rdr;
/**TDR (rw) register accessor: LPUART transmit data register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///LPUART transmit data register
pub mod tdr;
/**PRESC (rw) register accessor: LPUART prescaler register

You can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#LPUART1:PRESC)

For information about available fields see [`mod@presc`] module*/
pub type PRESC = crate::Reg<presc::PRESCrs>;
///LPUART prescaler register
pub mod presc;
