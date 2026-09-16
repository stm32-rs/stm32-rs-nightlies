#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mcr: MCR,
    msr: MSR,
    tsr: TSR,
    rfr: [RFR; 2],
    ier: IER,
    esr: ESR,
    btr: BTR,
    _reserved7: [u8; 0x0160],
    tx: [TX; 3],
    rx: [RX; 2],
    _reserved9: [u8; 0x30],
    fmr: FMR,
    fm1r: FM1R,
    _reserved11: [u8; 0x04],
    fs1r: FS1R,
    _reserved12: [u8; 0x04],
    ffa1r: FFA1R,
    _reserved13: [u8; 0x04],
    fa1r: FA1R,
    _reserved14: [u8; 0x20],
    fb: [FB; 28],
}
impl RegisterBlock {
    ///0x00 - CAN_MCR
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x04 - CAN_MSR
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    ///0x08 - CAN_TSR
    #[inline(always)]
    pub const fn tsr(&self) -> &TSR {
        &self.tsr
    }
    ///0x0c..0x14 - CAN_RF%sR
    #[inline(always)]
    pub const fn rfr(&self, n: usize) -> &RFR {
        &self.rfr[n]
    }
    ///Iterator for array of:
    ///0x0c..0x14 - CAN_RF%sR
    #[inline(always)]
    pub fn rfr_iter(&self) -> impl Iterator<Item = &RFR> {
        self.rfr.iter()
    }
    ///0x0c - CAN_RF0R
    #[inline(always)]
    pub const fn rf0r(&self) -> &RFR {
        self.rfr(0)
    }
    ///0x10 - CAN_RF1R
    #[inline(always)]
    pub const fn rf1r(&self) -> &RFR {
        self.rfr(1)
    }
    ///0x14 - CAN_IER
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x18 - CAN_ESR
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    ///0x1c - CAN_BTR
    #[inline(always)]
    pub const fn btr(&self) -> &BTR {
        &self.btr
    }
    ///0x180..0x1b0 - CAN Transmit cluster
    #[inline(always)]
    pub const fn tx(&self, n: usize) -> &TX {
        &self.tx[n]
    }
    ///Iterator for array of:
    ///0x180..0x1b0 - CAN Transmit cluster
    #[inline(always)]
    pub fn tx_iter(&self) -> impl Iterator<Item = &TX> {
        self.tx.iter()
    }
    ///0x1b0..0x1d0 - CAN Receive cluster
    #[inline(always)]
    pub const fn rx(&self, n: usize) -> &RX {
        &self.rx[n]
    }
    ///Iterator for array of:
    ///0x1b0..0x1d0 - CAN Receive cluster
    #[inline(always)]
    pub fn rx_iter(&self) -> impl Iterator<Item = &RX> {
        self.rx.iter()
    }
    ///0x200 - CAN_FMR
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    ///0x204 - CAN_FM1R
    #[inline(always)]
    pub const fn fm1r(&self) -> &FM1R {
        &self.fm1r
    }
    ///0x20c - CAN_FS1R
    #[inline(always)]
    pub const fn fs1r(&self) -> &FS1R {
        &self.fs1r
    }
    ///0x214 - CAN_FFA1R
    #[inline(always)]
    pub const fn ffa1r(&self) -> &FFA1R {
        &self.ffa1r
    }
    ///0x21c - CAN_FA1R
    #[inline(always)]
    pub const fn fa1r(&self) -> &FA1R {
        &self.fa1r
    }
    ///0x240..0x320 - CAN Filter Bank cluster
    #[inline(always)]
    pub const fn fb(&self, n: usize) -> &FB {
        &self.fb[n]
    }
    ///Iterator for array of:
    ///0x240..0x320 - CAN Filter Bank cluster
    #[inline(always)]
    pub fn fb_iter(&self) -> impl Iterator<Item = &FB> {
        self.fb.iter()
    }
}
/**MCR (rw) register accessor: CAN_MCR

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///CAN_MCR
pub mod mcr;
/**MSR (rw) register accessor: CAN_MSR

You can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:MSR)

For information about available fields see [`mod@msr`] module*/
pub type MSR = crate::Reg<msr::MSRrs>;
///CAN_MSR
pub mod msr;
/**TSR (rw) register accessor: CAN_TSR

You can [`read`](crate::Reg::read) this register and get [`tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:TSR)

For information about available fields see [`mod@tsr`] module*/
pub type TSR = crate::Reg<tsr::TSRrs>;
///CAN_TSR
pub mod tsr;
/**RFR (rw) register accessor: CAN_RF%sR

You can [`read`](crate::Reg::read) this register and get [`rfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:RF[0]R)

For information about available fields see [`mod@rfr`] module*/
pub type RFR = crate::Reg<rfr::RFRrs>;
///CAN_RF%sR
pub mod rfr;
/**IER (rw) register accessor: CAN_IER

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///CAN_IER
pub mod ier;
/**ESR (rw) register accessor: CAN_ESR

You can [`read`](crate::Reg::read) this register and get [`esr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:ESR)

For information about available fields see [`mod@esr`] module*/
pub type ESR = crate::Reg<esr::ESRrs>;
///CAN_ESR
pub mod esr;
/**BTR (rw) register accessor: CAN_BTR

You can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:BTR)

For information about available fields see [`mod@btr`] module*/
pub type BTR = crate::Reg<btr::BTRrs>;
///CAN_BTR
pub mod btr;
///CAN Transmit cluster
pub use self::tx::TX;
///Cluster
///CAN Transmit cluster
pub mod tx;
///CAN Receive cluster
pub use self::rx::RX;
///Cluster
///CAN Receive cluster
pub mod rx;
/**FMR (rw) register accessor: CAN_FMR

You can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:FMR)

For information about available fields see [`mod@fmr`] module*/
pub type FMR = crate::Reg<fmr::FMRrs>;
///CAN_FMR
pub mod fmr;
/**FM1R (rw) register accessor: CAN_FM1R

You can [`read`](crate::Reg::read) this register and get [`fm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:FM1R)

For information about available fields see [`mod@fm1r`] module*/
pub type FM1R = crate::Reg<fm1r::FM1Rrs>;
///CAN_FM1R
pub mod fm1r;
/**FS1R (rw) register accessor: CAN_FS1R

You can [`read`](crate::Reg::read) this register and get [`fs1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:FS1R)

For information about available fields see [`mod@fs1r`] module*/
pub type FS1R = crate::Reg<fs1r::FS1Rrs>;
///CAN_FS1R
pub mod fs1r;
/**FFA1R (rw) register accessor: CAN_FFA1R

You can [`read`](crate::Reg::read) this register and get [`ffa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:FFA1R)

For information about available fields see [`mod@ffa1r`] module*/
pub type FFA1R = crate::Reg<ffa1r::FFA1Rrs>;
///CAN_FFA1R
pub mod ffa1r;
/**FA1R (rw) register accessor: CAN_FA1R

You can [`read`](crate::Reg::read) this register and get [`fa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#CAN1:FA1R)

For information about available fields see [`mod@fa1r`] module*/
pub type FA1R = crate::Reg<fa1r::FA1Rrs>;
///CAN_FA1R
pub mod fa1r;
///CAN Filter Bank cluster
pub use self::fb::FB;
///Cluster
///CAN Filter Bank cluster
pub mod fb;
