#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode configuration register"]
    pub maccr: crate::Reg<maccr::MACCR_SPEC>,
    #[doc = "0x04 - Extended operating mode configuration register"]
    pub macecr: crate::Reg<macecr::MACECR_SPEC>,
    #[doc = "0x08 - Packet filtering control register"]
    pub macpfr: crate::Reg<macpfr::MACPFR_SPEC>,
    #[doc = "0x0c - Watchdog timeout register"]
    pub macwtr: crate::Reg<macwtr::MACWTR_SPEC>,
    #[doc = "0x10 - Hash Table 0 register"]
    pub macht0r: crate::Reg<macht0r::MACHT0R_SPEC>,
    #[doc = "0x14 - Hash Table 1 register"]
    pub macht1r: crate::Reg<macht1r::MACHT1R_SPEC>,
    _reserved6: [u8; 0x38],
    #[doc = "0x50 - VLAN tag register"]
    pub macvtr: crate::Reg<macvtr::MACVTR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x58 - VLAN Hash table register"]
    pub macvhtr: crate::Reg<macvhtr::MACVHTR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x60 - VLAN inclusion register"]
    pub macvir: crate::Reg<macvir::MACVIR_SPEC>,
    #[doc = "0x64 - Inner VLAN inclusion register"]
    pub macivir: crate::Reg<macivir::MACIVIR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x70 - Tx Queue flow control register"]
    pub macqtx_fcr: crate::Reg<macqtx_fcr::MACQTXFCR_SPEC>,
    _reserved11: [u8; 0x1c],
    #[doc = "0x90 - Rx flow control register"]
    pub macrx_fcr: crate::Reg<macrx_fcr::MACRXFCR_SPEC>,
    _reserved12: [u8; 0x1c],
    #[doc = "0xb0 - Interrupt status register"]
    pub macisr: crate::Reg<macisr::MACISR_SPEC>,
    #[doc = "0xb4 - Interrupt enable register"]
    pub macier: crate::Reg<macier::MACIER_SPEC>,
    #[doc = "0xb8 - Rx Tx status register"]
    pub macrx_tx_sr: crate::Reg<macrx_tx_sr::MACRXTXSR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0xc0 - PMT control status register"]
    pub macpcsr: crate::Reg<macpcsr::MACPCSR_SPEC>,
    #[doc = "0xc4 - Remove wakeup packet filter register"]
    pub macrwkpfr: crate::Reg<macrwkpfr::MACRWKPFR_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0xd0 - LPI control status register"]
    pub maclcsr: crate::Reg<maclcsr::MACLCSR_SPEC>,
    #[doc = "0xd4 - LPI timers control register"]
    pub macltcr: crate::Reg<macltcr::MACLTCR_SPEC>,
    #[doc = "0xd8 - LPI entry timer register"]
    pub macletr: crate::Reg<macletr::MACLETR_SPEC>,
    #[doc = "0xdc - 1-microsecond-tick counter register"]
    pub mac1ustcr: crate::Reg<mac1ustcr::MAC1USTCR_SPEC>,
    _reserved21: [u8; 0x30],
    #[doc = "0x110 - Version register"]
    pub macvr: crate::Reg<macvr::MACVR_SPEC>,
    #[doc = "0x114 - Debug register"]
    pub macdr: crate::Reg<macdr::MACDR_SPEC>,
    _reserved23: [u8; 0x08],
    #[doc = "0x120 - HW feature 1 register"]
    pub machwf1r: crate::Reg<machwf1r::MACHWF1R_SPEC>,
    #[doc = "0x124 - HW feature 2 register"]
    pub machwf2r: crate::Reg<machwf2r::MACHWF2R_SPEC>,
    _reserved25: [u8; 0xd8],
    #[doc = "0x200 - MDIO address register"]
    pub macmdioar: crate::Reg<macmdioar::MACMDIOAR_SPEC>,
    #[doc = "0x204 - MDIO data register"]
    pub macmdiodr: crate::Reg<macmdiodr::MACMDIODR_SPEC>,
    _reserved27: [u8; 0xf8],
    #[doc = "0x300 - Address 0 high register"]
    pub maca0hr: crate::Reg<maca0hr::MACA0HR_SPEC>,
    #[doc = "0x304 - Address 0 low register"]
    pub maca0lr: crate::Reg<maca0lr::MACA0LR_SPEC>,
    #[doc = "0x308 - Address 1 high register"]
    pub maca1hr: crate::Reg<maca1hr::MACA1HR_SPEC>,
    #[doc = "0x30c - Address 1 low register"]
    pub maca1lr: crate::Reg<maca1lr::MACA1LR_SPEC>,
    #[doc = "0x310 - Address 2 high register"]
    pub maca2hr: crate::Reg<maca2hr::MACA2HR_SPEC>,
    #[doc = "0x314 - Address 2 low register"]
    pub maca2lr: crate::Reg<maca2lr::MACA2LR_SPEC>,
    #[doc = "0x318 - Address 3 high register"]
    pub maca3hr: crate::Reg<maca3hr::MACA3HR_SPEC>,
    #[doc = "0x31c - Address 3 low register"]
    pub maca3lr: crate::Reg<maca3lr::MACA3LR_SPEC>,
    _reserved35: [u8; 0x03e0],
    #[doc = "0x700 - MMC control register"]
    pub mmc_control: crate::Reg<mmc_control::MMC_CONTROL_SPEC>,
    #[doc = "0x704 - MMC Rx interrupt register"]
    pub mmc_rx_interrupt: crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPT_SPEC>,
    #[doc = "0x708 - MMC Tx interrupt register"]
    pub mmc_tx_interrupt: crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPT_SPEC>,
    #[doc = "0x70c - MMC Rx interrupt mask register"]
    pub mmc_rx_interrupt_mask: crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASK_SPEC>,
    #[doc = "0x710 - MMC Tx interrupt mask register"]
    pub mmc_tx_interrupt_mask: crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASK_SPEC>,
    _reserved40: [u8; 0x38],
    #[doc = "0x74c - Tx single collision good packets register"]
    pub tx_single_collision_good_packets:
        crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>,
    #[doc = "0x750 - Tx multiple collision good packets register"]
    pub tx_multiple_collision_good_packets:
        crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>,
    _reserved42: [u8; 0x14],
    #[doc = "0x768 - Tx packet count good register"]
    pub tx_packet_count_good: crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOOD_SPEC>,
    _reserved43: [u8; 0x28],
    #[doc = "0x794 - Rx CRC error packets register"]
    pub rx_crc_error_packets: crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETS_SPEC>,
    #[doc = "0x798 - Rx alignment error packets register"]
    pub rx_alignment_error_packets:
        crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETS_SPEC>,
    _reserved45: [u8; 0x28],
    #[doc = "0x7c4 - Rx unicast packets good register"]
    pub rx_unicast_packets_good: crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOOD_SPEC>,
    _reserved46: [u8; 0x24],
    #[doc = "0x7ec - Tx LPI microsecond timer register"]
    pub tx_lpi_usec_cntr: crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTR_SPEC>,
    #[doc = "0x7f0 - Tx LPI transition counter register"]
    pub tx_lpi_tran_cntr: crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTR_SPEC>,
    #[doc = "0x7f4 - Rx LPI microsecond counter register"]
    pub rx_lpi_usec_cntr: crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTR_SPEC>,
    #[doc = "0x7f8 - Rx LPI transition counter register"]
    pub rx_lpi_tran_cntr: crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTR_SPEC>,
    _reserved50: [u8; 0x0104],
    #[doc = "0x900 - L3 and L4 control 0 register"]
    pub macl3l4c0r: crate::Reg<macl3l4c0r::MACL3L4C0R_SPEC>,
    #[doc = "0x904 - Layer4 address filter 0 register"]
    pub macl4a0r: crate::Reg<macl4a0r::MACL4A0R_SPEC>,
    _reserved52: [u8; 0x08],
    #[doc = "0x910 - MACL3A00R"]
    pub macl3a00r: crate::Reg<macl3a00r::MACL3A00R_SPEC>,
    #[doc = "0x914 - Layer3 address 1 filter 0 register"]
    pub macl3a10r: crate::Reg<macl3a10r::MACL3A10R_SPEC>,
    #[doc = "0x918 - Layer3 Address 2 filter 0 register"]
    pub macl3a20: crate::Reg<macl3a20::MACL3A20_SPEC>,
    #[doc = "0x91c - Layer3 Address 3 filter 0 register"]
    pub macl3a30: crate::Reg<macl3a30::MACL3A30_SPEC>,
    _reserved56: [u8; 0x10],
    #[doc = "0x930 - L3 and L4 control 1 register"]
    pub macl3l4c1r: crate::Reg<macl3l4c1r::MACL3L4C1R_SPEC>,
    #[doc = "0x934 - Layer 4 address filter 1 register"]
    pub macl4a1r: crate::Reg<macl4a1r::MACL4A1R_SPEC>,
    _reserved58: [u8; 0x08],
    #[doc = "0x940 - Layer3 address 0 filter 1 Register"]
    pub macl3a01r: crate::Reg<macl3a01r::MACL3A01R_SPEC>,
    #[doc = "0x944 - Layer3 address 1 filter 1 register"]
    pub macl3a11r: crate::Reg<macl3a11r::MACL3A11R_SPEC>,
    #[doc = "0x948 - Layer3 address 2 filter 1 Register"]
    pub macl3a21r: crate::Reg<macl3a21r::MACL3A21R_SPEC>,
    #[doc = "0x94c - Layer3 address 3 filter 1 register"]
    pub macl3a31r: crate::Reg<macl3a31r::MACL3A31R_SPEC>,
    _reserved62: [u8; 0x0190],
    #[doc = "0xae0 - ARP address register"]
    pub macarpar: crate::Reg<macarpar::MACARPAR_SPEC>,
    _reserved63: [u8; 0x1c],
    #[doc = "0xb00 - Timestamp control Register"]
    pub mactscr: crate::Reg<mactscr::MACTSCR_SPEC>,
    #[doc = "0xb04 - Sub-second increment register"]
    pub macssir: crate::Reg<macssir::MACSSIR_SPEC>,
    #[doc = "0xb08 - System time seconds register"]
    pub macstsr: crate::Reg<macstsr::MACSTSR_SPEC>,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub macstnr: crate::Reg<macstnr::MACSTNR_SPEC>,
    #[doc = "0xb10 - System time seconds update register"]
    pub macstsur: crate::Reg<macstsur::MACSTSUR_SPEC>,
    #[doc = "0xb14 - System time nanoseconds update register"]
    pub macstnur: crate::Reg<macstnur::MACSTNUR_SPEC>,
    #[doc = "0xb18 - Timestamp addend register"]
    pub mactsar: crate::Reg<mactsar::MACTSAR_SPEC>,
    _reserved70: [u8; 0x04],
    #[doc = "0xb20 - Timestamp status register"]
    pub mactssr: crate::Reg<mactssr::MACTSSR_SPEC>,
    _reserved71: [u8; 0x0c],
    #[doc = "0xb30 - Tx timestamp status nanoseconds register"]
    pub mactx_tssnr: crate::Reg<mactx_tssnr::MACTXTSSNR_SPEC>,
    #[doc = "0xb34 - Tx timestamp status seconds register"]
    pub mactx_tsssr: crate::Reg<mactx_tsssr::MACTXTSSSR_SPEC>,
    _reserved73: [u8; 0x08],
    #[doc = "0xb40 - Auxiliary control register"]
    pub macacr: crate::Reg<macacr::MACACR_SPEC>,
    _reserved74: [u8; 0x04],
    #[doc = "0xb48 - Auxiliary timestamp nanoseconds register"]
    pub macatsnr: crate::Reg<macatsnr::MACATSNR_SPEC>,
    #[doc = "0xb4c - Auxiliary timestamp seconds register"]
    pub macatssr: crate::Reg<macatssr::MACATSSR_SPEC>,
    #[doc = "0xb50 - Timestamp Ingress asymmetric correction register"]
    pub mactsiacr: crate::Reg<mactsiacr::MACTSIACR_SPEC>,
    #[doc = "0xb54 - Timestamp Egress asymmetric correction register"]
    pub mactseacr: crate::Reg<mactseacr::MACTSEACR_SPEC>,
    #[doc = "0xb58 - Timestamp Ingress correction nanosecond register"]
    pub mactsicnr: crate::Reg<mactsicnr::MACTSICNR_SPEC>,
    #[doc = "0xb5c - Timestamp Egress correction nanosecond register"]
    pub mactsecnr: crate::Reg<mactsecnr::MACTSECNR_SPEC>,
    _reserved80: [u8; 0x10],
    #[doc = "0xb70 - PPS control register"]
    pub macppscr: crate::Reg<macppscr::MACPPSCR_SPEC>,
    _reserved81: [u8; 0x0c],
    #[doc = "0xb80 - PPS target time seconds register"]
    pub macppsttsr: crate::Reg<macppsttsr::MACPPSTTSR_SPEC>,
    #[doc = "0xb84 - PPS target time nanoseconds register"]
    pub macppsttnr: crate::Reg<macppsttnr::MACPPSTTNR_SPEC>,
    #[doc = "0xb88 - PPS interval register"]
    pub macppsir: crate::Reg<macppsir::MACPPSIR_SPEC>,
    #[doc = "0xb8c - PPS width register"]
    pub macppswr: crate::Reg<macppswr::MACPPSWR_SPEC>,
    _reserved85: [u8; 0x30],
    #[doc = "0xbc0 - PTP Offload control register"]
    pub macpocr: crate::Reg<macpocr::MACPOCR_SPEC>,
    #[doc = "0xbc4 - PTP Source Port Identity 0 Register"]
    pub macspi0r: crate::Reg<macspi0r::MACSPI0R_SPEC>,
    #[doc = "0xbc8 - PTP Source port identity 1 register"]
    pub macspi1r: crate::Reg<macspi1r::MACSPI1R_SPEC>,
    #[doc = "0xbcc - PTP Source port identity 2 register"]
    pub macspi2r: crate::Reg<macspi2r::MACSPI2R_SPEC>,
    #[doc = "0xbd0 - Log message interval register"]
    pub maclmir: crate::Reg<maclmir::MACLMIR_SPEC>,
}
#[doc = "MACCR register accessor: an alias for `Reg<MACCR_SPEC>`"]
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
#[doc = "Operating mode configuration register"]
pub mod maccr;
#[doc = "MACECR register accessor: an alias for `Reg<MACECR_SPEC>`"]
pub type MACECR = crate::Reg<macecr::MACECR_SPEC>;
#[doc = "Extended operating mode configuration register"]
pub mod macecr;
#[doc = "MACPFR register accessor: an alias for `Reg<MACPFR_SPEC>`"]
pub type MACPFR = crate::Reg<macpfr::MACPFR_SPEC>;
#[doc = "Packet filtering control register"]
pub mod macpfr;
#[doc = "MACWTR register accessor: an alias for `Reg<MACWTR_SPEC>`"]
pub type MACWTR = crate::Reg<macwtr::MACWTR_SPEC>;
#[doc = "Watchdog timeout register"]
pub mod macwtr;
#[doc = "MACHT0R register accessor: an alias for `Reg<MACHT0R_SPEC>`"]
pub type MACHT0R = crate::Reg<macht0r::MACHT0R_SPEC>;
#[doc = "Hash Table 0 register"]
pub mod macht0r;
#[doc = "MACHT1R register accessor: an alias for `Reg<MACHT1R_SPEC>`"]
pub type MACHT1R = crate::Reg<macht1r::MACHT1R_SPEC>;
#[doc = "Hash Table 1 register"]
pub mod macht1r;
#[doc = "MACVTR register accessor: an alias for `Reg<MACVTR_SPEC>`"]
pub type MACVTR = crate::Reg<macvtr::MACVTR_SPEC>;
#[doc = "VLAN tag register"]
pub mod macvtr;
#[doc = "MACVHTR register accessor: an alias for `Reg<MACVHTR_SPEC>`"]
pub type MACVHTR = crate::Reg<macvhtr::MACVHTR_SPEC>;
#[doc = "VLAN Hash table register"]
pub mod macvhtr;
#[doc = "MACVIR register accessor: an alias for `Reg<MACVIR_SPEC>`"]
pub type MACVIR = crate::Reg<macvir::MACVIR_SPEC>;
#[doc = "VLAN inclusion register"]
pub mod macvir;
#[doc = "MACIVIR register accessor: an alias for `Reg<MACIVIR_SPEC>`"]
pub type MACIVIR = crate::Reg<macivir::MACIVIR_SPEC>;
#[doc = "Inner VLAN inclusion register"]
pub mod macivir;
#[doc = "MACQTxFCR register accessor: an alias for `Reg<MACQTXFCR_SPEC>`"]
pub type MACQTXFCR = crate::Reg<macqtx_fcr::MACQTXFCR_SPEC>;
#[doc = "Tx Queue flow control register"]
pub mod macqtx_fcr;
#[doc = "MACRxFCR register accessor: an alias for `Reg<MACRXFCR_SPEC>`"]
pub type MACRXFCR = crate::Reg<macrx_fcr::MACRXFCR_SPEC>;
#[doc = "Rx flow control register"]
pub mod macrx_fcr;
#[doc = "MACISR register accessor: an alias for `Reg<MACISR_SPEC>`"]
pub type MACISR = crate::Reg<macisr::MACISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod macisr;
#[doc = "MACIER register accessor: an alias for `Reg<MACIER_SPEC>`"]
pub type MACIER = crate::Reg<macier::MACIER_SPEC>;
#[doc = "Interrupt enable register"]
pub mod macier;
#[doc = "MACRxTxSR register accessor: an alias for `Reg<MACRXTXSR_SPEC>`"]
pub type MACRXTXSR = crate::Reg<macrx_tx_sr::MACRXTXSR_SPEC>;
#[doc = "Rx Tx status register"]
pub mod macrx_tx_sr;
#[doc = "MACPCSR register accessor: an alias for `Reg<MACPCSR_SPEC>`"]
pub type MACPCSR = crate::Reg<macpcsr::MACPCSR_SPEC>;
#[doc = "PMT control status register"]
pub mod macpcsr;
#[doc = "MACRWKPFR register accessor: an alias for `Reg<MACRWKPFR_SPEC>`"]
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFR_SPEC>;
#[doc = "Remove wakeup packet filter register"]
pub mod macrwkpfr;
#[doc = "MACLCSR register accessor: an alias for `Reg<MACLCSR_SPEC>`"]
pub type MACLCSR = crate::Reg<maclcsr::MACLCSR_SPEC>;
#[doc = "LPI control status register"]
pub mod maclcsr;
#[doc = "MACLTCR register accessor: an alias for `Reg<MACLTCR_SPEC>`"]
pub type MACLTCR = crate::Reg<macltcr::MACLTCR_SPEC>;
#[doc = "LPI timers control register"]
pub mod macltcr;
#[doc = "MACLETR register accessor: an alias for `Reg<MACLETR_SPEC>`"]
pub type MACLETR = crate::Reg<macletr::MACLETR_SPEC>;
#[doc = "LPI entry timer register"]
pub mod macletr;
#[doc = "MAC1USTCR register accessor: an alias for `Reg<MAC1USTCR_SPEC>`"]
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCR_SPEC>;
#[doc = "1-microsecond-tick counter register"]
pub mod mac1ustcr;
#[doc = "MACVR register accessor: an alias for `Reg<MACVR_SPEC>`"]
pub type MACVR = crate::Reg<macvr::MACVR_SPEC>;
#[doc = "Version register"]
pub mod macvr;
#[doc = "MACDR register accessor: an alias for `Reg<MACDR_SPEC>`"]
pub type MACDR = crate::Reg<macdr::MACDR_SPEC>;
#[doc = "Debug register"]
pub mod macdr;
#[doc = "MACHWF1R register accessor: an alias for `Reg<MACHWF1R_SPEC>`"]
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1R_SPEC>;
#[doc = "HW feature 1 register"]
pub mod machwf1r;
#[doc = "MACHWF2R register accessor: an alias for `Reg<MACHWF2R_SPEC>`"]
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2R_SPEC>;
#[doc = "HW feature 2 register"]
pub mod machwf2r;
#[doc = "MACMDIOAR register accessor: an alias for `Reg<MACMDIOAR_SPEC>`"]
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOAR_SPEC>;
#[doc = "MDIO address register"]
pub mod macmdioar;
#[doc = "MACMDIODR register accessor: an alias for `Reg<MACMDIODR_SPEC>`"]
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODR_SPEC>;
#[doc = "MDIO data register"]
pub mod macmdiodr;
#[doc = "MACARPAR register accessor: an alias for `Reg<MACARPAR_SPEC>`"]
pub type MACARPAR = crate::Reg<macarpar::MACARPAR_SPEC>;
#[doc = "ARP address register"]
pub mod macarpar;
#[doc = "MACA0HR register accessor: an alias for `Reg<MACA0HR_SPEC>`"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR register accessor: an alias for `Reg<MACA0LR_SPEC>`"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1LR register accessor: an alias for `Reg<MACA1LR_SPEC>`"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Address 1 low register"]
pub mod maca1lr;
#[doc = "MACA2LR register accessor: an alias for `Reg<MACA2LR_SPEC>`"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Address 2 low register"]
pub mod maca2lr;
#[doc = "MACA1HR register accessor: an alias for `Reg<MACA1HR_SPEC>`"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Address 1 high register"]
pub mod maca1hr;
#[doc = "MACA2HR register accessor: an alias for `Reg<MACA2HR_SPEC>`"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Address 2 high register"]
pub mod maca2hr;
#[doc = "MACA3HR register accessor: an alias for `Reg<MACA3HR_SPEC>`"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Address 3 high register"]
pub mod maca3hr;
#[doc = "MACA3LR register accessor: an alias for `Reg<MACA3LR_SPEC>`"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Address 3 low register"]
pub mod maca3lr;
#[doc = "MMC_CONTROL register accessor: an alias for `Reg<MMC_CONTROL_SPEC>`"]
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
#[doc = "MMC control register"]
pub mod mmc_control;
#[doc = "MMC_RX_INTERRUPT register accessor: an alias for `Reg<MMC_RX_INTERRUPT_SPEC>`"]
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPT_SPEC>;
#[doc = "MMC Rx interrupt register"]
pub mod mmc_rx_interrupt;
#[doc = "MMC_TX_INTERRUPT register accessor: an alias for `Reg<MMC_TX_INTERRUPT_SPEC>`"]
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPT_SPEC>;
#[doc = "MMC Tx interrupt register"]
pub mod mmc_tx_interrupt;
#[doc = "MMC_RX_INTERRUPT_MASK register accessor: an alias for `Reg<MMC_RX_INTERRUPT_MASK_SPEC>`"]
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Rx interrupt mask register"]
pub mod mmc_rx_interrupt_mask;
#[doc = "MMC_TX_INTERRUPT_MASK register accessor: an alias for `Reg<MMC_TX_INTERRUPT_MASK_SPEC>`"]
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Tx interrupt mask register"]
pub mod mmc_tx_interrupt_mask;
#[doc = "TX_SINGLE_COLLISION_GOOD_PACKETS register accessor: an alias for `Reg<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>`"]
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx single collision good packets register"]
pub mod tx_single_collision_good_packets;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_PACKETS register accessor: an alias for `Reg<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>`"]
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
#[doc = "Tx multiple collision good packets register"]
pub mod tx_multiple_collision_good_packets;
#[doc = "TX_PACKET_COUNT_GOOD register accessor: an alias for `Reg<TX_PACKET_COUNT_GOOD_SPEC>`"]
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOOD_SPEC>;
#[doc = "Tx packet count good register"]
pub mod tx_packet_count_good;
#[doc = "RX_CRC_ERROR_PACKETS register accessor: an alias for `Reg<RX_CRC_ERROR_PACKETS_SPEC>`"]
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETS_SPEC>;
#[doc = "Rx CRC error packets register"]
pub mod rx_crc_error_packets;
#[doc = "RX_ALIGNMENT_ERROR_PACKETS register accessor: an alias for `Reg<RX_ALIGNMENT_ERROR_PACKETS_SPEC>`"]
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
#[doc = "Rx alignment error packets register"]
pub mod rx_alignment_error_packets;
#[doc = "RX_UNICAST_PACKETS_GOOD register accessor: an alias for `Reg<RX_UNICAST_PACKETS_GOOD_SPEC>`"]
pub type RX_UNICAST_PACKETS_GOOD =
    crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOOD_SPEC>;
