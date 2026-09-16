#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    epr: (),
    _reserved1: [u8; 0x40],
    cntr: CNTR,
    _reserved2: [u8; 0x02],
    istr: ISTR,
    _reserved3: [u8; 0x02],
    fnr: FNR,
    _reserved4: [u8; 0x02],
    daddr: DADDR,
    _reserved5: [u8; 0x02],
    btable: BTABLE,
    _reserved6: [u8; 0x02],
    lpmcsr: LPMCSR,
    _reserved7: [u8; 0x02],
    bcdr: BCDR,
}
impl RegisterBlock {
    ///0x00..0x10 - endpoint %s register
    #[inline(always)]
    pub const fn epr(&self, n: usize) -> &EPR {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x10 - endpoint %s register
    #[inline(always)]
    pub fn epr_iter(&self) -> impl Iterator<Item = &EPR> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() })
    }
    ///0x00 - endpoint 0 register
    #[inline(always)]
    pub const fn ep0r(&self) -> &EPR {
        self.epr(0)
    }
    ///0x04 - endpoint 1 register
    #[inline(always)]
    pub const fn ep1r(&self) -> &EPR {
        self.epr(1)
    }
    ///0x08 - endpoint 2 register
    #[inline(always)]
    pub const fn ep2r(&self) -> &EPR {
        self.epr(2)
    }
    ///0x0c - endpoint 3 register
    #[inline(always)]
    pub const fn ep3r(&self) -> &EPR {
        self.epr(3)
    }
    ///0x10 - endpoint 4 register
    #[inline(always)]
    pub const fn ep4r(&self) -> &EPR {
        self.epr(4)
    }
    ///0x14 - endpoint 5 register
    #[inline(always)]
    pub const fn ep5r(&self) -> &EPR {
        self.epr(5)
    }
    ///0x18 - endpoint 6 register
    #[inline(always)]
    pub const fn ep6r(&self) -> &EPR {
        self.epr(6)
    }
    ///0x1c - endpoint 7 register
    #[inline(always)]
    pub const fn ep7r(&self) -> &EPR {
        self.epr(7)
    }
    ///0x40 - control register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x44 - interrupt status register
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    ///0x48 - frame number register
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    ///0x4c - device address
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    ///0x50 - Buffer table address
    #[inline(always)]
    pub const fn btable(&self) -> &BTABLE {
        &self.btable
    }
    ///0x54 - LPM control and status register
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        &self.lpmcsr
    }
    ///0x58 - Battery charging detector
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
}
/**EPR (rw) register accessor: endpoint %s register

You can [`read`](crate::Reg::read) this register and get [`epr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:EP[0]R)

For information about available fields see [`mod@epr`] module*/
pub type EPR = crate::Reg<epr::EPRrs>;
///endpoint %s register
pub mod epr;
/**CNTR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:CNTR)

For information about available fields see [`mod@cntr`] module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///control register
pub mod cntr;
/**ISTR (rw) register accessor: interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:ISTR)

For information about available fields see [`mod@istr`] module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///interrupt status register
pub mod istr;
/**FNR (r) register accessor: frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:FNR)

For information about available fields see [`mod@fnr`] module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///frame number register
pub mod fnr;
/**DADDR (rw) register accessor: device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:DADDR)

For information about available fields see [`mod@daddr`] module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///device address
pub mod daddr;
/**BTABLE (rw) register accessor: Buffer table address

You can [`read`](crate::Reg::read) this register and get [`btable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:BTABLE)

For information about available fields see [`mod@btable`] module*/
pub type BTABLE = crate::Reg<btable::BTABLErs>;
///Buffer table address
pub mod btable;
/**LPMCSR (rw) register accessor: LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:LPMCSR)

For information about available fields see [`mod@lpmcsr`] module*/
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
///LPM control and status register
pub mod lpmcsr;
/**BCDR (rw) register accessor: Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#USB:BCDR)

For information about available fields see [`mod@bcdr`] module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery charging detector
pub mod bcdr;
