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
    ///0x00 - CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - CR3 register
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - BRR register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x10 - GTPR register
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    ///0x14 - RTOR register
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    ///0x18 - RQR register
    #[inline(always)]
    pub const fn rqr(&self) -> &RQR {
        &self.rqr
    }
    ///0x1c - ISR register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x20 - ICR register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x24 - RDR register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x28 - TDR register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x2c - PRESC register
    #[inline(always)]
    pub const fn presc(&self) -> &PRESC {
        &self.presc
    }
}
/**CR1 (rw) register accessor: CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///CR1 register
pub mod cr1;
/**CR2 (rw) register accessor: CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///CR2 register
pub mod cr2;
/**CR3 (rw) register accessor: CR3 register

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///CR3 register
pub mod cr3;
/**BRR (rw) register accessor: BRR register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///BRR register
pub mod brr;
/**GTPR (rw) register accessor: GTPR register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:GTPR)

For information about available fields see [`mod@gtpr`] module*/
pub type GTPR = crate::Reg<gtpr::GTPRrs>;
///GTPR register
pub mod gtpr;
/**RTOR (rw) register accessor: RTOR register

You can [`read`](crate::Reg::read) this register and get [`rtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:RTOR)

For information about available fields see [`mod@rtor`] module*/
pub type RTOR = crate::Reg<rtor::RTORrs>;
///RTOR register
pub mod rtor;
/**RQR (rw) register accessor: RQR register

You can [`read`](crate::Reg::read) this register and get [`rqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:RQR)

For information about available fields see [`mod@rqr`] module*/
pub type RQR = crate::Reg<rqr::RQRrs>;
///RQR register
pub mod rqr;
/**ISR (r) register accessor: ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///ISR register
pub mod isr;
/**ICR (rw) register accessor: ICR register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///ICR register
pub mod icr;
/**RDR (r) register accessor: RDR register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///RDR register
pub mod rdr;
/**TDR (rw) register accessor: TDR register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///TDR register
pub mod tdr;
/**PRESC (rw) register accessor: PRESC register

You can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:PRESC)

For information about available fields see [`mod@presc`] module*/
pub type PRESC = crate::Reg<presc::PRESCrs>;
///PRESC register
pub mod presc;