#[doc = "Rx unicast packets good register"]
pub mod rx_unicast_packets_good;
#[doc = "TX_LPI_USEC_CNTR register accessor: an alias for `Reg<TX_LPI_USEC_CNTR_SPEC>`"]
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTR_SPEC>;
#[doc = "Tx LPI microsecond timer register"]
pub mod tx_lpi_usec_cntr;
#[doc = "TX_LPI_TRAN_CNTR register accessor: an alias for `Reg<TX_LPI_TRAN_CNTR_SPEC>`"]
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Tx LPI transition counter register"]
pub mod tx_lpi_tran_cntr;
#[doc = "RX_LPI_USEC_CNTR register accessor: an alias for `Reg<RX_LPI_USEC_CNTR_SPEC>`"]
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTR_SPEC>;
#[doc = "Rx LPI microsecond counter register"]
pub mod rx_lpi_usec_cntr;
#[doc = "RX_LPI_TRAN_CNTR register accessor: an alias for `Reg<RX_LPI_TRAN_CNTR_SPEC>`"]
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Rx LPI transition counter register"]
pub mod rx_lpi_tran_cntr;
#[doc = "MACL3L4C0R register accessor: an alias for `Reg<MACL3L4C0R_SPEC>`"]
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0R_SPEC>;
#[doc = "L3 and L4 control 0 register"]
pub mod macl3l4c0r;
#[doc = "MACL4A0R register accessor: an alias for `Reg<MACL4A0R_SPEC>`"]
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0R_SPEC>;
#[doc = "Layer4 address filter 0 register"]
pub mod macl4a0r;
#[doc = "MACL3A00R register accessor: an alias for `Reg<MACL3A00R_SPEC>`"]
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00R_SPEC>;
#[doc = "MACL3A00R"]
pub mod macl3a00r;
#[doc = "MACL3A10R register accessor: an alias for `Reg<MACL3A10R_SPEC>`"]
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10R_SPEC>;
#[doc = "Layer3 address 1 filter 0 register"]
pub mod macl3a10r;
#[doc = "MACL3A20 register accessor: an alias for `Reg<MACL3A20_SPEC>`"]
pub type MACL3A20 = crate::Reg<macl3a20::MACL3A20_SPEC>;
#[doc = "Layer3 Address 2 filter 0 register"]
pub mod macl3a20;
#[doc = "MACL3A30 register accessor: an alias for `Reg<MACL3A30_SPEC>`"]
pub type MACL3A30 = crate::Reg<macl3a30::MACL3A30_SPEC>;
#[doc = "Layer3 Address 3 filter 0 register"]
pub mod macl3a30;
#[doc = "MACL3L4C1R register accessor: an alias for `Reg<MACL3L4C1R_SPEC>`"]
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1R_SPEC>;
#[doc = "L3 and L4 control 1 register"]
pub mod macl3l4c1r;
#[doc = "MACL4A1R register accessor: an alias for `Reg<MACL4A1R_SPEC>`"]
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1R_SPEC>;
#[doc = "Layer 4 address filter 1 register"]
pub mod macl4a1r;
#[doc = "MACL3A01R register accessor: an alias for `Reg<MACL3A01R_SPEC>`"]
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01R_SPEC>;
#[doc = "Layer3 address 0 filter 1 Register"]
pub mod macl3a01r;
#[doc = "MACL3A11R register accessor: an alias for `Reg<MACL3A11R_SPEC>`"]
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11R_SPEC>;
#[doc = "Layer3 address 1 filter 1 register"]
pub mod macl3a11r;
#[doc = "MACL3A21R register accessor: an alias for `Reg<MACL3A21R_SPEC>`"]
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21R_SPEC>;
#[doc = "Layer3 address 2 filter 1 Register"]
pub mod macl3a21r;
#[doc = "MACL3A31R register accessor: an alias for `Reg<MACL3A31R_SPEC>`"]
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31R_SPEC>;
#[doc = "Layer3 address 3 filter 1 register"]
pub mod macl3a31r;
#[doc = "MACTSCR register accessor: an alias for `Reg<MACTSCR_SPEC>`"]
pub type MACTSCR = crate::Reg<mactscr::MACTSCR_SPEC>;
#[doc = "Timestamp control Register"]
pub mod mactscr;
#[doc = "MACSSIR register accessor: an alias for `Reg<MACSSIR_SPEC>`"]
pub type MACSSIR = crate::Reg<macssir::MACSSIR_SPEC>;
#[doc = "Sub-second increment register"]
pub mod macssir;
#[doc = "MACSTSR register accessor: an alias for `Reg<MACSTSR_SPEC>`"]
pub type MACSTSR = crate::Reg<macstsr::MACSTSR_SPEC>;
#[doc = "System time seconds register"]
pub mod macstsr;
#[doc = "MACSTNR register accessor: an alias for `Reg<MACSTNR_SPEC>`"]
pub type MACSTNR = crate::Reg<macstnr::MACSTNR_SPEC>;
#[doc = "System time nanoseconds register"]
pub mod macstnr;
#[doc = "MACSTSUR register accessor: an alias for `Reg<MACSTSUR_SPEC>`"]
pub type MACSTSUR = crate::Reg<macstsur::MACSTSUR_SPEC>;
#[doc = "System time seconds update register"]
pub mod macstsur;
#[doc = "MACSTNUR register accessor: an alias for `Reg<MACSTNUR_SPEC>`"]
pub type MACSTNUR = crate::Reg<macstnur::MACSTNUR_SPEC>;
#[doc = "System time nanoseconds update register"]
pub mod macstnur;
#[doc = "MACTSAR register accessor: an alias for `Reg<MACTSAR_SPEC>`"]
pub type MACTSAR = crate::Reg<mactsar::MACTSAR_SPEC>;
#[doc = "Timestamp addend register"]
pub mod mactsar;
#[doc = "MACTSSR register accessor: an alias for `Reg<MACTSSR_SPEC>`"]
pub type MACTSSR = crate::Reg<mactssr::MACTSSR_SPEC>;
#[doc = "Timestamp status register"]
pub mod mactssr;
#[doc = "MACTxTSSNR register accessor: an alias for `Reg<MACTXTSSNR_SPEC>`"]
pub type MACTXTSSNR = crate::Reg<mactx_tssnr::MACTXTSSNR_SPEC>;
#[doc = "Tx timestamp status nanoseconds register"]
pub mod mactx_tssnr;
#[doc = "MACTxTSSSR register accessor: an alias for `Reg<MACTXTSSSR_SPEC>`"]
pub type MACTXTSSSR = crate::Reg<mactx_tsssr::MACTXTSSSR_SPEC>;
#[doc = "Tx timestamp status seconds register"]
pub mod mactx_tsssr;
#[doc = "MACACR register accessor: an alias for `Reg<MACACR_SPEC>`"]
pub type MACACR = crate::Reg<macacr::MACACR_SPEC>;
#[doc = "Auxiliary control register"]
pub mod macacr;
#[doc = "MACATSNR register accessor: an alias for `Reg<MACATSNR_SPEC>`"]
pub type MACATSNR = crate::Reg<macatsnr::MACATSNR_SPEC>;
#[doc = "Auxiliary timestamp nanoseconds register"]
pub mod macatsnr;
#[doc = "MACATSSR register accessor: an alias for `Reg<MACATSSR_SPEC>`"]
pub type MACATSSR = crate::Reg<macatssr::MACATSSR_SPEC>;
#[doc = "Auxiliary timestamp seconds register"]
pub mod macatssr;
#[doc = "MACTSIACR register accessor: an alias for `Reg<MACTSIACR_SPEC>`"]
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACR_SPEC>;
#[doc = "Timestamp Ingress asymmetric correction register"]
pub mod mactsiacr;
#[doc = "MACTSEACR register accessor: an alias for `Reg<MACTSEACR_SPEC>`"]
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACR_SPEC>;
#[doc = "Timestamp Egress asymmetric correction register"]
pub mod mactseacr;
#[doc = "MACTSICNR register accessor: an alias for `Reg<MACTSICNR_SPEC>`"]
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNR_SPEC>;
#[doc = "Timestamp Ingress correction nanosecond register"]
pub mod mactsicnr;
#[doc = "MACTSECNR register accessor: an alias for `Reg<MACTSECNR_SPEC>`"]
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNR_SPEC>;
#[doc = "Timestamp Egress correction nanosecond register"]
pub mod mactsecnr;
#[doc = "MACPPSCR register accessor: an alias for `Reg<MACPPSCR_SPEC>`"]
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCR_SPEC>;
#[doc = "PPS control register"]
pub mod macppscr;
#[doc = "MACPPSTTSR register accessor: an alias for `Reg<MACPPSTTSR_SPEC>`"]
pub type MACPPSTTSR = crate::Reg<macppsttsr::MACPPSTTSR_SPEC>;
#[doc = "PPS target time seconds register"]
pub mod macppsttsr;
#[doc = "MACPPSTTNR register accessor: an alias for `Reg<MACPPSTTNR_SPEC>`"]
pub type MACPPSTTNR = crate::Reg<macppsttnr::MACPPSTTNR_SPEC>;
#[doc = "PPS target time nanoseconds register"]
pub mod macppsttnr;
#[doc = "MACPPSIR register accessor: an alias for `Reg<MACPPSIR_SPEC>`"]
pub type MACPPSIR = crate::Reg<macppsir::MACPPSIR_SPEC>;
#[doc = "PPS interval register"]
pub mod macppsir;
#[doc = "MACPPSWR register accessor: an alias for `Reg<MACPPSWR_SPEC>`"]
pub type MACPPSWR = crate::Reg<macppswr::MACPPSWR_SPEC>;
#[doc = "PPS width register"]
pub mod macppswr;
#[doc = "MACPOCR register accessor: an alias for `Reg<MACPOCR_SPEC>`"]
pub type MACPOCR = crate::Reg<macpocr::MACPOCR_SPEC>;
#[doc = "PTP Offload control register"]
pub mod macpocr;
#[doc = "MACSPI0R register accessor: an alias for `Reg<MACSPI0R_SPEC>`"]
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0R_SPEC>;
#[doc = "PTP Source Port Identity 0 Register"]
pub mod macspi0r;
#[doc = "MACSPI1R register accessor: an alias for `Reg<MACSPI1R_SPEC>`"]
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1R_SPEC>;
#[doc = "PTP Source port identity 1 register"]
pub mod macspi1r;
#[doc = "MACSPI2R register accessor: an alias for `Reg<MACSPI2R_SPEC>`"]
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2R_SPEC>;
#[doc = "PTP Source port identity 2 register"]
pub mod macspi2r;
#[doc = "MACLMIR register accessor: an alias for `Reg<MACLMIR_SPEC>`"]
pub type MACLMIR = crate::Reg<maclmir::MACLMIR_SPEC>;
#[doc = "Log message interval register"]
pub mod maclmir;
