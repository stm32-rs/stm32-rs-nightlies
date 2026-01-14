#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    isr: ISR,
    ifcr: IFCR,
    fgmar: FGMAR,
    fgor: FGOR,
    bgmar: BGMAR,
    bgor: BGOR,
    fgpfccr: FGPFCCR,
    fgcolr: FGCOLR,
    bgpfccr: BGPFCCR,
    bgcolr: BGCOLR,
    fgcmar: FGCMAR,
    bgcmar: BGCMAR,
    opfccr: OPFCCR,
    ocolr: OCOLR,
    omar: OMAR,
    oor: OOR,
    nlr: NLR,
    lwr: LWR,
    amtcr: AMTCR,
}
impl RegisterBlock {
    ///0x00 - DMA2D control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DMA2D Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x08 - DMA2D interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x0c - DMA2D foreground memory address register
    #[inline(always)]
    pub const fn fgmar(&self) -> &FGMAR {
        &self.fgmar
    }
    ///0x10 - DMA2D foreground offset register
    #[inline(always)]
    pub const fn fgor(&self) -> &FGOR {
        &self.fgor
    }
    ///0x14 - DMA2D background memory address register
    #[inline(always)]
    pub const fn bgmar(&self) -> &BGMAR {
        &self.bgmar
    }
    ///0x18 - DMA2D background offset register
    #[inline(always)]
    pub const fn bgor(&self) -> &BGOR {
        &self.bgor
    }
    ///0x1c - DMA2D foreground PFC control register
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &FGPFCCR {
        &self.fgpfccr
    }
    ///0x20 - DMA2D foreground color register
    #[inline(always)]
    pub const fn fgcolr(&self) -> &FGCOLR {
        &self.fgcolr
    }
    ///0x24 - DMA2D background PFC control register
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &BGPFCCR {
        &self.bgpfccr
    }
    ///0x28 - DMA2D background color register
    #[inline(always)]
    pub const fn bgcolr(&self) -> &BGCOLR {
        &self.bgcolr
    }
    ///0x2c - DMA2D foreground CLUT memory address register
    #[inline(always)]
    pub const fn fgcmar(&self) -> &FGCMAR {
        &self.fgcmar
    }
    ///0x30 - DMA2D background CLUT memory address register
    #[inline(always)]
    pub const fn bgcmar(&self) -> &BGCMAR {
        &self.bgcmar
    }
    ///0x34 - DMA2D output PFC control register
    #[inline(always)]
    pub const fn opfccr(&self) -> &OPFCCR {
        &self.opfccr
    }
    ///0x38 - DMA2D output color register
    #[inline(always)]
    pub const fn ocolr(&self) -> &OCOLR {
        &self.ocolr
    }
    ///0x3c - DMA2D output memory address register
    #[inline(always)]
    pub const fn omar(&self) -> &OMAR {
        &self.omar
    }
    ///0x40 - DMA2D output offset register
    #[inline(always)]
    pub const fn oor(&self) -> &OOR {
        &self.oor
    }
    ///0x44 - DMA2D number of line register
    #[inline(always)]
    pub const fn nlr(&self) -> &NLR {
        &self.nlr
    }
    ///0x48 - DMA2D line watermark register
    #[inline(always)]
    pub const fn lwr(&self) -> &LWR {
        &self.lwr
    }
    ///0x4c - DMA2D AXI master timer configuration register
    #[inline(always)]
    pub const fn amtcr(&self) -> &AMTCR {
        &self.amtcr
    }
}
/**CR (rw) register accessor: DMA2D control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DMA2D control register
pub mod cr;
/**ISR (r) register accessor: DMA2D Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DMA2D Interrupt Status Register
pub mod isr;
/**IFCR (rw) register accessor: DMA2D interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///DMA2D interrupt flag clear register
pub mod ifcr;
/**FGMAR (rw) register accessor: DMA2D foreground memory address register

You can [`read`](crate::Reg::read) this register and get [`fgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:FGMAR)

For information about available fields see [`mod@fgmar`] module*/
pub type FGMAR = crate::Reg<fgmar::FGMARrs>;
///DMA2D foreground memory address register
pub mod fgmar;
/**FGOR (rw) register accessor: DMA2D foreground offset register

You can [`read`](crate::Reg::read) this register and get [`fgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:FGOR)

For information about available fields see [`mod@fgor`] module*/
pub type FGOR = crate::Reg<fgor::FGORrs>;
///DMA2D foreground offset register
pub mod fgor;
/**BGMAR (rw) register accessor: DMA2D background memory address register

You can [`read`](crate::Reg::read) this register and get [`bgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:BGMAR)

For information about available fields see [`mod@bgmar`] module*/
pub type BGMAR = crate::Reg<bgmar::BGMARrs>;
///DMA2D background memory address register
pub mod bgmar;
/**BGOR (rw) register accessor: DMA2D background offset register

You can [`read`](crate::Reg::read) this register and get [`bgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:BGOR)

For information about available fields see [`mod@bgor`] module*/
pub type BGOR = crate::Reg<bgor::BGORrs>;
///DMA2D background offset register
pub mod bgor;
/**FGPFCCR (rw) register accessor: DMA2D foreground PFC control register

You can [`read`](crate::Reg::read) this register and get [`fgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:FGPFCCR)

For information about available fields see [`mod@fgpfccr`] module*/
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCRrs>;
///DMA2D foreground PFC control register
pub mod fgpfccr;
/**FGCOLR (rw) register accessor: DMA2D foreground color register

You can [`read`](crate::Reg::read) this register and get [`fgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:FGCOLR)

For information about available fields see [`mod@fgcolr`] module*/
pub type FGCOLR = crate::Reg<fgcolr::FGCOLRrs>;
///DMA2D foreground color register
pub mod fgcolr;
/**BGPFCCR (rw) register accessor: DMA2D background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:BGPFCCR)

For information about available fields see [`mod@bgpfccr`] module*/
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCRrs>;
///DMA2D background PFC control register
pub mod bgpfccr;
/**BGCOLR (rw) register accessor: DMA2D background color register

You can [`read`](crate::Reg::read) this register and get [`bgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:BGCOLR)

For information about available fields see [`mod@bgcolr`] module*/
pub type BGCOLR = crate::Reg<bgcolr::BGCOLRrs>;
///DMA2D background color register
pub mod bgcolr;
/**FGCMAR (rw) register accessor: DMA2D foreground CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`fgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:FGCMAR)

For information about available fields see [`mod@fgcmar`] module*/
pub type FGCMAR = crate::Reg<fgcmar::FGCMARrs>;
///DMA2D foreground CLUT memory address register
pub mod fgcmar;
/**BGCMAR (rw) register accessor: DMA2D background CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`bgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:BGCMAR)

For information about available fields see [`mod@bgcmar`] module*/
pub type BGCMAR = crate::Reg<bgcmar::BGCMARrs>;
///DMA2D background CLUT memory address register
pub mod bgcmar;
/**OPFCCR (rw) register accessor: DMA2D output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:OPFCCR)

For information about available fields see [`mod@opfccr`] module*/
pub type OPFCCR = crate::Reg<opfccr::OPFCCRrs>;
///DMA2D output PFC control register
pub mod opfccr;
/**OCOLR (rw) register accessor: DMA2D output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:OCOLR)

For information about available fields see [`mod@ocolr`] module*/
pub type OCOLR = crate::Reg<ocolr::OCOLRrs>;
///DMA2D output color register
pub mod ocolr;
/**OMAR (rw) register accessor: DMA2D output memory address register

You can [`read`](crate::Reg::read) this register and get [`omar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:OMAR)

For information about available fields see [`mod@omar`] module*/
pub type OMAR = crate::Reg<omar::OMARrs>;
///DMA2D output memory address register
pub mod omar;
/**OOR (rw) register accessor: DMA2D output offset register

You can [`read`](crate::Reg::read) this register and get [`oor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:OOR)

For information about available fields see [`mod@oor`] module*/
pub type OOR = crate::Reg<oor::OORrs>;
///DMA2D output offset register
pub mod oor;
/**NLR (rw) register accessor: DMA2D number of line register

You can [`read`](crate::Reg::read) this register and get [`nlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:NLR)

For information about available fields see [`mod@nlr`] module*/
pub type NLR = crate::Reg<nlr::NLRrs>;
///DMA2D number of line register
pub mod nlr;
/**LWR (rw) register accessor: DMA2D line watermark register

You can [`read`](crate::Reg::read) this register and get [`lwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:LWR)

For information about available fields see [`mod@lwr`] module*/
pub type LWR = crate::Reg<lwr::LWRrs>;
///DMA2D line watermark register
pub mod lwr;
/**AMTCR (rw) register accessor: DMA2D AXI master timer configuration register

You can [`read`](crate::Reg::read) this register and get [`amtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA2D:AMTCR)

For information about available fields see [`mod@amtcr`] module*/
pub type AMTCR = crate::Reg<amtcr::AMTCRrs>;
///DMA2D AXI master timer configuration register
pub mod amtcr;
