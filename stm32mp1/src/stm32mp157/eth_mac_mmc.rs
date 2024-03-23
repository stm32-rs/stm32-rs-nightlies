#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eth_maccr: ETH_MACCR,
    eth_macecr: ETH_MACECR,
    eth_macpfr: ETH_MACPFR,
    eth_macwtr: ETH_MACWTR,
    eth_macht0r: ETH_MACHT0R,
    eth_macht1r: ETH_MACHT1R,
    _reserved6: [u8; 0x38],
    eth_macvtr: ETH_MACVTR,
    _reserved7: [u8; 0x04],
    eth_macvhtr: ETH_MACVHTR,
    _reserved8: [u8; 0x04],
    eth_macvir: ETH_MACVIR,
    eth_macivir: ETH_MACIVIR,
    _reserved10: [u8; 0x08],
    eth_macq0tx_fcr: ETH_MACQ0TX_FCR,
    _reserved11: [u8; 0x1c],
    eth_macrx_fcr: ETH_MACRX_FCR,
    _reserved12: [u8; 0x04],
    eth_mactx_qpmr: ETH_MACTX_QPMR,
    _reserved13: [u8; 0x04],
    eth_macrx_qc0r: ETH_MACRX_QC0R,
    eth_macrx_qc1r: ETH_MACRX_QC1R,
    eth_macrx_qc2r: ETH_MACRX_QC2R,
    _reserved16: [u8; 0x04],
    eth_macisr: ETH_MACISR,
    eth_macier: ETH_MACIER,
    eth_macrx_tx_sr: ETH_MACRX_TX_SR,
    _reserved19: [u8; 0x04],
    eth_macpcsr: ETH_MACPCSR,
    eth_macrwkpfr: ETH_MACRWKPFR,
    _reserved21: [u8; 0x08],
    eth_maclcsr: ETH_MACLCSR,
    eth_macltcr: ETH_MACLTCR,
    eth_macletr: ETH_MACLETR,
    eth_mac1ustcr: ETH_MAC1USTCR,
    _reserved25: [u8; 0x18],
    eth_macphycsr: ETH_MACPHYCSR,
    _reserved26: [u8; 0x14],
    eth_macvr: ETH_MACVR,
    eth_macdr: ETH_MACDR,
    _reserved28: [u8; 0x08],
    eth_machwf1r: ETH_MACHWF1R,
    eth_machwf2r: ETH_MACHWF2R,
    _reserved30: [u8; 0xd8],
    eth_macmdioar: ETH_MACMDIOAR,
    eth_macmdiodr: ETH_MACMDIODR,
    _reserved32: [u8; 0xf8],
    eth_maca0hr: ETH_MACA0HR,
    eth_maca0lr: ETH_MACA0LR,
    eth_maca1hr: ETH_MACA1HR,
    eth_maca1lr: ETH_MACA1LR,
    eth_maca2hr: ETH_MACA2HR,
    eth_maca2lr: ETH_MACA2LR,
    eth_maca3hr: ETH_MACA3HR,
    eth_maca3lr: ETH_MACA3LR,
    _reserved40: [u8; 0x03e0],
    mmc_control: MMC_CONTROL,
    mmc_rx_interrupt: MMC_RX_INTERRUPT,
    mmc_tx_interrupt: MMC_TX_INTERRUPT,
    mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved45: [u8; 0x38],
    tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved47: [u8; 0x14],
    tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved48: [u8; 0x28],
    rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved50: [u8; 0x28],
    rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved51: [u8; 0x24],
    tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved55: [u8; 0x0104],
    eth_macl3l4c0r: ETH_MACL3L4C0R,
    eth_macl4a0r: ETH_MACL4A0R,
    _reserved57: [u8; 0x08],
    eth_macl3a00r: ETH_MACL3A00R,
    eth_macl3a10r: ETH_MACL3A10R,
    eth_macl3a20: ETH_MACL3A20,
    eth_macl3a30: ETH_MACL3A30,
    _reserved61: [u8; 0x10],
    eth_macl3l4c1r: ETH_MACL3L4C1R,
    eth_macl4a1r: ETH_MACL4A1R,
    _reserved63: [u8; 0x08],
    eth_macl3a01r: ETH_MACL3A01R,
    eth_macl3a11r: ETH_MACL3A11R,
    eth_macl3a21r: ETH_MACL3A21R,
    eth_macl3a31r: ETH_MACL3A31R,
    _reserved67: [u8; 0x0190],
    eth_macarpar: ETH_MACARPAR,
    _reserved68: [u8; 0x1c],
    eth_mactscr: ETH_MACTSCR,
    eth_macssir: ETH_MACSSIR,
    eth_macstsr: ETH_MACSTSR,
    eth_macstnr: ETH_MACSTNR,
    eth_macstsur: ETH_MACSTSUR,
    eth_macstnur: ETH_MACSTNUR,
    eth_mactsar: ETH_MACTSAR,
    _reserved75: [u8; 0x04],
    eth_mactssr: ETH_MACTSSR,
    _reserved76: [u8; 0x0c],
    eth_mactx_tssnr: ETH_MACTX_TSSNR,
    eth_mactx_tsssr: ETH_MACTX_TSSSR,
    _reserved78: [u8; 0x08],
    eth_macacr: ETH_MACACR,
    _reserved79: [u8; 0x04],
    eth_macatsnr: ETH_MACATSNR,
    eth_macatssr: ETH_MACATSSR,
    eth_mactsiacr: ETH_MACTSIACR,
    eth_mactseacr: ETH_MACTSEACR,
    eth_mactsicnr: ETH_MACTSICNR,
    eth_mactsecnr: ETH_MACTSECNR,
    _reserved85: [u8; 0x10],
    eth_macppscr: ETH_MACPPSCR,
    _reserved86: [u8; 0x0c],
    eth_macppsttsr: ETH_MACPPSTTSR,
    eth_macppsttnr: ETH_MACPPSTTNR,
    eth_macppsir: ETH_MACPPSIR,
    eth_macppswr: ETH_MACPPSWR,
    _reserved90: [u8; 0x30],
    eth_macpocr: ETH_MACPOCR,
    eth_macspi0r: ETH_MACSPI0R,
    eth_macspi1r: ETH_MACSPI1R,
    eth_macspi2r: ETH_MACSPI2R,
    eth_maclmir: ETH_MACLMIR,
}
impl RegisterBlock {
    #[doc = "0x00 - The MAC Configuration Register establishes the operating mode of the MAC."]
    #[inline(always)]
    pub const fn eth_maccr(&self) -> &ETH_MACCR {
        &self.eth_maccr
    }
    #[doc = "0x04 - The MAC Extended Configuration Register establishes the operating mode of the MAC."]
    #[inline(always)]
    pub const fn eth_macecr(&self) -> &ETH_MACECR {
        &self.eth_macecr
    }
    #[doc = "0x08 - The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets."]
    #[inline(always)]
    pub const fn eth_macpfr(&self) -> &ETH_MACPFR {
        &self.eth_macpfr
    }
    #[doc = "0x0c - The Watchdog Timeout register controls the watchdog timeout for received packets."]
    #[inline(always)]
    pub const fn eth_macwtr(&self) -> &ETH_MACWTR {
        &self.eth_macwtr
    }
    #[doc = "0x10 - The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
    #[inline(always)]
    pub const fn eth_macht0r(&self) -> &ETH_MACHT0R {
        &self.eth_macht0r
    }
    #[doc = "0x14 - The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
    #[inline(always)]
    pub const fn eth_macht1r(&self) -> &ETH_MACHT1R {
        &self.eth_macht1r
    }
    #[doc = "0x50 - The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets."]
    #[inline(always)]
    pub const fn eth_macvtr(&self) -> &ETH_MACVTR {
        &self.eth_macvtr
    }
    #[doc = "0x58 - When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written."]
    #[inline(always)]
    pub const fn eth_macvhtr(&self) -> &ETH_MACVHTR {
        &self.eth_macvhtr
    }
    #[doc = "0x60 - The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls."]
    #[inline(always)]
    pub const fn eth_macvir(&self) -> &ETH_MACVIR {
        &self.eth_macvir
    }
    #[doc = "0x64 - The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls."]
    #[inline(always)]
    pub const fn eth_macivir(&self) -> &ETH_MACIVIR {
        &self.eth_macivir
    }
    #[doc = "0x70 - The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register."]
    #[inline(always)]
    pub const fn eth_macq0tx_fcr(&self) -> &ETH_MACQ0TX_FCR {
        &self.eth_macq0tx_fcr
    }
    #[doc = "0x90 - The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet."]
    #[inline(always)]
    pub const fn eth_macrx_fcr(&self) -> &ETH_MACRX_FCR {
        &self.eth_macrx_fcr
    }
    #[doc = "0x98 - The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1."]
    #[inline(always)]
    pub const fn eth_mactx_qpmr(&self) -> &ETH_MACTX_QPMR {
        &self.eth_mactx_qpmr
    }
    #[doc = "0xa0 - The Receive Queue Control 0 register controls the queue management in the MAC Receiver."]
    #[inline(always)]
    pub const fn eth_macrx_qc0r(&self) -> &ETH_MACRX_QC0R {
        &self.eth_macrx_qc0r
    }
    #[doc = "0xa4 - The Receive Queue Control 1 register controls queue 1 management in the MAC receiver."]
    #[inline(always)]
    pub const fn eth_macrx_qc1r(&self) -> &ETH_MACRX_QC1R {
        &self.eth_macrx_qc1r
    }
    #[doc = "0xa8 - This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1."]
    #[inline(always)]
    pub const fn eth_macrx_qc2r(&self) -> &ETH_MACRX_QC2R {
        &self.eth_macrx_qc2r
    }
    #[doc = "0xb0 - The Interrupt Status register contains the status of interrupts."]
    #[inline(always)]
    pub const fn eth_macisr(&self) -> &ETH_MACISR {
        &self.eth_macisr
    }
    #[doc = "0xb4 - The Interrupt Enable register contains the masks for generating the interrupts."]
    #[inline(always)]
    pub const fn eth_macier(&self) -> &ETH_MACIER {
        &self.eth_macier
    }
    #[doc = "0xb8 - The Receive Transmit Status register contains the Receive and Transmit Error status."]
    #[inline(always)]
    pub const fn eth_macrx_tx_sr(&self) -> &ETH_MACRX_TX_SR {
        &self.eth_macrx_tx_sr
    }
    #[doc = "0xc0 - The PMT Control and Status Register is present only when you select the PMT module in coreConsultant."]
    #[inline(always)]
    pub const fn eth_macpcsr(&self) -> &ETH_MACPCSR {
        &self.eth_macpcsr
    }
    #[doc = "0xc4 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
    #[inline(always)]
    pub const fn eth_macrwkpfr(&self) -> &ETH_MACRWKPFR {
        &self.eth_macrwkpfr
    }
    #[doc = "0xd0 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
    #[inline(always)]
    pub const fn eth_maclcsr(&self) -> &ETH_MACLCSR {
        &self.eth_maclcsr
    }
    #[doc = "0xd4 - The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission."]
    #[inline(always)]
    pub const fn eth_macltcr(&self) -> &ETH_MACLTCR {
        &self.eth_macltcr
    }
    #[doc = "0xd8 - The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds."]
    #[inline(always)]
    pub const fn eth_macletr(&self) -> &ETH_MACLETR {
        &self.eth_macletr
    }
    #[doc = "0xdc - This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially."]
    #[inline(always)]
    pub const fn eth_mac1ustcr(&self) -> &ETH_MAC1USTCR {
        &self.eth_mac1ustcr
    }
    #[doc = "0xf8 - The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY."]
    #[inline(always)]
    pub const fn eth_macphycsr(&self) -> &ETH_MACPHYCSR {
        &self.eth_macphycsr
    }
    #[doc = "0x110 - The version register identifies the version of the Ethernet peripheral."]
    #[inline(always)]
    pub const fn eth_macvr(&self) -> &ETH_MACVR {
        &self.eth_macvr
    }
    #[doc = "0x114 - The Debug register provides the debug status of various MAC blocks."]
    #[inline(always)]
    pub const fn eth_macdr(&self) -> &ETH_MACDR {
        &self.eth_macdr
    }
    #[doc = "0x120 - This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
    #[inline(always)]
    pub const fn eth_machwf1r(&self) -> &ETH_MACHWF1R {
        &self.eth_machwf1r
    }
    #[doc = "0x124 - This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
    #[inline(always)]
    pub const fn eth_machwf2r(&self) -> &ETH_MACHWF2R {
        &self.eth_machwf2r
    }
    #[doc = "0x200 - The MDIO Address register controls the management cycles to external PHY through a management interface."]
    #[inline(always)]
    pub const fn eth_macmdioar(&self) -> &ETH_MACMDIOAR {
        &self.eth_macmdioar
    }
    #[doc = "0x204 - The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register."]
    #[inline(always)]
    pub const fn eth_macmdiodr(&self) -> &ETH_MACMDIODR {
        &self.eth_macmdiodr
    }
    #[doc = "0x300 - The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain."]
    #[inline(always)]
    pub const fn eth_maca0hr(&self) -> &ETH_MACA0HR {
        &self.eth_maca0hr
    }
    #[doc = "0x304 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca0lr(&self) -> &ETH_MACA0LR {
        &self.eth_maca0lr
    }
    #[doc = "0x308 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca1hr(&self) -> &ETH_MACA1HR {
        &self.eth_maca1hr
    }
    #[doc = "0x30c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca1lr(&self) -> &ETH_MACA1LR {
        &self.eth_maca1lr
    }
    #[doc = "0x310 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca2hr(&self) -> &ETH_MACA2HR {
        &self.eth_maca2hr
    }
    #[doc = "0x314 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca2lr(&self) -> &ETH_MACA2LR {
        &self.eth_maca2lr
    }
    #[doc = "0x318 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca3hr(&self) -> &ETH_MACA3HR {
        &self.eth_maca3hr
    }
    #[doc = "0x31c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    #[inline(always)]
    pub const fn eth_maca3lr(&self) -> &ETH_MACA3LR {
        &self.eth_maca3lr
    }
    #[doc = "0x700 - This register configures the MMC operating mode."]
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MMC_CONTROL {
        &self.mmc_control
    }
    #[doc = "0x704 - This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
    #[inline(always)]
    pub const fn mmc_rx_interrupt(&self) -> &MMC_RX_INTERRUPT {
        &self.mmc_rx_interrupt
    }
    #[doc = "0x708 - This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
    #[inline(always)]
    pub const fn mmc_tx_interrupt(&self) -> &MMC_TX_INTERRUPT {
        &self.mmc_tx_interrupt
    }
    #[doc = "0x70c - The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values."]
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(&self) -> &MMC_RX_INTERRUPT_MASK {
        &self.mmc_rx_interrupt_mask
    }
    #[doc = "0x710 - This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration."]
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(&self) -> &MMC_TX_INTERRUPT_MASK {
        &self.mmc_tx_interrupt_mask
    }
    #[doc = "0x74c - This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(&self) -> &TX_SINGLE_COLLISION_GOOD_PACKETS {
        &self.tx_single_collision_good_packets
    }
    #[doc = "0x750 - This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode."]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(&self) -> &TX_MULTIPLE_COLLISION_GOOD_PACKETS {
        &self.tx_multiple_collision_good_packets
    }
    #[doc = "0x768 - This register provides the number of good packets transmitted by Ethernet peripheral."]
    #[inline(always)]
    pub const fn tx_packet_count_good(&self) -> &TX_PACKET_COUNT_GOOD {
        &self.tx_packet_count_good
    }
    #[doc = "0x794 - This register provides the number of packets received by Ethernet peripheral with CRC error."]
    #[inline(always)]
    pub const fn rx_crc_error_packets(&self) -> &RX_CRC_ERROR_PACKETS {
        &self.rx_crc_error_packets
    }
    #[doc = "0x798 - This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode."]
    #[inline(always)]
    pub const fn rx_alignment_error_packets(&self) -> &RX_ALIGNMENT_ERROR_PACKETS {
        &self.rx_alignment_error_packets
    }
    #[doc = "0x7c4 - This register provides the number of good unicast packets received by Ethernet peripheral."]
    #[inline(always)]
    pub const fn rx_unicast_packets_good(&self) -> &RX_UNICAST_PACKETS_GOOD {
        &self.rx_unicast_packets_good
    }
    #[doc = "0x7ec - This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral."]
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(&self) -> &TX_LPI_USEC_CNTR {
        &self.tx_lpi_usec_cntr
    }
    #[doc = "0x7f0 - This register provides the number of times Ethernet peripheral has entered Tx LPI."]
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(&self) -> &TX_LPI_TRAN_CNTR {
        &self.tx_lpi_tran_cntr
    }
    #[doc = "0x7f4 - This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral."]
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(&self) -> &RX_LPI_USEC_CNTR {
        &self.rx_lpi_usec_cntr
    }
    #[doc = "0x7f8 - This register provides the number of times Ethernet peripheral has entered Rx LPI."]
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(&self) -> &RX_LPI_TRAN_CNTR {
        &self.rx_lpi_tran_cntr
    }
    #[doc = "0x900 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration."]
    #[inline(always)]
    pub const fn eth_macl3l4c0r(&self) -> &ETH_MACL3L4C0R {
        &self.eth_macl3l4c0r
    }
    #[doc = "0x904 - Layer4 address filter 0 register"]
    #[inline(always)]
    pub const fn eth_macl4a0r(&self) -> &ETH_MACL4A0R {
        &self.eth_macl4a0r
    }
    #[doc = "0x910 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a00r(&self) -> &ETH_MACL3A00R {
        &self.eth_macl3a00r
    }
    #[doc = "0x914 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a10r(&self) -> &ETH_MACL3A10R {
        &self.eth_macl3a10r
    }
    #[doc = "0x918 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a20(&self) -> &ETH_MACL3A20 {
        &self.eth_macl3a20
    }
    #[doc = "0x91c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a30(&self) -> &ETH_MACL3A30 {
        &self.eth_macl3a30
    }
    #[doc = "0x930 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4."]
    #[inline(always)]
    pub const fn eth_macl3l4c1r(&self) -> &ETH_MACL3L4C1R {
        &self.eth_macl3l4c1r
    }
    #[doc = "0x934 - The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock."]
    #[inline(always)]
    pub const fn eth_macl4a1r(&self) -> &ETH_MACL4A1R {
        &self.eth_macl4a1r
    }
    #[doc = "0x940 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a01r(&self) -> &ETH_MACL3A01R {
        &self.eth_macl3a01r
    }
    #[doc = "0x944 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a11r(&self) -> &ETH_MACL3A11R {
        &self.eth_macl3a11r
    }
    #[doc = "0x948 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a21r(&self) -> &ETH_MACL3A21R {
        &self.eth_macl3a21r
    }
    #[doc = "0x94c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
    #[inline(always)]
    pub const fn eth_macl3a31r(&self) -> &ETH_MACL3A31R {
        &self.eth_macl3a31r
    }
    #[doc = "0xae0 - The ARP Address register contains the IPv4 Destination Address of the MAC."]
    #[inline(always)]
    pub const fn eth_macarpar(&self) -> &ETH_MACARPAR {
        &self.eth_macarpar
    }
    #[doc = "0xb00 - This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver."]
    #[inline(always)]
    pub const fn eth_mactscr(&self) -> &ETH_MACTSCR {
        &self.eth_mactscr
    }
    #[doc = "0xb04 - The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow."]
    #[inline(always)]
    pub const fn eth_macssir(&self) -> &ETH_MACSSIR {
        &self.eth_macssir
    }
    #[doc = "0xb08 - The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    #[inline(always)]
    pub const fn eth_macstsr(&self) -> &ETH_MACSTSR {
        &self.eth_macstsr
    }
    #[doc = "0xb0c - The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    #[inline(always)]
    pub const fn eth_macstnr(&self) -> &ETH_MACSTNR {
        &self.eth_macstnr
    }
    #[doc = "0xb10 - The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    #[inline(always)]
    pub const fn eth_macstsur(&self) -> &ETH_MACSTSUR {
        &self.eth_macstsur
    }
    #[doc = "0xb14 - This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input."]
    #[inline(always)]
    pub const fn eth_macstnur(&self) -> &ETH_MACSTNUR {
        &self.eth_macstnur
    }
    #[doc = "0xb18 - The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows."]
    #[inline(always)]
    pub const fn eth_mactsar(&self) -> &ETH_MACTSAR {
        &self.eth_mactsar
    }
    #[doc = "0xb20 - The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register."]
    #[inline(always)]
    pub const fn eth_mactssr(&self) -> &ETH_MACTSSR {
        &self.eth_mactssr
    }
    #[doc = "0xb30 - This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled."]
    #[inline(always)]
    pub const fn eth_mactx_tssnr(&self) -> &ETH_MACTX_TSSNR {
        &self.eth_mactx_tssnr
    }
    #[doc = "0xb34 - The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted."]
    #[inline(always)]
    pub const fn eth_mactx_tsssr(&self) -> &ETH_MACTX_TSSSR {
        &self.eth_mactx_tsssr
    }
    #[doc = "0xb40 - The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot."]
    #[inline(always)]
    pub const fn eth_macacr(&self) -> &ETH_MACACR {
        &self.eth_macacr
    }
    #[doc = "0xb48 - The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read."]
    #[inline(always)]
    pub const fn eth_macatsnr(&self) -> &ETH_MACATSNR {
        &self.eth_macatsnr
    }
    #[doc = "0xb4c - The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register."]
    #[inline(always)]
    pub const fn eth_macatssr(&self) -> &ETH_MACATSSR {
        &self.eth_macatssr
    }
    #[doc = "0xb50 - The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages."]
    #[inline(always)]
    pub const fn eth_mactsiacr(&self) -> &ETH_MACTSIACR {
        &self.eth_mactsiacr
    }
    #[doc = "0xb54 - The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages."]
    #[inline(always)]
    pub const fn eth_mactseacr(&self) -> &ETH_MACTSEACR {
        &self.eth_mactseacr
    }
    #[doc = "0xb58 - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path."]
    #[inline(always)]
    pub const fn eth_mactsicnr(&self) -> &ETH_MACTSICNR {
        &self.eth_mactsicnr
    }
    #[doc = "0xb5c - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path."]
    #[inline(always)]
    pub const fn eth_mactsecnr(&self) -> &ETH_MACTSECNR {
        &self.eth_mactsecnr
    }
    #[doc = "0xb70 - The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected."]
    #[inline(always)]
    pub const fn eth_macppscr(&self) -> &ETH_MACPPSCR {
        &self.eth_macppscr
    }
    #[doc = "0xb80 - The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers."]
    #[inline(always)]
    pub const fn eth_macppsttsr(&self) -> &ETH_MACPPSTTSR {
        &self.eth_macppsttsr
    }
    #[doc = "0xb84 - The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected."]
    #[inline(always)]
    pub const fn eth_macppsttnr(&self) -> &ETH_MACPPSTTNR {
        &self.eth_macppsttnr
    }
    #[doc = "0xb88 - The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\])."]
    #[inline(always)]
    pub const fn eth_macppsir(&self) -> &ETH_MACPPSIR {
        &self.eth_macppsir
    }
    #[doc = "0xb8c - The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o)."]
    #[inline(always)]
    pub const fn eth_macppswr(&self) -> &ETH_MACPPSWR {
        &self.eth_macppswr
    }
    #[doc = "0xbc0 - This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    #[inline(always)]
    pub const fn eth_macpocr(&self) -> &ETH_MACPOCR {
        &self.eth_macpocr
    }
    #[doc = "0xbc4 - This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    #[inline(always)]
    pub const fn eth_macspi0r(&self) -> &ETH_MACSPI0R {
        &self.eth_macspi0r
    }
    #[doc = "0xbc8 - This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    #[inline(always)]
    pub const fn eth_macspi1r(&self) -> &ETH_MACSPI1R {
        &self.eth_macspi1r
    }
    #[doc = "0xbcc - This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node."]
    #[inline(always)]
    pub const fn eth_macspi2r(&self) -> &ETH_MACSPI2R {
        &self.eth_macspi2r
    }
    #[doc = "0xbd0 - This register contains the periodic intervals for automatic PTP packet generation."]
    #[inline(always)]
    pub const fn eth_maclmir(&self) -> &ETH_MACLMIR {
        &self.eth_maclmir
    }
}
#[doc = "ETH_MACCR (rw) register accessor: The MAC Configuration Register establishes the operating mode of the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maccr`]
module"]
pub type ETH_MACCR = crate::Reg<eth_maccr::ETH_MACCRrs>;
#[doc = "The MAC Configuration Register establishes the operating mode of the MAC."]
pub mod eth_maccr;
#[doc = "ETH_MACECR (rw) register accessor: The MAC Extended Configuration Register establishes the operating mode of the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macecr`]
module"]
pub type ETH_MACECR = crate::Reg<eth_macecr::ETH_MACECRrs>;
#[doc = "The MAC Extended Configuration Register establishes the operating mode of the MAC."]
pub mod eth_macecr;
#[doc = "ETH_MACPFR (rw) register accessor: The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macpfr`]
module"]
pub type ETH_MACPFR = crate::Reg<eth_macpfr::ETH_MACPFRrs>;
#[doc = "The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets."]
pub mod eth_macpfr;
#[doc = "ETH_MACWTR (rw) register accessor: The Watchdog Timeout register controls the watchdog timeout for received packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macwtr`]
module"]
pub type ETH_MACWTR = crate::Reg<eth_macwtr::ETH_MACWTRrs>;
#[doc = "The Watchdog Timeout register controls the watchdog timeout for received packets."]
pub mod eth_macwtr;
#[doc = "ETH_MACHT0R (rw) register accessor: The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macht0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macht0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macht0r`]
module"]
pub type ETH_MACHT0R = crate::Reg<eth_macht0r::ETH_MACHT0Rrs>;
#[doc = "The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
pub mod eth_macht0r;
#[doc = "ETH_MACHT1R (rw) register accessor: The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macht1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macht1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macht1r`]
module"]
pub type ETH_MACHT1R = crate::Reg<eth_macht1r::ETH_MACHT1Rrs>;
#[doc = "The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
pub mod eth_macht1r;
#[doc = "ETH_MACVTR (rw) register accessor: The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macvtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macvtr`]
module"]
pub type ETH_MACVTR = crate::Reg<eth_macvtr::ETH_MACVTRrs>;
#[doc = "The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets."]
pub mod eth_macvtr;
#[doc = "ETH_MACVHTR (rw) register accessor: When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macvhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macvhtr`]
module"]
pub type ETH_MACVHTR = crate::Reg<eth_macvhtr::ETH_MACVHTRrs>;
#[doc = "When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written."]
pub mod eth_macvhtr;
#[doc = "ETH_MACVIR (rw) register accessor: The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macvir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macvir`]
module"]
pub type ETH_MACVIR = crate::Reg<eth_macvir::ETH_MACVIRrs>;
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls."]
pub mod eth_macvir;
#[doc = "ETH_MACIVIR (rw) register accessor: The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macivir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macivir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macivir`]
module"]
pub type ETH_MACIVIR = crate::Reg<eth_macivir::ETH_MACIVIRrs>;
#[doc = "The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls."]
pub mod eth_macivir;
#[doc = "ETH_MACQ0TxFCR (rw) register accessor: The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macq0tx_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macq0tx_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macq0tx_fcr`]
module"]
#[doc(alias = "ETH_MACQ0TxFCR")]
pub type ETH_MACQ0TX_FCR = crate::Reg<eth_macq0tx_fcr::ETH_MACQ0TX_FCRrs>;
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register."]
pub mod eth_macq0tx_fcr;
#[doc = "ETH_MACRxFCR (rw) register accessor: The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrx_fcr`]
module"]
#[doc(alias = "ETH_MACRxFCR")]
pub type ETH_MACRX_FCR = crate::Reg<eth_macrx_fcr::ETH_MACRX_FCRrs>;
#[doc = "The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet."]
pub mod eth_macrx_fcr;
#[doc = "ETH_MACTxQPMR (r) register accessor: The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_qpmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactx_qpmr`]
module"]
#[doc(alias = "ETH_MACTxQPMR")]
pub type ETH_MACTX_QPMR = crate::Reg<eth_mactx_qpmr::ETH_MACTX_QPMRrs>;
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1."]
pub mod eth_mactx_qpmr;
#[doc = "ETH_MACRxQC0R (rw) register accessor: The Receive Queue Control 0 register controls the queue management in the MAC Receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrx_qc0r`]
module"]
#[doc(alias = "ETH_MACRxQC0R")]
pub type ETH_MACRX_QC0R = crate::Reg<eth_macrx_qc0r::ETH_MACRX_QC0Rrs>;
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver."]
pub mod eth_macrx_qc0r;
#[doc = "ETH_MACRxQC1R (rw) register accessor: The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrx_qc1r`]
module"]
#[doc(alias = "ETH_MACRxQC1R")]
pub type ETH_MACRX_QC1R = crate::Reg<eth_macrx_qc1r::ETH_MACRX_QC1Rrs>;
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver."]
pub mod eth_macrx_qc1r;
#[doc = "ETH_MACRxQC2R (rw) register accessor: This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrx_qc2r`]
module"]
#[doc(alias = "ETH_MACRxQC2R")]
pub type ETH_MACRX_QC2R = crate::Reg<eth_macrx_qc2r::ETH_MACRX_QC2Rrs>;
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1."]
pub mod eth_macrx_qc2r;
#[doc = "ETH_MACISR (r) register accessor: The Interrupt Status register contains the status of interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macisr`]
module"]
pub type ETH_MACISR = crate::Reg<eth_macisr::ETH_MACISRrs>;
#[doc = "The Interrupt Status register contains the status of interrupts."]
pub mod eth_macisr;
#[doc = "ETH_MACIER (rw) register accessor: The Interrupt Enable register contains the masks for generating the interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macier`]
module"]
pub type ETH_MACIER = crate::Reg<eth_macier::ETH_MACIERrs>;
#[doc = "The Interrupt Enable register contains the masks for generating the interrupts."]
pub mod eth_macier;
#[doc = "ETH_MACRxTxSR (r) register accessor: The Receive Transmit Status register contains the Receive and Transmit Error status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_tx_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrx_tx_sr`]
module"]
#[doc(alias = "ETH_MACRxTxSR")]
pub type ETH_MACRX_TX_SR = crate::Reg<eth_macrx_tx_sr::ETH_MACRX_TX_SRrs>;
#[doc = "The Receive Transmit Status register contains the Receive and Transmit Error status."]
pub mod eth_macrx_tx_sr;
#[doc = "ETH_MACPCSR (rw) register accessor: The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macpcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macpcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macpcsr`]
module"]
pub type ETH_MACPCSR = crate::Reg<eth_macpcsr::ETH_MACPCSRrs>;
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant."]
pub mod eth_macpcsr;
#[doc = "ETH_MACRWKPFR (rw) register accessor: The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrwkpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrwkpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macrwkpfr`]
module"]
pub type ETH_MACRWKPFR = crate::Reg<eth_macrwkpfr::ETH_MACRWKPFRrs>;
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
pub mod eth_macrwkpfr;
#[doc = "ETH_MACLCSR (rw) register accessor: The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maclcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maclcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maclcsr`]
module"]
pub type ETH_MACLCSR = crate::Reg<eth_maclcsr::ETH_MACLCSRrs>;
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
pub mod eth_maclcsr;
#[doc = "ETH_MACLTCR (rw) register accessor: The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macltcr`]
module"]
pub type ETH_MACLTCR = crate::Reg<eth_macltcr::ETH_MACLTCRrs>;
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission."]
pub mod eth_macltcr;
#[doc = "ETH_MACLETR (rw) register accessor: The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macletr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macletr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macletr`]
module"]
pub type ETH_MACLETR = crate::Reg<eth_macletr::ETH_MACLETRrs>;
#[doc = "The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds."]
pub mod eth_macletr;
#[doc = "ETH_MAC1USTCR (rw) register accessor: This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mac1ustcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mac1ustcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mac1ustcr`]
module"]
pub type ETH_MAC1USTCR = crate::Reg<eth_mac1ustcr::ETH_MAC1USTCRrs>;
#[doc = "This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially."]
pub mod eth_mac1ustcr;
#[doc = "ETH_MACPHYCSR (rw) register accessor: The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macphycsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macphycsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macphycsr`]
module"]
pub type ETH_MACPHYCSR = crate::Reg<eth_macphycsr::ETH_MACPHYCSRrs>;
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY."]
pub mod eth_macphycsr;
#[doc = "ETH_MACVR (r) register accessor: The version register identifies the version of the Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macvr`]
module"]
pub type ETH_MACVR = crate::Reg<eth_macvr::ETH_MACVRrs>;
#[doc = "The version register identifies the version of the Ethernet peripheral."]
pub mod eth_macvr;
#[doc = "ETH_MACDR (r) register accessor: The Debug register provides the debug status of various MAC blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macdr`]
module"]
pub type ETH_MACDR = crate::Reg<eth_macdr::ETH_MACDRrs>;
#[doc = "The Debug register provides the debug status of various MAC blocks."]
pub mod eth_macdr;
#[doc = "ETH_MACHWF1R (r) register accessor: This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_machwf1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_machwf1r`]
module"]
pub type ETH_MACHWF1R = crate::Reg<eth_machwf1r::ETH_MACHWF1Rrs>;
#[doc = "This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
pub mod eth_machwf1r;
#[doc = "ETH_MACHWF2R (r) register accessor: This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_machwf2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_machwf2r`]
module"]
pub type ETH_MACHWF2R = crate::Reg<eth_machwf2r::ETH_MACHWF2Rrs>;
#[doc = "This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
pub mod eth_machwf2r;
#[doc = "ETH_MACMDIOAR (rw) register accessor: The MDIO Address register controls the management cycles to external PHY through a management interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macmdioar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macmdioar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macmdioar`]
module"]
pub type ETH_MACMDIOAR = crate::Reg<eth_macmdioar::ETH_MACMDIOARrs>;
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface."]
pub mod eth_macmdioar;
#[doc = "ETH_MACMDIODR (rw) register accessor: The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macmdiodr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macmdiodr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macmdiodr`]
module"]
pub type ETH_MACMDIODR = crate::Reg<eth_macmdiodr::ETH_MACMDIODRrs>;
#[doc = "The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register."]
pub mod eth_macmdiodr;
#[doc = "ETH_MACA0HR (rw) register accessor: The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca0hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca0hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca0hr`]
module"]
pub type ETH_MACA0HR = crate::Reg<eth_maca0hr::ETH_MACA0HRrs>;
#[doc = "The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain."]
pub mod eth_maca0hr;
#[doc = "ETH_MACA0LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca0lr`]
module"]
pub type ETH_MACA0LR = crate::Reg<eth_maca0lr::ETH_MACA0LRrs>;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca0lr;
#[doc = "ETH_MACA1HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca1hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca1hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca1hr`]
module"]
pub type ETH_MACA1HR = crate::Reg<eth_maca1hr::ETH_MACA1HRrs>;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca1hr;
#[doc = "ETH_MACA1LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca1lr`]
module"]
pub type ETH_MACA1LR = crate::Reg<eth_maca1lr::ETH_MACA1LRrs>;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca1lr;
#[doc = "ETH_MACA2HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca2hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca2hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca2hr`]
module"]
pub type ETH_MACA2HR = crate::Reg<eth_maca2hr::ETH_MACA2HRrs>;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca2hr;
#[doc = "ETH_MACA2LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca2lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca2lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca2lr`]
module"]
pub type ETH_MACA2LR = crate::Reg<eth_maca2lr::ETH_MACA2LRrs>;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca2lr;
#[doc = "ETH_MACA3HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca3hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca3hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca3hr`]
module"]
pub type ETH_MACA3HR = crate::Reg<eth_maca3hr::ETH_MACA3HRrs>;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca3hr;
#[doc = "ETH_MACA3LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca3lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca3lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maca3lr`]
module"]
pub type ETH_MACA3LR = crate::Reg<eth_maca3lr::ETH_MACA3LRrs>;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca3lr;
#[doc = "MMC_CONTROL (rw) register accessor: This register configures the MMC operating mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_control`]
module"]
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROLrs>;
#[doc = "This register configures the MMC operating mode."]
pub mod mmc_control;
#[doc = "MMC_RX_INTERRUPT (r) register accessor: This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_interrupt`]
module"]
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPTrs>;
#[doc = "This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
pub mod mmc_rx_interrupt;
#[doc = "MMC_TX_INTERRUPT (r) register accessor: This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_interrupt`]
module"]
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPTrs>;
#[doc = "This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
pub mod mmc_tx_interrupt;
#[doc = "MMC_RX_INTERRUPT_MASK (rw) register accessor: The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_interrupt_mask`]
module"]
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASKrs>;
#[doc = "The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values."]
pub mod mmc_rx_interrupt_mask;
#[doc = "MMC_TX_INTERRUPT_MASK (rw) register accessor: This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_interrupt_mask`]
module"]
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASKrs>;
#[doc = "This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration."]
pub mod mmc_tx_interrupt_mask;
#[doc = "TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_single_collision_good_packets`]
module"]
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode."]
pub mod tx_single_collision_good_packets;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multiple_collision_good_packets`]
module"]
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETSrs>;
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode."]
pub mod tx_multiple_collision_good_packets;
#[doc = "TX_PACKET_COUNT_GOOD (r) register accessor: This register provides the number of good packets transmitted by Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_packet_count_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_packet_count_good`]
module"]
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOODrs>;
#[doc = "This register provides the number of good packets transmitted by Ethernet peripheral."]
pub mod tx_packet_count_good;
#[doc = "RX_CRC_ERROR_PACKETS (r) register accessor: This register provides the number of packets received by Ethernet peripheral with CRC error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_error_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_error_packets`]
module"]
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETSrs>;
#[doc = "This register provides the number of packets received by Ethernet peripheral with CRC error."]
pub mod rx_crc_error_packets;
#[doc = "RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_alignment_error_packets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_alignment_error_packets`]
module"]
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETSrs>;
#[doc = "This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode."]
pub mod rx_alignment_error_packets;
#[doc = "RX_UNICAST_PACKETS_GOOD (r) register accessor: This register provides the number of good unicast packets received by Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_unicast_packets_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_unicast_packets_good`]
module"]
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOODrs>;
#[doc = "This register provides the number of good unicast packets received by Ethernet peripheral."]
pub mod rx_unicast_packets_good;
#[doc = "TX_LPI_USEC_CNTR (r) register accessor: This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_usec_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_lpi_usec_cntr`]
module"]
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTRrs>;
#[doc = "This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral."]
pub mod tx_lpi_usec_cntr;
#[doc = "TX_LPI_TRAN_CNTR (r) register accessor: This register provides the number of times Ethernet peripheral has entered Tx LPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_tran_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_lpi_tran_cntr`]
module"]
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTRrs>;
#[doc = "This register provides the number of times Ethernet peripheral has entered Tx LPI."]
pub mod tx_lpi_tran_cntr;
#[doc = "RX_LPI_USEC_CNTR (r) register accessor: This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_usec_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_lpi_usec_cntr`]
module"]
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTRrs>;
#[doc = "This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral."]
pub mod rx_lpi_usec_cntr;
#[doc = "RX_LPI_TRAN_CNTR (r) register accessor: This register provides the number of times Ethernet peripheral has entered Rx LPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_tran_cntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_lpi_tran_cntr`]
module"]
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTRrs>;
#[doc = "This register provides the number of times Ethernet peripheral has entered Rx LPI."]
pub mod rx_lpi_tran_cntr;
#[doc = "ETH_MACL3L4C0R (rw) register accessor: The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3l4c0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3l4c0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3l4c0r`]
module"]
pub type ETH_MACL3L4C0R = crate::Reg<eth_macl3l4c0r::ETH_MACL3L4C0Rrs>;
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration."]
pub mod eth_macl3l4c0r;
#[doc = "ETH_MACL4A0R (rw) register accessor: Layer4 address filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl4a0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl4a0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl4a0r`]
module"]
pub type ETH_MACL4A0R = crate::Reg<eth_macl4a0r::ETH_MACL4A0Rrs>;
#[doc = "Layer4 address filter 0 register"]
pub mod eth_macl4a0r;
#[doc = "ETH_MACL3A00R (rw) register accessor: For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a00r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a00r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a00r`]
module"]
pub type ETH_MACL3A00R = crate::Reg<eth_macl3a00r::ETH_MACL3A00Rrs>;
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a00r;
#[doc = "ETH_MACL3A10R (rw) register accessor: For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a10r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a10r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a10r`]
module"]
pub type ETH_MACL3A10R = crate::Reg<eth_macl3a10r::ETH_MACL3A10Rrs>;
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a10r;
#[doc = "ETH_MACL3A20 (rw) register accessor: The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a20`]
module"]
pub type ETH_MACL3A20 = crate::Reg<eth_macl3a20::ETH_MACL3A20rs>;
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a20;
#[doc = "ETH_MACL3A30 (rw) register accessor: The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a30`]
module"]
pub type ETH_MACL3A30 = crate::Reg<eth_macl3a30::ETH_MACL3A30rs>;
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a30;
#[doc = "ETH_MACL3L4C1R (rw) register accessor: The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3l4c1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3l4c1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3l4c1r`]
module"]
pub type ETH_MACL3L4C1R = crate::Reg<eth_macl3l4c1r::ETH_MACL3L4C1Rrs>;
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4."]
pub mod eth_macl3l4c1r;
#[doc = "ETH_MACL4A1R (rw) register accessor: The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl4a1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl4a1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl4a1r`]
module"]
pub type ETH_MACL4A1R = crate::Reg<eth_macl4a1r::ETH_MACL4A1Rrs>;
#[doc = "The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock."]
pub mod eth_macl4a1r;
#[doc = "ETH_MACL3A01R (rw) register accessor: For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a01r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a01r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a01r`]
module"]
pub type ETH_MACL3A01R = crate::Reg<eth_macl3a01r::ETH_MACL3A01Rrs>;
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a01r;
#[doc = "ETH_MACL3A11R (rw) register accessor: For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a11r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a11r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a11r`]
module"]
pub type ETH_MACL3A11R = crate::Reg<eth_macl3a11r::ETH_MACL3A11Rrs>;
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a11r;
#[doc = "ETH_MACL3A21R (rw) register accessor: The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a21r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a21r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a21r`]
module"]
pub type ETH_MACL3A21R = crate::Reg<eth_macl3a21r::ETH_MACL3A21Rrs>;
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a21r;
#[doc = "ETH_MACL3A31R (rw) register accessor: The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a31r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a31r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macl3a31r`]
module"]
pub type ETH_MACL3A31R = crate::Reg<eth_macl3a31r::ETH_MACL3A31Rrs>;
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a31r;
#[doc = "ETH_MACARPAR (rw) register accessor: The ARP Address register contains the IPv4 Destination Address of the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macarpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macarpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macarpar`]
module"]
pub type ETH_MACARPAR = crate::Reg<eth_macarpar::ETH_MACARPARrs>;
#[doc = "The ARP Address register contains the IPv4 Destination Address of the MAC."]
pub mod eth_macarpar;
#[doc = "ETH_MACTSCR (rw) register accessor: This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactscr`]
module"]
pub type ETH_MACTSCR = crate::Reg<eth_mactscr::ETH_MACTSCRrs>;
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver."]
pub mod eth_mactscr;
#[doc = "ETH_MACSSIR (rw) register accessor: The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macssir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macssir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macssir`]
module"]
pub type ETH_MACSSIR = crate::Reg<eth_macssir::ETH_MACSSIRrs>;
#[doc = "The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow."]
pub mod eth_macssir;
#[doc = "ETH_MACSTSR (r) register accessor: The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macstsr`]
module"]
pub type ETH_MACSTSR = crate::Reg<eth_macstsr::ETH_MACSTSRrs>;
#[doc = "The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstsr;
#[doc = "ETH_MACSTNR (r) register accessor: The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macstnr`]
module"]
pub type ETH_MACSTNR = crate::Reg<eth_macstnr::ETH_MACSTNRrs>;
#[doc = "The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstnr;
#[doc = "ETH_MACSTSUR (rw) register accessor: The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstsur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macstsur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macstsur`]
module"]
pub type ETH_MACSTSUR = crate::Reg<eth_macstsur::ETH_MACSTSURrs>;
#[doc = "The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstsur;
#[doc = "ETH_MACSTNUR (rw) register accessor: This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macstnur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macstnur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macstnur`]
module"]
pub type ETH_MACSTNUR = crate::Reg<eth_macstnur::ETH_MACSTNURrs>;
#[doc = "This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input."]
pub mod eth_macstnur;
#[doc = "ETH_MACTSAR (rw) register accessor: The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactsar`]
module"]
pub type ETH_MACTSAR = crate::Reg<eth_mactsar::ETH_MACTSARrs>;
#[doc = "The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows."]
pub mod eth_mactsar;
#[doc = "ETH_MACTSSR (r) register accessor: The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactssr`]
module"]
pub type ETH_MACTSSR = crate::Reg<eth_mactssr::ETH_MACTSSRrs>;
#[doc = "The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register."]
pub mod eth_mactssr;
#[doc = "ETH_MACTxTSSNR (r) register accessor: This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_tssnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactx_tssnr`]
module"]
#[doc(alias = "ETH_MACTxTSSNR")]
pub type ETH_MACTX_TSSNR = crate::Reg<eth_mactx_tssnr::ETH_MACTX_TSSNRrs>;
#[doc = "This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled."]
pub mod eth_mactx_tssnr;
#[doc = "ETH_MACTxTSSSR (r) register accessor: The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactx_tsssr`]
module"]
#[doc(alias = "ETH_MACTxTSSSR")]
pub type ETH_MACTX_TSSSR = crate::Reg<eth_mactx_tsssr::ETH_MACTX_TSSSRrs>;
#[doc = "The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted."]
pub mod eth_mactx_tsssr;
#[doc = "ETH_MACACR (rw) register accessor: The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macacr`]
module"]
pub type ETH_MACACR = crate::Reg<eth_macacr::ETH_MACACRrs>;
#[doc = "The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot."]
pub mod eth_macacr;
#[doc = "ETH_MACATSNR (r) register accessor: The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macatsnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macatsnr`]
module"]
pub type ETH_MACATSNR = crate::Reg<eth_macatsnr::ETH_MACATSNRrs>;
#[doc = "The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read."]
pub mod eth_macatsnr;
#[doc = "ETH_MACATSSR (r) register accessor: The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macatssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macatssr`]
module"]
pub type ETH_MACATSSR = crate::Reg<eth_macatssr::ETH_MACATSSRrs>;
#[doc = "The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register."]
pub mod eth_macatssr;
#[doc = "ETH_MACTSIACR (rw) register accessor: The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsiacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsiacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactsiacr`]
module"]
pub type ETH_MACTSIACR = crate::Reg<eth_mactsiacr::ETH_MACTSIACRrs>;
#[doc = "The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages."]
pub mod eth_mactsiacr;
#[doc = "ETH_MACTSEACR (rw) register accessor: The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactseacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactseacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactseacr`]
module"]
pub type ETH_MACTSEACR = crate::Reg<eth_mactseacr::ETH_MACTSEACRrs>;
#[doc = "The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages."]
pub mod eth_mactseacr;
#[doc = "ETH_MACTSICNR (rw) register accessor: This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsicnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsicnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactsicnr`]
module"]
pub type ETH_MACTSICNR = crate::Reg<eth_mactsicnr::ETH_MACTSICNRrs>;
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path."]
pub mod eth_mactsicnr;
#[doc = "ETH_MACTSECNR (rw) register accessor: This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactsecnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mactsecnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mactsecnr`]
module"]
pub type ETH_MACTSECNR = crate::Reg<eth_mactsecnr::ETH_MACTSECNRrs>;
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path."]
pub mod eth_mactsecnr;
#[doc = "ETH_MACPPSCR (rw) register accessor: The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macppscr`]
module"]
pub type ETH_MACPPSCR = crate::Reg<eth_macppscr::ETH_MACPPSCRrs>;
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected."]
pub mod eth_macppscr;
#[doc = "ETH_MACPPSTTSR (rw) register accessor: The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsttsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsttsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macppsttsr`]
module"]
pub type ETH_MACPPSTTSR = crate::Reg<eth_macppsttsr::ETH_MACPPSTTSRrs>;
#[doc = "The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers."]
pub mod eth_macppsttsr;
#[doc = "ETH_MACPPSTTNR (rw) register accessor: The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsttnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsttnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macppsttnr`]
module"]
pub type ETH_MACPPSTTNR = crate::Reg<eth_macppsttnr::ETH_MACPPSTTNRrs>;
#[doc = "The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected."]
pub mod eth_macppsttnr;
#[doc = "ETH_MACPPSIR (rw) register accessor: The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\]).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macppsir`]
module"]
pub type ETH_MACPPSIR = crate::Reg<eth_macppsir::ETH_MACPPSIRrs>;
#[doc = "The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\])."]
pub mod eth_macppsir;
#[doc = "ETH_MACPPSWR (rw) register accessor: The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppswr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppswr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macppswr`]
module"]
pub type ETH_MACPPSWR = crate::Reg<eth_macppswr::ETH_MACPPSWRrs>;
#[doc = "The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o)."]
pub mod eth_macppswr;
#[doc = "ETH_MACPOCR (rw) register accessor: This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macpocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macpocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macpocr`]
module"]
pub type ETH_MACPOCR = crate::Reg<eth_macpocr::ETH_MACPOCRrs>;
#[doc = "This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macpocr;
#[doc = "ETH_MACSPI0R (rw) register accessor: This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macspi0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macspi0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macspi0r`]
module"]
pub type ETH_MACSPI0R = crate::Reg<eth_macspi0r::ETH_MACSPI0Rrs>;
#[doc = "This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macspi0r;
#[doc = "ETH_MACSPI1R (rw) register accessor: This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macspi1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macspi1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macspi1r`]
module"]
pub type ETH_MACSPI1R = crate::Reg<eth_macspi1r::ETH_MACSPI1Rrs>;
#[doc = "This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macspi1r;
#[doc = "ETH_MACSPI2R (rw) register accessor: This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macspi2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macspi2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_macspi2r`]
module"]
pub type ETH_MACSPI2R = crate::Reg<eth_macspi2r::ETH_MACSPI2Rrs>;
#[doc = "This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node."]
pub mod eth_macspi2r;
#[doc = "ETH_MACLMIR (rw) register accessor: This register contains the periodic intervals for automatic PTP packet generation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maclmir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maclmir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_maclmir`]
module"]
pub type ETH_MACLMIR = crate::Reg<eth_maclmir::ETH_MACLMIRrs>;
#[doc = "This register contains the periodic intervals for automatic PTP packet generation."]
pub mod eth_maclmir;
