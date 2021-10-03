#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: crate::Reg<ep0r::EP0R_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: crate::Reg<ep1r::EP1R_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: crate::Reg<ep2r::EP2R_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: crate::Reg<ep3r::EP3R_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: crate::Reg<ep4r::EP4R_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: crate::Reg<ep5r::EP5R_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: crate::Reg<ep6r::EP6R_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: crate::Reg<ep7r::EP7R_SPEC>,
    _reserved8: [u8; 0x22],
    #[doc = "0x40 - control register"]
    pub cntr: crate::Reg<cntr::CNTR_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x44 - interrupt status register"]
    pub istr: crate::Reg<istr::ISTR_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x48 - frame number register"]
    pub fnr: crate::Reg<fnr::FNR_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x4c - device address"]
    pub daddr: crate::Reg<daddr::DADDR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x50 - Buffer table address"]
    pub btable: crate::Reg<btable::BTABLE_SPEC>,
    #[doc = "0x52 - Transmission byte count 0"]
    pub count0_tx: crate::Reg<count0_tx::COUNT0_TX_SPEC>,
    _reserved_14_lpmcsr: [u8; 0x02],
    #[doc = "0x56 - Reception byte count 0"]
    pub count0_rx: crate::Reg<count0_rx::COUNT0_RX_SPEC>,
    #[doc = "0x58 - Battery charging detector("]
    pub bcdr: crate::Reg<bcdr::BCDR_SPEC>,
    #[doc = "0x5a - Transmission byte count 0"]
    pub count1_tx: crate::Reg<count1_tx::COUNT1_TX_SPEC>,
    #[doc = "0x5c - Reception buffer address 0"]
    pub addr1_rx: crate::Reg<addr1_rx::ADDR1_RX_SPEC>,
    #[doc = "0x5e - Reception byte count 0"]
    pub count1_rx: crate::Reg<count1_rx::COUNT1_RX_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x62 - Transmission byte count 0"]
    pub count2_tx: crate::Reg<count2_tx::COUNT2_TX_SPEC>,
    #[doc = "0x64 - Reception buffer address 0"]
    pub addr2_rx: crate::Reg<addr2_rx::ADDR2_RX_SPEC>,
    #[doc = "0x66 - Reception byte count 0"]
    pub count2_rx: crate::Reg<count2_rx::COUNT2_RX_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x6a - Transmission byte count 0"]
    pub count3_tx: crate::Reg<count3_tx::COUNT3_TX_SPEC>,
    #[doc = "0x6c - Reception buffer address 0"]
    pub addr3_rx: crate::Reg<addr3_rx::ADDR3_RX_SPEC>,
    #[doc = "0x6e - Reception byte count 0"]
    pub count3_rx: crate::Reg<count3_rx::COUNT3_RX_SPEC>,
    _reserved26: [u8; 0x02],
    #[doc = "0x72 - Transmission byte count 0"]
    pub count4_tx: crate::Reg<count4_tx::COUNT4_TX_SPEC>,
    #[doc = "0x74 - Reception buffer address 0"]
    pub addr4_rx: crate::Reg<addr4_rx::ADDR4_RX_SPEC>,
    #[doc = "0x76 - Reception byte count 0"]
    pub count4_rx: crate::Reg<count4_rx::COUNT4_RX_SPEC>,
    _reserved29: [u8; 0x02],
    #[doc = "0x7a - Transmission byte count 0"]
    pub count5_tx: crate::Reg<count5_tx::COUNT5_TX_SPEC>,
    #[doc = "0x7c - Reception buffer address 0"]
    pub addr5_rx: crate::Reg<addr5_rx::ADDR5_RX_SPEC>,
    #[doc = "0x7e - Reception byte count 0"]
    pub count5_rx: crate::Reg<count5_rx::COUNT5_RX_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x82 - Transmission byte count 0"]
    pub count6_tx: crate::Reg<count6_tx::COUNT6_TX_SPEC>,
    #[doc = "0x84 - Reception buffer address 0"]
    pub addr6_rx: crate::Reg<addr6_rx::ADDR6_RX_SPEC>,
    #[doc = "0x86 - Reception byte count 0"]
    pub count6_rx: crate::Reg<count6_rx::COUNT6_RX_SPEC>,
    _reserved35: [u8; 0x02],
    #[doc = "0x8a - Transmission byte count 0"]
    pub count7_tx: crate::Reg<count7_tx::COUNT7_TX_SPEC>,
    #[doc = "0x8c - Reception buffer address 0"]
    pub addr7_rx: crate::Reg<addr7_rx::ADDR7_RX_SPEC>,
    #[doc = "0x8e - Reception byte count 0"]
    pub count7_rx: crate::Reg<count7_rx::COUNT7_RX_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x54 - control and status register"]
    #[inline(always)]
    pub fn lpmcsr(&self) -> &crate::Reg<lpmcsr::LPMCSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<lpmcsr::LPMCSR_SPEC>)
        }
    }
    #[doc = "0x54 - Reception buffer address 0"]
    #[inline(always)]
    pub fn addr0_rx(&self) -> &crate::Reg<addr0_rx::ADDR0_RX_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<addr0_rx::ADDR0_RX_SPEC>)
        }
    }
}
#[doc = "EP0R register accessor: an alias for `Reg<EP0R_SPEC>`"]
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "EP1R register accessor: an alias for `Reg<EP1R_SPEC>`"]
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "EP2R register accessor: an alias for `Reg<EP2R_SPEC>`"]
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "EP3R register accessor: an alias for `Reg<EP3R_SPEC>`"]
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "EP4R register accessor: an alias for `Reg<EP4R_SPEC>`"]
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "EP5R register accessor: an alias for `Reg<EP5R_SPEC>`"]
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "EP6R register accessor: an alias for `Reg<EP6R_SPEC>`"]
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "EP7R register accessor: an alias for `Reg<EP7R_SPEC>`"]
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "CNTR register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "control register"]
pub mod cntr;
#[doc = "ISTR register accessor: an alias for `Reg<ISTR_SPEC>`"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR register accessor: an alias for `Reg<FNR_SPEC>`"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE register accessor: an alias for `Reg<BTABLE_SPEC>`"]
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "COUNT0_TX register accessor: an alias for `Reg<COUNT0_TX_SPEC>`"]
pub type COUNT0_TX = crate::Reg<count0_tx::COUNT0_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count0_tx;
#[doc = "COUNT1_TX register accessor: an alias for `Reg<COUNT1_TX_SPEC>`"]
pub type COUNT1_TX = crate::Reg<count1_tx::COUNT1_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count1_tx;
#[doc = "COUNT2_TX register accessor: an alias for `Reg<COUNT2_TX_SPEC>`"]
pub type COUNT2_TX = crate::Reg<count2_tx::COUNT2_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count2_tx;
#[doc = "COUNT3_TX register accessor: an alias for `Reg<COUNT3_TX_SPEC>`"]
pub type COUNT3_TX = crate::Reg<count3_tx::COUNT3_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count3_tx;
#[doc = "COUNT4_TX register accessor: an alias for `Reg<COUNT4_TX_SPEC>`"]
pub type COUNT4_TX = crate::Reg<count4_tx::COUNT4_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count4_tx;
#[doc = "COUNT5_TX register accessor: an alias for `Reg<COUNT5_TX_SPEC>`"]
pub type COUNT5_TX = crate::Reg<count5_tx::COUNT5_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count5_tx;
#[doc = "COUNT6_TX register accessor: an alias for `Reg<COUNT6_TX_SPEC>`"]
pub type COUNT6_TX = crate::Reg<count6_tx::COUNT6_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count6_tx;
#[doc = "COUNT7_TX register accessor: an alias for `Reg<COUNT7_TX_SPEC>`"]
pub type COUNT7_TX = crate::Reg<count7_tx::COUNT7_TX_SPEC>;
#[doc = "Transmission byte count 0"]
pub mod count7_tx;
#[doc = "ADDR0_RX register accessor: an alias for `Reg<ADDR0_RX_SPEC>`"]
pub type ADDR0_RX = crate::Reg<addr0_rx::ADDR0_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr0_rx;
#[doc = "ADDR1_RX register accessor: an alias for `Reg<ADDR1_RX_SPEC>`"]
pub type ADDR1_RX = crate::Reg<addr1_rx::ADDR1_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr1_rx;
#[doc = "ADDR2_RX register accessor: an alias for `Reg<ADDR2_RX_SPEC>`"]
pub type ADDR2_RX = crate::Reg<addr2_rx::ADDR2_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr2_rx;
#[doc = "ADDR3_RX register accessor: an alias for `Reg<ADDR3_RX_SPEC>`"]
pub type ADDR3_RX = crate::Reg<addr3_rx::ADDR3_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr3_rx;
#[doc = "ADDR4_RX register accessor: an alias for `Reg<ADDR4_RX_SPEC>`"]
pub type ADDR4_RX = crate::Reg<addr4_rx::ADDR4_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr4_rx;
#[doc = "ADDR5_RX register accessor: an alias for `Reg<ADDR5_RX_SPEC>`"]
pub type ADDR5_RX = crate::Reg<addr5_rx::ADDR5_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr5_rx;
#[doc = "ADDR6_RX register accessor: an alias for `Reg<ADDR6_RX_SPEC>`"]
pub type ADDR6_RX = crate::Reg<addr6_rx::ADDR6_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr6_rx;
#[doc = "ADDR7_RX register accessor: an alias for `Reg<ADDR7_RX_SPEC>`"]
pub type ADDR7_RX = crate::Reg<addr7_rx::ADDR7_RX_SPEC>;
#[doc = "Reception buffer address 0"]
pub mod addr7_rx;
#[doc = "COUNT0_RX register accessor: an alias for `Reg<COUNT0_RX_SPEC>`"]
pub type COUNT0_RX = crate::Reg<count0_rx::COUNT0_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count0_rx;
#[doc = "COUNT1_RX register accessor: an alias for `Reg<COUNT1_RX_SPEC>`"]
pub type COUNT1_RX = crate::Reg<count1_rx::COUNT1_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count1_rx;
#[doc = "COUNT2_RX register accessor: an alias for `Reg<COUNT2_RX_SPEC>`"]
pub type COUNT2_RX = crate::Reg<count2_rx::COUNT2_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count2_rx;
#[doc = "COUNT3_RX register accessor: an alias for `Reg<COUNT3_RX_SPEC>`"]
pub type COUNT3_RX = crate::Reg<count3_rx::COUNT3_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count3_rx;
#[doc = "COUNT4_RX register accessor: an alias for `Reg<COUNT4_RX_SPEC>`"]
pub type COUNT4_RX = crate::Reg<count4_rx::COUNT4_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count4_rx;
#[doc = "COUNT5_RX register accessor: an alias for `Reg<COUNT5_RX_SPEC>`"]
pub type COUNT5_RX = crate::Reg<count5_rx::COUNT5_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count5_rx;
#[doc = "COUNT6_RX register accessor: an alias for `Reg<COUNT6_RX_SPEC>`"]
pub type COUNT6_RX = crate::Reg<count6_rx::COUNT6_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count6_rx;
#[doc = "COUNT7_RX register accessor: an alias for `Reg<COUNT7_RX_SPEC>`"]
pub type COUNT7_RX = crate::Reg<count7_rx::COUNT7_RX_SPEC>;
#[doc = "Reception byte count 0"]
pub mod count7_rx;
#[doc = "LPMCSR register accessor: an alias for `Reg<LPMCSR_SPEC>`"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
#[doc = "control and status register"]
pub mod lpmcsr;
#[doc = "BCDR register accessor: an alias for `Reg<BCDR_SPEC>`"]
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
#[doc = "Battery charging detector("]
pub mod bcdr;
