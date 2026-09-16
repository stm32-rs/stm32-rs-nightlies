#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    brr: BRR,
    _reserved2: [u8; 0x04],
    isr: ISR,
    icr: ICR,
    ier: IER,
    rfl: RFL,
    tdr: TDR,
    rdr: RDR,
    or: OR,
}
impl RegisterBlock {
    ///0x00 - SWPMI Configuration/Control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - SWPMI Bitrate register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x0c - SWPMI Interrupt and Status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10 - SWPMI Interrupt Flag Clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x14 - SWPMI Interrupt Enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x18 - SWPMI Receive Frame Length register
    #[inline(always)]
    pub const fn rfl(&self) -> &RFL {
        &self.rfl
    }
    ///0x1c - SWPMI Transmit data register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x20 - SWPMI Receive data register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x24 - SWPMI Option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
/**CR (rw) register accessor: SWPMI Configuration/Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///SWPMI Configuration/Control register
pub mod cr;
/**BRR (rw) register accessor: SWPMI Bitrate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:BRR)

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRRrs>;
///SWPMI Bitrate register
pub mod brr;
/**ISR (r) register accessor: SWPMI Interrupt and Status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///SWPMI Interrupt and Status register
pub mod isr;
/**ICR (w) register accessor: SWPMI Interrupt Flag Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///SWPMI Interrupt Flag Clear register
pub mod icr;
/**IER (rw) register accessor: SWPMI Interrupt Enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///SWPMI Interrupt Enable register
pub mod ier;
/**RFL (r) register accessor: SWPMI Receive Frame Length register

You can [`read`](crate::Reg::read) this register and get [`rfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:RFL)

For information about available fields see [`mod@rfl`] module*/
pub type RFL = crate::Reg<rfl::RFLrs>;
///SWPMI Receive Frame Length register
pub mod rfl;
/**TDR (w) register accessor: SWPMI Transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///SWPMI Transmit data register
pub mod tdr;
/**RDR (r) register accessor: SWPMI Receive data register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:RDR)

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDRrs>;
///SWPMI Receive data register
pub mod rdr;
/**OR (rw) register accessor: SWPMI Option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///SWPMI Option register
pub mod or;
