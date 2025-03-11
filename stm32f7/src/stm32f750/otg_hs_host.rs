#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    hprt: HPRT,
    _reserved7: [u8; 0xbc],
    hc: [HC; 16],
}
impl RegisterBlock {
    ///0x00 - OTG_HS host configuration register
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    ///0x04 - OTG_HS Host frame interval register
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    ///0x08 - OTG_HS host frame number/frame time remaining register
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    ///0x10 - OTG_HS_Host periodic transmit FIFO/queue status register
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    ///0x14 - OTG_HS Host all channels interrupt register
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    ///0x18 - OTG_HS host all channels interrupt mask register
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    ///0x40 - OTG_HS host port control and status register
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    ///0x100..0x300 - Host channel
    #[inline(always)]
    pub const fn hc(&self, n: usize) -> &HC {
        &self.hc[n]
    }
    ///Iterator for array of:
    ///0x100..0x300 - Host channel
    #[inline(always)]
    pub fn hc_iter(&self) -> impl Iterator<Item = &HC> {
        self.hc.iter()
    }
}
/**HCFG (rw) register accessor: OTG_HS host configuration register

You can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HCFG)

For information about available fields see [`mod@hcfg`] module*/
pub type HCFG = crate::Reg<hcfg::HCFGrs>;
///OTG_HS host configuration register
pub mod hcfg;
/**HFIR (rw) register accessor: OTG_HS Host frame interval register

You can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HFIR)

For information about available fields see [`mod@hfir`] module*/
pub type HFIR = crate::Reg<hfir::HFIRrs>;
///OTG_HS Host frame interval register
pub mod hfir;
/**HFNUM (r) register accessor: OTG_HS host frame number/frame time remaining register

You can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HFNUM)

For information about available fields see [`mod@hfnum`] module*/
pub type HFNUM = crate::Reg<hfnum::HFNUMrs>;
///OTG_HS host frame number/frame time remaining register
pub mod hfnum;
/**HPTXSTS (rw) register accessor: OTG_HS_Host periodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HPTXSTS)

For information about available fields see [`mod@hptxsts`] module*/
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTSrs>;
///OTG_HS_Host periodic transmit FIFO/queue status register
pub mod hptxsts;
/**HAINT (r) register accessor: OTG_HS Host all channels interrupt register

You can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HAINT)

For information about available fields see [`mod@haint`] module*/
pub type HAINT = crate::Reg<haint::HAINTrs>;
///OTG_HS Host all channels interrupt register
pub mod haint;
/**HAINTMSK (rw) register accessor: OTG_HS host all channels interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HAINTMSK)

For information about available fields see [`mod@haintmsk`] module*/
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSKrs>;
///OTG_HS host all channels interrupt mask register
pub mod haintmsk;
/**HPRT (rw) register accessor: OTG_HS host port control and status register

You can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#OTG_HS_HOST:HPRT)

For information about available fields see [`mod@hprt`] module*/
pub type HPRT = crate::Reg<hprt::HPRTrs>;
///OTG_HS host port control and status register
pub mod hprt;
///Host channel
pub use self::hc::HC;
///Cluster
///Host channel
pub mod hc;
