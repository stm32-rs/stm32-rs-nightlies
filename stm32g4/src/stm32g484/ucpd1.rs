#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x04 - UCPD configuration register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - UCPD configuration register 2"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x10 - UCPD Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x14 - UCPD Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x18 - UCPD Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x1c - UCPD Tx Ordered Set Type Register"]
    pub tx_ordset: crate::Reg<tx_ordset::TX_ORDSET_SPEC>,
    #[doc = "0x20 - UCPD Tx Paysize Register"]
    pub tx_paysz: crate::Reg<tx_paysz::TX_PAYSZ_SPEC>,
    #[doc = "0x24 - UCPD Tx Data Register"]
    pub txdr: crate::Reg<txdr::TXDR_SPEC>,
    #[doc = "0x28 - UCPD Rx Ordered Set Register"]
    pub rx_ordset: crate::Reg<rx_ordset::RX_ORDSET_SPEC>,
    #[doc = "0x2c - UCPD Rx Paysize Register"]
    pub rx_paysz: crate::Reg<rx_paysz::RX_PAYSZ_SPEC>,
    #[doc = "0x30 - UCPD Rx Data Register"]
    pub rxdr: crate::Reg<rxdr::RXDR_SPEC>,
    #[doc = "0x34 - UCPD Rx Ordered Set Extension Register 1"]
    pub rx_ordext1: crate::Reg<rx_ordext1::RX_ORDEXT1_SPEC>,
    #[doc = "0x38 - UCPD Rx Ordered Set Extension Register 2"]
    pub rx_ordext2: crate::Reg<rx_ordext2::RX_ORDEXT2_SPEC>,
}
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "UCPD configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "UCPD configuration register 2"]
pub mod cfg2;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "UCPD configuration register 2"]
pub mod cr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "UCPD Interrupt Mask Register"]
pub mod imr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "UCPD Status Register"]
pub mod sr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "UCPD Interrupt Clear Register"]
pub mod icr;
#[doc = "TX_ORDSET register accessor: an alias for `Reg<TX_ORDSET_SPEC>`"]
pub type TX_ORDSET = crate::Reg<tx_ordset::TX_ORDSET_SPEC>;
#[doc = "UCPD Tx Ordered Set Type Register"]
pub mod tx_ordset;
#[doc = "TX_PAYSZ register accessor: an alias for `Reg<TX_PAYSZ_SPEC>`"]
pub type TX_PAYSZ = crate::Reg<tx_paysz::TX_PAYSZ_SPEC>;
#[doc = "UCPD Tx Paysize Register"]
pub mod tx_paysz;
#[doc = "TXDR register accessor: an alias for `Reg<TXDR_SPEC>`"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "UCPD Tx Data Register"]
pub mod txdr;
#[doc = "RX_ORDSET register accessor: an alias for `Reg<RX_ORDSET_SPEC>`"]
pub type RX_ORDSET = crate::Reg<rx_ordset::RX_ORDSET_SPEC>;
#[doc = "UCPD Rx Ordered Set Register"]
pub mod rx_ordset;
#[doc = "RX_PAYSZ register accessor: an alias for `Reg<RX_PAYSZ_SPEC>`"]
pub type RX_PAYSZ = crate::Reg<rx_paysz::RX_PAYSZ_SPEC>;
#[doc = "UCPD Rx Paysize Register"]
pub mod rx_paysz;
#[doc = "RXDR register accessor: an alias for `Reg<RXDR_SPEC>`"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "UCPD Rx Data Register"]
pub mod rxdr;
#[doc = "RX_ORDEXT1 register accessor: an alias for `Reg<RX_ORDEXT1_SPEC>`"]
pub type RX_ORDEXT1 = crate::Reg<rx_ordext1::RX_ORDEXT1_SPEC>;
#[doc = "UCPD Rx Ordered Set Extension Register 1"]
pub mod rx_ordext1;
#[doc = "RX_ORDEXT2 register accessor: an alias for `Reg<RX_ORDEXT2_SPEC>`"]
pub type RX_ORDEXT2 = crate::Reg<rx_ordext2::RX_ORDEXT2_SPEC>;
#[doc = "UCPD Rx Ordered Set Extension Register 2"]
pub mod rx_ordext2;
