#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    epr: [EPR; 8],
    _reserved1: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    btable: BTABLE,
    _reserved6: [u8; 0x04],
    bcdr: BCDR,
}
impl RegisterBlock {
    ///0x00..0x20 - USB endpoint n register
    #[inline(always)]
    pub const fn epr(&self, n: usize) -> &EPR {
        &self.epr[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - USB endpoint n register
    #[inline(always)]
    pub fn epr_iter(&self) -> impl Iterator<Item = &EPR> {
        self.epr.iter()
    }
    ///0x00 - USB endpoint n register
    #[inline(always)]
    pub const fn ep0r(&self) -> &EPR {
        self.epr(0)
    }
    ///0x04 - USB endpoint n register
    #[inline(always)]
    pub const fn ep1r(&self) -> &EPR {
        self.epr(1)
    }
    ///0x08 - USB endpoint n register
    #[inline(always)]
    pub const fn ep2r(&self) -> &EPR {
        self.epr(2)
    }
    ///0x0c - USB endpoint n register
    #[inline(always)]
    pub const fn ep3r(&self) -> &EPR {
        self.epr(3)
    }
    ///0x10 - USB endpoint n register
    #[inline(always)]
    pub const fn ep4r(&self) -> &EPR {
        self.epr(4)
    }
    ///0x14 - USB endpoint n register
    #[inline(always)]
    pub const fn ep5r(&self) -> &EPR {
        self.epr(5)
    }
    ///0x18 - USB endpoint n register
    #[inline(always)]
    pub const fn ep6r(&self) -> &EPR {
        self.epr(6)
    }
    ///0x1c - USB endpoint n register
    #[inline(always)]
    pub const fn ep7r(&self) -> &EPR {
        self.epr(7)
    }
    ///0x40 - USB control register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x44 - USB interrupt status register
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    ///0x48 - USB frame number register
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    ///0x4c - USB device address
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    ///0x50 - Buffer table address
    #[inline(always)]
    pub const fn btable(&self) -> &BTABLE {
        &self.btable
    }
    ///0x58 - Battery Charging Detector
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
}
/**EPR (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`epr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:EP[0]R)

For information about available fields see [`mod@epr`] module*/
pub type EPR = crate::Reg<epr::EPRrs>;
///USB endpoint n register
pub mod epr;
/**CNTR (rw) register accessor: USB control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:CNTR)

For information about available fields see [`mod@cntr`] module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///USB control register
pub mod cntr;
/**ISTR (rw) register accessor: USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:ISTR)

For information about available fields see [`mod@istr`] module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///USB interrupt status register
pub mod istr;
/**FNR (r) register accessor: USB frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:FNR)

For information about available fields see [`mod@fnr`] module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///USB frame number register
pub mod fnr;
/**DADDR (rw) register accessor: USB device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:DADDR)

For information about available fields see [`mod@daddr`] module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///USB device address
pub mod daddr;
/**BTABLE (rw) register accessor: Buffer table address

You can [`read`](crate::Reg::read) this register and get [`btable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:BTABLE)

For information about available fields see [`mod@btable`] module*/
pub type BTABLE = crate::Reg<btable::BTABLErs>;
///Buffer table address
pub mod btable;
/**BCDR (rw) register accessor: Battery Charging Detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#USB:BCDR)

For information about available fields see [`mod@bcdr`] module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery Charging Detector
pub mod bcdr;
