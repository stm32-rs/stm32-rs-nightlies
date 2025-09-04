#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    maccr: MACCR,
    macecr: MACECR,
    macpfr: MACPFR,
    macwtr: MACWTR,
    macht0r: MACHT0R,
    macht1r: MACHT1R,
    _reserved6: [u8; 0x38],
    macvtr: MACVTR,
    _reserved7: [u8; 0x04],
    macvhtr: MACVHTR,
    _reserved8: [u8; 0x04],
    macvir: MACVIR,
    macivir: MACIVIR,
    _reserved10: [u8; 0x08],
    macqtxfcr: MACQTXFCR,
    _reserved11: [u8; 0x1c],
    macrxfcr: MACRXFCR,
    _reserved12: [u8; 0x1c],
    macisr: MACISR,
    macier: MACIER,
    macrxtxsr: MACRXTXSR,
    _reserved15: [u8; 0x04],
    macpcsr: MACPCSR,
    macrwkpfr: MACRWKPFR,
    _reserved17: [u8; 0x08],
    maclcsr: MACLCSR,
    macltcr: MACLTCR,
    macletr: MACLETR,
    mac1ustcr: MAC1USTCR,
    _reserved21: [u8; 0x30],
    macvr: MACVR,
    macdr: MACDR,
    _reserved23: [u8; 0x04],
    machwf0r: MACHWF0R,
    machwf1r: MACHWF1R,
    machwf2r: MACHWF2R,
    machwf3r: MACHWF3R,
    _reserved27: [u8; 0xd4],
    macmdioar: MACMDIOAR,
    macmdiodr: MACMDIODR,
    _reserved29: [u8; 0x08],
    macarpar: MACARPAR,
    _reserved30: [u8; 0x1c],
    maccsrswcr: MACCSRSWCR,
    _reserved31: [u8; 0xcc],
    maca0hr: MACA0HR,
    maca0lr: MACA0LR,
    maca1hr: MACA1HR,
    maca1lr: MACA1LR,
    maca2hr: MACA2HR,
    maca2lr: MACA2LR,
    maca3hr: MACA3HR,
    maca3lr: MACA3LR,
    _reserved39: [u8; 0x03e0],
    mmc_control: MMC_CONTROL,
    mmc_rx_interrupt: MMC_RX_INTERRUPT,
    mmc_tx_interrupt: MMC_TX_INTERRUPT,
    mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved44: [u8; 0x38],
    tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved46: [u8; 0x14],
    tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved47: [u8; 0x28],
    rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved49: [u8; 0x28],
    rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved50: [u8; 0x24],
    tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved54: [u8; 0x0104],
    macl3l4c0r: MACL3L4C0R,
    macl4a0r: MACL4A0R,
    _reserved56: [u8; 0x08],
    macl3a00r: MACL3A00R,
    macl3a10r: MACL3A10R,
    macl3a20r: MACL3A20R,
    macl3a30r: MACL3A30R,
    _reserved60: [u8; 0x10],
    macl3l4c1r: MACL3L4C1R,
    macl4a1r: MACL4A1R,
    _reserved62: [u8; 0x08],
    macl3a01r: MACL3A01R,
    macl3a11r: MACL3A11R,
    macl3a21r: MACL3A21R,
    macl3a31r: MACL3A31R,
    _reserved66: [u8; 0x01b0],
    mactscr: MACTSCR,
    macssir: MACSSIR,
    macstsr: MACSTSR,
    macstnr: MACSTNR,
    macstsur: MACSTSUR,
    macstnur: MACSTNUR,
    mactsar: MACTSAR,
    _reserved73: [u8; 0x04],
    mactssr: MACTSSR,
    _reserved74: [u8; 0x0c],
    mactxtssnr: MACTXTSSNR,
    mactxtsssr: MACTXTSSSR,
    _reserved76: [u8; 0x08],
    macacr: MACACR,
    _reserved77: [u8; 0x04],
    macatsnr: MACATSNR,
    macatssr: MACATSSR,
    mactsiacr: MACTSIACR,
    mactseacr: MACTSEACR,
    mactsicnr: MACTSICNR,
    mactsecnr: MACTSECNR,
    _reserved83: [u8; 0x10],
    _reserved_83_macppscr: [u8; 0x04],
    _reserved84: [u8; 0x0c],
    macppsttsr: MACPPSTTSR,
    macppsttnr: MACPPSTTNR,
    macppsir: MACPPSIR,
    macppswr: MACPPSWR,
    _reserved88: [u8; 0x30],
    macpocr: MACPOCR,
    macspi0r: MACSPI0R,
    macspi1r: MACSPI1R,
    macspi2r: MACSPI2R,
    maclmir: MACLMIR,
    _reserved93: [u8; 0x2c],
    mtlomr: MTLOMR,
    _reserved94: [u8; 0x1c],
    mtlisr: MTLISR,
    _reserved95: [u8; 0xdc],
    mtltxqomr: MTLTXQOMR,
    mtltxqur: MTLTXQUR,
    mtltxqdr: MTLTXQDR,
    _reserved98: [u8; 0x20],
    mtlqicsr: MTLQICSR,
    mtlrxqomr: MTLRXQOMR,
    mtlrxqmpocr: MTLRXQMPOCR,
    mtlrxqdr: MTLRXQDR,
    _reserved102: [u8; 0x02c4],
    dmamr: DMAMR,
    dmasbmr: DMASBMR,
    dmaisr: DMAISR,
    dmadsr: DMADSR,
    _reserved106: [u8; 0xf0],
    dmaccr: DMACCR,
    dmactxcr: DMACTXCR,
    dmacrxcr: DMACRXCR,
    _reserved109: [u8; 0x08],
    dmactxdlar: DMACTXDLAR,
    _reserved110: [u8; 0x04],
    dmacrxdlar: DMACRXDLAR,
    dmactxdtpr: DMACTXDTPR,
    _reserved112: [u8; 0x04],
    dmacrxdtpr: DMACRXDTPR,
    dmactxrlr: DMACTXRLR,
    dmacrxrlr: DMACRXRLR,
    dmacier: DMACIER,
    dmacrxiwtr: DMACRXIWTR,
    _reserved117: [u8; 0x08],
    dmaccatxdr: DMACCATXDR,
    _reserved118: [u8; 0x04],
    dmaccarxdr: DMACCARXDR,
    _reserved119: [u8; 0x04],
    dmaccatxbr: DMACCATXBR,
    _reserved120: [u8; 0x04],
    dmaccarxbr: DMACCARXBR,
    dmacsr: DMACSR,
    _reserved122: [u8; 0x08],
    dmacmfcr: DMACMFCR,
}
impl RegisterBlock {
    ///0x00 - Operating mode configuration register
    #[inline(always)]
    pub const fn maccr(&self) -> &MACCR {
        &self.maccr
    }
    ///0x04 - Extended operating mode configuration register
    #[inline(always)]
    pub const fn macecr(&self) -> &MACECR {
        &self.macecr
    }
    ///0x08 - Packet filtering control register
    #[inline(always)]
    pub const fn macpfr(&self) -> &MACPFR {
        &self.macpfr
    }
    ///0x0c - Watchdog timeout register
    #[inline(always)]
    pub const fn macwtr(&self) -> &MACWTR {
        &self.macwtr
    }
    ///0x10 - Hash Table 0 register
    #[inline(always)]
    pub const fn macht0r(&self) -> &MACHT0R {
        &self.macht0r
    }
    ///0x14 - Hash Table 1 register
    #[inline(always)]
    pub const fn macht1r(&self) -> &MACHT1R {
        &self.macht1r
    }
    ///0x50 - VLAN tag register
    #[inline(always)]
    pub const fn macvtr(&self) -> &MACVTR {
        &self.macvtr
    }
    ///0x58 - VLAN Hash table register
    #[inline(always)]
    pub const fn macvhtr(&self) -> &MACVHTR {
        &self.macvhtr
    }
    ///0x60 - VLAN inclusion register
    #[inline(always)]
    pub const fn macvir(&self) -> &MACVIR {
        &self.macvir
    }
    ///0x64 - Inner VLAN inclusion register
    #[inline(always)]
    pub const fn macivir(&self) -> &MACIVIR {
        &self.macivir
    }
    ///0x70 - Tx Queue flow control register
    #[inline(always)]
    pub const fn macqtxfcr(&self) -> &MACQTXFCR {
        &self.macqtxfcr
    }
    ///0x90 - Rx flow control register
    #[inline(always)]
    pub const fn macrxfcr(&self) -> &MACRXFCR {
        &self.macrxfcr
    }
    ///0xb0 - Interrupt status register
    #[inline(always)]
    pub const fn macisr(&self) -> &MACISR {
        &self.macisr
    }
    ///0xb4 - Interrupt enable register
    #[inline(always)]
    pub const fn macier(&self) -> &MACIER {
        &self.macier
    }
    ///0xb8 - Rx Tx status register
    #[inline(always)]
    pub const fn macrxtxsr(&self) -> &MACRXTXSR {
        &self.macrxtxsr
    }
    ///0xc0 - PMT control status register
    #[inline(always)]
    pub const fn macpcsr(&self) -> &MACPCSR {
        &self.macpcsr
    }
    ///0xc4 - Remote wakeup packet filter register
    #[inline(always)]
    pub const fn macrwkpfr(&self) -> &MACRWKPFR {
        &self.macrwkpfr
    }
    ///0xd0 - LPI control and status register
    #[inline(always)]
    pub const fn maclcsr(&self) -> &MACLCSR {
        &self.maclcsr
    }
    ///0xd4 - LPI timers control register
    #[inline(always)]
    pub const fn macltcr(&self) -> &MACLTCR {
        &self.macltcr
    }
    ///0xd8 - LPI entry timer register
    #[inline(always)]
    pub const fn macletr(&self) -> &MACLETR {
        &self.macletr
    }
    ///0xdc - One-microsecond-tick counter register
    #[inline(always)]
    pub const fn mac1ustcr(&self) -> &MAC1USTCR {
        &self.mac1ustcr
    }
    ///0x110 - Version register
    #[inline(always)]
    pub const fn macvr(&self) -> &MACVR {
        &self.macvr
    }
    ///0x114 - Debug register
    #[inline(always)]
    pub const fn macdr(&self) -> &MACDR {
        &self.macdr
    }
    ///0x11c - HW feature 0 register
    #[inline(always)]
    pub const fn machwf0r(&self) -> &MACHWF0R {
        &self.machwf0r
    }
    ///0x120 - HW feature 1 register
    #[inline(always)]
    pub const fn machwf1r(&self) -> &MACHWF1R {
        &self.machwf1r
    }
    ///0x124 - HW feature 2 register
    #[inline(always)]
    pub const fn machwf2r(&self) -> &MACHWF2R {
        &self.machwf2r
    }
    ///0x128 - HW feature 3 register
    #[inline(always)]
    pub const fn machwf3r(&self) -> &MACHWF3R {
        &self.machwf3r
    }
    ///0x200 - MDIO address register
    #[inline(always)]
    pub const fn macmdioar(&self) -> &MACMDIOAR {
        &self.macmdioar
    }
    ///0x204 - MDIO data register
    #[inline(always)]
    pub const fn macmdiodr(&self) -> &MACMDIODR {
        &self.macmdiodr
    }
    ///0x210 - ARP address register
    #[inline(always)]
    pub const fn macarpar(&self) -> &MACARPAR {
        &self.macarpar
    }
    ///0x230 - CSR software control register
    #[inline(always)]
    pub const fn maccsrswcr(&self) -> &MACCSRSWCR {
        &self.maccsrswcr
    }
    ///0x300 - MAC Address 0 high register
    #[inline(always)]
    pub const fn maca0hr(&self) -> &MACA0HR {
        &self.maca0hr
    }
    ///0x304 - MAC Address 0 low register
    #[inline(always)]
    pub const fn maca0lr(&self) -> &MACA0LR {
        &self.maca0lr
    }
    ///0x308 - MAC Address 1 high register
    #[inline(always)]
    pub const fn maca1hr(&self) -> &MACA1HR {
        &self.maca1hr
    }
    ///0x30c - MAC Address 1 low register
    #[inline(always)]
    pub const fn maca1lr(&self) -> &MACA1LR {
        &self.maca1lr
    }
    ///0x310 - MAC Address 2 high register
    #[inline(always)]
    pub const fn maca2hr(&self) -> &MACA2HR {
        &self.maca2hr
    }
    ///0x314 - MAC Address 2 low register
    #[inline(always)]
    pub const fn maca2lr(&self) -> &MACA2LR {
        &self.maca2lr
    }
    ///0x318 - MAC Address 3 high register
    #[inline(always)]
    pub const fn maca3hr(&self) -> &MACA3HR {
        &self.maca3hr
    }
    ///0x31c - MAC Address 3 low register
    #[inline(always)]
    pub const fn maca3lr(&self) -> &MACA3LR {
        &self.maca3lr
    }
    ///0x700 - MMC control register
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MMC_CONTROL {
        &self.mmc_control
    }
    ///0x704 - MMC Rx interrupt register
    #[inline(always)]
    pub const fn mmc_rx_interrupt(&self) -> &MMC_RX_INTERRUPT {
        &self.mmc_rx_interrupt
    }
    ///0x708 - MMC Tx interrupt register
    #[inline(always)]
    pub const fn mmc_tx_interrupt(&self) -> &MMC_TX_INTERRUPT {
        &self.mmc_tx_interrupt
    }
    ///0x70c - MMC Rx interrupt mask register
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(&self) -> &MMC_RX_INTERRUPT_MASK {
        &self.mmc_rx_interrupt_mask
    }
    ///0x710 - MMC Tx interrupt mask register
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(&self) -> &MMC_TX_INTERRUPT_MASK {
        &self.mmc_tx_interrupt_mask
    }
    ///0x74c - Tx single collision good packets register
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(&self) -> &TX_SINGLE_COLLISION_GOOD_PACKETS {
        &self.tx_single_collision_good_packets
    }
    ///0x750 - Tx multiple collision good packets register
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(&self) -> &TX_MULTIPLE_COLLISION_GOOD_PACKETS {
        &self.tx_multiple_collision_good_packets
    }
    ///0x768 - Tx packet count good register
    #[inline(always)]
    pub const fn tx_packet_count_good(&self) -> &TX_PACKET_COUNT_GOOD {
        &self.tx_packet_count_good
    }
    ///0x794 - Rx CRC error packets register
    #[inline(always)]
    pub const fn rx_crc_error_packets(&self) -> &RX_CRC_ERROR_PACKETS {
        &self.rx_crc_error_packets
    }
    ///0x798 - Rx alignment error packets register
    #[inline(always)]
    pub const fn rx_alignment_error_packets(&self) -> &RX_ALIGNMENT_ERROR_PACKETS {
        &self.rx_alignment_error_packets
    }
    ///0x7c4 - Rx unicast packets good register
    #[inline(always)]
    pub const fn rx_unicast_packets_good(&self) -> &RX_UNICAST_PACKETS_GOOD {
        &self.rx_unicast_packets_good
    }
    ///0x7ec - Tx LPI microsecond timer register
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(&self) -> &TX_LPI_USEC_CNTR {
        &self.tx_lpi_usec_cntr
    }
    ///0x7f0 - Tx LPI transition counter register
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(&self) -> &TX_LPI_TRAN_CNTR {
        &self.tx_lpi_tran_cntr
    }
    ///0x7f4 - Rx LPI microsecond counter register
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(&self) -> &RX_LPI_USEC_CNTR {
        &self.rx_lpi_usec_cntr
    }
    ///0x7f8 - Rx LPI transition counter register
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(&self) -> &RX_LPI_TRAN_CNTR {
        &self.rx_lpi_tran_cntr
    }
    ///0x900 - L3 and L4 control 0 register
    #[inline(always)]
    pub const fn macl3l4c0r(&self) -> &MACL3L4C0R {
        &self.macl3l4c0r
    }
    ///0x904 - Layer4 Address filter 0 register
    #[inline(always)]
    pub const fn macl4a0r(&self) -> &MACL4A0R {
        &self.macl4a0r
    }
    ///0x910 - Layer3 Address 0 filter 0 register
    #[inline(always)]
    pub const fn macl3a00r(&self) -> &MACL3A00R {
        &self.macl3a00r
    }
    ///0x914 - Layer3 Address 1 filter 0 register
    #[inline(always)]
    pub const fn macl3a10r(&self) -> &MACL3A10R {
        &self.macl3a10r
    }
    ///0x918 - Layer3 Address 2 filter 0 register
    #[inline(always)]
    pub const fn macl3a20r(&self) -> &MACL3A20R {
        &self.macl3a20r
    }
    ///0x91c - Layer3 Address 3 filter 0 register
    #[inline(always)]
    pub const fn macl3a30r(&self) -> &MACL3A30R {
        &self.macl3a30r
    }
    ///0x930 - L3 and L4 control 1 register
    #[inline(always)]
    pub const fn macl3l4c1r(&self) -> &MACL3L4C1R {
        &self.macl3l4c1r
    }
    ///0x934 - Layer 4 address filter 1 register
    #[inline(always)]
    pub const fn macl4a1r(&self) -> &MACL4A1R {
        &self.macl4a1r
    }
    ///0x940 - Layer3 address 0 filter 1 Register
    #[inline(always)]
    pub const fn macl3a01r(&self) -> &MACL3A01R {
        &self.macl3a01r
    }
    ///0x944 - Layer3 address 1 filter 1 register
    #[inline(always)]
    pub const fn macl3a11r(&self) -> &MACL3A11R {
        &self.macl3a11r
    }
    ///0x948 - Layer3 address 2 filter 1 Register
    #[inline(always)]
    pub const fn macl3a21r(&self) -> &MACL3A21R {
        &self.macl3a21r
    }
    ///0x94c - Layer3 address 3 filter 1 register
    #[inline(always)]
    pub const fn macl3a31r(&self) -> &MACL3A31R {
        &self.macl3a31r
    }
    ///0xb00 - Timestamp control Register
    #[inline(always)]
    pub const fn mactscr(&self) -> &MACTSCR {
        &self.mactscr
    }
    ///0xb04 - Subsecond increment register
    #[inline(always)]
    pub const fn macssir(&self) -> &MACSSIR {
        &self.macssir
    }
    ///0xb08 - System time seconds register
    #[inline(always)]
    pub const fn macstsr(&self) -> &MACSTSR {
        &self.macstsr
    }
    ///0xb0c - System time nanoseconds register
    #[inline(always)]
    pub const fn macstnr(&self) -> &MACSTNR {
        &self.macstnr
    }
    ///0xb10 - System time seconds update register
    #[inline(always)]
    pub const fn macstsur(&self) -> &MACSTSUR {
        &self.macstsur
    }
    ///0xb14 - System time nanoseconds update register
    #[inline(always)]
    pub const fn macstnur(&self) -> &MACSTNUR {
        &self.macstnur
    }
    ///0xb18 - Timestamp addend register
    #[inline(always)]
    pub const fn mactsar(&self) -> &MACTSAR {
        &self.mactsar
    }
    ///0xb20 - Timestamp status register
    #[inline(always)]
    pub const fn mactssr(&self) -> &MACTSSR {
        &self.mactssr
    }
    ///0xb30 - Tx timestamp status nanoseconds register
    #[inline(always)]
    pub const fn mactxtssnr(&self) -> &MACTXTSSNR {
        &self.mactxtssnr
    }
    ///0xb34 - Tx timestamp status seconds register
    #[inline(always)]
    pub const fn mactxtsssr(&self) -> &MACTXTSSSR {
        &self.mactxtsssr
    }
    ///0xb40 - Auxiliary control register
    #[inline(always)]
    pub const fn macacr(&self) -> &MACACR {
        &self.macacr
    }
    ///0xb48 - Auxiliary timestamp nanoseconds register
    #[inline(always)]
    pub const fn macatsnr(&self) -> &MACATSNR {
        &self.macatsnr
    }
    ///0xb4c - Auxiliary timestamp seconds register
    #[inline(always)]
    pub const fn macatssr(&self) -> &MACATSSR {
        &self.macatssr
    }
    ///0xb50 - Timestamp Ingress asymmetric correction register
    #[inline(always)]
    pub const fn mactsiacr(&self) -> &MACTSIACR {
        &self.mactsiacr
    }
    ///0xb54 - Timestamp Egress asymmetric correction register
    #[inline(always)]
    pub const fn mactseacr(&self) -> &MACTSEACR {
        &self.mactseacr
    }
    ///0xb58 - Timestamp Ingress correction nanosecond register
    #[inline(always)]
    pub const fn mactsicnr(&self) -> &MACTSICNR {
        &self.mactsicnr
    }
    ///0xb5c - Timestamp Egress correction nanosecond register
    #[inline(always)]
    pub const fn mactsecnr(&self) -> &MACTSECNR {
        &self.mactsecnr
    }
    ///0xb70 - PPS control register
    #[inline(always)]
    pub const fn macppscr_alternate(&self) -> &MACPPSCR_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2928).cast() }
    }
    ///0xb70 - PPS control register
    #[inline(always)]
    pub const fn macppscr(&self) -> &MACPPSCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2928).cast() }
    }
    ///0xb80 - PPS target time seconds register
    #[inline(always)]
    pub const fn macppsttsr(&self) -> &MACPPSTTSR {
        &self.macppsttsr
    }
    ///0xb84 - PPS target time nanoseconds register
    #[inline(always)]
    pub const fn macppsttnr(&self) -> &MACPPSTTNR {
        &self.macppsttnr
    }
    ///0xb88 - PPS interval register
    #[inline(always)]
    pub const fn macppsir(&self) -> &MACPPSIR {
        &self.macppsir
    }
    ///0xb8c - PPS width register
    #[inline(always)]
    pub const fn macppswr(&self) -> &MACPPSWR {
        &self.macppswr
    }
    ///0xbc0 - PTP Offload control register
    #[inline(always)]
    pub const fn macpocr(&self) -> &MACPOCR {
        &self.macpocr
    }
    ///0xbc4 - PTP Source Port Identity 0 Register
    #[inline(always)]
    pub const fn macspi0r(&self) -> &MACSPI0R {
        &self.macspi0r
    }
    ///0xbc8 - PTP Source port identity 1 register
    #[inline(always)]
    pub const fn macspi1r(&self) -> &MACSPI1R {
        &self.macspi1r
    }
    ///0xbcc - PTP Source port identity 2 register
    #[inline(always)]
    pub const fn macspi2r(&self) -> &MACSPI2R {
        &self.macspi2r
    }
    ///0xbd0 - Log message interval register
    #[inline(always)]
    pub const fn maclmir(&self) -> &MACLMIR {
        &self.maclmir
    }
    ///0xc00 - Operating mode Register
    #[inline(always)]
    pub const fn mtlomr(&self) -> &MTLOMR {
        &self.mtlomr
    }
    ///0xc20 - Interrupt status Register
    #[inline(always)]
    pub const fn mtlisr(&self) -> &MTLISR {
        &self.mtlisr
    }
    ///0xd00 - Tx queue operating mode Register
    #[inline(always)]
    pub const fn mtltxqomr(&self) -> &MTLTXQOMR {
        &self.mtltxqomr
    }
    ///0xd04 - Tx queue underflow register
    #[inline(always)]
    pub const fn mtltxqur(&self) -> &MTLTXQUR {
        &self.mtltxqur
    }
    ///0xd08 - Tx queue debug Register
    #[inline(always)]
    pub const fn mtltxqdr(&self) -> &MTLTXQDR {
        &self.mtltxqdr
    }
    ///0xd2c - Queue interrupt control status Register
    #[inline(always)]
    pub const fn mtlqicsr(&self) -> &MTLQICSR {
        &self.mtlqicsr
    }
    ///0xd30 - Rx queue operating mode register
    #[inline(always)]
    pub const fn mtlrxqomr(&self) -> &MTLRXQOMR {
        &self.mtlrxqomr
    }
    ///0xd34 - Rx queue missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrxqmpocr(&self) -> &MTLRXQMPOCR {
        &self.mtlrxqmpocr
    }
    ///0xd38 - Rx queue debug register
    #[inline(always)]
    pub const fn mtlrxqdr(&self) -> &MTLRXQDR {
        &self.mtlrxqdr
    }
    ///0x1000 - DMA mode register
    #[inline(always)]
    pub const fn dmamr(&self) -> &DMAMR {
        &self.dmamr
    }
    ///0x1004 - System bus mode register
    #[inline(always)]
    pub const fn dmasbmr(&self) -> &DMASBMR {
        &self.dmasbmr
    }
    ///0x1008 - Interrupt status register
    #[inline(always)]
    pub const fn dmaisr(&self) -> &DMAISR {
        &self.dmaisr
    }
    ///0x100c - Debug status register
    #[inline(always)]
    pub const fn dmadsr(&self) -> &DMADSR {
        &self.dmadsr
    }
    ///0x1100 - Channel control register
    #[inline(always)]
    pub const fn dmaccr(&self) -> &DMACCR {
        &self.dmaccr
    }
    ///0x1104 - Channel transmit control register
    #[inline(always)]
    pub const fn dmactxcr(&self) -> &DMACTXCR {
        &self.dmactxcr
    }
    ///0x1108 - Channel receive control register
    #[inline(always)]
    pub const fn dmacrxcr(&self) -> &DMACRXCR {
        &self.dmacrxcr
    }
    ///0x1114 - Channel Tx descriptor list address register
    #[inline(always)]
    pub const fn dmactxdlar(&self) -> &DMACTXDLAR {
        &self.dmactxdlar
    }
    ///0x111c - Channel Rx descriptor list address register
    #[inline(always)]
    pub const fn dmacrxdlar(&self) -> &DMACRXDLAR {
        &self.dmacrxdlar
    }
    ///0x1120 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmactxdtpr(&self) -> &DMACTXDTPR {
        &self.dmactxdtpr
    }
    ///0x1128 - Channel Rx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmacrxdtpr(&self) -> &DMACRXDTPR {
        &self.dmacrxdtpr
    }
    ///0x112c - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn dmactxrlr(&self) -> &DMACTXRLR {
        &self.dmactxrlr
    }
    ///0x1130 - Channel Rx descriptor ring length register
    #[inline(always)]
    pub const fn dmacrxrlr(&self) -> &DMACRXRLR {
        &self.dmacrxrlr
    }
    ///0x1134 - Channel interrupt enable register
    #[inline(always)]
    pub const fn dmacier(&self) -> &DMACIER {
        &self.dmacier
    }
    ///0x1138 - Channel Rx interrupt watchdog timer register
    #[inline(always)]
    pub const fn dmacrxiwtr(&self) -> &DMACRXIWTR {
        &self.dmacrxiwtr
    }
    ///0x1144 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn dmaccatxdr(&self) -> &DMACCATXDR {
        &self.dmaccatxdr
    }
    ///0x114c - Channel current application receive descriptor register
    #[inline(always)]
    pub const fn dmaccarxdr(&self) -> &DMACCARXDR {
        &self.dmaccarxdr
    }
    ///0x1154 - Channel current application transmit buffer register
    #[inline(always)]
    pub const fn dmaccatxbr(&self) -> &DMACCATXBR {
        &self.dmaccatxbr
    }
    ///0x115c - Channel current application receive buffer register
    #[inline(always)]
    pub const fn dmaccarxbr(&self) -> &DMACCARXBR {
        &self.dmaccarxbr
    }
    ///0x1160 - Channel status register
    #[inline(always)]
    pub const fn dmacsr(&self) -> &DMACSR {
        &self.dmacsr
    }
    ///0x116c - Channel missed frame count register
    #[inline(always)]
    pub const fn dmacmfcr(&self) -> &DMACMFCR {
        &self.dmacmfcr
    }
}
/**MACCR (rw) register accessor: Operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACCR)

For information about available fields see [`mod@maccr`] module*/
pub type MACCR = crate::Reg<maccr::MACCRrs>;
///Operating mode configuration register
pub mod maccr;
/**MACECR (rw) register accessor: Extended operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`macecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACECR)

For information about available fields see [`mod@macecr`] module*/
pub type MACECR = crate::Reg<macecr::MACECRrs>;
///Extended operating mode configuration register
pub mod macecr;
/**MACPFR (rw) register accessor: Packet filtering control register

You can [`read`](crate::Reg::read) this register and get [`macpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPFR)

For information about available fields see [`mod@macpfr`] module*/
pub type MACPFR = crate::Reg<macpfr::MACPFRrs>;
///Packet filtering control register
pub mod macpfr;
/**MACWTR (rw) register accessor: Watchdog timeout register

You can [`read`](crate::Reg::read) this register and get [`macwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACWTR)

For information about available fields see [`mod@macwtr`] module*/
pub type MACWTR = crate::Reg<macwtr::MACWTRrs>;
///Watchdog timeout register
pub mod macwtr;
/**MACHT0R (rw) register accessor: Hash Table 0 register

You can [`read`](crate::Reg::read) this register and get [`macht0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHT0R)

For information about available fields see [`mod@macht0r`] module*/
pub type MACHT0R = crate::Reg<macht0r::MACHT0Rrs>;
///Hash Table 0 register
pub mod macht0r;
/**MACHT1R (rw) register accessor: Hash Table 1 register

You can [`read`](crate::Reg::read) this register and get [`macht1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHT1R)

For information about available fields see [`mod@macht1r`] module*/
pub type MACHT1R = crate::Reg<macht1r::MACHT1Rrs>;
///Hash Table 1 register
pub mod macht1r;
/**MACVTR (rw) register accessor: VLAN tag register

You can [`read`](crate::Reg::read) this register and get [`macvtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACVTR)

For information about available fields see [`mod@macvtr`] module*/
pub type MACVTR = crate::Reg<macvtr::MACVTRrs>;
///VLAN tag register
pub mod macvtr;
/**MACVHTR (rw) register accessor: VLAN Hash table register

You can [`read`](crate::Reg::read) this register and get [`macvhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACVHTR)

For information about available fields see [`mod@macvhtr`] module*/
pub type MACVHTR = crate::Reg<macvhtr::MACVHTRrs>;
///VLAN Hash table register
pub mod macvhtr;
/**MACVIR (rw) register accessor: VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macvir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACVIR)

For information about available fields see [`mod@macvir`] module*/
pub type MACVIR = crate::Reg<macvir::MACVIRrs>;
///VLAN inclusion register
pub mod macvir;
/**MACIVIR (rw) register accessor: Inner VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macivir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macivir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACIVIR)

For information about available fields see [`mod@macivir`] module*/
pub type MACIVIR = crate::Reg<macivir::MACIVIRrs>;
///Inner VLAN inclusion register
pub mod macivir;
/**MACQTXFCR (rw) register accessor: Tx Queue flow control register

You can [`read`](crate::Reg::read) this register and get [`macqtxfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macqtxfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACQTXFCR)

For information about available fields see [`mod@macqtxfcr`] module*/
pub type MACQTXFCR = crate::Reg<macqtxfcr::MACQTXFCRrs>;
///Tx Queue flow control register
pub mod macqtxfcr;
/**MACRXFCR (rw) register accessor: Rx flow control register

You can [`read`](crate::Reg::read) this register and get [`macrxfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACRXFCR)

For information about available fields see [`mod@macrxfcr`] module*/
pub type MACRXFCR = crate::Reg<macrxfcr::MACRXFCRrs>;
///Rx flow control register
pub mod macrxfcr;
/**MACISR (rw) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`macisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACISR)

For information about available fields see [`mod@macisr`] module*/
pub type MACISR = crate::Reg<macisr::MACISRrs>;
///Interrupt status register
pub mod macisr;
/**MACIER (rw) register accessor: Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`macier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACIER)

For information about available fields see [`mod@macier`] module*/
pub type MACIER = crate::Reg<macier::MACIERrs>;
///Interrupt enable register
pub mod macier;
/**MACRXTXSR (rw) register accessor: Rx Tx status register

You can [`read`](crate::Reg::read) this register and get [`macrxtxsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxtxsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACRXTXSR)

For information about available fields see [`mod@macrxtxsr`] module*/
pub type MACRXTXSR = crate::Reg<macrxtxsr::MACRXTXSRrs>;
///Rx Tx status register
pub mod macrxtxsr;
/**MACPCSR (rw) register accessor: PMT control status register

You can [`read`](crate::Reg::read) this register and get [`macpcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPCSR)

For information about available fields see [`mod@macpcsr`] module*/
pub type MACPCSR = crate::Reg<macpcsr::MACPCSRrs>;
///PMT control status register
pub mod macpcsr;
/**MACRWKPFR (rw) register accessor: Remote wakeup packet filter register

You can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACRWKPFR)

For information about available fields see [`mod@macrwkpfr`] module*/
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFRrs>;
///Remote wakeup packet filter register
pub mod macrwkpfr;
/**MACLCSR (rw) register accessor: LPI control and status register

You can [`read`](crate::Reg::read) this register and get [`maclcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACLCSR)

For information about available fields see [`mod@maclcsr`] module*/
pub type MACLCSR = crate::Reg<maclcsr::MACLCSRrs>;
///LPI control and status register
pub mod maclcsr;
/**MACLTCR (rw) register accessor: LPI timers control register

You can [`read`](crate::Reg::read) this register and get [`macltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACLTCR)

For information about available fields see [`mod@macltcr`] module*/
pub type MACLTCR = crate::Reg<macltcr::MACLTCRrs>;
///LPI timers control register
pub mod macltcr;
/**MACLETR (rw) register accessor: LPI entry timer register

You can [`read`](crate::Reg::read) this register and get [`macletr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACLETR)

For information about available fields see [`mod@macletr`] module*/
pub type MACLETR = crate::Reg<macletr::MACLETRrs>;
///LPI entry timer register
pub mod macletr;
/**MAC1USTCR (rw) register accessor: One-microsecond-tick counter register

You can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MAC1USTCR)

For information about available fields see [`mod@mac1ustcr`] module*/
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCRrs>;
///One-microsecond-tick counter register
pub mod mac1ustcr;
/**MACVR (r) register accessor: Version register

You can [`read`](crate::Reg::read) this register and get [`macvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACVR)

For information about available fields see [`mod@macvr`] module*/
pub type MACVR = crate::Reg<macvr::MACVRrs>;
///Version register
pub mod macvr;
/**MACDR (r) register accessor: Debug register

You can [`read`](crate::Reg::read) this register and get [`macdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACDR)

For information about available fields see [`mod@macdr`] module*/
pub type MACDR = crate::Reg<macdr::MACDRrs>;
///Debug register
pub mod macdr;
/**MACHWF0R (r) register accessor: HW feature 0 register

You can [`read`](crate::Reg::read) this register and get [`machwf0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHWF0R)

For information about available fields see [`mod@machwf0r`] module*/
pub type MACHWF0R = crate::Reg<machwf0r::MACHWF0Rrs>;
///HW feature 0 register
pub mod machwf0r;
/**MACHWF1R (r) register accessor: HW feature 1 register

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHWF1R)

For information about available fields see [`mod@machwf1r`] module*/
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1Rrs>;
///HW feature 1 register
pub mod machwf1r;
/**MACHWF2R (r) register accessor: HW feature 2 register

You can [`read`](crate::Reg::read) this register and get [`machwf2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHWF2R)

For information about available fields see [`mod@machwf2r`] module*/
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2Rrs>;
///HW feature 2 register
pub mod machwf2r;
/**MACHWF3R (r) register accessor: HW feature 3 register

You can [`read`](crate::Reg::read) this register and get [`machwf3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACHWF3R)

For information about available fields see [`mod@machwf3r`] module*/
pub type MACHWF3R = crate::Reg<machwf3r::MACHWF3Rrs>;
///HW feature 3 register
pub mod machwf3r;
/**MACMDIOAR (rw) register accessor: MDIO address register

You can [`read`](crate::Reg::read) this register and get [`macmdioar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACMDIOAR)

For information about available fields see [`mod@macmdioar`] module*/
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOARrs>;
///MDIO address register
pub mod macmdioar;
/**MACMDIODR (rw) register accessor: MDIO data register

You can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACMDIODR)

For information about available fields see [`mod@macmdiodr`] module*/
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODRrs>;
///MDIO data register
pub mod macmdiodr;
/**MACARPAR (rw) register accessor: ARP address register

You can [`read`](crate::Reg::read) this register and get [`macarpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACARPAR)

For information about available fields see [`mod@macarpar`] module*/
pub type MACARPAR = crate::Reg<macarpar::MACARPARrs>;
///ARP address register
pub mod macarpar;
/**MACCSRSWCR (rw) register accessor: CSR software control register

You can [`read`](crate::Reg::read) this register and get [`maccsrswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccsrswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACCSRSWCR)

For information about available fields see [`mod@maccsrswcr`] module*/
pub type MACCSRSWCR = crate::Reg<maccsrswcr::MACCSRSWCRrs>;
///CSR software control register
pub mod maccsrswcr;
/**MACA0HR (rw) register accessor: MAC Address 0 high register

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA0HR)

For information about available fields see [`mod@maca0hr`] module*/
pub type MACA0HR = crate::Reg<maca0hr::MACA0HRrs>;
///MAC Address 0 high register
pub mod maca0hr;
/**MACA0LR (rw) register accessor: MAC Address 0 low register

You can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA0LR)

For information about available fields see [`mod@maca0lr`] module*/
pub type MACA0LR = crate::Reg<maca0lr::MACA0LRrs>;
///MAC Address 0 low register
pub mod maca0lr;
/**MACA1HR (rw) register accessor: MAC Address 1 high register

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA1HR)

For information about available fields see [`mod@maca1hr`] module*/
pub type MACA1HR = crate::Reg<maca1hr::MACA1HRrs>;
///MAC Address 1 high register
pub mod maca1hr;
/**MACA1LR (rw) register accessor: MAC Address 1 low register

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA1LR)

For information about available fields see [`mod@maca1lr`] module*/
pub type MACA1LR = crate::Reg<maca1lr::MACA1LRrs>;
///MAC Address 1 low register
pub mod maca1lr;
/**MACA2HR (rw) register accessor: MAC Address 2 high register

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA2HR)

For information about available fields see [`mod@maca2hr`] module*/
pub type MACA2HR = crate::Reg<maca2hr::MACA2HRrs>;
///MAC Address 2 high register
pub mod maca2hr;
/**MACA2LR (rw) register accessor: MAC Address 2 low register

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA2LR)

For information about available fields see [`mod@maca2lr`] module*/
pub type MACA2LR = crate::Reg<maca2lr::MACA2LRrs>;
///MAC Address 2 low register
pub mod maca2lr;
/**MACA3HR (rw) register accessor: MAC Address 3 high register

You can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA3HR)

For information about available fields see [`mod@maca3hr`] module*/
pub type MACA3HR = crate::Reg<maca3hr::MACA3HRrs>;
///MAC Address 3 high register
pub mod maca3hr;
/**MACA3LR (rw) register accessor: MAC Address 3 low register

You can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACA3LR)

For information about available fields see [`mod@maca3lr`] module*/
pub type MACA3LR = crate::Reg<maca3lr::MACA3LRrs>;
///MAC Address 3 low register
pub mod maca3lr;
/**MMC_CONTROL (rw) register accessor: MMC control register

You can [`read`](crate::Reg::read) this register and get [`mmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MMC_CONTROL)

For information about available fields see [`mod@mmc_control`] module*/
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROLrs>;
///MMC control register
pub mod mmc_control;
/**MMC_RX_INTERRUPT (rw) register accessor: MMC Rx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MMC_RX_INTERRUPT)

For information about available fields see [`mod@mmc_rx_interrupt`] module*/
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPTrs>;
///MMC Rx interrupt register
pub mod mmc_rx_interrupt;
/**MMC_TX_INTERRUPT (rw) register accessor: MMC Tx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MMC_TX_INTERRUPT)

For information about available fields see [`mod@mmc_tx_interrupt`] module*/
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPTrs>;
///MMC Tx interrupt register
pub mod mmc_tx_interrupt;
/**MMC_RX_INTERRUPT_MASK (rw) register accessor: MMC Rx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MMC_RX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_rx_interrupt_mask`] module*/
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASKrs>;
///MMC Rx interrupt mask register
pub mod mmc_rx_interrupt_mask;
/**MMC_TX_INTERRUPT_MASK (rw) register accessor: MMC Tx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MMC_TX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_tx_interrupt_mask`] module*/
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASKrs>;
///MMC Tx interrupt mask register
pub mod mmc_tx_interrupt_mask;
/**TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx single collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:TX_SINGLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_single_collision_good_packets`] module*/
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
///Tx single collision good packets register
pub mod tx_single_collision_good_packets;
/**TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx multiple collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:TX_MULTIPLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_multiple_collision_good_packets`] module*/
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETSrs>;
///Tx multiple collision good packets register
pub mod tx_multiple_collision_good_packets;
/**TX_PACKET_COUNT_GOOD (r) register accessor: Tx packet count good register

You can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:TX_PACKET_COUNT_GOOD)

For information about available fields see [`mod@tx_packet_count_good`] module*/
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOODrs>;
///Tx packet count good register
pub mod tx_packet_count_good;
/**RX_CRC_ERROR_PACKETS (r) register accessor: Rx CRC error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:RX_CRC_ERROR_PACKETS)

For information about available fields see [`mod@rx_crc_error_packets`] module*/
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETSrs>;
///Rx CRC error packets register
pub mod rx_crc_error_packets;
/**RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: Rx alignment error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:RX_ALIGNMENT_ERROR_PACKETS)

For information about available fields see [`mod@rx_alignment_error_packets`] module*/
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETSrs>;
///Rx alignment error packets register
pub mod rx_alignment_error_packets;
/**RX_UNICAST_PACKETS_GOOD (r) register accessor: Rx unicast packets good register

You can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:RX_UNICAST_PACKETS_GOOD)

For information about available fields see [`mod@rx_unicast_packets_good`] module*/
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOODrs>;
///Rx unicast packets good register
pub mod rx_unicast_packets_good;
/**TX_LPI_USEC_CNTR (r) register accessor: Tx LPI microsecond timer register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:TX_LPI_USEC_CNTR)

For information about available fields see [`mod@tx_lpi_usec_cntr`] module*/
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTRrs>;
///Tx LPI microsecond timer register
pub mod tx_lpi_usec_cntr;
/**TX_LPI_TRAN_CNTR (r) register accessor: Tx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:TX_LPI_TRAN_CNTR)

For information about available fields see [`mod@tx_lpi_tran_cntr`] module*/
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTRrs>;
///Tx LPI transition counter register
pub mod tx_lpi_tran_cntr;
/**RX_LPI_USEC_CNTR (r) register accessor: Rx LPI microsecond counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:RX_LPI_USEC_CNTR)

For information about available fields see [`mod@rx_lpi_usec_cntr`] module*/
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTRrs>;
///Rx LPI microsecond counter register
pub mod rx_lpi_usec_cntr;
/**RX_LPI_TRAN_CNTR (r) register accessor: Rx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:RX_LPI_TRAN_CNTR)

For information about available fields see [`mod@rx_lpi_tran_cntr`] module*/
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTRrs>;
///Rx LPI transition counter register
pub mod rx_lpi_tran_cntr;
/**MACL3L4C0R (rw) register accessor: L3 and L4 control 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3L4C0R)

For information about available fields see [`mod@macl3l4c0r`] module*/
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0Rrs>;
///L3 and L4 control 0 register
pub mod macl3l4c0r;
/**MACL4A0R (rw) register accessor: Layer4 Address filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL4A0R)

For information about available fields see [`mod@macl4a0r`] module*/
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0Rrs>;
///Layer4 Address filter 0 register
pub mod macl4a0r;
/**MACL3A00R (rw) register accessor: Layer3 Address 0 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A00R)

For information about available fields see [`mod@macl3a00r`] module*/
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00Rrs>;
///Layer3 Address 0 filter 0 register
pub mod macl3a00r;
/**MACL3A10R (rw) register accessor: Layer3 Address 1 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A10R)

For information about available fields see [`mod@macl3a10r`] module*/
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10Rrs>;
///Layer3 Address 1 filter 0 register
pub mod macl3a10r;
/**MACL3A20R (rw) register accessor: Layer3 Address 2 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A20R)

For information about available fields see [`mod@macl3a20r`] module*/
pub type MACL3A20R = crate::Reg<macl3a20r::MACL3A20Rrs>;
///Layer3 Address 2 filter 0 register
pub mod macl3a20r;
/**MACL3A30R (rw) register accessor: Layer3 Address 3 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A30R)

For information about available fields see [`mod@macl3a30r`] module*/
pub type MACL3A30R = crate::Reg<macl3a30r::MACL3A30Rrs>;
///Layer3 Address 3 filter 0 register
pub mod macl3a30r;
/**MACL3L4C1R (rw) register accessor: L3 and L4 control 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3L4C1R)

For information about available fields see [`mod@macl3l4c1r`] module*/
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1Rrs>;
///L3 and L4 control 1 register
pub mod macl3l4c1r;
/**MACL4A1R (rw) register accessor: Layer 4 address filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL4A1R)

For information about available fields see [`mod@macl4a1r`] module*/
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1Rrs>;
///Layer 4 address filter 1 register
pub mod macl4a1r;
/**MACL3A01R (rw) register accessor: Layer3 address 0 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A01R)

For information about available fields see [`mod@macl3a01r`] module*/
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01Rrs>;
///Layer3 address 0 filter 1 Register
pub mod macl3a01r;
/**MACL3A11R (rw) register accessor: Layer3 address 1 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A11R)

For information about available fields see [`mod@macl3a11r`] module*/
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11Rrs>;
///Layer3 address 1 filter 1 register
pub mod macl3a11r;
/**MACL3A21R (rw) register accessor: Layer3 address 2 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A21R)

For information about available fields see [`mod@macl3a21r`] module*/
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21Rrs>;
///Layer3 address 2 filter 1 Register
pub mod macl3a21r;
/**MACL3A31R (rw) register accessor: Layer3 address 3 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACL3A31R)

For information about available fields see [`mod@macl3a31r`] module*/
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31Rrs>;
///Layer3 address 3 filter 1 register
pub mod macl3a31r;
/**MACTSCR (rw) register accessor: Timestamp control Register

You can [`read`](crate::Reg::read) this register and get [`mactscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSCR)

For information about available fields see [`mod@mactscr`] module*/
pub type MACTSCR = crate::Reg<mactscr::MACTSCRrs>;
///Timestamp control Register
pub mod mactscr;
/**MACSSIR (rw) register accessor: Subsecond increment register

You can [`read`](crate::Reg::read) this register and get [`macssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSSIR)

For information about available fields see [`mod@macssir`] module*/
pub type MACSSIR = crate::Reg<macssir::MACSSIRrs>;
///Subsecond increment register
pub mod macssir;
/**MACSTSR (r) register accessor: System time seconds register

You can [`read`](crate::Reg::read) this register and get [`macstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSTSR)

For information about available fields see [`mod@macstsr`] module*/
pub type MACSTSR = crate::Reg<macstsr::MACSTSRrs>;
///System time seconds register
pub mod macstsr;
/**MACSTNR (r) register accessor: System time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macstnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSTNR)

For information about available fields see [`mod@macstnr`] module*/
pub type MACSTNR = crate::Reg<macstnr::MACSTNRrs>;
///System time nanoseconds register
pub mod macstnr;
/**MACSTSUR (rw) register accessor: System time seconds update register

You can [`read`](crate::Reg::read) this register and get [`macstsur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSTSUR)

For information about available fields see [`mod@macstsur`] module*/
pub type MACSTSUR = crate::Reg<macstsur::MACSTSURrs>;
///System time seconds update register
pub mod macstsur;
/**MACSTNUR (rw) register accessor: System time nanoseconds update register

You can [`read`](crate::Reg::read) this register and get [`macstnur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSTNUR)

For information about available fields see [`mod@macstnur`] module*/
pub type MACSTNUR = crate::Reg<macstnur::MACSTNURrs>;
///System time nanoseconds update register
pub mod macstnur;
/**MACTSAR (rw) register accessor: Timestamp addend register

You can [`read`](crate::Reg::read) this register and get [`mactsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSAR)

For information about available fields see [`mod@mactsar`] module*/
pub type MACTSAR = crate::Reg<mactsar::MACTSARrs>;
///Timestamp addend register
pub mod mactsar;
/**MACTSSR (rw) register accessor: Timestamp status register

You can [`read`](crate::Reg::read) this register and get [`mactssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSSR)

For information about available fields see [`mod@mactssr`] module*/
pub type MACTSSR = crate::Reg<mactssr::MACTSSRrs>;
///Timestamp status register
pub mod mactssr;
/**MACTXTSSNR (rw) register accessor: Tx timestamp status nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtssnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactxtssnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTXTSSNR)

For information about available fields see [`mod@mactxtssnr`] module*/
pub type MACTXTSSNR = crate::Reg<mactxtssnr::MACTXTSSNRrs>;
///Tx timestamp status nanoseconds register
pub mod mactxtssnr;
/**MACTXTSSSR (r) register accessor: Tx timestamp status seconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTXTSSSR)

For information about available fields see [`mod@mactxtsssr`] module*/
pub type MACTXTSSSR = crate::Reg<mactxtsssr::MACTXTSSSRrs>;
///Tx timestamp status seconds register
pub mod mactxtsssr;
/**MACACR (rw) register accessor: Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`macacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACACR)

For information about available fields see [`mod@macacr`] module*/
pub type MACACR = crate::Reg<macacr::MACACRrs>;
///Auxiliary control register
pub mod macacr;
/**MACATSNR (r) register accessor: Auxiliary timestamp nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macatsnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACATSNR)

For information about available fields see [`mod@macatsnr`] module*/
pub type MACATSNR = crate::Reg<macatsnr::MACATSNRrs>;
///Auxiliary timestamp nanoseconds register
pub mod macatsnr;
/**MACATSSR (r) register accessor: Auxiliary timestamp seconds register

You can [`read`](crate::Reg::read) this register and get [`macatssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACATSSR)

For information about available fields see [`mod@macatssr`] module*/
pub type MACATSSR = crate::Reg<macatssr::MACATSSRrs>;
///Auxiliary timestamp seconds register
pub mod macatssr;
/**MACTSIACR (rw) register accessor: Timestamp Ingress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSIACR)

For information about available fields see [`mod@mactsiacr`] module*/
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACRrs>;
///Timestamp Ingress asymmetric correction register
pub mod mactsiacr;
/**MACTSEACR (rw) register accessor: Timestamp Egress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactseacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSEACR)

For information about available fields see [`mod@mactseacr`] module*/
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACRrs>;
///Timestamp Egress asymmetric correction register
pub mod mactseacr;
/**MACTSICNR (rw) register accessor: Timestamp Ingress correction nanosecond register

You can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSICNR)

For information about available fields see [`mod@mactsicnr`] module*/
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNRrs>;
///Timestamp Ingress correction nanosecond register
pub mod mactsicnr;
/**MACTSECNR (rw) register accessor: Timestamp Egress correction nanosecond register

You can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACTSECNR)

For information about available fields see [`mod@mactsecnr`] module*/
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNRrs>;
///Timestamp Egress correction nanosecond register
pub mod mactsecnr;
/**MACPPSCR (rw) register accessor: PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSCR)

For information about available fields see [`mod@macppscr`] module*/
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCRrs>;
///PPS control register
pub mod macppscr;
/**MACPPSCR_alternate (rw) register accessor: PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSCR_alternate)

For information about available fields see [`mod@macppscr_alternate`] module*/
#[doc(alias = "MACPPSCR_alternate")]
pub type MACPPSCR_ALTERNATE = crate::Reg<macppscr_alternate::MACPPSCR_ALTERNATErs>;
///PPS control register
pub mod macppscr_alternate;
/**MACPPSTTSR (rw) register accessor: PPS target time seconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSTTSR)

For information about available fields see [`mod@macppsttsr`] module*/
pub type MACPPSTTSR = crate::Reg<macppsttsr::MACPPSTTSRrs>;
///PPS target time seconds register
pub mod macppsttsr;
/**MACPPSTTNR (rw) register accessor: PPS target time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSTTNR)

For information about available fields see [`mod@macppsttnr`] module*/
pub type MACPPSTTNR = crate::Reg<macppsttnr::MACPPSTTNRrs>;
///PPS target time nanoseconds register
pub mod macppsttnr;
/**MACPPSIR (rw) register accessor: PPS interval register

You can [`read`](crate::Reg::read) this register and get [`macppsir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSIR)

For information about available fields see [`mod@macppsir`] module*/
pub type MACPPSIR = crate::Reg<macppsir::MACPPSIRrs>;
///PPS interval register
pub mod macppsir;
/**MACPPSWR (rw) register accessor: PPS width register

You can [`read`](crate::Reg::read) this register and get [`macppswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSWR)

For information about available fields see [`mod@macppswr`] module*/
pub type MACPPSWR = crate::Reg<macppswr::MACPPSWRrs>;
///PPS width register
pub mod macppswr;
/**MACPOCR (rw) register accessor: PTP Offload control register

You can [`read`](crate::Reg::read) this register and get [`macpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPOCR)

For information about available fields see [`mod@macpocr`] module*/
pub type MACPOCR = crate::Reg<macpocr::MACPOCRrs>;
///PTP Offload control register
pub mod macpocr;
/**MACSPI0R (rw) register accessor: PTP Source Port Identity 0 Register

You can [`read`](crate::Reg::read) this register and get [`macspi0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSPI0R)

For information about available fields see [`mod@macspi0r`] module*/
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0Rrs>;
///PTP Source Port Identity 0 Register
pub mod macspi0r;
/**MACSPI1R (rw) register accessor: PTP Source port identity 1 register

You can [`read`](crate::Reg::read) this register and get [`macspi1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSPI1R)

For information about available fields see [`mod@macspi1r`] module*/
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1Rrs>;
///PTP Source port identity 1 register
pub mod macspi1r;
/**MACSPI2R (rw) register accessor: PTP Source port identity 2 register

You can [`read`](crate::Reg::read) this register and get [`macspi2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACSPI2R)

For information about available fields see [`mod@macspi2r`] module*/
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2Rrs>;
///PTP Source port identity 2 register
pub mod macspi2r;
/**MACLMIR (rw) register accessor: Log message interval register

You can [`read`](crate::Reg::read) this register and get [`maclmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACLMIR)

For information about available fields see [`mod@maclmir`] module*/
pub type MACLMIR = crate::Reg<maclmir::MACLMIRrs>;
///Log message interval register
pub mod maclmir;
/**MTLOMR (rw) register accessor: Operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLOMR)

For information about available fields see [`mod@mtlomr`] module*/
pub type MTLOMR = crate::Reg<mtlomr::MTLOMRrs>;
///Operating mode Register
pub mod mtlomr;
/**MTLISR (r) register accessor: Interrupt status Register

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLISR)

For information about available fields see [`mod@mtlisr`] module*/
pub type MTLISR = crate::Reg<mtlisr::MTLISRrs>;
///Interrupt status Register
pub mod mtlisr;
/**MTLTXQOMR (rw) register accessor: Tx queue operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltxqomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxqomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLTXQOMR)

For information about available fields see [`mod@mtltxqomr`] module*/
pub type MTLTXQOMR = crate::Reg<mtltxqomr::MTLTXQOMRrs>;
///Tx queue operating mode Register
pub mod mtltxqomr;
/**MTLTXQUR (rw) register accessor: Tx queue underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxqur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxqur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLTXQUR)

For information about available fields see [`mod@mtltxqur`] module*/
pub type MTLTXQUR = crate::Reg<mtltxqur::MTLTXQURrs>;
///Tx queue underflow register
pub mod mtltxqur;
/**MTLTXQDR (r) register accessor: Tx queue debug Register

You can [`read`](crate::Reg::read) this register and get [`mtltxqdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLTXQDR)

For information about available fields see [`mod@mtltxqdr`] module*/
pub type MTLTXQDR = crate::Reg<mtltxqdr::MTLTXQDRrs>;
///Tx queue debug Register
pub mod mtltxqdr;
/**MTLQICSR (rw) register accessor: Queue interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlqicsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlqicsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLQICSR)

For information about available fields see [`mod@mtlqicsr`] module*/
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSRrs>;
///Queue interrupt control status Register
pub mod mtlqicsr;
/**MTLRXQOMR (rw) register accessor: Rx queue operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxqomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLRXQOMR)

For information about available fields see [`mod@mtlrxqomr`] module*/
pub type MTLRXQOMR = crate::Reg<mtlrxqomr::MTLRXQOMRrs>;
///Rx queue operating mode register
pub mod mtlrxqomr;
/**MTLRXQMPOCR (rw) register accessor: Rx queue missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqmpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxqmpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLRXQMPOCR)

For information about available fields see [`mod@mtlrxqmpocr`] module*/
pub type MTLRXQMPOCR = crate::Reg<mtlrxqmpocr::MTLRXQMPOCRrs>;
///Rx queue missed packet and overflow counter register
pub mod mtlrxqmpocr;
/**MTLRXQDR (r) register accessor: Rx queue debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLRXQDR)

For information about available fields see [`mod@mtlrxqdr`] module*/
pub type MTLRXQDR = crate::Reg<mtlrxqdr::MTLRXQDRrs>;
///Rx queue debug register
pub mod mtlrxqdr;
/**DMAMR (rw) register accessor: DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMAMR)

For information about available fields see [`mod@dmamr`] module*/
pub type DMAMR = crate::Reg<dmamr::DMAMRrs>;
///DMA mode register
pub mod dmamr;
/**DMASBMR (rw) register accessor: System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMASBMR)

For information about available fields see [`mod@dmasbmr`] module*/
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMRrs>;
///System bus mode register
pub mod dmasbmr;
/**DMAISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMAISR)

For information about available fields see [`mod@dmaisr`] module*/
pub type DMAISR = crate::Reg<dmaisr::DMAISRrs>;
///Interrupt status register
pub mod dmaisr;
/**DMADSR (r) register accessor: Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMADSR)

For information about available fields see [`mod@dmadsr`] module*/
pub type DMADSR = crate::Reg<dmadsr::DMADSRrs>;
///Debug status register
pub mod dmadsr;
/**DMACCR (rw) register accessor: Channel control register

You can [`read`](crate::Reg::read) this register and get [`dmaccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACCR)

For information about available fields see [`mod@dmaccr`] module*/
pub type DMACCR = crate::Reg<dmaccr::DMACCRrs>;
///Channel control register
pub mod dmaccr;
/**DMACTXCR (rw) register accessor: Channel transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmactxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACTXCR)

For information about available fields see [`mod@dmactxcr`] module*/
pub type DMACTXCR = crate::Reg<dmactxcr::DMACTXCRrs>;
///Channel transmit control register
pub mod dmactxcr;
/**DMACRXCR (rw) register accessor: Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`dmacrxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXCR)

For information about available fields see [`mod@dmacrxcr`] module*/
pub type DMACRXCR = crate::Reg<dmacrxcr::DMACRXCRrs>;
///Channel receive control register
pub mod dmacrxcr;
/**DMACTXDLAR (rw) register accessor: Channel Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmactxdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACTXDLAR)

For information about available fields see [`mod@dmactxdlar`] module*/
pub type DMACTXDLAR = crate::Reg<dmactxdlar::DMACTXDLARrs>;
///Channel Tx descriptor list address register
pub mod dmactxdlar;
/**DMACRXDLAR (rw) register accessor: Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmacrxdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXDLAR)

For information about available fields see [`mod@dmacrxdlar`] module*/
pub type DMACRXDLAR = crate::Reg<dmacrxdlar::DMACRXDLARrs>;
///Channel Rx descriptor list address register
pub mod dmacrxdlar;
/**DMACTXDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmactxdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACTXDTPR)

For information about available fields see [`mod@dmactxdtpr`] module*/
pub type DMACTXDTPR = crate::Reg<dmactxdtpr::DMACTXDTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod dmactxdtpr;
/**DMACRXDTPR (rw) register accessor: Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmacrxdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXDTPR)

For information about available fields see [`mod@dmacrxdtpr`] module*/
pub type DMACRXDTPR = crate::Reg<dmacrxdtpr::DMACRXDTPRrs>;
///Channel Rx descriptor tail pointer register
pub mod dmacrxdtpr;
/**DMACTXRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmactxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACTXRLR)

For information about available fields see [`mod@dmactxrlr`] module*/
pub type DMACTXRLR = crate::Reg<dmactxrlr::DMACTXRLRrs>;
///Channel Tx descriptor ring length register
pub mod dmactxrlr;
/**DMACRXRLR (rw) register accessor: Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmacrxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXRLR)

For information about available fields see [`mod@dmacrxrlr`] module*/
pub type DMACRXRLR = crate::Reg<dmacrxrlr::DMACRXRLRrs>;
///Channel Rx descriptor ring length register
pub mod dmacrxrlr;
/**DMACIER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmacier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACIER)

For information about available fields see [`mod@dmacier`] module*/
pub type DMACIER = crate::Reg<dmacier::DMACIERrs>;
///Channel interrupt enable register
pub mod dmacier;
/**DMACRXIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmacrxiwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxiwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXIWTR)

For information about available fields see [`mod@dmacrxiwtr`] module*/
pub type DMACRXIWTR = crate::Reg<dmacrxiwtr::DMACRXIWTRrs>;
///Channel Rx interrupt watchdog timer register
pub mod dmacrxiwtr;
/**DMACCATXDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccatxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACCATXDR)

For information about available fields see [`mod@dmaccatxdr`] module*/
pub type DMACCATXDR = crate::Reg<dmaccatxdr::DMACCATXDRrs>;
///Channel current application transmit descriptor register
pub mod dmaccatxdr;
/**DMACCARXDR (r) register accessor: Channel current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccarxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACCARXDR)

For information about available fields see [`mod@dmaccarxdr`] module*/
pub type DMACCARXDR = crate::Reg<dmaccarxdr::DMACCARXDRrs>;
///Channel current application receive descriptor register
pub mod dmaccarxdr;
/**DMACCATXBR (r) register accessor: Channel current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccatxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACCATXBR)

For information about available fields see [`mod@dmaccatxbr`] module*/
pub type DMACCATXBR = crate::Reg<dmaccatxbr::DMACCATXBRrs>;
///Channel current application transmit buffer register
pub mod dmaccatxbr;
/**DMACCARXBR (r) register accessor: Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccarxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACCARXBR)

For information about available fields see [`mod@dmaccarxbr`] module*/
pub type DMACCARXBR = crate::Reg<dmaccarxbr::DMACCARXBRrs>;
///Channel current application receive buffer register
pub mod dmaccarxbr;
/**DMACSR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`dmacsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACSR)

For information about available fields see [`mod@dmacsr`] module*/
pub type DMACSR = crate::Reg<dmacsr::DMACSRrs>;
///Channel status register
pub mod dmacsr;
/**DMACMFCR (rw) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmacmfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacmfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACMFCR)

For information about available fields see [`mod@dmacmfcr`] module*/
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCRrs>;
///Channel missed frame count register
pub mod dmacmfcr;
