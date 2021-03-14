#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode configuration register"]
    pub maccr: MACCR,
    #[doc = "0x04 - Extended operating mode configuration register"]
    pub macecr: MACECR,
    #[doc = "0x08 - Packet filtering control register"]
    pub macpfr: MACPFR,
    #[doc = "0x0c - Watchdog timeout register"]
    pub macwtr: MACWTR,
    #[doc = "0x10 - Hash Table 0 register"]
    pub macht0r: MACHT0R,
    #[doc = "0x14 - Hash Table 1 register"]
    pub macht1r: MACHT1R,
    _reserved6: [u8; 56usize],
    #[doc = "0x50 - VLAN tag register"]
    pub macvtr: MACVTR,
    _reserved7: [u8; 4usize],
    #[doc = "0x58 - VLAN Hash table register"]
    pub macvhtr: MACVHTR,
    _reserved8: [u8; 4usize],
    #[doc = "0x60 - VLAN inclusion register"]
    pub macvir: MACVIR,
    #[doc = "0x64 - Inner VLAN inclusion register"]
    pub macivir: MACIVIR,
    _reserved10: [u8; 8usize],
    #[doc = "0x70 - Tx Queue flow control register"]
    pub macqtx_fcr: MACQTXFCR,
    _reserved11: [u8; 28usize],
    #[doc = "0x90 - Rx flow control register"]
    pub macrx_fcr: MACRXFCR,
    _reserved12: [u8; 28usize],
    #[doc = "0xb0 - Interrupt status register"]
    pub macisr: MACISR,
    #[doc = "0xb4 - Interrupt enable register"]
    pub macier: MACIER,
    #[doc = "0xb8 - Rx Tx status register"]
    pub macrx_tx_sr: MACRXTXSR,
    _reserved15: [u8; 4usize],
    #[doc = "0xc0 - PMT control status register"]
    pub macpcsr: MACPCSR,
    #[doc = "0xc4 - Remove wakeup packet filter register"]
    pub macrwkpfr: MACRWKPFR,
    _reserved17: [u8; 8usize],
    #[doc = "0xd0 - LPI control status register"]
    pub maclcsr: MACLCSR,
    #[doc = "0xd4 - LPI timers control register"]
    pub macltcr: MACLTCR,
    #[doc = "0xd8 - LPI entry timer register"]
    pub macletr: MACLETR,
    #[doc = "0xdc - 1-microsecond-tick counter register"]
    pub mac1ustcr: MAC1USTCR,
    _reserved21: [u8; 48usize],
    #[doc = "0x110 - Version register"]
    pub macvr: MACVR,
    #[doc = "0x114 - Debug register"]
    pub macdr: MACDR,
    _reserved23: [u8; 8usize],
    #[doc = "0x120 - HW feature 1 register"]
    pub machwf1r: MACHWF1R,
    #[doc = "0x124 - HW feature 2 register"]
    pub machwf2r: MACHWF2R,
    _reserved25: [u8; 216usize],
    #[doc = "0x200 - MDIO address register"]
    pub macmdioar: MACMDIOAR,
    #[doc = "0x204 - MDIO data register"]
    pub macmdiodr: MACMDIODR,
    _reserved27: [u8; 248usize],
    #[doc = "0x300 - Address 0 high register"]
    pub maca0hr: MACA0HR,
    #[doc = "0x304 - Address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x308 - Address 1 high register"]
    pub maca1hr: MACA1HR,
    #[doc = "0x30c - Address 1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x310 - Address 2 high register"]
    pub maca2hr: MACA2HR,
    #[doc = "0x314 - Address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x318 - Address 3 high register"]
    pub maca3hr: MACA3HR,
    #[doc = "0x31c - Address 3 low register"]
    pub maca3lr: MACA3LR,
    _reserved35: [u8; 992usize],
    #[doc = "0x700 - MMC control register"]
    pub mmc_control: MMC_CONTROL,
    #[doc = "0x704 - MMC Rx interrupt register"]
    pub mmc_rx_interrupt: MMC_RX_INTERRUPT,
    #[doc = "0x708 - MMC Tx interrupt register"]
    pub mmc_tx_interrupt: MMC_TX_INTERRUPT,
    #[doc = "0x70c - MMC Rx interrupt mask register"]
    pub mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    #[doc = "0x710 - MMC Tx interrupt mask register"]
    pub mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved40: [u8; 56usize],
    #[doc = "0x74c - Tx single collision good packets register"]
    pub tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    #[doc = "0x750 - Tx multiple collision good packets register"]
    pub tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved42: [u8; 20usize],
    #[doc = "0x768 - Tx packet count good register"]
    pub tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved43: [u8; 40usize],
    #[doc = "0x794 - Rx CRC error packets register"]
    pub rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    #[doc = "0x798 - Rx alignment error packets register"]
    pub rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved45: [u8; 40usize],
    #[doc = "0x7c4 - Rx unicast packets good register"]
    pub rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved46: [u8; 36usize],
    #[doc = "0x7ec - Tx LPI microsecond timer register"]
    pub tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    #[doc = "0x7f0 - Tx LPI transition counter register"]
    pub tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    #[doc = "0x7f4 - Rx LPI microsecond counter register"]
    pub rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    #[doc = "0x7f8 - Rx LPI transition counter register"]
    pub rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved50: [u8; 260usize],
    #[doc = "0x900 - L3 and L4 control 0 register"]
    pub macl3l4c0r: MACL3L4C0R,
    #[doc = "0x904 - Layer4 address filter 0 register"]
    pub macl4a0r: MACL4A0R,
    _reserved52: [u8; 8usize],
    #[doc = "0x910 - MACL3A00R"]
    pub macl3a00r: MACL3A00R,
    #[doc = "0x914 - Layer3 address 1 filter 0 register"]
    pub macl3a10r: MACL3A10R,
    #[doc = "0x918 - Layer3 Address 2 filter 0 register"]
    pub macl3a20: MACL3A20,
    #[doc = "0x91c - Layer3 Address 3 filter 0 register"]
    pub macl3a30: MACL3A30,
    _reserved56: [u8; 16usize],
    #[doc = "0x930 - L3 and L4 control 1 register"]
    pub macl3l4c1r: MACL3L4C1R,
    #[doc = "0x934 - Layer 4 address filter 1 register"]
    pub macl4a1r: MACL4A1R,
    _reserved58: [u8; 8usize],
    #[doc = "0x940 - Layer3 address 0 filter 1 Register"]
    pub macl3a01r: MACL3A01R,
    #[doc = "0x944 - Layer3 address 1 filter 1 register"]
    pub macl3a11r: MACL3A11R,
    #[doc = "0x948 - Layer3 address 2 filter 1 Register"]
    pub macl3a21r: MACL3A21R,
    #[doc = "0x94c - Layer3 address 3 filter 1 register"]
    pub macl3a31r: MACL3A31R,
    _reserved62: [u8; 400usize],
    #[doc = "0xae0 - ARP address register"]
    pub macarpar: MACARPAR,
    _reserved63: [u8; 28usize],
    #[doc = "0xb00 - Timestamp control Register"]
    pub mactscr: MACTSCR,
    #[doc = "0xb04 - Sub-second increment register"]
    pub macssir: MACSSIR,
    #[doc = "0xb08 - System time seconds register"]
    pub macstsr: MACSTSR,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub macstnr: MACSTNR,
    #[doc = "0xb10 - System time seconds update register"]
    pub macstsur: MACSTSUR,
    #[doc = "0xb14 - System time nanoseconds update register"]
    pub macstnur: MACSTNUR,
    #[doc = "0xb18 - Timestamp addend register"]
    pub mactsar: MACTSAR,
    _reserved70: [u8; 4usize],
    #[doc = "0xb20 - Timestamp status register"]
    pub mactssr: MACTSSR,
    _reserved71: [u8; 12usize],
    #[doc = "0xb30 - Tx timestamp status nanoseconds register"]
    pub mactx_tssnr: MACTXTSSNR,
    #[doc = "0xb34 - Tx timestamp status seconds register"]
    pub mactx_tsssr: MACTXTSSSR,
    _reserved73: [u8; 8usize],
    #[doc = "0xb40 - Auxiliary control register"]
    pub macacr: MACACR,
    _reserved74: [u8; 4usize],
    #[doc = "0xb48 - Auxiliary timestamp nanoseconds register"]
    pub macatsnr: MACATSNR,
    #[doc = "0xb4c - Auxiliary timestamp seconds register"]
    pub macatssr: MACATSSR,
    #[doc = "0xb50 - Timestamp Ingress asymmetric correction register"]
    pub mactsiacr: MACTSIACR,
    #[doc = "0xb54 - Timestamp Egress asymmetric correction register"]
    pub mactseacr: MACTSEACR,
    #[doc = "0xb58 - Timestamp Ingress correction nanosecond register"]
    pub mactsicnr: MACTSICNR,
    #[doc = "0xb5c - Timestamp Egress correction nanosecond register"]
    pub mactsecnr: MACTSECNR,
    _reserved80: [u8; 16usize],
    #[doc = "0xb70 - PPS control register"]
    pub macppscr: MACPPSCR,
    _reserved81: [u8; 12usize],
    #[doc = "0xb80 - PPS target time seconds register"]
    pub macppsttsr: MACPPSTTSR,
    #[doc = "0xb84 - PPS target time nanoseconds register"]
    pub macppsttnr: MACPPSTTNR,
    #[doc = "0xb88 - PPS interval register"]
    pub macppsir: MACPPSIR,
    #[doc = "0xb8c - PPS width register"]
    pub macppswr: MACPPSWR,
    _reserved85: [u8; 48usize],
    #[doc = "0xbc0 - PTP Offload control register"]
    pub macpocr: MACPOCR,
    #[doc = "0xbc4 - PTP Source Port Identity 0 Register"]
    pub macspi0r: MACSPI0R,
    #[doc = "0xbc8 - PTP Source port identity 1 register"]
    pub macspi1r: MACSPI1R,
    #[doc = "0xbcc - PTP Source port identity 2 register"]
    pub macspi2r: MACSPI2R,
    #[doc = "0xbd0 - Log message interval register"]
    pub maclmir: MACLMIR,
}
#[doc = "Operating mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maccr](maccr) module"]
pub type MACCR = crate::Reg<u32, _MACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACCR;
#[doc = "`read()` method returns [maccr::R](maccr::R) reader structure"]
impl crate::Readable for MACCR {}
#[doc = "`write(|w| ..)` method takes [maccr::W](maccr::W) writer structure"]
impl crate::Writable for MACCR {}
#[doc = "Operating mode configuration register"]
pub mod maccr;
#[doc = "Extended operating mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macecr](macecr) module"]
pub type MACECR = crate::Reg<u32, _MACECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACECR;
#[doc = "`read()` method returns [macecr::R](macecr::R) reader structure"]
impl crate::Readable for MACECR {}
#[doc = "`write(|w| ..)` method takes [macecr::W](macecr::W) writer structure"]
impl crate::Writable for MACECR {}
#[doc = "Extended operating mode configuration register"]
pub mod macecr;
#[doc = "Packet filtering control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpfr](macpfr) module"]
pub type MACPFR = crate::Reg<u32, _MACPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPFR;
#[doc = "`read()` method returns [macpfr::R](macpfr::R) reader structure"]
impl crate::Readable for MACPFR {}
#[doc = "`write(|w| ..)` method takes [macpfr::W](macpfr::W) writer structure"]
impl crate::Writable for MACPFR {}
#[doc = "Packet filtering control register"]
pub mod macpfr;
#[doc = "Watchdog timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macwtr](macwtr) module"]
pub type MACWTR = crate::Reg<u32, _MACWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACWTR;
#[doc = "`read()` method returns [macwtr::R](macwtr::R) reader structure"]
impl crate::Readable for MACWTR {}
#[doc = "`write(|w| ..)` method takes [macwtr::W](macwtr::W) writer structure"]
impl crate::Writable for MACWTR {}
#[doc = "Watchdog timeout register"]
pub mod macwtr;
#[doc = "Hash Table 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macht0r](macht0r) module"]
pub type MACHT0R = crate::Reg<u32, _MACHT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHT0R;
#[doc = "`read()` method returns [macht0r::R](macht0r::R) reader structure"]
impl crate::Readable for MACHT0R {}
#[doc = "`write(|w| ..)` method takes [macht0r::W](macht0r::W) writer structure"]
impl crate::Writable for MACHT0R {}
#[doc = "Hash Table 0 register"]
pub mod macht0r;
#[doc = "Hash Table 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macht1r](macht1r) module"]
pub type MACHT1R = crate::Reg<u32, _MACHT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHT1R;
#[doc = "`read()` method returns [macht1r::R](macht1r::R) reader structure"]
impl crate::Readable for MACHT1R {}
#[doc = "`write(|w| ..)` method takes [macht1r::W](macht1r::W) writer structure"]
impl crate::Writable for MACHT1R {}
#[doc = "Hash Table 1 register"]
pub mod macht1r;
#[doc = "VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvtr](macvtr) module"]
pub type MACVTR = crate::Reg<u32, _MACVTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVTR;
#[doc = "`read()` method returns [macvtr::R](macvtr::R) reader structure"]
impl crate::Readable for MACVTR {}
#[doc = "`write(|w| ..)` method takes [macvtr::W](macvtr::W) writer structure"]
impl crate::Writable for MACVTR {}
#[doc = "VLAN tag register"]
pub mod macvtr;
#[doc = "VLAN Hash table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvhtr](macvhtr) module"]
pub type MACVHTR = crate::Reg<u32, _MACVHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVHTR;
#[doc = "`read()` method returns [macvhtr::R](macvhtr::R) reader structure"]
impl crate::Readable for MACVHTR {}
#[doc = "`write(|w| ..)` method takes [macvhtr::W](macvhtr::W) writer structure"]
impl crate::Writable for MACVHTR {}
#[doc = "VLAN Hash table register"]
pub mod macvhtr;
#[doc = "VLAN inclusion register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvir](macvir) module"]
pub type MACVIR = crate::Reg<u32, _MACVIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVIR;
#[doc = "`read()` method returns [macvir::R](macvir::R) reader structure"]
impl crate::Readable for MACVIR {}
#[doc = "`write(|w| ..)` method takes [macvir::W](macvir::W) writer structure"]
impl crate::Writable for MACVIR {}
#[doc = "VLAN inclusion register"]
pub mod macvir;
#[doc = "Inner VLAN inclusion register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macivir](macivir) module"]
pub type MACIVIR = crate::Reg<u32, _MACIVIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACIVIR;
#[doc = "`read()` method returns [macivir::R](macivir::R) reader structure"]
impl crate::Readable for MACIVIR {}
#[doc = "`write(|w| ..)` method takes [macivir::W](macivir::W) writer structure"]
impl crate::Writable for MACIVIR {}
#[doc = "Inner VLAN inclusion register"]
pub mod macivir;
#[doc = "Tx Queue flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macqtx_fcr](macqtx_fcr) module"]
pub type MACQTXFCR = crate::Reg<u32, _MACQTXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACQTXFCR;
#[doc = "`read()` method returns [macqtx_fcr::R](macqtx_fcr::R) reader structure"]
impl crate::Readable for MACQTXFCR {}
#[doc = "`write(|w| ..)` method takes [macqtx_fcr::W](macqtx_fcr::W) writer structure"]
impl crate::Writable for MACQTXFCR {}
#[doc = "Tx Queue flow control register"]
pub mod macqtx_fcr;
#[doc = "Rx flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrx_fcr](macrx_fcr) module"]
pub type MACRXFCR = crate::Reg<u32, _MACRXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACRXFCR;
#[doc = "`read()` method returns [macrx_fcr::R](macrx_fcr::R) reader structure"]
impl crate::Readable for MACRXFCR {}
#[doc = "`write(|w| ..)` method takes [macrx_fcr::W](macrx_fcr::W) writer structure"]
impl crate::Writable for MACRXFCR {}
#[doc = "Rx flow control register"]
pub mod macrx_fcr;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macisr](macisr) module"]
pub type MACISR = crate::Reg<u32, _MACISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACISR;
#[doc = "`read()` method returns [macisr::R](macisr::R) reader structure"]
impl crate::Readable for MACISR {}
#[doc = "Interrupt status register"]
pub mod macisr;
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macier](macier) module"]
pub type MACIER = crate::Reg<u32, _MACIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACIER;
#[doc = "`read()` method returns [macier::R](macier::R) reader structure"]
impl crate::Readable for MACIER {}
#[doc = "`write(|w| ..)` method takes [macier::W](macier::W) writer structure"]
impl crate::Writable for MACIER {}
#[doc = "Interrupt enable register"]
pub mod macier;
#[doc = "Rx Tx status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrx_tx_sr](macrx_tx_sr) module"]
pub type MACRXTXSR = crate::Reg<u32, _MACRXTXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACRXTXSR;
#[doc = "`read()` method returns [macrx_tx_sr::R](macrx_tx_sr::R) reader structure"]
impl crate::Readable for MACRXTXSR {}
#[doc = "Rx Tx status register"]
pub mod macrx_tx_sr;
#[doc = "PMT control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpcsr](macpcsr) module"]
pub type MACPCSR = crate::Reg<u32, _MACPCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPCSR;
#[doc = "`read()` method returns [macpcsr::R](macpcsr::R) reader structure"]
impl crate::Readable for MACPCSR {}
#[doc = "`write(|w| ..)` method takes [macpcsr::W](macpcsr::W) writer structure"]
impl crate::Writable for MACPCSR {}
#[doc = "PMT control status register"]
pub mod macpcsr;
#[doc = "Remove wakeup packet filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrwkpfr](macrwkpfr) module"]
pub type MACRWKPFR = crate::Reg<u32, _MACRWKPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACRWKPFR;
#[doc = "`read()` method returns [macrwkpfr::R](macrwkpfr::R) reader structure"]
impl crate::Readable for MACRWKPFR {}
#[doc = "`write(|w| ..)` method takes [macrwkpfr::W](macrwkpfr::W) writer structure"]
impl crate::Writable for MACRWKPFR {}
#[doc = "Remove wakeup packet filter register"]
pub mod macrwkpfr;
#[doc = "LPI control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maclcsr](maclcsr) module"]
pub type MACLCSR = crate::Reg<u32, _MACLCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACLCSR;
#[doc = "`read()` method returns [maclcsr::R](maclcsr::R) reader structure"]
impl crate::Readable for MACLCSR {}
#[doc = "`write(|w| ..)` method takes [maclcsr::W](maclcsr::W) writer structure"]
impl crate::Writable for MACLCSR {}
#[doc = "LPI control status register"]
pub mod maclcsr;
#[doc = "LPI timers control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macltcr](macltcr) module"]
pub type MACLTCR = crate::Reg<u32, _MACLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACLTCR;
#[doc = "`read()` method returns [macltcr::R](macltcr::R) reader structure"]
impl crate::Readable for MACLTCR {}
#[doc = "`write(|w| ..)` method takes [macltcr::W](macltcr::W) writer structure"]
impl crate::Writable for MACLTCR {}
#[doc = "LPI timers control register"]
pub mod macltcr;
#[doc = "LPI entry timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macletr](macletr) module"]
pub type MACLETR = crate::Reg<u32, _MACLETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACLETR;
#[doc = "`read()` method returns [macletr::R](macletr::R) reader structure"]
impl crate::Readable for MACLETR {}
#[doc = "`write(|w| ..)` method takes [macletr::W](macletr::W) writer structure"]
impl crate::Writable for MACLETR {}
#[doc = "LPI entry timer register"]
pub mod macletr;
#[doc = "1-microsecond-tick counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac1ustcr](mac1ustcr) module"]
pub type MAC1USTCR = crate::Reg<u32, _MAC1USTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC1USTCR;
#[doc = "`read()` method returns [mac1ustcr::R](mac1ustcr::R) reader structure"]
impl crate::Readable for MAC1USTCR {}
#[doc = "`write(|w| ..)` method takes [mac1ustcr::W](mac1ustcr::W) writer structure"]
impl crate::Writable for MAC1USTCR {}
#[doc = "1-microsecond-tick counter register"]
pub mod mac1ustcr;
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvr](macvr) module"]
pub type MACVR = crate::Reg<u32, _MACVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACVR;
#[doc = "`read()` method returns [macvr::R](macvr::R) reader structure"]
impl crate::Readable for MACVR {}
#[doc = "Version register"]
pub mod macvr;
#[doc = "HW feature 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machwf1r](machwf1r) module"]
pub type MACHWF1R = crate::Reg<u32, _MACHWF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHWF1R;
#[doc = "`read()` method returns [machwf1r::R](machwf1r::R) reader structure"]
impl crate::Readable for MACHWF1R {}
#[doc = "HW feature 1 register"]
pub mod machwf1r;
#[doc = "HW feature 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machwf2r](machwf2r) module"]
pub type MACHWF2R = crate::Reg<u32, _MACHWF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACHWF2R;
#[doc = "`read()` method returns [machwf2r::R](machwf2r::R) reader structure"]
impl crate::Readable for MACHWF2R {}
#[doc = "HW feature 2 register"]
pub mod machwf2r;
#[doc = "MDIO address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdioar](macmdioar) module"]
pub type MACMDIOAR = crate::Reg<u32, _MACMDIOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMDIOAR;
#[doc = "`read()` method returns [macmdioar::R](macmdioar::R) reader structure"]
impl crate::Readable for MACMDIOAR {}
#[doc = "`write(|w| ..)` method takes [macmdioar::W](macmdioar::W) writer structure"]
impl crate::Writable for MACMDIOAR {}
#[doc = "MDIO address register"]
pub mod macmdioar;
#[doc = "MDIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdiodr](macmdiodr) module"]
pub type MACMDIODR = crate::Reg<u32, _MACMDIODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACMDIODR;
#[doc = "`read()` method returns [macmdiodr::R](macmdiodr::R) reader structure"]
impl crate::Readable for MACMDIODR {}
#[doc = "`write(|w| ..)` method takes [macmdiodr::W](macmdiodr::W) writer structure"]
impl crate::Writable for MACMDIODR {}
#[doc = "MDIO data register"]
pub mod macmdiodr;
#[doc = "ARP address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macarpar](macarpar) module"]
pub type MACARPAR = crate::Reg<u32, _MACARPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACARPAR;
#[doc = "`read()` method returns [macarpar::R](macarpar::R) reader structure"]
impl crate::Readable for MACARPAR {}
#[doc = "`write(|w| ..)` method takes [macarpar::W](macarpar::W) writer structure"]
impl crate::Writable for MACARPAR {}
#[doc = "ARP address register"]
pub mod macarpar;
#[doc = "Address 0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0hr](maca0hr) module"]
pub type MACA0HR = crate::Reg<u32, _MACA0HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0HR;
#[doc = "`read()` method returns [maca0hr::R](maca0hr::R) reader structure"]
impl crate::Readable for MACA0HR {}
#[doc = "`write(|w| ..)` method takes [maca0hr::W](maca0hr::W) writer structure"]
impl crate::Writable for MACA0HR {}
#[doc = "Address 0 high register"]
pub mod maca0hr;
#[doc = "Address 0 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0lr](maca0lr) module"]
pub type MACA0LR = crate::Reg<u32, _MACA0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA0LR;
#[doc = "`read()` method returns [maca0lr::R](maca0lr::R) reader structure"]
impl crate::Readable for MACA0LR {}
#[doc = "`write(|w| ..)` method takes [maca0lr::W](maca0lr::W) writer structure"]
impl crate::Writable for MACA0LR {}
#[doc = "Address 0 low register"]
pub mod maca0lr;
#[doc = "Address 1 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1lr](maca1lr) module"]
pub type MACA1LR = crate::Reg<u32, _MACA1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1LR;
#[doc = "`read()` method returns [maca1lr::R](maca1lr::R) reader structure"]
impl crate::Readable for MACA1LR {}
#[doc = "`write(|w| ..)` method takes [maca1lr::W](maca1lr::W) writer structure"]
impl crate::Writable for MACA1LR {}
#[doc = "Address 1 low register"]
pub mod maca1lr;
#[doc = "Address 2 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2lr](maca2lr) module"]
pub type MACA2LR = crate::Reg<u32, _MACA2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2LR;
#[doc = "`read()` method returns [maca2lr::R](maca2lr::R) reader structure"]
impl crate::Readable for MACA2LR {}
#[doc = "`write(|w| ..)` method takes [maca2lr::W](maca2lr::W) writer structure"]
impl crate::Writable for MACA2LR {}
#[doc = "Address 2 low register"]
pub mod maca2lr;
#[doc = "Address 3 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3lr](maca3lr) module"]
pub type MACA3LR = crate::Reg<u32, _MACA3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3LR;
#[doc = "`read()` method returns [maca3lr::R](maca3lr::R) reader structure"]
impl crate::Readable for MACA3LR {}
#[doc = "`write(|w| ..)` method takes [maca3lr::W](maca3lr::W) writer structure"]
impl crate::Writable for MACA3LR {}
#[doc = "Address 3 low register"]
pub mod maca3lr;
#[doc = "Address 1 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1hr](maca1hr) module"]
pub type MACA1HR = crate::Reg<u32, _MACA1HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA1HR;
#[doc = "`read()` method returns [maca1hr::R](maca1hr::R) reader structure"]
impl crate::Readable for MACA1HR {}
#[doc = "`write(|w| ..)` method takes [maca1hr::W](maca1hr::W) writer structure"]
impl crate::Writable for MACA1HR {}
#[doc = "Address 1 high register"]
pub mod maca1hr;
#[doc = "Address 2 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2hr](maca2hr) module"]
pub type MACA2HR = crate::Reg<u32, _MACA2HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA2HR;
#[doc = "`read()` method returns [maca2hr::R](maca2hr::R) reader structure"]
impl crate::Readable for MACA2HR {}
#[doc = "`write(|w| ..)` method takes [maca2hr::W](maca2hr::W) writer structure"]
impl crate::Writable for MACA2HR {}
#[doc = "Address 2 high register"]
pub mod maca2hr;
#[doc = "Address 3 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3hr](maca3hr) module"]
pub type MACA3HR = crate::Reg<u32, _MACA3HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACA3HR;
#[doc = "`read()` method returns [maca3hr::R](maca3hr::R) reader structure"]
impl crate::Readable for MACA3HR {}
#[doc = "`write(|w| ..)` method takes [maca3hr::W](maca3hr::W) writer structure"]
impl crate::Writable for MACA3HR {}
#[doc = "Address 3 high register"]
pub mod maca3hr;
#[doc = "MMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_control](mmc_control) module"]
pub type MMC_CONTROL = crate::Reg<u32, _MMC_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_CONTROL;
#[doc = "`read()` method returns [mmc_control::R](mmc_control::R) reader structure"]
impl crate::Readable for MMC_CONTROL {}
#[doc = "`write(|w| ..)` method takes [mmc_control::W](mmc_control::W) writer structure"]
impl crate::Writable for MMC_CONTROL {}
#[doc = "MMC control register"]
pub mod mmc_control;
#[doc = "MMC Rx interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt](mmc_rx_interrupt) module"]
pub type MMC_RX_INTERRUPT = crate::Reg<u32, _MMC_RX_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RX_INTERRUPT;
#[doc = "`read()` method returns [mmc_rx_interrupt::R](mmc_rx_interrupt::R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT {}
#[doc = "MMC Rx interrupt register"]
pub mod mmc_rx_interrupt;
#[doc = "MMC Tx interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt](mmc_tx_interrupt) module"]
pub type MMC_TX_INTERRUPT = crate::Reg<u32, _MMC_TX_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TX_INTERRUPT;
#[doc = "`read()` method returns [mmc_tx_interrupt::R](mmc_tx_interrupt::R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT {}
#[doc = "MMC Tx interrupt register"]
pub mod mmc_tx_interrupt;
#[doc = "MMC Rx interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt_mask](mmc_rx_interrupt_mask) module"]
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<u32, _MMC_RX_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RX_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_rx_interrupt_mask::R](mmc_rx_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_rx_interrupt_mask::W](mmc_rx_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPT_MASK {}
#[doc = "MMC Rx interrupt mask register"]
pub mod mmc_rx_interrupt_mask;
#[doc = "MMC Tx interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt_mask](mmc_tx_interrupt_mask) module"]
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<u32, _MMC_TX_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TX_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_tx_interrupt_mask::R](mmc_tx_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_tx_interrupt_mask::W](mmc_tx_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASK {}
#[doc = "MMC Tx interrupt mask register"]
pub mod mmc_tx_interrupt_mask;
#[doc = "Tx single collision good packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_single_collision_good_packets](tx_single_collision_good_packets) module"]
pub type TX_SINGLE_COLLISION_GOOD_PACKETS = crate::Reg<u32, _TX_SINGLE_COLLISION_GOOD_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_SINGLE_COLLISION_GOOD_PACKETS;
#[doc = "`read()` method returns [tx_single_collision_good_packets::R](tx_single_collision_good_packets::R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETS {}
#[doc = "Tx single collision good packets register"]
pub mod tx_single_collision_good_packets;
#[doc = "Tx multiple collision good packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_multiple_collision_good_packets](tx_multiple_collision_good_packets) module"]
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS = crate::Reg<u32, _TX_MULTIPLE_COLLISION_GOOD_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_MULTIPLE_COLLISION_GOOD_PACKETS;
#[doc = "`read()` method returns [tx_multiple_collision_good_packets::R](tx_multiple_collision_good_packets::R) reader structure"]
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_PACKETS {}
#[doc = "Tx multiple collision good packets register"]
pub mod tx_multiple_collision_good_packets;
#[doc = "Tx packet count good register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_packet_count_good](tx_packet_count_good) module"]
pub type TX_PACKET_COUNT_GOOD = crate::Reg<u32, _TX_PACKET_COUNT_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PACKET_COUNT_GOOD;
#[doc = "`read()` method returns [tx_packet_count_good::R](tx_packet_count_good::R) reader structure"]
impl crate::Readable for TX_PACKET_COUNT_GOOD {}
#[doc = "Tx packet count good register"]
pub mod tx_packet_count_good;
#[doc = "Rx CRC error packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_crc_error_packets](rx_crc_error_packets) module"]
pub type RX_CRC_ERROR_PACKETS = crate::Reg<u32, _RX_CRC_ERROR_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CRC_ERROR_PACKETS;
#[doc = "`read()` method returns [rx_crc_error_packets::R](rx_crc_error_packets::R) reader structure"]
impl crate::Readable for RX_CRC_ERROR_PACKETS {}
#[doc = "Rx CRC error packets register"]
pub mod rx_crc_error_packets;
#[doc = "Rx alignment error packets register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_alignment_error_packets](rx_alignment_error_packets) module"]
pub type RX_ALIGNMENT_ERROR_PACKETS = crate::Reg<u32, _RX_ALIGNMENT_ERROR_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ALIGNMENT_ERROR_PACKETS;
#[doc = "`read()` method returns [rx_alignment_error_packets::R](rx_alignment_error_packets::R) reader structure"]
impl crate::Readable for RX_ALIGNMENT_ERROR_PACKETS {}
#[doc = "Rx alignment error packets register"]
pub mod rx_alignment_error_packets;
#[doc = "Rx unicast packets good register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_unicast_packets_good](rx_unicast_packets_good) module"]
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<u32, _RX_UNICAST_PACKETS_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_UNICAST_PACKETS_GOOD;
#[doc = "`read()` method returns [rx_unicast_packets_good::R](rx_unicast_packets_good::R) reader structure"]
impl crate::Readable for RX_UNICAST_PACKETS_GOOD {}
#[doc = "Rx unicast packets good register"]
pub mod rx_unicast_packets_good;
#[doc = "Tx LPI microsecond timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lpi_usec_cntr](tx_lpi_usec_cntr) module"]
pub type TX_LPI_USEC_CNTR = crate::Reg<u32, _TX_LPI_USEC_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_LPI_USEC_CNTR;
#[doc = "`read()` method returns [tx_lpi_usec_cntr::R](tx_lpi_usec_cntr::R) reader structure"]
impl crate::Readable for TX_LPI_USEC_CNTR {}
#[doc = "Tx LPI microsecond timer register"]
pub mod tx_lpi_usec_cntr;
#[doc = "Tx LPI transition counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lpi_tran_cntr](tx_lpi_tran_cntr) module"]
pub type TX_LPI_TRAN_CNTR = crate::Reg<u32, _TX_LPI_TRAN_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_LPI_TRAN_CNTR;
#[doc = "`read()` method returns [tx_lpi_tran_cntr::R](tx_lpi_tran_cntr::R) reader structure"]
impl crate::Readable for TX_LPI_TRAN_CNTR {}
#[doc = "Tx LPI transition counter register"]
pub mod tx_lpi_tran_cntr;
#[doc = "Rx LPI microsecond counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi_usec_cntr](rx_lpi_usec_cntr) module"]
pub type RX_LPI_USEC_CNTR = crate::Reg<u32, _RX_LPI_USEC_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_LPI_USEC_CNTR;
#[doc = "`read()` method returns [rx_lpi_usec_cntr::R](rx_lpi_usec_cntr::R) reader structure"]
impl crate::Readable for RX_LPI_USEC_CNTR {}
#[doc = "Rx LPI microsecond counter register"]
pub mod rx_lpi_usec_cntr;
#[doc = "Rx LPI transition counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi_tran_cntr](rx_lpi_tran_cntr) module"]
pub type RX_LPI_TRAN_CNTR = crate::Reg<u32, _RX_LPI_TRAN_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_LPI_TRAN_CNTR;
#[doc = "`read()` method returns [rx_lpi_tran_cntr::R](rx_lpi_tran_cntr::R) reader structure"]
impl crate::Readable for RX_LPI_TRAN_CNTR {}
#[doc = "Rx LPI transition counter register"]
pub mod rx_lpi_tran_cntr;
#[doc = "L3 and L4 control 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3l4c0r](macl3l4c0r) module"]
pub type MACL3L4C0R = crate::Reg<u32, _MACL3L4C0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3L4C0R;
#[doc = "`read()` method returns [macl3l4c0r::R](macl3l4c0r::R) reader structure"]
impl crate::Readable for MACL3L4C0R {}
#[doc = "`write(|w| ..)` method takes [macl3l4c0r::W](macl3l4c0r::W) writer structure"]
impl crate::Writable for MACL3L4C0R {}
#[doc = "L3 and L4 control 0 register"]
pub mod macl3l4c0r;
#[doc = "Layer4 address filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl4a0r](macl4a0r) module"]
pub type MACL4A0R = crate::Reg<u32, _MACL4A0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL4A0R;
#[doc = "`read()` method returns [macl4a0r::R](macl4a0r::R) reader structure"]
impl crate::Readable for MACL4A0R {}
#[doc = "`write(|w| ..)` method takes [macl4a0r::W](macl4a0r::W) writer structure"]
impl crate::Writable for MACL4A0R {}
#[doc = "Layer4 address filter 0 register"]
pub mod macl4a0r;
#[doc = "Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdr](macdr) module"]
pub type MACDR = crate::Reg<u32, _MACDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACDR;
#[doc = "`read()` method returns [macdr::R](macdr::R) reader structure"]
impl crate::Readable for MACDR {}
#[doc = "Debug register"]
pub mod macdr;
#[doc = "MACL3A00R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a00r](macl3a00r) module"]
pub type MACL3A00R = crate::Reg<u32, _MACL3A00R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A00R;
#[doc = "`read()` method returns [macl3a00r::R](macl3a00r::R) reader structure"]
impl crate::Readable for MACL3A00R {}
#[doc = "`write(|w| ..)` method takes [macl3a00r::W](macl3a00r::W) writer structure"]
impl crate::Writable for MACL3A00R {}
#[doc = "MACL3A00R"]
pub mod macl3a00r;
#[doc = "Layer3 address 1 filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a10r](macl3a10r) module"]
pub type MACL3A10R = crate::Reg<u32, _MACL3A10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A10R;
#[doc = "`read()` method returns [macl3a10r::R](macl3a10r::R) reader structure"]
impl crate::Readable for MACL3A10R {}
#[doc = "`write(|w| ..)` method takes [macl3a10r::W](macl3a10r::W) writer structure"]
impl crate::Writable for MACL3A10R {}
#[doc = "Layer3 address 1 filter 0 register"]
pub mod macl3a10r;
#[doc = "Layer3 Address 2 filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a20](macl3a20) module"]
pub type MACL3A20 = crate::Reg<u32, _MACL3A20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A20;
#[doc = "`read()` method returns [macl3a20::R](macl3a20::R) reader structure"]
impl crate::Readable for MACL3A20 {}
#[doc = "`write(|w| ..)` method takes [macl3a20::W](macl3a20::W) writer structure"]
impl crate::Writable for MACL3A20 {}
#[doc = "Layer3 Address 2 filter 0 register"]
pub mod macl3a20;
#[doc = "Layer3 Address 3 filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a30](macl3a30) module"]
pub type MACL3A30 = crate::Reg<u32, _MACL3A30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A30;
#[doc = "`read()` method returns [macl3a30::R](macl3a30::R) reader structure"]
impl crate::Readable for MACL3A30 {}
#[doc = "`write(|w| ..)` method takes [macl3a30::W](macl3a30::W) writer structure"]
impl crate::Writable for MACL3A30 {}
#[doc = "Layer3 Address 3 filter 0 register"]
pub mod macl3a30;
#[doc = "L3 and L4 control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3l4c1r](macl3l4c1r) module"]
pub type MACL3L4C1R = crate::Reg<u32, _MACL3L4C1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3L4C1R;
#[doc = "`read()` method returns [macl3l4c1r::R](macl3l4c1r::R) reader structure"]
impl crate::Readable for MACL3L4C1R {}
#[doc = "`write(|w| ..)` method takes [macl3l4c1r::W](macl3l4c1r::W) writer structure"]
impl crate::Writable for MACL3L4C1R {}
#[doc = "L3 and L4 control 1 register"]
pub mod macl3l4c1r;
#[doc = "Layer 4 address filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl4a1r](macl4a1r) module"]
pub type MACL4A1R = crate::Reg<u32, _MACL4A1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL4A1R;
#[doc = "`read()` method returns [macl4a1r::R](macl4a1r::R) reader structure"]
impl crate::Readable for MACL4A1R {}
#[doc = "`write(|w| ..)` method takes [macl4a1r::W](macl4a1r::W) writer structure"]
impl crate::Writable for MACL4A1R {}
#[doc = "Layer 4 address filter 1 register"]
pub mod macl4a1r;
#[doc = "Layer3 address 0 filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a01r](macl3a01r) module"]
pub type MACL3A01R = crate::Reg<u32, _MACL3A01R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A01R;
#[doc = "`read()` method returns [macl3a01r::R](macl3a01r::R) reader structure"]
impl crate::Readable for MACL3A01R {}
#[doc = "`write(|w| ..)` method takes [macl3a01r::W](macl3a01r::W) writer structure"]
impl crate::Writable for MACL3A01R {}
#[doc = "Layer3 address 0 filter 1 Register"]
pub mod macl3a01r;
#[doc = "Layer3 address 1 filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a11r](macl3a11r) module"]
pub type MACL3A11R = crate::Reg<u32, _MACL3A11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A11R;
#[doc = "`read()` method returns [macl3a11r::R](macl3a11r::R) reader structure"]
impl crate::Readable for MACL3A11R {}
#[doc = "`write(|w| ..)` method takes [macl3a11r::W](macl3a11r::W) writer structure"]
impl crate::Writable for MACL3A11R {}
#[doc = "Layer3 address 1 filter 1 register"]
pub mod macl3a11r;
#[doc = "Layer3 address 2 filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a21r](macl3a21r) module"]
pub type MACL3A21R = crate::Reg<u32, _MACL3A21R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A21R;
#[doc = "`read()` method returns [macl3a21r::R](macl3a21r::R) reader structure"]
impl crate::Readable for MACL3A21R {}
#[doc = "`write(|w| ..)` method takes [macl3a21r::W](macl3a21r::W) writer structure"]
impl crate::Writable for MACL3A21R {}
#[doc = "Layer3 address 2 filter 1 Register"]
pub mod macl3a21r;
#[doc = "Layer3 address 3 filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a31r](macl3a31r) module"]
pub type MACL3A31R = crate::Reg<u32, _MACL3A31R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACL3A31R;
#[doc = "`read()` method returns [macl3a31r::R](macl3a31r::R) reader structure"]
impl crate::Readable for MACL3A31R {}
#[doc = "`write(|w| ..)` method takes [macl3a31r::W](macl3a31r::W) writer structure"]
impl crate::Writable for MACL3A31R {}
#[doc = "Layer3 address 3 filter 1 register"]
pub mod macl3a31r;
#[doc = "Timestamp control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactscr](mactscr) module"]
pub type MACTSCR = crate::Reg<u32, _MACTSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSCR;
#[doc = "`read()` method returns [mactscr::R](mactscr::R) reader structure"]
impl crate::Readable for MACTSCR {}
#[doc = "`write(|w| ..)` method takes [mactscr::W](mactscr::W) writer structure"]
impl crate::Writable for MACTSCR {}
#[doc = "Timestamp control Register"]
pub mod mactscr;
#[doc = "Sub-second increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macssir](macssir) module"]
pub type MACSSIR = crate::Reg<u32, _MACSSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSSIR;
#[doc = "`read()` method returns [macssir::R](macssir::R) reader structure"]
impl crate::Readable for MACSSIR {}
#[doc = "`write(|w| ..)` method takes [macssir::W](macssir::W) writer structure"]
impl crate::Writable for MACSSIR {}
#[doc = "Sub-second increment register"]
pub mod macssir;
#[doc = "System time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstsr](macstsr) module"]
pub type MACSTSR = crate::Reg<u32, _MACSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSTSR;
#[doc = "`read()` method returns [macstsr::R](macstsr::R) reader structure"]
impl crate::Readable for MACSTSR {}
#[doc = "System time seconds register"]
pub mod macstsr;
#[doc = "System time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstnr](macstnr) module"]
pub type MACSTNR = crate::Reg<u32, _MACSTNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSTNR;
#[doc = "`read()` method returns [macstnr::R](macstnr::R) reader structure"]
impl crate::Readable for MACSTNR {}
#[doc = "System time nanoseconds register"]
pub mod macstnr;
#[doc = "System time seconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstsur](macstsur) module"]
pub type MACSTSUR = crate::Reg<u32, _MACSTSUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSTSUR;
#[doc = "`read()` method returns [macstsur::R](macstsur::R) reader structure"]
impl crate::Readable for MACSTSUR {}
#[doc = "`write(|w| ..)` method takes [macstsur::W](macstsur::W) writer structure"]
impl crate::Writable for MACSTSUR {}
#[doc = "System time seconds update register"]
pub mod macstsur;
#[doc = "System time nanoseconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstnur](macstnur) module"]
pub type MACSTNUR = crate::Reg<u32, _MACSTNUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSTNUR;
#[doc = "`read()` method returns [macstnur::R](macstnur::R) reader structure"]
impl crate::Readable for MACSTNUR {}
#[doc = "`write(|w| ..)` method takes [macstnur::W](macstnur::W) writer structure"]
impl crate::Writable for MACSTNUR {}
#[doc = "System time nanoseconds update register"]
pub mod macstnur;
#[doc = "Timestamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsar](mactsar) module"]
pub type MACTSAR = crate::Reg<u32, _MACTSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSAR;
#[doc = "`read()` method returns [mactsar::R](mactsar::R) reader structure"]
impl crate::Readable for MACTSAR {}
#[doc = "`write(|w| ..)` method takes [mactsar::W](mactsar::W) writer structure"]
impl crate::Writable for MACTSAR {}
#[doc = "Timestamp addend register"]
pub mod mactsar;
#[doc = "Timestamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactssr](mactssr) module"]
pub type MACTSSR = crate::Reg<u32, _MACTSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSSR;
#[doc = "`read()` method returns [mactssr::R](mactssr::R) reader structure"]
impl crate::Readable for MACTSSR {}
#[doc = "Timestamp status register"]
pub mod mactssr;
#[doc = "Tx timestamp status nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactx_tssnr](mactx_tssnr) module"]
pub type MACTXTSSNR = crate::Reg<u32, _MACTXTSSNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTXTSSNR;
#[doc = "`read()` method returns [mactx_tssnr::R](mactx_tssnr::R) reader structure"]
impl crate::Readable for MACTXTSSNR {}
#[doc = "Tx timestamp status nanoseconds register"]
pub mod mactx_tssnr;
#[doc = "Tx timestamp status seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactx_tsssr](mactx_tsssr) module"]
pub type MACTXTSSSR = crate::Reg<u32, _MACTXTSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTXTSSSR;
#[doc = "`read()` method returns [mactx_tsssr::R](mactx_tsssr::R) reader structure"]
impl crate::Readable for MACTXTSSSR {}
#[doc = "Tx timestamp status seconds register"]
pub mod mactx_tsssr;
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macacr](macacr) module"]
pub type MACACR = crate::Reg<u32, _MACACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACACR;
#[doc = "`read()` method returns [macacr::R](macacr::R) reader structure"]
impl crate::Readable for MACACR {}
#[doc = "`write(|w| ..)` method takes [macacr::W](macacr::W) writer structure"]
impl crate::Writable for MACACR {}
#[doc = "Auxiliary control register"]
pub mod macacr;
#[doc = "Auxiliary timestamp nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macatsnr](macatsnr) module"]
pub type MACATSNR = crate::Reg<u32, _MACATSNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACATSNR;
#[doc = "`read()` method returns [macatsnr::R](macatsnr::R) reader structure"]
impl crate::Readable for MACATSNR {}
#[doc = "Auxiliary timestamp nanoseconds register"]
pub mod macatsnr;
#[doc = "Auxiliary timestamp seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macatssr](macatssr) module"]
pub type MACATSSR = crate::Reg<u32, _MACATSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACATSSR;
#[doc = "`read()` method returns [macatssr::R](macatssr::R) reader structure"]
impl crate::Readable for MACATSSR {}
#[doc = "Auxiliary timestamp seconds register"]
pub mod macatssr;
#[doc = "Timestamp Ingress asymmetric correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsiacr](mactsiacr) module"]
pub type MACTSIACR = crate::Reg<u32, _MACTSIACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSIACR;
#[doc = "`read()` method returns [mactsiacr::R](mactsiacr::R) reader structure"]
impl crate::Readable for MACTSIACR {}
#[doc = "`write(|w| ..)` method takes [mactsiacr::W](mactsiacr::W) writer structure"]
impl crate::Writable for MACTSIACR {}
#[doc = "Timestamp Ingress asymmetric correction register"]
pub mod mactsiacr;
#[doc = "Timestamp Egress asymmetric correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactseacr](mactseacr) module"]
pub type MACTSEACR = crate::Reg<u32, _MACTSEACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSEACR;
#[doc = "`read()` method returns [mactseacr::R](mactseacr::R) reader structure"]
impl crate::Readable for MACTSEACR {}
#[doc = "`write(|w| ..)` method takes [mactseacr::W](mactseacr::W) writer structure"]
impl crate::Writable for MACTSEACR {}
#[doc = "Timestamp Egress asymmetric correction register"]
pub mod mactseacr;
#[doc = "Timestamp Ingress correction nanosecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsicnr](mactsicnr) module"]
pub type MACTSICNR = crate::Reg<u32, _MACTSICNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSICNR;
#[doc = "`read()` method returns [mactsicnr::R](mactsicnr::R) reader structure"]
impl crate::Readable for MACTSICNR {}
#[doc = "`write(|w| ..)` method takes [mactsicnr::W](mactsicnr::W) writer structure"]
impl crate::Writable for MACTSICNR {}
#[doc = "Timestamp Ingress correction nanosecond register"]
pub mod mactsicnr;
#[doc = "Timestamp Egress correction nanosecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsecnr](mactsecnr) module"]
pub type MACTSECNR = crate::Reg<u32, _MACTSECNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACTSECNR;
#[doc = "`read()` method returns [mactsecnr::R](mactsecnr::R) reader structure"]
impl crate::Readable for MACTSECNR {}
#[doc = "`write(|w| ..)` method takes [mactsecnr::W](mactsecnr::W) writer structure"]
impl crate::Writable for MACTSECNR {}
#[doc = "Timestamp Egress correction nanosecond register"]
pub mod mactsecnr;
#[doc = "PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppscr](macppscr) module"]
pub type MACPPSCR = crate::Reg<u32, _MACPPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPPSCR;
#[doc = "`read()` method returns [macppscr::R](macppscr::R) reader structure"]
impl crate::Readable for MACPPSCR {}
#[doc = "`write(|w| ..)` method takes [macppscr::W](macppscr::W) writer structure"]
impl crate::Writable for MACPPSCR {}
#[doc = "PPS control register"]
pub mod macppscr;
#[doc = "PPS target time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsttsr](macppsttsr) module"]
pub type MACPPSTTSR = crate::Reg<u32, _MACPPSTTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPPSTTSR;
#[doc = "`read()` method returns [macppsttsr::R](macppsttsr::R) reader structure"]
impl crate::Readable for MACPPSTTSR {}
#[doc = "`write(|w| ..)` method takes [macppsttsr::W](macppsttsr::W) writer structure"]
impl crate::Writable for MACPPSTTSR {}
#[doc = "PPS target time seconds register"]
pub mod macppsttsr;
#[doc = "PPS target time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsttnr](macppsttnr) module"]
pub type MACPPSTTNR = crate::Reg<u32, _MACPPSTTNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPPSTTNR;
#[doc = "`read()` method returns [macppsttnr::R](macppsttnr::R) reader structure"]
impl crate::Readable for MACPPSTTNR {}
#[doc = "`write(|w| ..)` method takes [macppsttnr::W](macppsttnr::W) writer structure"]
impl crate::Writable for MACPPSTTNR {}
#[doc = "PPS target time nanoseconds register"]
pub mod macppsttnr;
#[doc = "PPS interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsir](macppsir) module"]
pub type MACPPSIR = crate::Reg<u32, _MACPPSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPPSIR;
#[doc = "`read()` method returns [macppsir::R](macppsir::R) reader structure"]
impl crate::Readable for MACPPSIR {}
#[doc = "`write(|w| ..)` method takes [macppsir::W](macppsir::W) writer structure"]
impl crate::Writable for MACPPSIR {}
#[doc = "PPS interval register"]
pub mod macppsir;
#[doc = "PPS width register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppswr](macppswr) module"]
pub type MACPPSWR = crate::Reg<u32, _MACPPSWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPPSWR;
#[doc = "`read()` method returns [macppswr::R](macppswr::R) reader structure"]
impl crate::Readable for MACPPSWR {}
#[doc = "`write(|w| ..)` method takes [macppswr::W](macppswr::W) writer structure"]
impl crate::Writable for MACPPSWR {}
#[doc = "PPS width register"]
pub mod macppswr;
#[doc = "PTP Offload control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpocr](macpocr) module"]
pub type MACPOCR = crate::Reg<u32, _MACPOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACPOCR;
#[doc = "`read()` method returns [macpocr::R](macpocr::R) reader structure"]
impl crate::Readable for MACPOCR {}
#[doc = "`write(|w| ..)` method takes [macpocr::W](macpocr::W) writer structure"]
impl crate::Writable for MACPOCR {}
#[doc = "PTP Offload control register"]
pub mod macpocr;
#[doc = "PTP Source Port Identity 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macspi0r](macspi0r) module"]
pub type MACSPI0R = crate::Reg<u32, _MACSPI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSPI0R;
#[doc = "`read()` method returns [macspi0r::R](macspi0r::R) reader structure"]
impl crate::Readable for MACSPI0R {}
#[doc = "`write(|w| ..)` method takes [macspi0r::W](macspi0r::W) writer structure"]
impl crate::Writable for MACSPI0R {}
#[doc = "PTP Source Port Identity 0 Register"]
pub mod macspi0r;
#[doc = "PTP Source port identity 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macspi1r](macspi1r) module"]
pub type MACSPI1R = crate::Reg<u32, _MACSPI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSPI1R;
#[doc = "`read()` method returns [macspi1r::R](macspi1r::R) reader structure"]
impl crate::Readable for MACSPI1R {}
#[doc = "`write(|w| ..)` method takes [macspi1r::W](macspi1r::W) writer structure"]
impl crate::Writable for MACSPI1R {}
#[doc = "PTP Source port identity 1 register"]
pub mod macspi1r;
#[doc = "PTP Source port identity 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macspi2r](macspi2r) module"]
pub type MACSPI2R = crate::Reg<u32, _MACSPI2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACSPI2R;
#[doc = "`read()` method returns [macspi2r::R](macspi2r::R) reader structure"]
impl crate::Readable for MACSPI2R {}
#[doc = "`write(|w| ..)` method takes [macspi2r::W](macspi2r::W) writer structure"]
impl crate::Writable for MACSPI2R {}
#[doc = "PTP Source port identity 2 register"]
pub mod macspi2r;
#[doc = "Log message interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maclmir](maclmir) module"]
pub type MACLMIR = crate::Reg<u32, _MACLMIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACLMIR;
#[doc = "`read()` method returns [maclmir::R](maclmir::R) reader structure"]
impl crate::Readable for MACLMIR {}
#[doc = "`write(|w| ..)` method takes [maclmir::W](maclmir::W) writer structure"]
impl crate::Writable for MACLMIR {}
#[doc = "Log message interval register"]
pub mod maclmir;
