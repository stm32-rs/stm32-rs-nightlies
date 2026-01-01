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
    count0_tx: COUNT0_TX,
    _reserved_7_lpmcsr: [u8; 0x02],
    count0_rx: COUNT0_RX,
    bcdr: BCDR,
    count1_tx: COUNT1_TX,
    addr1_rx: ADDR1_RX,
    count1_rx: COUNT1_RX,
    _reserved13: [u8; 0x02],
    count2_tx: COUNT2_TX,
    addr2_rx: ADDR2_RX,
    count2_rx: COUNT2_RX,
    _reserved16: [u8; 0x02],
    count3_tx: COUNT3_TX,
    addr3_rx: ADDR3_RX,
    count3_rx: COUNT3_RX,
    _reserved19: [u8; 0x02],
    count4_tx: COUNT4_TX,
    addr4_rx: ADDR4_RX,
    count4_rx: COUNT4_RX,
    _reserved22: [u8; 0x02],
    count5_tx: COUNT5_TX,
    addr5_rx: ADDR5_RX,
    count5_rx: COUNT5_RX,
    _reserved25: [u8; 0x02],
    count6_tx: COUNT6_TX,
    addr6_rx: ADDR6_RX,
    count6_rx: COUNT6_RX,
    _reserved28: [u8; 0x02],
    count7_tx: COUNT7_TX,
    addr7_rx: ADDR7_RX,
    count7_rx: COUNT7_RX,
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
    ///0x52 - Transmission byte count 0
    #[inline(always)]
    pub const fn count0_tx(&self) -> &COUNT0_TX {
        &self.count0_tx
    }
    ///0x54 - control and status register
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    ///0x54 - Reception buffer address 0
    #[inline(always)]
    pub const fn addr0_rx(&self) -> &ADDR0_RX {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    ///0x56 - Reception byte count 0
    #[inline(always)]
    pub const fn count0_rx(&self) -> &COUNT0_RX {
        &self.count0_rx
    }
    ///0x58 - Battery charging detector(
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
    ///0x5a - Transmission byte count 0
    #[inline(always)]
    pub const fn count1_tx(&self) -> &COUNT1_TX {
        &self.count1_tx
    }
    ///0x5c - Reception buffer address 0
    #[inline(always)]
    pub const fn addr1_rx(&self) -> &ADDR1_RX {
        &self.addr1_rx
    }
    ///0x5e - Reception byte count 0
    #[inline(always)]
    pub const fn count1_rx(&self) -> &COUNT1_RX {
        &self.count1_rx
    }
    ///0x62 - Transmission byte count 0
    #[inline(always)]
    pub const fn count2_tx(&self) -> &COUNT2_TX {
        &self.count2_tx
    }
    ///0x64 - Reception buffer address 0
    #[inline(always)]
    pub const fn addr2_rx(&self) -> &ADDR2_RX {
        &self.addr2_rx
    }
    ///0x66 - Reception byte count 0
    #[inline(always)]
    pub const fn count2_rx(&self) -> &COUNT2_RX {
        &self.count2_rx
    }
    ///0x6a - Transmission byte count 0
    #[inline(always)]
    pub const fn count3_tx(&self) -> &COUNT3_TX {
        &self.count3_tx
    }
    ///0x6c - Reception buffer address 0
    #[inline(always)]
    pub const fn addr3_rx(&self) -> &ADDR3_RX {
        &self.addr3_rx
    }
    ///0x6e - Reception byte count 0
    #[inline(always)]
    pub const fn count3_rx(&self) -> &COUNT3_RX {
        &self.count3_rx
    }
    ///0x72 - Transmission byte count 0
    #[inline(always)]
    pub const fn count4_tx(&self) -> &COUNT4_TX {
        &self.count4_tx
    }
    ///0x74 - Reception buffer address 0
    #[inline(always)]
    pub const fn addr4_rx(&self) -> &ADDR4_RX {
        &self.addr4_rx
    }
    ///0x76 - Reception byte count 0
    #[inline(always)]
    pub const fn count4_rx(&self) -> &COUNT4_RX {
        &self.count4_rx
    }
    ///0x7a - Transmission byte count 0
    #[inline(always)]
    pub const fn count5_tx(&self) -> &COUNT5_TX {
        &self.count5_tx
    }
    ///0x7c - Reception buffer address 0
    #[inline(always)]
    pub const fn addr5_rx(&self) -> &ADDR5_RX {
        &self.addr5_rx
    }
    ///0x7e - Reception byte count 0
    #[inline(always)]
    pub const fn count5_rx(&self) -> &COUNT5_RX {
        &self.count5_rx
    }
    ///0x82 - Transmission byte count 0
    #[inline(always)]
    pub const fn count6_tx(&self) -> &COUNT6_TX {
        &self.count6_tx
    }
    ///0x84 - Reception buffer address 0
    #[inline(always)]
    pub const fn addr6_rx(&self) -> &ADDR6_RX {
        &self.addr6_rx
    }
    ///0x86 - Reception byte count 0
    #[inline(always)]
    pub const fn count6_rx(&self) -> &COUNT6_RX {
        &self.count6_rx
    }
    ///0x8a - Transmission byte count 0
    #[inline(always)]
    pub const fn count7_tx(&self) -> &COUNT7_TX {
        &self.count7_tx
    }
    ///0x8c - Reception buffer address 0
    #[inline(always)]
    pub const fn addr7_rx(&self) -> &ADDR7_RX {
        &self.addr7_rx
    }
    ///0x8e - Reception byte count 0
    #[inline(always)]
    pub const fn count7_rx(&self) -> &COUNT7_RX {
        &self.count7_rx
    }
}
/**EPR (rw) register accessor: endpoint %s register

You can [`read`](crate::Reg::read) this register and get [`epr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:EP[0]R)

For information about available fields see [`mod@epr`] module*/
pub type EPR = crate::Reg<epr::EPRrs>;
///endpoint %s register
pub mod epr;
/**CNTR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:CNTR)

For information about available fields see [`mod@cntr`] module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///control register
pub mod cntr;
/**ISTR (rw) register accessor: interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ISTR)

For information about available fields see [`mod@istr`] module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///interrupt status register
pub mod istr;
/**FNR (r) register accessor: frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:FNR)

For information about available fields see [`mod@fnr`] module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///frame number register
pub mod fnr;
/**DADDR (rw) register accessor: device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:DADDR)

For information about available fields see [`mod@daddr`] module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///device address
pub mod daddr;
/**BTABLE (rw) register accessor: Buffer table address

You can [`read`](crate::Reg::read) this register and get [`btable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:BTABLE)

For information about available fields see [`mod@btable`] module*/
pub type BTABLE = crate::Reg<btable::BTABLErs>;
///Buffer table address
pub mod btable;
/**COUNT0_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count0_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT0_TX)

For information about available fields see [`mod@count0_tx`] module*/
pub type COUNT0_TX = crate::Reg<count0_tx::COUNT0_TXrs>;
///Transmission byte count 0
pub mod count0_tx;
/**COUNT1_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count1_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count1_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT1_TX)

For information about available fields see [`mod@count1_tx`] module*/
pub type COUNT1_TX = crate::Reg<count1_tx::COUNT1_TXrs>;
///Transmission byte count 0
pub mod count1_tx;
/**COUNT2_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count2_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count2_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT2_TX)

For information about available fields see [`mod@count2_tx`] module*/
pub type COUNT2_TX = crate::Reg<count2_tx::COUNT2_TXrs>;
///Transmission byte count 0
pub mod count2_tx;
/**COUNT3_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count3_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count3_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT3_TX)

For information about available fields see [`mod@count3_tx`] module*/
pub type COUNT3_TX = crate::Reg<count3_tx::COUNT3_TXrs>;
///Transmission byte count 0
pub mod count3_tx;
/**COUNT4_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count4_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count4_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT4_TX)

For information about available fields see [`mod@count4_tx`] module*/
pub type COUNT4_TX = crate::Reg<count4_tx::COUNT4_TXrs>;
///Transmission byte count 0
pub mod count4_tx;
/**COUNT5_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count5_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count5_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT5_TX)

For information about available fields see [`mod@count5_tx`] module*/
pub type COUNT5_TX = crate::Reg<count5_tx::COUNT5_TXrs>;
///Transmission byte count 0
pub mod count5_tx;
/**COUNT6_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count6_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count6_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT6_TX)

For information about available fields see [`mod@count6_tx`] module*/
pub type COUNT6_TX = crate::Reg<count6_tx::COUNT6_TXrs>;
///Transmission byte count 0
pub mod count6_tx;
/**COUNT7_TX (rw) register accessor: Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count7_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count7_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT7_TX)

For information about available fields see [`mod@count7_tx`] module*/
pub type COUNT7_TX = crate::Reg<count7_tx::COUNT7_TXrs>;
///Transmission byte count 0
pub mod count7_tx;
/**ADDR0_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr0_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR0_RX)

For information about available fields see [`mod@addr0_rx`] module*/
pub type ADDR0_RX = crate::Reg<addr0_rx::ADDR0_RXrs>;
///Reception buffer address 0
pub mod addr0_rx;
/**ADDR1_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr1_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR1_RX)

For information about available fields see [`mod@addr1_rx`] module*/
pub type ADDR1_RX = crate::Reg<addr1_rx::ADDR1_RXrs>;
///Reception buffer address 0
pub mod addr1_rx;
/**ADDR2_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr2_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR2_RX)

For information about available fields see [`mod@addr2_rx`] module*/
pub type ADDR2_RX = crate::Reg<addr2_rx::ADDR2_RXrs>;
///Reception buffer address 0
pub mod addr2_rx;
/**ADDR3_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr3_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR3_RX)

For information about available fields see [`mod@addr3_rx`] module*/
pub type ADDR3_RX = crate::Reg<addr3_rx::ADDR3_RXrs>;
///Reception buffer address 0
pub mod addr3_rx;
/**ADDR4_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr4_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr4_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR4_RX)

For information about available fields see [`mod@addr4_rx`] module*/
pub type ADDR4_RX = crate::Reg<addr4_rx::ADDR4_RXrs>;
///Reception buffer address 0
pub mod addr4_rx;
/**ADDR5_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr5_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr5_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR5_RX)

For information about available fields see [`mod@addr5_rx`] module*/
pub type ADDR5_RX = crate::Reg<addr5_rx::ADDR5_RXrs>;
///Reception buffer address 0
pub mod addr5_rx;
/**ADDR6_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr6_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr6_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR6_RX)

For information about available fields see [`mod@addr6_rx`] module*/
pub type ADDR6_RX = crate::Reg<addr6_rx::ADDR6_RXrs>;
///Reception buffer address 0
pub mod addr6_rx;
/**ADDR7_RX (rw) register accessor: Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr7_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR7_RX)

For information about available fields see [`mod@addr7_rx`] module*/
pub type ADDR7_RX = crate::Reg<addr7_rx::ADDR7_RXrs>;
///Reception buffer address 0
pub mod addr7_rx;
/**COUNT0_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count0_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT0_RX)

For information about available fields see [`mod@count0_rx`] module*/
pub type COUNT0_RX = crate::Reg<count0_rx::COUNT0_RXrs>;
///Reception byte count 0
pub mod count0_rx;
/**COUNT1_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count1_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count1_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT1_RX)

For information about available fields see [`mod@count1_rx`] module*/
pub type COUNT1_RX = crate::Reg<count1_rx::COUNT1_RXrs>;
///Reception byte count 0
pub mod count1_rx;
/**COUNT2_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count2_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count2_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT2_RX)

For information about available fields see [`mod@count2_rx`] module*/
pub type COUNT2_RX = crate::Reg<count2_rx::COUNT2_RXrs>;
///Reception byte count 0
pub mod count2_rx;
/**COUNT3_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count3_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count3_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT3_RX)

For information about available fields see [`mod@count3_rx`] module*/
pub type COUNT3_RX = crate::Reg<count3_rx::COUNT3_RXrs>;
///Reception byte count 0
pub mod count3_rx;
/**COUNT4_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count4_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count4_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT4_RX)

For information about available fields see [`mod@count4_rx`] module*/
pub type COUNT4_RX = crate::Reg<count4_rx::COUNT4_RXrs>;
///Reception byte count 0
pub mod count4_rx;
/**COUNT5_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count5_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count5_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT5_RX)

For information about available fields see [`mod@count5_rx`] module*/
pub type COUNT5_RX = crate::Reg<count5_rx::COUNT5_RXrs>;
///Reception byte count 0
pub mod count5_rx;
/**COUNT6_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count6_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count6_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT6_RX)

For information about available fields see [`mod@count6_rx`] module*/
pub type COUNT6_RX = crate::Reg<count6_rx::COUNT6_RXrs>;
///Reception byte count 0
pub mod count6_rx;
/**COUNT7_RX (rw) register accessor: Reception byte count 0

You can [`read`](crate::Reg::read) this register and get [`count7_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count7_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT7_RX)

For information about available fields see [`mod@count7_rx`] module*/
pub type COUNT7_RX = crate::Reg<count7_rx::COUNT7_RXrs>;
///Reception byte count 0
pub mod count7_rx;
/**LPMCSR (rw) register accessor: control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:LPMCSR)

For information about available fields see [`mod@lpmcsr`] module*/
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
///control and status register
pub mod lpmcsr;
/**BCDR (rw) register accessor: Battery charging detector(

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:BCDR)

For information about available fields see [`mod@bcdr`] module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery charging detector(
pub mod bcdr;
