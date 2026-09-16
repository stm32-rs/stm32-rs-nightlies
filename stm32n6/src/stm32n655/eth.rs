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
    macvtcr: MACVTCR,
    macvtdr: MACVTDR,
    macvhtr: MACVHTR,
    _reserved9: [u8; 0x04],
    _reserved_9_macvir: [u8; 0x04],
    macivir: MACIVIR,
    _reserved11: [u8; 0x08],
    macq0txfcr: MACQ0TXFCR,
    _reserved12: [u8; 0x1c],
    macrxfcr: MACRXFCR,
    macrxqcr: MACRXQCR,
    _reserved14: [u8; 0x08],
    macrxqc0r: MACRXQC0R,
    macrxqc1r: MACRXQC1R,
    macrxqc2r: MACRXQC2R,
    _reserved17: [u8; 0x04],
    macisr: MACISR,
    macier: MACIER,
    macrxtxsr: MACRXTXSR,
    _reserved20: [u8; 0x04],
    macpcsr: MACPCSR,
    macrwkpfr: MACRWKPFR,
    _reserved22: [u8; 0x08],
    maclcsr: MACLCSR,
    macltcr: MACLTCR,
    macletr: MACLETR,
    mac1ustcr: MAC1USTCR,
    _reserved26: [u8; 0x18],
    macphycsr: MACPHYCSR,
    _reserved27: [u8; 0x14],
    macvr: MACVR,
    macdr: MACDR,
    _reserved29: [u8; 0x04],
    machwf0r: MACHWF0R,
    machwf1r: MACHWF1R,
    machwf2r: MACHWF2R,
    machwf3r: MACHWF3R,
    _reserved33: [u8; 0xd4],
    macmdioar: MACMDIOAR,
    macmdiodr: MACMDIODR,
    _reserved35: [u8; 0x08],
    macarpar: MACARPAR,
    _reserved36: [u8; 0x1c],
    maccsrswcr: MACCSRSWCR,
    macfpecsr: MACFPECSR,
    _reserved38: [u8; 0x08],
    macprstimr: MACPRSTIMR,
    macprstimur: MACPRSTIMUR,
    _reserved40: [u8; 0xb8],
    maca0hr: MACA0HR,
    maca0lr: MACA0LR,
    maca1hr: MACA1HR,
    maca1lr: MACA1LR,
    maca2hr: MACA2HR,
    maca2lr: MACA2LR,
    maca3hr: MACA3HR,
    maca3lr: MACA3LR,
    _reserved48: [u8; 0x03e0],
    mmc_control: MMC_CONTROL,
    mmc_rx_interrupt: MMC_RX_INTERRUPT,
    mmc_tx_interrupt: MMC_TX_INTERRUPT,
    mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved53: [u8; 0x38],
    tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved55: [u8; 0x14],
    tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved56: [u8; 0x28],
    rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved58: [u8; 0x28],
    rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved59: [u8; 0x24],
    tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved63: [u8; 0xa4],
    mmc_fpe_tx_isr: MMC_FPE_TX_ISR,
    mmc_fpe_tx_imr: MMC_FPE_TX_IMR,
    mmc_fpe_tx_fcr: MMC_FPE_TX_FCR,
    mmc_tx_hrcr: MMC_TX_HRCR,
    _reserved67: [u8; 0x10],
    mmc_fpe_rx_isr: MMC_FPE_RX_ISR,
    mmc_fpe_rx_imr: MMC_FPE_RX_IMR,
    rx_packet_asm_err: RX_PACKET_ASM_ERR,
    rx_packet_smd_err: RX_PACKET_SMD_ERR,
    rx_packet_asm_okr: RX_PACKET_ASM_OKR,
    rx_fpe_frag_cr: RX_FPE_FRAG_CR,
    _reserved73: [u8; 0x28],
    macl3l4c0r: MACL3L4C0R,
    macl4a0r: MACL4A0R,
    _reserved75: [u8; 0x08],
    macl3a00r: MACL3A00R,
    macl3a10r: MACL3A10R,
    macl3a20r: MACL3A20R,
    macl3a30r: MACL3A30R,
    _reserved79: [u8; 0x10],
    macl3l4c1r: MACL3L4C1R,
    macl4a1r: MACL4A1R,
    _reserved81: [u8; 0x08],
    macl3a01r: MACL3A01R,
    macl3a11r: MACL3A11R,
    macl3a21r: MACL3A21R,
    macl3a31r: MACL3A31R,
    _reserved85: [u8; 0x0120],
    mac_iacr: MAC_IACR,
    mac_tmrqr: MAC_TMRQR,
    _reserved87: [u8; 0x88],
    mactscr: MACTSCR,
    macssir: MACSSIR,
    macstsr: MACSTSR,
    macstnr: MACSTNR,
    macstsur: MACSTSUR,
    macstnur: MACSTNUR,
    mactsar: MACTSAR,
    _reserved94: [u8; 0x04],
    mactssr: MACTSSR,
    _reserved95: [u8; 0x0c],
    mactxtssnr: MACTXTSSNR,
    mactxtsssr: MACTXTSSSR,
    _reserved97: [u8; 0x08],
    macacr: MACACR,
    _reserved98: [u8; 0x04],
    macatsnr: MACATSNR,
    macatssr: MACATSSR,
    mactsiacr: MACTSIACR,
    mactseacr: MACTSEACR,
    mactsicnr: MACTSICNR,
    mactsecnr: MACTSECNR,
    _reserved104: [u8; 0x08],
    mactsilr: MACTSILR,
    mactselr: MACTSELR,
    _reserved_106_macppscr: [u8; 0x04],
    _reserved107: [u8; 0x0c],
    macppstts0r: MACPPSTTS0R,
    macppsttn0r: MACPPSTTN0R,
    macppsi0r: MACPPSI0R,
    macppsw0r: MACPPSW0R,
    macppstts1r: MACPPSTTS1R,
    macppsttn1r: MACPPSTTN1R,
    macppsi1r: MACPPSI1R,
    macppsw1r: MACPPSW1R,
    _reserved115: [u8; 0x20],
    macpocr: MACPOCR,
    macspi0r: MACSPI0R,
    macspi1r: MACSPI1R,
    macspi2r: MACSPI2R,
    maclmir: MACLMIR,
    _reserved120: [u8; 0x2c],
    mtlomr: MTLOMR,
    _reserved121: [u8; 0x1c],
    mtlisr: MTLISR,
    _reserved122: [u8; 0x0c],
    mtlrxqdmamr: MTLRXQDMAMR,
    _reserved123: [u8; 0x0c],
    mtltbscr: MTLTBSCR,
    _reserved124: [u8; 0x0c],
    mtlestcr: MTLESTCR,
    mtlestecr: MTLESTECR,
    mtlestsr: MTLESTSR,
    _reserved127: [u8; 0x04],
    mtlestscher: MTLESTSCHER,
    mtlestfser: MTLESTFSER,
    mtlestfscr: MTLESTFSCR,
    _reserved130: [u8; 0x04],
    mtlestier: MTLESTIER,
    _reserved131: [u8; 0x0c],
    mtlestgclcr: MTLESTGCLCR,
    mtlestgcldr: MTLESTGCLDR,
    _reserved133: [u8; 0x08],
    mtlfpecsr: MTLFPECSR,
    mtlfpear: MTLFPEAR,
    _reserved135: [u8; 0x68],
    mtltxq0omr: MTLTXQ0OMR,
    mtltxq0ur: MTLTXQ0UR,
    mtltxq0dr: MTLTXQ0DR,
    _reserved138: [u8; 0x08],
    mtltxq0esr: MTLTXQ0ESR,
    mtltxq0qwr: MTLTXQ0QWR,
    _reserved140: [u8; 0x10],
    mtlq0icsr: MTLQ0ICSR,
    mtlrxq0omr: MTLRXQ0OMR,
    mtlrxq0mpocr: MTLRXQ0MPOCR,
    mtlrxq0dr: MTLRXQ0DR,
    mtlrxq0cr: MTLRXQ0CR,
    mtltxq1omr: MTLTXQ1OMR,
    mtltxq1ur: MTLTXQ1UR,
    mtltxq1dr: MTLTXQ1DR,
    _reserved148: [u8; 0x04],
    mtltxq1ecr: MTLTXQ1ECR,
    mtltxq1esr: MTLTXQ1ESR,
    mtltxq1qwr: MTLTXQ1QWR,
    mtltxq1sscr: MTLTXQ1SSCR,
    mtltxq1hcr: MTLTXQ1HCR,
    mtltxq1lcr: MTLTXQ1LCR,
    _reserved154: [u8; 0x04],
    mtlq1icsr: MTLQ1ICSR,
    mtlrxq1omr: MTLRXQ1OMR,
    mtlrxq1mpocr: MTLRXQ1MPOCR,
    mtlrxq1dr: MTLRXQ1DR,
    mtlrxq1cr: MTLRXQ1CR,
    _reserved159: [u8; 0x0280],
    dmamr: DMAMR,
    dmasbmr: DMASBMR,
    dmaisr: DMAISR,
    dmadsr: DMADSR,
    _reserved163: [u8; 0x10],
    dmaa4txacr: DMAA4TXACR,
    dmaa4rxacr: DMAA4RXACR,
    dmaa4dacr: DMAA4DACR,
    _reserved166: [u8; 0x14],
    dmalpiei: DMALPIEI,
    _reserved167: [u8; 0x0c],
    dmatbsctrl0r: DMATBSCTRL0R,
    _reserved168: [u8; 0xac],
    dmac0cr: DMAC0CR,
    dmac0txcr: DMAC0TXCR,
    dmac0rxcr: DMAC0RXCR,
    _reserved171: [u8; 0x08],
    dmac0txdlar: DMAC0TXDLAR,
    _reserved172: [u8; 0x04],
    dmac0rxdlar: DMAC0RXDLAR,
    dmac0txdtpr: DMAC0TXDTPR,
    _reserved174: [u8; 0x04],
    dmac0rxdtpr: DMAC0RXDTPR,
    dmac0txrlr: DMAC0TXRLR,
    dmac0rxrlr: DMAC0RXRLR,
    dmac0ier: DMAC0IER,
    dmac0rxiwtr: DMAC0RXIWTR,
    dmac0sfcsr: DMAC0SFCSR,
    _reserved180: [u8; 0x04],
    dmac0catxdr: DMAC0CATXDR,
    _reserved181: [u8; 0x04],
    dmac0carxdr: DMAC0CARXDR,
    _reserved182: [u8; 0x04],
    dmac0catxbr: DMAC0CATXBR,
    _reserved183: [u8; 0x04],
    dmac0carxbr: DMAC0CARXBR,
    dmac0sr: DMAC0SR,
    dmac0mfcr: DMAC0MFCR,
    _reserved186: [u8; 0x18],
    dmac1cr: DMAC1CR,
    dmac1txcr: DMAC1TXCR,
    dmac1rxcr: DMAC1RXCR,
    _reserved189: [u8; 0x08],
    dmac1txdlar: DMAC1TXDLAR,
    _reserved190: [u8; 0x08],
    dmac1txdtpr: DMAC1TXDTPR,
    _reserved191: [u8; 0x04],
    dmac1rxdtpr: DMAC1RXDTPR,
    dmac1txrlr: DMAC1TXRLR,
    dmac1rxrlr: DMAC1RXRLR,
    dmac1ier: DMAC1IER,
    dmac1rxiwtr: DMAC1RXIWTR,
    dmac1sfcsr: DMAC1SFCSR,
    _reserved197: [u8; 0x04],
    dmac1catxdr: DMAC1CATXDR,
    _reserved198: [u8; 0x04],
    dmac1carxdr: DMAC1CARXDR,
    _reserved199: [u8; 0x04],
    dmac1catxbr: DMAC1CATXBR,
    _reserved200: [u8; 0x04],
    dmac1carxbr: DMAC1CARXBR,
    dmac1sr: DMAC1SR,
    dmac1mfcr: DMAC1MFCR,
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
    ///0x50 - VLAN tag Control register
    #[inline(always)]
    pub const fn macvtcr(&self) -> &MACVTCR {
        &self.macvtcr
    }
    ///0x54 - VLAN tag data register
    #[inline(always)]
    pub const fn macvtdr(&self) -> &MACVTDR {
        &self.macvtdr
    }
    ///0x58 - VLAN Hash table register
    #[inline(always)]
    pub const fn macvhtr(&self) -> &MACVHTR {
        &self.macvhtr
    }
    ///0x60 - VLAN inclusion register
    #[inline(always)]
    pub const fn macvir_alternate(&self) -> &MACVIR_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    ///0x60 - VLAN inclusion register
    #[inline(always)]
    pub const fn macvir(&self) -> &MACVIR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    ///0x64 - Inner VLAN inclusion register
    #[inline(always)]
    pub const fn macivir(&self) -> &MACIVIR {
        &self.macivir
    }
    ///0x70 - Tx Queue 0 flow control register
    #[inline(always)]
    pub const fn macq0txfcr(&self) -> &MACQ0TXFCR {
        &self.macq0txfcr
    }
    ///0x90 - Rx flow control register
    #[inline(always)]
    pub const fn macrxfcr(&self) -> &MACRXFCR {
        &self.macrxfcr
    }
    ///0x94 - Rx Queue control register
    #[inline(always)]
    pub const fn macrxqcr(&self) -> &MACRXQCR {
        &self.macrxqcr
    }
    ///0xa0 - Rx queue control 0 register
    #[inline(always)]
    pub const fn macrxqc0r(&self) -> &MACRXQC0R {
        &self.macrxqc0r
    }
    ///0xa4 - Rx queue control 1 register
    #[inline(always)]
    pub const fn macrxqc1r(&self) -> &MACRXQC1R {
        &self.macrxqc1r
    }
    ///0xa8 - Rx queue control 2 register
    #[inline(always)]
    pub const fn macrxqc2r(&self) -> &MACRXQC2R {
        &self.macrxqc2r
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
    ///0xc4 - Remote wake-up packet filter register
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
    ///0xf8 - PHYIF control status register
    #[inline(always)]
    pub const fn macphycsr(&self) -> &MACPHYCSR {
        &self.macphycsr
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
    ///0x234 - FPE control and status register
    #[inline(always)]
    pub const fn macfpecsr(&self) -> &MACFPECSR {
        &self.macfpecsr
    }
    ///0x240 - MAC presentation time register
    #[inline(always)]
    pub const fn macprstimr(&self) -> &MACPRSTIMR {
        &self.macprstimr
    }
    ///0x244 - MAC presentation time update register
    #[inline(always)]
    pub const fn macprstimur(&self) -> &MACPRSTIMUR {
        &self.macprstimur
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
    ///0x8a0 - MMC FPE Tx interrupt status register
    #[inline(always)]
    pub const fn mmc_fpe_tx_isr(&self) -> &MMC_FPE_TX_ISR {
        &self.mmc_fpe_tx_isr
    }
    ///0x8a4 - MMC FPE Tx interrupt mask register
    #[inline(always)]
    pub const fn mmc_fpe_tx_imr(&self) -> &MMC_FPE_TX_IMR {
        &self.mmc_fpe_tx_imr
    }
    ///0x8a8 - MMC FPE Tx fragment counter register
    #[inline(always)]
    pub const fn mmc_fpe_tx_fcr(&self) -> &MMC_FPE_TX_FCR {
        &self.mmc_fpe_tx_fcr
    }
    ///0x8ac - MMC Tx hold request counter register
    #[inline(always)]
    pub const fn mmc_tx_hrcr(&self) -> &MMC_TX_HRCR {
        &self.mmc_tx_hrcr
    }
    ///0x8c0 - MMC FPE Rx interrupt status register
    #[inline(always)]
    pub const fn mmc_fpe_rx_isr(&self) -> &MMC_FPE_RX_ISR {
        &self.mmc_fpe_rx_isr
    }
    ///0x8c4 - MMC FPE Rx interrupt mask register
    #[inline(always)]
    pub const fn mmc_fpe_rx_imr(&self) -> &MMC_FPE_RX_IMR {
        &self.mmc_fpe_rx_imr
    }
    ///0x8c8 - MMC Rx packet assembly error register
    #[inline(always)]
    pub const fn rx_packet_asm_err(&self) -> &RX_PACKET_ASM_ERR {
        &self.rx_packet_asm_err
    }
    ///0x8cc - MMC Rx packet SMD error register
    #[inline(always)]
    pub const fn rx_packet_smd_err(&self) -> &RX_PACKET_SMD_ERR {
        &self.rx_packet_smd_err
    }
    ///0x8d0 - MMC Rx packet assembly OK register
    #[inline(always)]
    pub const fn rx_packet_asm_okr(&self) -> &RX_PACKET_ASM_OKR {
        &self.rx_packet_asm_okr
    }
    ///0x8d4 - MMC Rx FPE fragments counter register
    #[inline(always)]
    pub const fn rx_fpe_frag_cr(&self) -> &RX_FPE_FRAG_CR {
        &self.rx_fpe_frag_cr
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
    ///0xa70 - MAC Indirect Access Control register
    #[inline(always)]
    pub const fn mac_iacr(&self) -> &MAC_IACR {
        &self.mac_iacr
    }
    ///0xa74 - MAC type-based Rx Queue mapping register
    #[inline(always)]
    pub const fn mac_tmrqr(&self) -> &MAC_TMRQR {
        &self.mac_tmrqr
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
    ///0xb68 - Timestamp Ingress Latency register
    #[inline(always)]
    pub const fn mactsilr(&self) -> &MACTSILR {
        &self.mactsilr
    }
    ///0xb6c - Timestamp Egress Latency register
    #[inline(always)]
    pub const fn mactselr(&self) -> &MACTSELR {
        &self.mactselr
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
    ///0xb80 - PPS 0 target time seconds register
    #[inline(always)]
    pub const fn macppstts0r(&self) -> &MACPPSTTS0R {
        &self.macppstts0r
    }
    ///0xb84 - PPS 0 target time nanoseconds register
    #[inline(always)]
    pub const fn macppsttn0r(&self) -> &MACPPSTTN0R {
        &self.macppsttn0r
    }
    ///0xb88 - PPS 0 interval register
    #[inline(always)]
    pub const fn macppsi0r(&self) -> &MACPPSI0R {
        &self.macppsi0r
    }
    ///0xb8c - PPS 0 width register
    #[inline(always)]
    pub const fn macppsw0r(&self) -> &MACPPSW0R {
        &self.macppsw0r
    }
    ///0xb90 - PPS 1 target time seconds register
    #[inline(always)]
    pub const fn macppstts1r(&self) -> &MACPPSTTS1R {
        &self.macppstts1r
    }
    ///0xb94 - PPS 1 target time nanoseconds register
    #[inline(always)]
    pub const fn macppsttn1r(&self) -> &MACPPSTTN1R {
        &self.macppsttn1r
    }
    ///0xb98 - PPS 1 interval register
    #[inline(always)]
    pub const fn macppsi1r(&self) -> &MACPPSI1R {
        &self.macppsi1r
    }
    ///0xb9c - PPS 1 width register
    #[inline(always)]
    pub const fn macppsw1r(&self) -> &MACPPSW1R {
        &self.macppsw1r
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
    ///0xc30 - Rx Queue and DMA Channel Mapping Register
    #[inline(always)]
    pub const fn mtlrxqdmamr(&self) -> &MTLRXQDMAMR {
        &self.mtlrxqdmamr
    }
    ///0xc40 - TBS control register
    #[inline(always)]
    pub const fn mtltbscr(&self) -> &MTLTBSCR {
        &self.mtltbscr
    }
    ///0xc50 - EST Control Register
    #[inline(always)]
    pub const fn mtlestcr(&self) -> &MTLESTCR {
        &self.mtlestcr
    }
    ///0xc54 - EST Extended Control Register
    #[inline(always)]
    pub const fn mtlestecr(&self) -> &MTLESTECR {
        &self.mtlestecr
    }
    ///0xc58 - EST Status Register
    #[inline(always)]
    pub const fn mtlestsr(&self) -> &MTLESTSR {
        &self.mtlestsr
    }
    ///0xc60 - EST Schedule Error Register
    #[inline(always)]
    pub const fn mtlestscher(&self) -> &MTLESTSCHER {
        &self.mtlestscher
    }
    ///0xc64 - EST Frame size Error Register
    #[inline(always)]
    pub const fn mtlestfser(&self) -> &MTLESTFSER {
        &self.mtlestfser
    }
    ///0xc68 - EST Frame size Capture Register
    #[inline(always)]
    pub const fn mtlestfscr(&self) -> &MTLESTFSCR {
        &self.mtlestfscr
    }
    ///0xc70 - EST Interrupt Enable Register
    #[inline(always)]
    pub const fn mtlestier(&self) -> &MTLESTIER {
        &self.mtlestier
    }
    ///0xc80 - EST Gate Control List Register
    #[inline(always)]
    pub const fn mtlestgclcr(&self) -> &MTLESTGCLCR {
        &self.mtlestgclcr
    }
    ///0xc84 - EST Gate Control List Data Register
    #[inline(always)]
    pub const fn mtlestgcldr(&self) -> &MTLESTGCLDR {
        &self.mtlestgcldr
    }
    ///0xc90 - FPE Frame Preemption Control Status Register
    #[inline(always)]
    pub const fn mtlfpecsr(&self) -> &MTLFPECSR {
        &self.mtlfpecsr
    }
    ///0xc94 - FPE Frame Preemption Advance Register
    #[inline(always)]
    pub const fn mtlfpear(&self) -> &MTLFPEAR {
        &self.mtlfpear
    }
    ///0xd00 - T0 queue 0 operating mode Register
    #[inline(always)]
    pub const fn mtltxq0omr(&self) -> &MTLTXQ0OMR {
        &self.mtltxq0omr
    }
    ///0xd04 - T0 queue 0 underflow register
    #[inline(always)]
    pub const fn mtltxq0ur(&self) -> &MTLTXQ0UR {
        &self.mtltxq0ur
    }
    ///0xd08 - T0 queue 0 debug register
    #[inline(always)]
    pub const fn mtltxq0dr(&self) -> &MTLTXQ0DR {
        &self.mtltxq0dr
    }
    ///0xd14 - T0 queue 0 ETS status Register
    #[inline(always)]
    pub const fn mtltxq0esr(&self) -> &MTLTXQ0ESR {
        &self.mtltxq0esr
    }
    ///0xd18 - Tx queue 0 quantum weight register
    #[inline(always)]
    pub const fn mtltxq0qwr(&self) -> &MTLTXQ0QWR {
        &self.mtltxq0qwr
    }
    ///0xd2c - Queue 0 interrupt control status Register
    #[inline(always)]
    pub const fn mtlq0icsr(&self) -> &MTLQ0ICSR {
        &self.mtlq0icsr
    }
    ///0xd30 - R0 queue 0 operating mode register
    #[inline(always)]
    pub const fn mtlrxq0omr(&self) -> &MTLRXQ0OMR {
        &self.mtlrxq0omr
    }
    ///0xd34 - R0 queue 0 missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrxq0mpocr(&self) -> &MTLRXQ0MPOCR {
        &self.mtlrxq0mpocr
    }
    ///0xd38 - R0 queue 0 debug register
    #[inline(always)]
    pub const fn mtlrxq0dr(&self) -> &MTLRXQ0DR {
        &self.mtlrxq0dr
    }
    ///0xd3c - R0 queue 0 control register
    #[inline(always)]
    pub const fn mtlrxq0cr(&self) -> &MTLRXQ0CR {
        &self.mtlrxq0cr
    }
    ///0xd40 - T1 queue 1 operating mode Register
    #[inline(always)]
    pub const fn mtltxq1omr(&self) -> &MTLTXQ1OMR {
        &self.mtltxq1omr
    }
    ///0xd44 - T1 queue 1 underflow register
    #[inline(always)]
    pub const fn mtltxq1ur(&self) -> &MTLTXQ1UR {
        &self.mtltxq1ur
    }
    ///0xd48 - T1 queue 1 debug register
    #[inline(always)]
    pub const fn mtltxq1dr(&self) -> &MTLTXQ1DR {
        &self.mtltxq1dr
    }
    ///0xd50 - Tx queue 1 ETS control Register
    #[inline(always)]
    pub const fn mtltxq1ecr(&self) -> &MTLTXQ1ECR {
        &self.mtltxq1ecr
    }
    ///0xd54 - T1 queue 1 ETS status Register
    #[inline(always)]
    pub const fn mtltxq1esr(&self) -> &MTLTXQ1ESR {
        &self.mtltxq1esr
    }
    ///0xd58 - Tx queue 1 quantum weight register
    #[inline(always)]
    pub const fn mtltxq1qwr(&self) -> &MTLTXQ1QWR {
        &self.mtltxq1qwr
    }
    ///0xd5c - Tx queue 1 send slope credit Register
    #[inline(always)]
    pub const fn mtltxq1sscr(&self) -> &MTLTXQ1SSCR {
        &self.mtltxq1sscr
    }
    ///0xd60 - Tx Queue 1 hiCredit register
    #[inline(always)]
    pub const fn mtltxq1hcr(&self) -> &MTLTXQ1HCR {
        &self.mtltxq1hcr
    }
    ///0xd64 - Tx queue 1 loCredit register
    #[inline(always)]
    pub const fn mtltxq1lcr(&self) -> &MTLTXQ1LCR {
        &self.mtltxq1lcr
    }
    ///0xd6c - Queue 1 interrupt control status Register
    #[inline(always)]
    pub const fn mtlq1icsr(&self) -> &MTLQ1ICSR {
        &self.mtlq1icsr
    }
    ///0xd70 - R1 queue 1 operating mode register
    #[inline(always)]
    pub const fn mtlrxq1omr(&self) -> &MTLRXQ1OMR {
        &self.mtlrxq1omr
    }
    ///0xd74 - R1 queue 1 missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrxq1mpocr(&self) -> &MTLRXQ1MPOCR {
        &self.mtlrxq1mpocr
    }
    ///0xd78 - R1 queue 1 debug register
    #[inline(always)]
    pub const fn mtlrxq1dr(&self) -> &MTLRXQ1DR {
        &self.mtlrxq1dr
    }
    ///0xd7c - R1 queue 1 control register
    #[inline(always)]
    pub const fn mtlrxq1cr(&self) -> &MTLRXQ1CR {
        &self.mtlrxq1cr
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
    ///0x1020 - AXI4 transmit channel ACE control register
    #[inline(always)]
    pub const fn dmaa4txacr(&self) -> &DMAA4TXACR {
        &self.dmaa4txacr
    }
    ///0x1024 - AXI4 receive channel ACE control register
    #[inline(always)]
    pub const fn dmaa4rxacr(&self) -> &DMAA4RXACR {
        &self.dmaa4rxacr
    }
    ///0x1028 - AXI4 descriptor ACE control register
    #[inline(always)]
    pub const fn dmaa4dacr(&self) -> &DMAA4DACR {
        &self.dmaa4dacr
    }
    ///0x1040 - AXI4 LPI Entry Interval register
    #[inline(always)]
    pub const fn dmalpiei(&self) -> &DMALPIEI {
        &self.dmalpiei
    }
    ///0x1050 - DMA TBS control register 0
    #[inline(always)]
    pub const fn dmatbsctrl0r(&self) -> &DMATBSCTRL0R {
        &self.dmatbsctrl0r
    }
    ///0x1100 - Channel 0 control register
    #[inline(always)]
    pub const fn dmac0cr(&self) -> &DMAC0CR {
        &self.dmac0cr
    }
    ///0x1104 - Channel 0 transmit control register
    #[inline(always)]
    pub const fn dmac0txcr(&self) -> &DMAC0TXCR {
        &self.dmac0txcr
    }
    ///0x1108 - Channel 0 receive control register
    #[inline(always)]
    pub const fn dmac0rxcr(&self) -> &DMAC0RXCR {
        &self.dmac0rxcr
    }
    ///0x1114 - Channel 0 T0 descriptor list address register
    #[inline(always)]
    pub const fn dmac0txdlar(&self) -> &DMAC0TXDLAR {
        &self.dmac0txdlar
    }
    ///0x111c - Channel 0 Rx descriptor list address register
    #[inline(always)]
    pub const fn dmac0rxdlar(&self) -> &DMAC0RXDLAR {
        &self.dmac0rxdlar
    }
    ///0x1120 - Channel 0 T0 descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac0txdtpr(&self) -> &DMAC0TXDTPR {
        &self.dmac0txdtpr
    }
    ///0x1128 - Channel 0 R0 descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac0rxdtpr(&self) -> &DMAC0RXDTPR {
        &self.dmac0rxdtpr
    }
    ///0x112c - Channel 0 T0 descriptor ring length register
    #[inline(always)]
    pub const fn dmac0txrlr(&self) -> &DMAC0TXRLR {
        &self.dmac0txrlr
    }
    ///0x1130 - Channel 0 R0 descriptor ring length register
    #[inline(always)]
    pub const fn dmac0rxrlr(&self) -> &DMAC0RXRLR {
        &self.dmac0rxrlr
    }
    ///0x1134 - Channel 0 interrupt enable register
    #[inline(always)]
    pub const fn dmac0ier(&self) -> &DMAC0IER {
        &self.dmac0ier
    }
    ///0x1138 - Channel 0 R0 interrupt watchdog timer register
    #[inline(always)]
    pub const fn dmac0rxiwtr(&self) -> &DMAC0RXIWTR {
        &self.dmac0rxiwtr
    }
    ///0x113c - Channel 0 slot function control status register
    #[inline(always)]
    pub const fn dmac0sfcsr(&self) -> &DMAC0SFCSR {
        &self.dmac0sfcsr
    }
    ///0x1144 - Channel 0 current application transmit descriptor register
    #[inline(always)]
    pub const fn dmac0catxdr(&self) -> &DMAC0CATXDR {
        &self.dmac0catxdr
    }
    ///0x114c - Channel 0 current application receive descriptor register
    #[inline(always)]
    pub const fn dmac0carxdr(&self) -> &DMAC0CARXDR {
        &self.dmac0carxdr
    }
    ///0x1154 - Channel 0 current application transmit buffer register
    #[inline(always)]
    pub const fn dmac0catxbr(&self) -> &DMAC0CATXBR {
        &self.dmac0catxbr
    }
    ///0x115c - Channel 0 current application receive buffer register
    #[inline(always)]
    pub const fn dmac0carxbr(&self) -> &DMAC0CARXBR {
        &self.dmac0carxbr
    }
    ///0x1160 - Channel 0 status register
    #[inline(always)]
    pub const fn dmac0sr(&self) -> &DMAC0SR {
        &self.dmac0sr
    }
    ///0x1164 - Channel 0 missed frame count register
    #[inline(always)]
    pub const fn dmac0mfcr(&self) -> &DMAC0MFCR {
        &self.dmac0mfcr
    }
    ///0x1180 - Channel 1 control register
    #[inline(always)]
    pub const fn dmac1cr(&self) -> &DMAC1CR {
        &self.dmac1cr
    }
    ///0x1184 - Channel 1 transmit control register
    #[inline(always)]
    pub const fn dmac1txcr(&self) -> &DMAC1TXCR {
        &self.dmac1txcr
    }
    ///0x1188 - Channel 1 receive control register
    #[inline(always)]
    pub const fn dmac1rxcr(&self) -> &DMAC1RXCR {
        &self.dmac1rxcr
    }
    ///0x1194 - Channel 1 T1 descriptor list address register
    #[inline(always)]
    pub const fn dmac1txdlar(&self) -> &DMAC1TXDLAR {
        &self.dmac1txdlar
    }
    ///0x11a0 - Channel 1 T1 descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac1txdtpr(&self) -> &DMAC1TXDTPR {
        &self.dmac1txdtpr
    }
    ///0x11a8 - Channel 1 R1 descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac1rxdtpr(&self) -> &DMAC1RXDTPR {
        &self.dmac1rxdtpr
    }
    ///0x11ac - Channel 1 T1 descriptor ring length register
    #[inline(always)]
    pub const fn dmac1txrlr(&self) -> &DMAC1TXRLR {
        &self.dmac1txrlr
    }
    ///0x11b0 - Channel 1 R1 descriptor ring length register
    #[inline(always)]
    pub const fn dmac1rxrlr(&self) -> &DMAC1RXRLR {
        &self.dmac1rxrlr
    }
    ///0x11b4 - Channel 1 interrupt enable register
    #[inline(always)]
    pub const fn dmac1ier(&self) -> &DMAC1IER {
        &self.dmac1ier
    }
    ///0x11b8 - Channel 1 R1 interrupt watchdog timer register
    #[inline(always)]
    pub const fn dmac1rxiwtr(&self) -> &DMAC1RXIWTR {
        &self.dmac1rxiwtr
    }
    ///0x11bc - Channel 1 slot function control status register
    #[inline(always)]
    pub const fn dmac1sfcsr(&self) -> &DMAC1SFCSR {
        &self.dmac1sfcsr
    }
    ///0x11c4 - Channel 1 current application transmit descriptor register
    #[inline(always)]
    pub const fn dmac1catxdr(&self) -> &DMAC1CATXDR {
        &self.dmac1catxdr
    }
    ///0x11cc - Channel 1 current application receive descriptor register
    #[inline(always)]
    pub const fn dmac1carxdr(&self) -> &DMAC1CARXDR {
        &self.dmac1carxdr
    }
    ///0x11d4 - Channel 1 current application transmit buffer register
    #[inline(always)]
    pub const fn dmac1catxbr(&self) -> &DMAC1CATXBR {
        &self.dmac1catxbr
    }
    ///0x11dc - Channel 1 current application receive buffer register
    #[inline(always)]
    pub const fn dmac1carxbr(&self) -> &DMAC1CARXBR {
        &self.dmac1carxbr
    }
    ///0x11e0 - Channel 1 status register
    #[inline(always)]
    pub const fn dmac1sr(&self) -> &DMAC1SR {
        &self.dmac1sr
    }
    ///0x11e4 - Channel 1 missed frame count register
    #[inline(always)]
    pub const fn dmac1mfcr(&self) -> &DMAC1MFCR {
        &self.dmac1mfcr
    }
}
/**MACCR (rw) register accessor: Operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACCR)

For information about available fields see [`mod@maccr`] module*/
pub type MACCR = crate::Reg<maccr::MACCRrs>;
///Operating mode configuration register
pub mod maccr;
/**MACECR (rw) register accessor: Extended operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`macecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACECR)

For information about available fields see [`mod@macecr`] module*/
pub type MACECR = crate::Reg<macecr::MACECRrs>;
///Extended operating mode configuration register
pub mod macecr;
/**MACPFR (rw) register accessor: Packet filtering control register

You can [`read`](crate::Reg::read) this register and get [`macpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPFR)

For information about available fields see [`mod@macpfr`] module*/
pub type MACPFR = crate::Reg<macpfr::MACPFRrs>;
///Packet filtering control register
pub mod macpfr;
/**MACWTR (rw) register accessor: Watchdog timeout register

You can [`read`](crate::Reg::read) this register and get [`macwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACWTR)

For information about available fields see [`mod@macwtr`] module*/
pub type MACWTR = crate::Reg<macwtr::MACWTRrs>;
///Watchdog timeout register
pub mod macwtr;
/**MACHT0R (rw) register accessor: Hash Table 0 register

You can [`read`](crate::Reg::read) this register and get [`macht0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHT0R)

For information about available fields see [`mod@macht0r`] module*/
pub type MACHT0R = crate::Reg<macht0r::MACHT0Rrs>;
///Hash Table 0 register
pub mod macht0r;
/**MACHT1R (rw) register accessor: Hash Table 1 register

You can [`read`](crate::Reg::read) this register and get [`macht1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHT1R)

For information about available fields see [`mod@macht1r`] module*/
pub type MACHT1R = crate::Reg<macht1r::MACHT1Rrs>;
///Hash Table 1 register
pub mod macht1r;
/**MACVTCR (rw) register accessor: VLAN tag Control register

You can [`read`](crate::Reg::read) this register and get [`macvtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVTCR)

For information about available fields see [`mod@macvtcr`] module*/
pub type MACVTCR = crate::Reg<macvtcr::MACVTCRrs>;
///VLAN tag Control register
pub mod macvtcr;
/**MACVTDR (rw) register accessor: VLAN tag data register

You can [`read`](crate::Reg::read) this register and get [`macvtdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVTDR)

For information about available fields see [`mod@macvtdr`] module*/
pub type MACVTDR = crate::Reg<macvtdr::MACVTDRrs>;
///VLAN tag data register
pub mod macvtdr;
/**MACVHTR (rw) register accessor: VLAN Hash table register

You can [`read`](crate::Reg::read) this register and get [`macvhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVHTR)

For information about available fields see [`mod@macvhtr`] module*/
pub type MACVHTR = crate::Reg<macvhtr::MACVHTRrs>;
///VLAN Hash table register
pub mod macvhtr;
/**MACVIR (rw) register accessor: VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macvir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVIR)

For information about available fields see [`mod@macvir`] module*/
pub type MACVIR = crate::Reg<macvir::MACVIRrs>;
///VLAN inclusion register
pub mod macvir;
/**MACVIR_alternate (rw) register accessor: VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macvir_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVIR_alternate)

For information about available fields see [`mod@macvir_alternate`] module*/
#[doc(alias = "MACVIR_alternate")]
pub type MACVIR_ALTERNATE = crate::Reg<macvir_alternate::MACVIR_ALTERNATErs>;
///VLAN inclusion register
pub mod macvir_alternate;
/**MACIVIR (rw) register accessor: Inner VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macivir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macivir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACIVIR)

For information about available fields see [`mod@macivir`] module*/
pub type MACIVIR = crate::Reg<macivir::MACIVIRrs>;
///Inner VLAN inclusion register
pub mod macivir;
/**MACQ0TXFCR (rw) register accessor: Tx Queue 0 flow control register

You can [`read`](crate::Reg::read) this register and get [`macq0txfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macq0txfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACQ0TXFCR)

For information about available fields see [`mod@macq0txfcr`] module*/
pub type MACQ0TXFCR = crate::Reg<macq0txfcr::MACQ0TXFCRrs>;
///Tx Queue 0 flow control register
pub mod macq0txfcr;
/**MACRXFCR (rw) register accessor: Rx flow control register

You can [`read`](crate::Reg::read) this register and get [`macrxfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXFCR)

For information about available fields see [`mod@macrxfcr`] module*/
pub type MACRXFCR = crate::Reg<macrxfcr::MACRXFCRrs>;
///Rx flow control register
pub mod macrxfcr;
/**MACRXQCR (rw) register accessor: Rx Queue control register

You can [`read`](crate::Reg::read) this register and get [`macrxqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXQCR)

For information about available fields see [`mod@macrxqcr`] module*/
pub type MACRXQCR = crate::Reg<macrxqcr::MACRXQCRrs>;
///Rx Queue control register
pub mod macrxqcr;
/**MACRXQC0R (rw) register accessor: Rx queue control 0 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXQC0R)

For information about available fields see [`mod@macrxqc0r`] module*/
pub type MACRXQC0R = crate::Reg<macrxqc0r::MACRXQC0Rrs>;
///Rx queue control 0 register
pub mod macrxqc0r;
/**MACRXQC1R (rw) register accessor: Rx queue control 1 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXQC1R)

For information about available fields see [`mod@macrxqc1r`] module*/
pub type MACRXQC1R = crate::Reg<macrxqc1r::MACRXQC1Rrs>;
///Rx queue control 1 register
pub mod macrxqc1r;
/**MACRXQC2R (rw) register accessor: Rx queue control 2 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXQC2R)

For information about available fields see [`mod@macrxqc2r`] module*/
pub type MACRXQC2R = crate::Reg<macrxqc2r::MACRXQC2Rrs>;
///Rx queue control 2 register
pub mod macrxqc2r;
/**MACISR (rw) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`macisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACISR)

For information about available fields see [`mod@macisr`] module*/
pub type MACISR = crate::Reg<macisr::MACISRrs>;
///Interrupt status register
pub mod macisr;
/**MACIER (rw) register accessor: Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`macier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACIER)

For information about available fields see [`mod@macier`] module*/
pub type MACIER = crate::Reg<macier::MACIERrs>;
///Interrupt enable register
pub mod macier;
/**MACRXTXSR (rw) register accessor: Rx Tx status register

You can [`read`](crate::Reg::read) this register and get [`macrxtxsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxtxsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRXTXSR)

For information about available fields see [`mod@macrxtxsr`] module*/
pub type MACRXTXSR = crate::Reg<macrxtxsr::MACRXTXSRrs>;
///Rx Tx status register
pub mod macrxtxsr;
/**MACPCSR (rw) register accessor: PMT control status register

You can [`read`](crate::Reg::read) this register and get [`macpcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPCSR)

For information about available fields see [`mod@macpcsr`] module*/
pub type MACPCSR = crate::Reg<macpcsr::MACPCSRrs>;
///PMT control status register
pub mod macpcsr;
/**MACRWKPFR (rw) register accessor: Remote wake-up packet filter register

You can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACRWKPFR)

For information about available fields see [`mod@macrwkpfr`] module*/
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFRrs>;
///Remote wake-up packet filter register
pub mod macrwkpfr;
/**MACLCSR (rw) register accessor: LPI control and status register

You can [`read`](crate::Reg::read) this register and get [`maclcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACLCSR)

For information about available fields see [`mod@maclcsr`] module*/
pub type MACLCSR = crate::Reg<maclcsr::MACLCSRrs>;
///LPI control and status register
pub mod maclcsr;
/**MACLTCR (rw) register accessor: LPI timers control register

You can [`read`](crate::Reg::read) this register and get [`macltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACLTCR)

For information about available fields see [`mod@macltcr`] module*/
pub type MACLTCR = crate::Reg<macltcr::MACLTCRrs>;
///LPI timers control register
pub mod macltcr;
/**MACLETR (rw) register accessor: LPI entry timer register

You can [`read`](crate::Reg::read) this register and get [`macletr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACLETR)

For information about available fields see [`mod@macletr`] module*/
pub type MACLETR = crate::Reg<macletr::MACLETRrs>;
///LPI entry timer register
pub mod macletr;
/**MAC1USTCR (rw) register accessor: One-microsecond-tick counter register

You can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MAC1USTCR)

For information about available fields see [`mod@mac1ustcr`] module*/
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCRrs>;
///One-microsecond-tick counter register
pub mod mac1ustcr;
/**MACPHYCSR (rw) register accessor: PHYIF control status register

You can [`read`](crate::Reg::read) this register and get [`macphycsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macphycsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPHYCSR)

For information about available fields see [`mod@macphycsr`] module*/
pub type MACPHYCSR = crate::Reg<macphycsr::MACPHYCSRrs>;
///PHYIF control status register
pub mod macphycsr;
/**MACVR (r) register accessor: Version register

You can [`read`](crate::Reg::read) this register and get [`macvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACVR)

For information about available fields see [`mod@macvr`] module*/
pub type MACVR = crate::Reg<macvr::MACVRrs>;
///Version register
pub mod macvr;
/**MACDR (r) register accessor: Debug register

You can [`read`](crate::Reg::read) this register and get [`macdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACDR)

For information about available fields see [`mod@macdr`] module*/
pub type MACDR = crate::Reg<macdr::MACDRrs>;
///Debug register
pub mod macdr;
/**MACHWF0R (r) register accessor: HW feature 0 register

You can [`read`](crate::Reg::read) this register and get [`machwf0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHWF0R)

For information about available fields see [`mod@machwf0r`] module*/
pub type MACHWF0R = crate::Reg<machwf0r::MACHWF0Rrs>;
///HW feature 0 register
pub mod machwf0r;
/**MACHWF1R (r) register accessor: HW feature 1 register

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHWF1R)

For information about available fields see [`mod@machwf1r`] module*/
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1Rrs>;
///HW feature 1 register
pub mod machwf1r;
/**MACHWF2R (r) register accessor: HW feature 2 register

You can [`read`](crate::Reg::read) this register and get [`machwf2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHWF2R)

For information about available fields see [`mod@machwf2r`] module*/
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2Rrs>;
///HW feature 2 register
pub mod machwf2r;
/**MACHWF3R (r) register accessor: HW feature 3 register

You can [`read`](crate::Reg::read) this register and get [`machwf3r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACHWF3R)

For information about available fields see [`mod@machwf3r`] module*/
pub type MACHWF3R = crate::Reg<machwf3r::MACHWF3Rrs>;
///HW feature 3 register
pub mod machwf3r;
/**MACMDIOAR (rw) register accessor: MDIO address register

You can [`read`](crate::Reg::read) this register and get [`macmdioar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACMDIOAR)

For information about available fields see [`mod@macmdioar`] module*/
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOARrs>;
///MDIO address register
pub mod macmdioar;
/**MACMDIODR (rw) register accessor: MDIO data register

You can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACMDIODR)

For information about available fields see [`mod@macmdiodr`] module*/
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODRrs>;
///MDIO data register
pub mod macmdiodr;
/**MACARPAR (rw) register accessor: ARP address register

You can [`read`](crate::Reg::read) this register and get [`macarpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACARPAR)

For information about available fields see [`mod@macarpar`] module*/
pub type MACARPAR = crate::Reg<macarpar::MACARPARrs>;
///ARP address register
pub mod macarpar;
/**MACCSRSWCR (rw) register accessor: CSR software control register

You can [`read`](crate::Reg::read) this register and get [`maccsrswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccsrswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACCSRSWCR)

For information about available fields see [`mod@maccsrswcr`] module*/
pub type MACCSRSWCR = crate::Reg<maccsrswcr::MACCSRSWCRrs>;
///CSR software control register
pub mod maccsrswcr;
/**MACFPECSR (rw) register accessor: FPE control and status register

You can [`read`](crate::Reg::read) this register and get [`macfpecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfpecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACFPECSR)

For information about available fields see [`mod@macfpecsr`] module*/
pub type MACFPECSR = crate::Reg<macfpecsr::MACFPECSRrs>;
///FPE control and status register
pub mod macfpecsr;
/**MACPRSTIMR (rw) register accessor: MAC presentation time register

You can [`read`](crate::Reg::read) this register and get [`macprstimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macprstimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPRSTIMR)

For information about available fields see [`mod@macprstimr`] module*/
pub type MACPRSTIMR = crate::Reg<macprstimr::MACPRSTIMRrs>;
///MAC presentation time register
pub mod macprstimr;
/**MACPRSTIMUR (rw) register accessor: MAC presentation time update register

You can [`read`](crate::Reg::read) this register and get [`macprstimur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macprstimur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPRSTIMUR)

For information about available fields see [`mod@macprstimur`] module*/
pub type MACPRSTIMUR = crate::Reg<macprstimur::MACPRSTIMURrs>;
///MAC presentation time update register
pub mod macprstimur;
/**MACA0HR (rw) register accessor: MAC Address 0 high register

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA0HR)

For information about available fields see [`mod@maca0hr`] module*/
pub type MACA0HR = crate::Reg<maca0hr::MACA0HRrs>;
///MAC Address 0 high register
pub mod maca0hr;
/**MACA0LR (rw) register accessor: MAC Address 0 low register

You can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA0LR)

For information about available fields see [`mod@maca0lr`] module*/
pub type MACA0LR = crate::Reg<maca0lr::MACA0LRrs>;
///MAC Address 0 low register
pub mod maca0lr;
/**MACA1HR (rw) register accessor: MAC Address 1 high register

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA1HR)

For information about available fields see [`mod@maca1hr`] module*/
pub type MACA1HR = crate::Reg<maca1hr::MACA1HRrs>;
///MAC Address 1 high register
pub mod maca1hr;
/**MACA1LR (rw) register accessor: MAC Address 1 low register

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA1LR)

For information about available fields see [`mod@maca1lr`] module*/
pub type MACA1LR = crate::Reg<maca1lr::MACA1LRrs>;
///MAC Address 1 low register
pub mod maca1lr;
/**MACA2HR (rw) register accessor: MAC Address 2 high register

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA2HR)

For information about available fields see [`mod@maca2hr`] module*/
pub type MACA2HR = crate::Reg<maca2hr::MACA2HRrs>;
///MAC Address 2 high register
pub mod maca2hr;
/**MACA2LR (rw) register accessor: MAC Address 2 low register

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA2LR)

For information about available fields see [`mod@maca2lr`] module*/
pub type MACA2LR = crate::Reg<maca2lr::MACA2LRrs>;
///MAC Address 2 low register
pub mod maca2lr;
/**MACA3HR (rw) register accessor: MAC Address 3 high register

You can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA3HR)

For information about available fields see [`mod@maca3hr`] module*/
pub type MACA3HR = crate::Reg<maca3hr::MACA3HRrs>;
///MAC Address 3 high register
pub mod maca3hr;
/**MACA3LR (rw) register accessor: MAC Address 3 low register

You can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACA3LR)

For information about available fields see [`mod@maca3lr`] module*/
pub type MACA3LR = crate::Reg<maca3lr::MACA3LRrs>;
///MAC Address 3 low register
pub mod maca3lr;
/**MMC_CONTROL (rw) register accessor: MMC control register

You can [`read`](crate::Reg::read) this register and get [`mmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_CONTROL)

For information about available fields see [`mod@mmc_control`] module*/
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROLrs>;
///MMC control register
pub mod mmc_control;
/**MMC_RX_INTERRUPT (rw) register accessor: MMC Rx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_RX_INTERRUPT)

For information about available fields see [`mod@mmc_rx_interrupt`] module*/
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPTrs>;
///MMC Rx interrupt register
pub mod mmc_rx_interrupt;
/**MMC_TX_INTERRUPT (rw) register accessor: MMC Tx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_TX_INTERRUPT)

For information about available fields see [`mod@mmc_tx_interrupt`] module*/
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPTrs>;
///MMC Tx interrupt register
pub mod mmc_tx_interrupt;
/**MMC_RX_INTERRUPT_MASK (rw) register accessor: MMC Rx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_RX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_rx_interrupt_mask`] module*/
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASKrs>;
///MMC Rx interrupt mask register
pub mod mmc_rx_interrupt_mask;
/**MMC_TX_INTERRUPT_MASK (rw) register accessor: MMC Tx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_TX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_tx_interrupt_mask`] module*/
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASKrs>;
///MMC Tx interrupt mask register
pub mod mmc_tx_interrupt_mask;
/**TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx single collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:TX_SINGLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_single_collision_good_packets`] module*/
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
///Tx single collision good packets register
pub mod tx_single_collision_good_packets;
/**TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx multiple collision good packets register

You can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:TX_MULTIPLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_multiple_collision_good_packets`] module*/
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETSrs>;
///Tx multiple collision good packets register
pub mod tx_multiple_collision_good_packets;
/**TX_PACKET_COUNT_GOOD (r) register accessor: Tx packet count good register

You can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:TX_PACKET_COUNT_GOOD)

For information about available fields see [`mod@tx_packet_count_good`] module*/
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOODrs>;
///Tx packet count good register
pub mod tx_packet_count_good;
/**RX_CRC_ERROR_PACKETS (r) register accessor: Rx CRC error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_CRC_ERROR_PACKETS)

For information about available fields see [`mod@rx_crc_error_packets`] module*/
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETSrs>;
///Rx CRC error packets register
pub mod rx_crc_error_packets;
/**RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: Rx alignment error packets register

You can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_ALIGNMENT_ERROR_PACKETS)

For information about available fields see [`mod@rx_alignment_error_packets`] module*/
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETSrs>;
///Rx alignment error packets register
pub mod rx_alignment_error_packets;
/**RX_UNICAST_PACKETS_GOOD (r) register accessor: Rx unicast packets good register

You can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_UNICAST_PACKETS_GOOD)

For information about available fields see [`mod@rx_unicast_packets_good`] module*/
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOODrs>;
///Rx unicast packets good register
pub mod rx_unicast_packets_good;
/**TX_LPI_USEC_CNTR (r) register accessor: Tx LPI microsecond timer register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:TX_LPI_USEC_CNTR)

For information about available fields see [`mod@tx_lpi_usec_cntr`] module*/
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTRrs>;
///Tx LPI microsecond timer register
pub mod tx_lpi_usec_cntr;
/**TX_LPI_TRAN_CNTR (r) register accessor: Tx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:TX_LPI_TRAN_CNTR)

For information about available fields see [`mod@tx_lpi_tran_cntr`] module*/
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTRrs>;
///Tx LPI transition counter register
pub mod tx_lpi_tran_cntr;
/**RX_LPI_USEC_CNTR (r) register accessor: Rx LPI microsecond counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_LPI_USEC_CNTR)

For information about available fields see [`mod@rx_lpi_usec_cntr`] module*/
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTRrs>;
///Rx LPI microsecond counter register
pub mod rx_lpi_usec_cntr;
/**RX_LPI_TRAN_CNTR (r) register accessor: Rx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_LPI_TRAN_CNTR)

For information about available fields see [`mod@rx_lpi_tran_cntr`] module*/
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTRrs>;
///Rx LPI transition counter register
pub mod rx_lpi_tran_cntr;
/**MMC_FPE_TX_ISR (rw) register accessor: MMC FPE Tx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_tx_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_TX_ISR)

For information about available fields see [`mod@mmc_fpe_tx_isr`] module*/
pub type MMC_FPE_TX_ISR = crate::Reg<mmc_fpe_tx_isr::MMC_FPE_TX_ISRrs>;
///MMC FPE Tx interrupt status register
pub mod mmc_fpe_tx_isr;
/**MMC_FPE_TX_IMR (rw) register accessor: MMC FPE Tx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_tx_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_TX_IMR)

For information about available fields see [`mod@mmc_fpe_tx_imr`] module*/
pub type MMC_FPE_TX_IMR = crate::Reg<mmc_fpe_tx_imr::MMC_FPE_TX_IMRrs>;
///MMC FPE Tx interrupt mask register
pub mod mmc_fpe_tx_imr;
/**MMC_FPE_TX_FCR (r) register accessor: MMC FPE Tx fragment counter register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_fcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_TX_FCR)

For information about available fields see [`mod@mmc_fpe_tx_fcr`] module*/
pub type MMC_FPE_TX_FCR = crate::Reg<mmc_fpe_tx_fcr::MMC_FPE_TX_FCRrs>;
///MMC FPE Tx fragment counter register
pub mod mmc_fpe_tx_fcr;
/**MMC_TX_HRCR (r) register accessor: MMC Tx hold request counter register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_hrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_TX_HRCR)

For information about available fields see [`mod@mmc_tx_hrcr`] module*/
pub type MMC_TX_HRCR = crate::Reg<mmc_tx_hrcr::MMC_TX_HRCRrs>;
///MMC Tx hold request counter register
pub mod mmc_tx_hrcr;
/**MMC_FPE_RX_ISR (r) register accessor: MMC FPE Rx interrupt status register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_rx_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_RX_ISR)

For information about available fields see [`mod@mmc_fpe_rx_isr`] module*/
pub type MMC_FPE_RX_ISR = crate::Reg<mmc_fpe_rx_isr::MMC_FPE_RX_ISRrs>;
///MMC FPE Rx interrupt status register
pub mod mmc_fpe_rx_isr;
/**MMC_FPE_RX_IMR (rw) register accessor: MMC FPE Rx interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_rx_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_fpe_rx_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MMC_FPE_RX_IMR)

For information about available fields see [`mod@mmc_fpe_rx_imr`] module*/
pub type MMC_FPE_RX_IMR = crate::Reg<mmc_fpe_rx_imr::MMC_FPE_RX_IMRrs>;
///MMC FPE Rx interrupt mask register
pub mod mmc_fpe_rx_imr;
/**RX_PACKET_ASM_ERR (r) register accessor: MMC Rx packet assembly error register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_asm_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_PACKET_ASM_ERR)

For information about available fields see [`mod@rx_packet_asm_err`] module*/
pub type RX_PACKET_ASM_ERR = crate::Reg<rx_packet_asm_err::RX_PACKET_ASM_ERRrs>;
///MMC Rx packet assembly error register
pub mod rx_packet_asm_err;
/**RX_PACKET_SMD_ERR (r) register accessor: MMC Rx packet SMD error register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_smd_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_PACKET_SMD_ERR)

For information about available fields see [`mod@rx_packet_smd_err`] module*/
pub type RX_PACKET_SMD_ERR = crate::Reg<rx_packet_smd_err::RX_PACKET_SMD_ERRrs>;
///MMC Rx packet SMD error register
pub mod rx_packet_smd_err;
/**RX_PACKET_ASM_OKR (r) register accessor: MMC Rx packet assembly OK register

You can [`read`](crate::Reg::read) this register and get [`rx_packet_asm_okr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_PACKET_ASM_OKR)

For information about available fields see [`mod@rx_packet_asm_okr`] module*/
pub type RX_PACKET_ASM_OKR = crate::Reg<rx_packet_asm_okr::RX_PACKET_ASM_OKRrs>;
///MMC Rx packet assembly OK register
pub mod rx_packet_asm_okr;
/**RX_FPE_FRAG_CR (r) register accessor: MMC Rx FPE fragments counter register

You can [`read`](crate::Reg::read) this register and get [`rx_fpe_frag_cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_FPE_FRAG_CR)

For information about available fields see [`mod@rx_fpe_frag_cr`] module*/
pub type RX_FPE_FRAG_CR = crate::Reg<rx_fpe_frag_cr::RX_FPE_FRAG_CRrs>;
///MMC Rx FPE fragments counter register
pub mod rx_fpe_frag_cr;
/**MACL3L4C0R (rw) register accessor: L3 and L4 control 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3L4C0R)

For information about available fields see [`mod@macl3l4c0r`] module*/
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0Rrs>;
///L3 and L4 control 0 register
pub mod macl3l4c0r;
/**MACL4A0R (rw) register accessor: Layer4 Address filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL4A0R)

For information about available fields see [`mod@macl4a0r`] module*/
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0Rrs>;
///Layer4 Address filter 0 register
pub mod macl4a0r;
/**MACL3A00R (rw) register accessor: Layer3 Address 0 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A00R)

For information about available fields see [`mod@macl3a00r`] module*/
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00Rrs>;
///Layer3 Address 0 filter 0 register
pub mod macl3a00r;
/**MACL3A10R (rw) register accessor: Layer3 Address 1 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A10R)

For information about available fields see [`mod@macl3a10r`] module*/
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10Rrs>;
///Layer3 Address 1 filter 0 register
pub mod macl3a10r;
/**MACL3A20R (rw) register accessor: Layer3 Address 2 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A20R)

For information about available fields see [`mod@macl3a20r`] module*/
pub type MACL3A20R = crate::Reg<macl3a20r::MACL3A20Rrs>;
///Layer3 Address 2 filter 0 register
pub mod macl3a20r;
/**MACL3A30R (rw) register accessor: Layer3 Address 3 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A30R)

For information about available fields see [`mod@macl3a30r`] module*/
pub type MACL3A30R = crate::Reg<macl3a30r::MACL3A30Rrs>;
///Layer3 Address 3 filter 0 register
pub mod macl3a30r;
/**MACL3L4C1R (rw) register accessor: L3 and L4 control 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3L4C1R)

For information about available fields see [`mod@macl3l4c1r`] module*/
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1Rrs>;
///L3 and L4 control 1 register
pub mod macl3l4c1r;
/**MACL4A1R (rw) register accessor: Layer 4 address filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL4A1R)

For information about available fields see [`mod@macl4a1r`] module*/
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1Rrs>;
///Layer 4 address filter 1 register
pub mod macl4a1r;
/**MACL3A01R (rw) register accessor: Layer3 address 0 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A01R)

For information about available fields see [`mod@macl3a01r`] module*/
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01Rrs>;
///Layer3 address 0 filter 1 Register
pub mod macl3a01r;
/**MACL3A11R (rw) register accessor: Layer3 address 1 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A11R)

For information about available fields see [`mod@macl3a11r`] module*/
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11Rrs>;
///Layer3 address 1 filter 1 register
pub mod macl3a11r;
/**MACL3A21R (rw) register accessor: Layer3 address 2 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A21R)

For information about available fields see [`mod@macl3a21r`] module*/
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21Rrs>;
///Layer3 address 2 filter 1 Register
pub mod macl3a21r;
/**MACL3A31R (rw) register accessor: Layer3 address 3 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACL3A31R)

For information about available fields see [`mod@macl3a31r`] module*/
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31Rrs>;
///Layer3 address 3 filter 1 register
pub mod macl3a31r;
/**MAC_IACR (rw) register accessor: MAC Indirect Access Control register

You can [`read`](crate::Reg::read) this register and get [`mac_iacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_iacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MAC_IACR)

For information about available fields see [`mod@mac_iacr`] module*/
pub type MAC_IACR = crate::Reg<mac_iacr::MAC_IACRrs>;
///MAC Indirect Access Control register
pub mod mac_iacr;
/**MAC_TMRQR (rw) register accessor: MAC type-based Rx Queue mapping register

You can [`read`](crate::Reg::read) this register and get [`mac_tmrqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_tmrqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MAC_TMRQR)

For information about available fields see [`mod@mac_tmrqr`] module*/
pub type MAC_TMRQR = crate::Reg<mac_tmrqr::MAC_TMRQRrs>;
///MAC type-based Rx Queue mapping register
pub mod mac_tmrqr;
/**MACTSCR (rw) register accessor: Timestamp control Register

You can [`read`](crate::Reg::read) this register and get [`mactscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSCR)

For information about available fields see [`mod@mactscr`] module*/
pub type MACTSCR = crate::Reg<mactscr::MACTSCRrs>;
///Timestamp control Register
pub mod mactscr;
/**MACSSIR (rw) register accessor: Subsecond increment register

You can [`read`](crate::Reg::read) this register and get [`macssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSSIR)

For information about available fields see [`mod@macssir`] module*/
pub type MACSSIR = crate::Reg<macssir::MACSSIRrs>;
///Subsecond increment register
pub mod macssir;
/**MACSTSR (r) register accessor: System time seconds register

You can [`read`](crate::Reg::read) this register and get [`macstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSTSR)

For information about available fields see [`mod@macstsr`] module*/
pub type MACSTSR = crate::Reg<macstsr::MACSTSRrs>;
///System time seconds register
pub mod macstsr;
/**MACSTNR (r) register accessor: System time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macstnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSTNR)

For information about available fields see [`mod@macstnr`] module*/
pub type MACSTNR = crate::Reg<macstnr::MACSTNRrs>;
///System time nanoseconds register
pub mod macstnr;
/**MACSTSUR (rw) register accessor: System time seconds update register

You can [`read`](crate::Reg::read) this register and get [`macstsur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSTSUR)

For information about available fields see [`mod@macstsur`] module*/
pub type MACSTSUR = crate::Reg<macstsur::MACSTSURrs>;
///System time seconds update register
pub mod macstsur;
/**MACSTNUR (rw) register accessor: System time nanoseconds update register

You can [`read`](crate::Reg::read) this register and get [`macstnur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSTNUR)

For information about available fields see [`mod@macstnur`] module*/
pub type MACSTNUR = crate::Reg<macstnur::MACSTNURrs>;
///System time nanoseconds update register
pub mod macstnur;
/**MACTSAR (rw) register accessor: Timestamp addend register

You can [`read`](crate::Reg::read) this register and get [`mactsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSAR)

For information about available fields see [`mod@mactsar`] module*/
pub type MACTSAR = crate::Reg<mactsar::MACTSARrs>;
///Timestamp addend register
pub mod mactsar;
/**MACTSSR (rw) register accessor: Timestamp status register

You can [`read`](crate::Reg::read) this register and get [`mactssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSSR)

For information about available fields see [`mod@mactssr`] module*/
pub type MACTSSR = crate::Reg<mactssr::MACTSSRrs>;
///Timestamp status register
pub mod mactssr;
/**MACTXTSSNR (rw) register accessor: Tx timestamp status nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtssnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactxtssnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTXTSSNR)

For information about available fields see [`mod@mactxtssnr`] module*/
pub type MACTXTSSNR = crate::Reg<mactxtssnr::MACTXTSSNRrs>;
///Tx timestamp status nanoseconds register
pub mod mactxtssnr;
/**MACTXTSSSR (r) register accessor: Tx timestamp status seconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTXTSSSR)

For information about available fields see [`mod@mactxtsssr`] module*/
pub type MACTXTSSSR = crate::Reg<mactxtsssr::MACTXTSSSRrs>;
///Tx timestamp status seconds register
pub mod mactxtsssr;
/**MACACR (rw) register accessor: Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`macacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACACR)

For information about available fields see [`mod@macacr`] module*/
pub type MACACR = crate::Reg<macacr::MACACRrs>;
///Auxiliary control register
pub mod macacr;
/**MACATSNR (r) register accessor: Auxiliary timestamp nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macatsnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACATSNR)

For information about available fields see [`mod@macatsnr`] module*/
pub type MACATSNR = crate::Reg<macatsnr::MACATSNRrs>;
///Auxiliary timestamp nanoseconds register
pub mod macatsnr;
/**MACATSSR (r) register accessor: Auxiliary timestamp seconds register

You can [`read`](crate::Reg::read) this register and get [`macatssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACATSSR)

For information about available fields see [`mod@macatssr`] module*/
pub type MACATSSR = crate::Reg<macatssr::MACATSSRrs>;
///Auxiliary timestamp seconds register
pub mod macatssr;
/**MACTSIACR (rw) register accessor: Timestamp Ingress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSIACR)

For information about available fields see [`mod@mactsiacr`] module*/
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACRrs>;
///Timestamp Ingress asymmetric correction register
pub mod mactsiacr;
/**MACTSEACR (rw) register accessor: Timestamp Egress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactseacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSEACR)

For information about available fields see [`mod@mactseacr`] module*/
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACRrs>;
///Timestamp Egress asymmetric correction register
pub mod mactseacr;
/**MACTSICNR (rw) register accessor: Timestamp Ingress correction nanosecond register

You can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSICNR)

For information about available fields see [`mod@mactsicnr`] module*/
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNRrs>;
///Timestamp Ingress correction nanosecond register
pub mod mactsicnr;
/**MACTSECNR (rw) register accessor: Timestamp Egress correction nanosecond register

You can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSECNR)

For information about available fields see [`mod@mactsecnr`] module*/
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNRrs>;
///Timestamp Egress correction nanosecond register
pub mod mactsecnr;
/**MACTSILR (r) register accessor: Timestamp Ingress Latency register

You can [`read`](crate::Reg::read) this register and get [`mactsilr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSILR)

For information about available fields see [`mod@mactsilr`] module*/
pub type MACTSILR = crate::Reg<mactsilr::MACTSILRrs>;
///Timestamp Ingress Latency register
pub mod mactsilr;
/**MACTSELR (r) register accessor: Timestamp Egress Latency register

You can [`read`](crate::Reg::read) this register and get [`mactselr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACTSELR)

For information about available fields see [`mod@mactselr`] module*/
pub type MACTSELR = crate::Reg<mactselr::MACTSELRrs>;
///Timestamp Egress Latency register
pub mod mactselr;
/**MACPPSCR (rw) register accessor: PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSCR)

For information about available fields see [`mod@macppscr`] module*/
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCRrs>;
///PPS control register
pub mod macppscr;
/**MACPPSCR_alternate (rw) register accessor: PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSCR_alternate)

For information about available fields see [`mod@macppscr_alternate`] module*/
#[doc(alias = "MACPPSCR_alternate")]
pub type MACPPSCR_ALTERNATE = crate::Reg<macppscr_alternate::MACPPSCR_ALTERNATErs>;
///PPS control register
pub mod macppscr_alternate;
/**MACPPSTTS0R (rw) register accessor: PPS 0 target time seconds register

You can [`read`](crate::Reg::read) this register and get [`macppstts0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppstts0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSTTS0R)

For information about available fields see [`mod@macppstts0r`] module*/
pub type MACPPSTTS0R = crate::Reg<macppstts0r::MACPPSTTS0Rrs>;
///PPS 0 target time seconds register
pub mod macppstts0r;
/**MACPPSTTN0R (rw) register accessor: PPS 0 target time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttn0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttn0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSTTN0R)

For information about available fields see [`mod@macppsttn0r`] module*/
pub type MACPPSTTN0R = crate::Reg<macppsttn0r::MACPPSTTN0Rrs>;
///PPS 0 target time nanoseconds register
pub mod macppsttn0r;
/**MACPPSI0R (rw) register accessor: PPS 0 interval register

You can [`read`](crate::Reg::read) this register and get [`macppsi0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsi0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSI0R)

For information about available fields see [`mod@macppsi0r`] module*/
pub type MACPPSI0R = crate::Reg<macppsi0r::MACPPSI0Rrs>;
///PPS 0 interval register
pub mod macppsi0r;
/**MACPPSW0R (rw) register accessor: PPS 0 width register

You can [`read`](crate::Reg::read) this register and get [`macppsw0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsw0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSW0R)

For information about available fields see [`mod@macppsw0r`] module*/
pub type MACPPSW0R = crate::Reg<macppsw0r::MACPPSW0Rrs>;
///PPS 0 width register
pub mod macppsw0r;
/**MACPPSTTS1R (rw) register accessor: PPS 1 target time seconds register

You can [`read`](crate::Reg::read) this register and get [`macppstts1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppstts1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSTTS1R)

For information about available fields see [`mod@macppstts1r`] module*/
pub type MACPPSTTS1R = crate::Reg<macppstts1r::MACPPSTTS1Rrs>;
///PPS 1 target time seconds register
pub mod macppstts1r;
/**MACPPSTTN1R (rw) register accessor: PPS 1 target time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttn1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttn1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSTTN1R)

For information about available fields see [`mod@macppsttn1r`] module*/
pub type MACPPSTTN1R = crate::Reg<macppsttn1r::MACPPSTTN1Rrs>;
///PPS 1 target time nanoseconds register
pub mod macppsttn1r;
/**MACPPSI1R (rw) register accessor: PPS 1 interval register

You can [`read`](crate::Reg::read) this register and get [`macppsi1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsi1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSI1R)

For information about available fields see [`mod@macppsi1r`] module*/
pub type MACPPSI1R = crate::Reg<macppsi1r::MACPPSI1Rrs>;
///PPS 1 interval register
pub mod macppsi1r;
/**MACPPSW1R (rw) register accessor: PPS 1 width register

You can [`read`](crate::Reg::read) this register and get [`macppsw1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsw1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSW1R)

For information about available fields see [`mod@macppsw1r`] module*/
pub type MACPPSW1R = crate::Reg<macppsw1r::MACPPSW1Rrs>;
///PPS 1 width register
pub mod macppsw1r;
/**MACPOCR (rw) register accessor: PTP Offload control register

You can [`read`](crate::Reg::read) this register and get [`macpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPOCR)

For information about available fields see [`mod@macpocr`] module*/
pub type MACPOCR = crate::Reg<macpocr::MACPOCRrs>;
///PTP Offload control register
pub mod macpocr;
/**MACSPI0R (rw) register accessor: PTP Source Port Identity 0 Register

You can [`read`](crate::Reg::read) this register and get [`macspi0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSPI0R)

For information about available fields see [`mod@macspi0r`] module*/
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0Rrs>;
///PTP Source Port Identity 0 Register
pub mod macspi0r;
/**MACSPI1R (rw) register accessor: PTP Source port identity 1 register

You can [`read`](crate::Reg::read) this register and get [`macspi1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSPI1R)

For information about available fields see [`mod@macspi1r`] module*/
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1Rrs>;
///PTP Source port identity 1 register
pub mod macspi1r;
/**MACSPI2R (rw) register accessor: PTP Source port identity 2 register

You can [`read`](crate::Reg::read) this register and get [`macspi2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSPI2R)

For information about available fields see [`mod@macspi2r`] module*/
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2Rrs>;
///PTP Source port identity 2 register
pub mod macspi2r;
/**MACLMIR (rw) register accessor: Log message interval register

You can [`read`](crate::Reg::read) this register and get [`maclmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACLMIR)

For information about available fields see [`mod@maclmir`] module*/
pub type MACLMIR = crate::Reg<maclmir::MACLMIRrs>;
///Log message interval register
pub mod maclmir;
/**MTLOMR (rw) register accessor: Operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLOMR)

For information about available fields see [`mod@mtlomr`] module*/
pub type MTLOMR = crate::Reg<mtlomr::MTLOMRrs>;
///Operating mode Register
pub mod mtlomr;
/**MTLISR (r) register accessor: Interrupt status Register

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLISR)

For information about available fields see [`mod@mtlisr`] module*/
pub type MTLISR = crate::Reg<mtlisr::MTLISRrs>;
///Interrupt status Register
pub mod mtlisr;
/**MTLRXQDMAMR (rw) register accessor: Rx Queue and DMA Channel Mapping Register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqdmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxqdmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQDMAMR)

For information about available fields see [`mod@mtlrxqdmamr`] module*/
pub type MTLRXQDMAMR = crate::Reg<mtlrxqdmamr::MTLRXQDMAMRrs>;
///Rx Queue and DMA Channel Mapping Register
pub mod mtlrxqdmamr;
/**MTLTBSCR (rw) register accessor: TBS control register

You can [`read`](crate::Reg::read) this register and get [`mtltbscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltbscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTBSCR)

For information about available fields see [`mod@mtltbscr`] module*/
pub type MTLTBSCR = crate::Reg<mtltbscr::MTLTBSCRrs>;
///TBS control register
pub mod mtltbscr;
/**MTLESTCR (rw) register accessor: EST Control Register

You can [`read`](crate::Reg::read) this register and get [`mtlestcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTCR)

For information about available fields see [`mod@mtlestcr`] module*/
pub type MTLESTCR = crate::Reg<mtlestcr::MTLESTCRrs>;
///EST Control Register
pub mod mtlestcr;
/**MTLESTECR (rw) register accessor: EST Extended Control Register

You can [`read`](crate::Reg::read) this register and get [`mtlestecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTECR)

For information about available fields see [`mod@mtlestecr`] module*/
pub type MTLESTECR = crate::Reg<mtlestecr::MTLESTECRrs>;
///EST Extended Control Register
pub mod mtlestecr;
/**MTLESTSR (rw) register accessor: EST Status Register

You can [`read`](crate::Reg::read) this register and get [`mtlestsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTSR)

For information about available fields see [`mod@mtlestsr`] module*/
pub type MTLESTSR = crate::Reg<mtlestsr::MTLESTSRrs>;
///EST Status Register
pub mod mtlestsr;
/**MTLESTSCHER (rw) register accessor: EST Schedule Error Register

You can [`read`](crate::Reg::read) this register and get [`mtlestscher::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestscher::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTSCHER)

For information about available fields see [`mod@mtlestscher`] module*/
pub type MTLESTSCHER = crate::Reg<mtlestscher::MTLESTSCHERrs>;
///EST Schedule Error Register
pub mod mtlestscher;
/**MTLESTFSER (rw) register accessor: EST Frame size Error Register

You can [`read`](crate::Reg::read) this register and get [`mtlestfser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestfser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTFSER)

For information about available fields see [`mod@mtlestfser`] module*/
pub type MTLESTFSER = crate::Reg<mtlestfser::MTLESTFSERrs>;
///EST Frame size Error Register
pub mod mtlestfser;
/**MTLESTFSCR (r) register accessor: EST Frame size Capture Register

You can [`read`](crate::Reg::read) this register and get [`mtlestfscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTFSCR)

For information about available fields see [`mod@mtlestfscr`] module*/
pub type MTLESTFSCR = crate::Reg<mtlestfscr::MTLESTFSCRrs>;
///EST Frame size Capture Register
pub mod mtlestfscr;
/**MTLESTIER (rw) register accessor: EST Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`mtlestier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTIER)

For information about available fields see [`mod@mtlestier`] module*/
pub type MTLESTIER = crate::Reg<mtlestier::MTLESTIERrs>;
///EST Interrupt Enable Register
pub mod mtlestier;
/**MTLESTGCLCR (rw) register accessor: EST Gate Control List Register

You can [`read`](crate::Reg::read) this register and get [`mtlestgclcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestgclcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTGCLCR)

For information about available fields see [`mod@mtlestgclcr`] module*/
pub type MTLESTGCLCR = crate::Reg<mtlestgclcr::MTLESTGCLCRrs>;
///EST Gate Control List Register
pub mod mtlestgclcr;
/**MTLESTGCLDR (rw) register accessor: EST Gate Control List Data Register

You can [`read`](crate::Reg::read) this register and get [`mtlestgcldr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestgcldr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLESTGCLDR)

For information about available fields see [`mod@mtlestgcldr`] module*/
pub type MTLESTGCLDR = crate::Reg<mtlestgcldr::MTLESTGCLDRrs>;
///EST Gate Control List Data Register
pub mod mtlestgcldr;
/**MTLFPECSR (rw) register accessor: FPE Frame Preemption Control Status Register

You can [`read`](crate::Reg::read) this register and get [`mtlfpecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlfpecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLFPECSR)

For information about available fields see [`mod@mtlfpecsr`] module*/
pub type MTLFPECSR = crate::Reg<mtlfpecsr::MTLFPECSRrs>;
///FPE Frame Preemption Control Status Register
pub mod mtlfpecsr;
/**MTLFPEAR (rw) register accessor: FPE Frame Preemption Advance Register

You can [`read`](crate::Reg::read) this register and get [`mtlfpear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlfpear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLFPEAR)

For information about available fields see [`mod@mtlfpear`] module*/
pub type MTLFPEAR = crate::Reg<mtlfpear::MTLFPEARrs>;
///FPE Frame Preemption Advance Register
pub mod mtlfpear;
/**MTLTXQ0OMR (rw) register accessor: T0 queue 0 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0OMR)

For information about available fields see [`mod@mtltxq0omr`] module*/
pub type MTLTXQ0OMR = crate::Reg<mtltxq0omr::MTLTXQ0OMRrs>;
///T0 queue 0 operating mode Register
pub mod mtltxq0omr;
/**MTLTXQ0UR (rw) register accessor: T0 queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0ur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0ur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0UR)

For information about available fields see [`mod@mtltxq0ur`] module*/
pub type MTLTXQ0UR = crate::Reg<mtltxq0ur::MTLTXQ0URrs>;
///T0 queue 0 underflow register
pub mod mtltxq0ur;
/**MTLTXQ0DR (r) register accessor: T0 queue 0 debug register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0DR)

For information about available fields see [`mod@mtltxq0dr`] module*/
pub type MTLTXQ0DR = crate::Reg<mtltxq0dr::MTLTXQ0DRrs>;
///T0 queue 0 debug register
pub mod mtltxq0dr;
/**MTLTXQ0ESR (r) register accessor: T0 queue 0 ETS status Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0ESR)

For information about available fields see [`mod@mtltxq0esr`] module*/
pub type MTLTXQ0ESR = crate::Reg<mtltxq0esr::MTLTXQ0ESRrs>;
///T0 queue 0 ETS status Register
pub mod mtltxq0esr;
/**MTLTXQ0QWR (rw) register accessor: Tx queue 0 quantum weight register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0qwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0qwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ0QWR)

For information about available fields see [`mod@mtltxq0qwr`] module*/
pub type MTLTXQ0QWR = crate::Reg<mtltxq0qwr::MTLTXQ0QWRrs>;
///Tx queue 0 quantum weight register
pub mod mtltxq0qwr;
/**MTLQ0ICSR (rw) register accessor: Queue 0 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq0icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq0icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLQ0ICSR)

For information about available fields see [`mod@mtlq0icsr`] module*/
pub type MTLQ0ICSR = crate::Reg<mtlq0icsr::MTLQ0ICSRrs>;
///Queue 0 interrupt control status Register
pub mod mtlq0icsr;
/**MTLRXQ0OMR (rw) register accessor: R0 queue 0 operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ0OMR)

For information about available fields see [`mod@mtlrxq0omr`] module*/
pub type MTLRXQ0OMR = crate::Reg<mtlrxq0omr::MTLRXQ0OMRrs>;
///R0 queue 0 operating mode register
pub mod mtlrxq0omr;
/**MTLRXQ0MPOCR (rw) register accessor: R0 queue 0 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0mpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq0mpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ0MPOCR)

For information about available fields see [`mod@mtlrxq0mpocr`] module*/
pub type MTLRXQ0MPOCR = crate::Reg<mtlrxq0mpocr::MTLRXQ0MPOCRrs>;
///R0 queue 0 missed packet and overflow counter register
pub mod mtlrxq0mpocr;
/**MTLRXQ0DR (r) register accessor: R0 queue 0 debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ0DR)

For information about available fields see [`mod@mtlrxq0dr`] module*/
pub type MTLRXQ0DR = crate::Reg<mtlrxq0dr::MTLRXQ0DRrs>;
///R0 queue 0 debug register
pub mod mtlrxq0dr;
/**MTLRXQ0CR (rw) register accessor: R0 queue 0 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ0CR)

For information about available fields see [`mod@mtlrxq0cr`] module*/
pub type MTLRXQ0CR = crate::Reg<mtlrxq0cr::MTLRXQ0CRrs>;
///R0 queue 0 control register
pub mod mtlrxq0cr;
/**MTLTXQ1OMR (rw) register accessor: T1 queue 1 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1OMR)

For information about available fields see [`mod@mtltxq1omr`] module*/
pub type MTLTXQ1OMR = crate::Reg<mtltxq1omr::MTLTXQ1OMRrs>;
///T1 queue 1 operating mode Register
pub mod mtltxq1omr;
/**MTLTXQ1UR (rw) register accessor: T1 queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1ur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1ur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1UR)

For information about available fields see [`mod@mtltxq1ur`] module*/
pub type MTLTXQ1UR = crate::Reg<mtltxq1ur::MTLTXQ1URrs>;
///T1 queue 1 underflow register
pub mod mtltxq1ur;
/**MTLTXQ1DR (r) register accessor: T1 queue 1 debug register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1DR)

For information about available fields see [`mod@mtltxq1dr`] module*/
pub type MTLTXQ1DR = crate::Reg<mtltxq1dr::MTLTXQ1DRrs>;
///T1 queue 1 debug register
pub mod mtltxq1dr;
/**MTLTXQ1ECR (rw) register accessor: Tx queue 1 ETS control Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1ECR)

For information about available fields see [`mod@mtltxq1ecr`] module*/
pub type MTLTXQ1ECR = crate::Reg<mtltxq1ecr::MTLTXQ1ECRrs>;
///Tx queue 1 ETS control Register
pub mod mtltxq1ecr;
/**MTLTXQ1ESR (r) register accessor: T1 queue 1 ETS status Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1ESR)

For information about available fields see [`mod@mtltxq1esr`] module*/
pub type MTLTXQ1ESR = crate::Reg<mtltxq1esr::MTLTXQ1ESRrs>;
///T1 queue 1 ETS status Register
pub mod mtltxq1esr;
/**MTLTXQ1QWR (rw) register accessor: Tx queue 1 quantum weight register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1qwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1qwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1QWR)

For information about available fields see [`mod@mtltxq1qwr`] module*/
pub type MTLTXQ1QWR = crate::Reg<mtltxq1qwr::MTLTXQ1QWRrs>;
///Tx queue 1 quantum weight register
pub mod mtltxq1qwr;
/**MTLTXQ1SSCR (rw) register accessor: Tx queue 1 send slope credit Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1SSCR)

For information about available fields see [`mod@mtltxq1sscr`] module*/
pub type MTLTXQ1SSCR = crate::Reg<mtltxq1sscr::MTLTXQ1SSCRrs>;
///Tx queue 1 send slope credit Register
pub mod mtltxq1sscr;
/**MTLTXQ1HCR (rw) register accessor: Tx Queue 1 hiCredit register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1HCR)

For information about available fields see [`mod@mtltxq1hcr`] module*/
pub type MTLTXQ1HCR = crate::Reg<mtltxq1hcr::MTLTXQ1HCRrs>;
///Tx Queue 1 hiCredit register
pub mod mtltxq1hcr;
/**MTLTXQ1LCR (rw) register accessor: Tx queue 1 loCredit register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1LCR)

For information about available fields see [`mod@mtltxq1lcr`] module*/
pub type MTLTXQ1LCR = crate::Reg<mtltxq1lcr::MTLTXQ1LCRrs>;
///Tx queue 1 loCredit register
pub mod mtltxq1lcr;
/**MTLQ1ICSR (rw) register accessor: Queue 1 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq1icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq1icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLQ1ICSR)

For information about available fields see [`mod@mtlq1icsr`] module*/
pub type MTLQ1ICSR = crate::Reg<mtlq1icsr::MTLQ1ICSRrs>;
///Queue 1 interrupt control status Register
pub mod mtlq1icsr;
/**MTLRXQ1OMR (rw) register accessor: R1 queue 1 operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ1OMR)

For information about available fields see [`mod@mtlrxq1omr`] module*/
pub type MTLRXQ1OMR = crate::Reg<mtlrxq1omr::MTLRXQ1OMRrs>;
///R1 queue 1 operating mode register
pub mod mtlrxq1omr;
/**MTLRXQ1MPOCR (rw) register accessor: R1 queue 1 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq1mpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq1mpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ1MPOCR)

For information about available fields see [`mod@mtlrxq1mpocr`] module*/
pub type MTLRXQ1MPOCR = crate::Reg<mtlrxq1mpocr::MTLRXQ1MPOCRrs>;
///R1 queue 1 missed packet and overflow counter register
pub mod mtlrxq1mpocr;
/**MTLRXQ1DR (r) register accessor: R1 queue 1 debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ1DR)

For information about available fields see [`mod@mtlrxq1dr`] module*/
pub type MTLRXQ1DR = crate::Reg<mtlrxq1dr::MTLRXQ1DRrs>;
///R1 queue 1 debug register
pub mod mtlrxq1dr;
/**MTLRXQ1CR (rw) register accessor: R1 queue 1 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ1CR)

For information about available fields see [`mod@mtlrxq1cr`] module*/
pub type MTLRXQ1CR = crate::Reg<mtlrxq1cr::MTLRXQ1CRrs>;
///R1 queue 1 control register
pub mod mtlrxq1cr;
/**DMAMR (rw) register accessor: DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAMR)

For information about available fields see [`mod@dmamr`] module*/
pub type DMAMR = crate::Reg<dmamr::DMAMRrs>;
///DMA mode register
pub mod dmamr;
/**DMASBMR (rw) register accessor: System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMASBMR)

For information about available fields see [`mod@dmasbmr`] module*/
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMRrs>;
///System bus mode register
pub mod dmasbmr;
/**DMAISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAISR)

For information about available fields see [`mod@dmaisr`] module*/
pub type DMAISR = crate::Reg<dmaisr::DMAISRrs>;
///Interrupt status register
pub mod dmaisr;
/**DMADSR (r) register accessor: Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMADSR)

For information about available fields see [`mod@dmadsr`] module*/
pub type DMADSR = crate::Reg<dmadsr::DMADSRrs>;
///Debug status register
pub mod dmadsr;
/**DMAA4TXACR (rw) register accessor: AXI4 transmit channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4txacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4txacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAA4TXACR)

For information about available fields see [`mod@dmaa4txacr`] module*/
pub type DMAA4TXACR = crate::Reg<dmaa4txacr::DMAA4TXACRrs>;
///AXI4 transmit channel ACE control register
pub mod dmaa4txacr;
/**DMAA4RXACR (rw) register accessor: AXI4 receive channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4rxacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4rxacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAA4RXACR)

For information about available fields see [`mod@dmaa4rxacr`] module*/
pub type DMAA4RXACR = crate::Reg<dmaa4rxacr::DMAA4RXACRrs>;
///AXI4 receive channel ACE control register
pub mod dmaa4rxacr;
/**DMAA4DACR (rw) register accessor: AXI4 descriptor ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAA4DACR)

For information about available fields see [`mod@dmaa4dacr`] module*/
pub type DMAA4DACR = crate::Reg<dmaa4dacr::DMAA4DACRrs>;
///AXI4 descriptor ACE control register
pub mod dmaa4dacr;
/**DMALPIEI (rw) register accessor: AXI4 LPI Entry Interval register

You can [`read`](crate::Reg::read) this register and get [`dmalpiei::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmalpiei::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMALPIEI)

For information about available fields see [`mod@dmalpiei`] module*/
pub type DMALPIEI = crate::Reg<dmalpiei::DMALPIEIrs>;
///AXI4 LPI Entry Interval register
pub mod dmalpiei;
/**DMATBSCTRL0R (rw) register accessor: DMA TBS control register 0

You can [`read`](crate::Reg::read) this register and get [`dmatbsctrl0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatbsctrl0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMATBSCTRL0R)

For information about available fields see [`mod@dmatbsctrl0r`] module*/
pub type DMATBSCTRL0R = crate::Reg<dmatbsctrl0r::DMATBSCTRL0Rrs>;
///DMA TBS control register 0
pub mod dmatbsctrl0r;
/**DMAC0CR (rw) register accessor: Channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`dmac0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CR)

For information about available fields see [`mod@dmac0cr`] module*/
pub type DMAC0CR = crate::Reg<dmac0cr::DMAC0CRrs>;
///Channel 0 control register
pub mod dmac0cr;
/**DMAC0TXCR (rw) register accessor: Channel 0 transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmac0txcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0TXCR)

For information about available fields see [`mod@dmac0txcr`] module*/
pub type DMAC0TXCR = crate::Reg<dmac0txcr::DMAC0TXCRrs>;
///Channel 0 transmit control register
pub mod dmac0txcr;
/**DMAC0RXCR (rw) register accessor: Channel 0 receive control register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXCR)

For information about available fields see [`mod@dmac0rxcr`] module*/
pub type DMAC0RXCR = crate::Reg<dmac0rxcr::DMAC0RXCRrs>;
///Channel 0 receive control register
pub mod dmac0rxcr;
/**DMAC0TXDLAR (rw) register accessor: Channel 0 T0 descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0txdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0TXDLAR)

For information about available fields see [`mod@dmac0txdlar`] module*/
pub type DMAC0TXDLAR = crate::Reg<dmac0txdlar::DMAC0TXDLARrs>;
///Channel 0 T0 descriptor list address register
pub mod dmac0txdlar;
/**DMAC0RXDLAR (rw) register accessor: Channel 0 Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXDLAR)

For information about available fields see [`mod@dmac0rxdlar`] module*/
pub type DMAC0RXDLAR = crate::Reg<dmac0rxdlar::DMAC0RXDLARrs>;
///Channel 0 Rx descriptor list address register
pub mod dmac0rxdlar;
/**DMAC0TXDTPR (rw) register accessor: Channel 0 T0 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0txdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0TXDTPR)

For information about available fields see [`mod@dmac0txdtpr`] module*/
pub type DMAC0TXDTPR = crate::Reg<dmac0txdtpr::DMAC0TXDTPRrs>;
///Channel 0 T0 descriptor tail pointer register
pub mod dmac0txdtpr;
/**DMAC0RXDTPR (rw) register accessor: Channel 0 R0 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXDTPR)

For information about available fields see [`mod@dmac0rxdtpr`] module*/
pub type DMAC0RXDTPR = crate::Reg<dmac0rxdtpr::DMAC0RXDTPRrs>;
///Channel 0 R0 descriptor tail pointer register
pub mod dmac0rxdtpr;
/**DMAC0TXRLR (rw) register accessor: Channel 0 T0 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0txrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0TXRLR)

For information about available fields see [`mod@dmac0txrlr`] module*/
pub type DMAC0TXRLR = crate::Reg<dmac0txrlr::DMAC0TXRLRrs>;
///Channel 0 T0 descriptor ring length register
pub mod dmac0txrlr;
/**DMAC0RXRLR (rw) register accessor: Channel 0 R0 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXRLR)

For information about available fields see [`mod@dmac0rxrlr`] module*/
pub type DMAC0RXRLR = crate::Reg<dmac0rxrlr::DMAC0RXRLRrs>;
///Channel 0 R0 descriptor ring length register
pub mod dmac0rxrlr;
/**DMAC0IER (rw) register accessor: Channel 0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmac0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0IER)

For information about available fields see [`mod@dmac0ier`] module*/
pub type DMAC0IER = crate::Reg<dmac0ier::DMAC0IERrs>;
///Channel 0 interrupt enable register
pub mod dmac0ier;
/**DMAC0RXIWTR (rw) register accessor: Channel 0 R0 interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxiwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxiwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXIWTR)

For information about available fields see [`mod@dmac0rxiwtr`] module*/
pub type DMAC0RXIWTR = crate::Reg<dmac0rxiwtr::DMAC0RXIWTRrs>;
///Channel 0 R0 interrupt watchdog timer register
pub mod dmac0rxiwtr;
/**DMAC0SFCSR (rw) register accessor: Channel 0 slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac0sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0SFCSR)

For information about available fields see [`mod@dmac0sfcsr`] module*/
pub type DMAC0SFCSR = crate::Reg<dmac0sfcsr::DMAC0SFCSRrs>;
///Channel 0 slot function control status register
pub mod dmac0sfcsr;
/**DMAC0CATXDR (r) register accessor: Channel 0 current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0catxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CATXDR)

For information about available fields see [`mod@dmac0catxdr`] module*/
pub type DMAC0CATXDR = crate::Reg<dmac0catxdr::DMAC0CATXDRrs>;
///Channel 0 current application transmit descriptor register
pub mod dmac0catxdr;
/**DMAC0CARXDR (r) register accessor: Channel 0 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0carxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CARXDR)

For information about available fields see [`mod@dmac0carxdr`] module*/
pub type DMAC0CARXDR = crate::Reg<dmac0carxdr::DMAC0CARXDRrs>;
///Channel 0 current application receive descriptor register
pub mod dmac0carxdr;
/**DMAC0CATXBR (r) register accessor: Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac0catxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CATXBR)

For information about available fields see [`mod@dmac0catxbr`] module*/
pub type DMAC0CATXBR = crate::Reg<dmac0catxbr::DMAC0CATXBRrs>;
///Channel 0 current application transmit buffer register
pub mod dmac0catxbr;
/**DMAC0CARXBR (r) register accessor: Channel 0 current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac0carxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CARXBR)

For information about available fields see [`mod@dmac0carxbr`] module*/
pub type DMAC0CARXBR = crate::Reg<dmac0carxbr::DMAC0CARXBRrs>;
///Channel 0 current application receive buffer register
pub mod dmac0carxbr;
/**DMAC0SR (rw) register accessor: Channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`dmac0sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0SR)

For information about available fields see [`mod@dmac0sr`] module*/
pub type DMAC0SR = crate::Reg<dmac0sr::DMAC0SRrs>;
///Channel 0 status register
pub mod dmac0sr;
/**DMAC0MFCR (rw) register accessor: Channel 0 missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac0mfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0mfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0MFCR)

For information about available fields see [`mod@dmac0mfcr`] module*/
pub type DMAC0MFCR = crate::Reg<dmac0mfcr::DMAC0MFCRrs>;
///Channel 0 missed frame count register
pub mod dmac0mfcr;
/**DMAC1CR (rw) register accessor: Channel 1 control register

You can [`read`](crate::Reg::read) this register and get [`dmac1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CR)

For information about available fields see [`mod@dmac1cr`] module*/
pub type DMAC1CR = crate::Reg<dmac1cr::DMAC1CRrs>;
///Channel 1 control register
pub mod dmac1cr;
/**DMAC1TXCR (rw) register accessor: Channel 1 transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmac1txcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1txcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1TXCR)

For information about available fields see [`mod@dmac1txcr`] module*/
pub type DMAC1TXCR = crate::Reg<dmac1txcr::DMAC1TXCRrs>;
///Channel 1 transmit control register
pub mod dmac1txcr;
/**DMAC1RXCR (rw) register accessor: Channel 1 receive control register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1RXCR)

For information about available fields see [`mod@dmac1rxcr`] module*/
pub type DMAC1RXCR = crate::Reg<dmac1rxcr::DMAC1RXCRrs>;
///Channel 1 receive control register
pub mod dmac1rxcr;
/**DMAC1TXDLAR (rw) register accessor: Channel 1 T1 descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac1txdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1txdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1TXDLAR)

For information about available fields see [`mod@dmac1txdlar`] module*/
pub type DMAC1TXDLAR = crate::Reg<dmac1txdlar::DMAC1TXDLARrs>;
///Channel 1 T1 descriptor list address register
pub mod dmac1txdlar;
/**DMAC1TXDTPR (rw) register accessor: Channel 1 T1 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac1txdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1txdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1TXDTPR)

For information about available fields see [`mod@dmac1txdtpr`] module*/
pub type DMAC1TXDTPR = crate::Reg<dmac1txdtpr::DMAC1TXDTPRrs>;
///Channel 1 T1 descriptor tail pointer register
pub mod dmac1txdtpr;
/**DMAC1RXDTPR (rw) register accessor: Channel 1 R1 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxdtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxdtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1RXDTPR)

For information about available fields see [`mod@dmac1rxdtpr`] module*/
pub type DMAC1RXDTPR = crate::Reg<dmac1rxdtpr::DMAC1RXDTPRrs>;
///Channel 1 R1 descriptor tail pointer register
pub mod dmac1rxdtpr;
/**DMAC1TXRLR (rw) register accessor: Channel 1 T1 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac1txrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1txrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1TXRLR)

For information about available fields see [`mod@dmac1txrlr`] module*/
pub type DMAC1TXRLR = crate::Reg<dmac1txrlr::DMAC1TXRLRrs>;
///Channel 1 T1 descriptor ring length register
pub mod dmac1txrlr;
/**DMAC1RXRLR (rw) register accessor: Channel 1 R1 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1RXRLR)

For information about available fields see [`mod@dmac1rxrlr`] module*/
pub type DMAC1RXRLR = crate::Reg<dmac1rxrlr::DMAC1RXRLRrs>;
///Channel 1 R1 descriptor ring length register
pub mod dmac1rxrlr;
/**DMAC1IER (rw) register accessor: Channel 1 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmac1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1IER)

For information about available fields see [`mod@dmac1ier`] module*/
pub type DMAC1IER = crate::Reg<dmac1ier::DMAC1IERrs>;
///Channel 1 interrupt enable register
pub mod dmac1ier;
/**DMAC1RXIWTR (rw) register accessor: Channel 1 R1 interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxiwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxiwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1RXIWTR)

For information about available fields see [`mod@dmac1rxiwtr`] module*/
pub type DMAC1RXIWTR = crate::Reg<dmac1rxiwtr::DMAC1RXIWTRrs>;
///Channel 1 R1 interrupt watchdog timer register
pub mod dmac1rxiwtr;
/**DMAC1SFCSR (rw) register accessor: Channel 1 slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1SFCSR)

For information about available fields see [`mod@dmac1sfcsr`] module*/
pub type DMAC1SFCSR = crate::Reg<dmac1sfcsr::DMAC1SFCSRrs>;
///Channel 1 slot function control status register
pub mod dmac1sfcsr;
/**DMAC1CATXDR (r) register accessor: Channel 1 current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac1catxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CATXDR)

For information about available fields see [`mod@dmac1catxdr`] module*/
pub type DMAC1CATXDR = crate::Reg<dmac1catxdr::DMAC1CATXDRrs>;
///Channel 1 current application transmit descriptor register
pub mod dmac1catxdr;
/**DMAC1CARXDR (r) register accessor: Channel 1 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac1carxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CARXDR)

For information about available fields see [`mod@dmac1carxdr`] module*/
pub type DMAC1CARXDR = crate::Reg<dmac1carxdr::DMAC1CARXDRrs>;
///Channel 1 current application receive descriptor register
pub mod dmac1carxdr;
/**DMAC1CATXBR (r) register accessor: Channel 1 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1catxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CATXBR)

For information about available fields see [`mod@dmac1catxbr`] module*/
pub type DMAC1CATXBR = crate::Reg<dmac1catxbr::DMAC1CATXBRrs>;
///Channel 1 current application transmit buffer register
pub mod dmac1catxbr;
/**DMAC1CARXBR (r) register accessor: Channel 1 current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1carxbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CARXBR)

For information about available fields see [`mod@dmac1carxbr`] module*/
pub type DMAC1CARXBR = crate::Reg<dmac1carxbr::DMAC1CARXBRrs>;
///Channel 1 current application receive buffer register
pub mod dmac1carxbr;
/**DMAC1SR (rw) register accessor: Channel 1 status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1SR)

For information about available fields see [`mod@dmac1sr`] module*/
pub type DMAC1SR = crate::Reg<dmac1sr::DMAC1SRrs>;
///Channel 1 status register
pub mod dmac1sr;
/**DMAC1MFCR (rw) register accessor: Channel 1 missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac1mfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1mfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1MFCR)

For information about available fields see [`mod@dmac1mfcr`] module*/
pub type DMAC1MFCR = crate::Reg<dmac1mfcr::DMAC1MFCRrs>;
///Channel 1 missed frame count register
pub mod dmac1mfcr;
