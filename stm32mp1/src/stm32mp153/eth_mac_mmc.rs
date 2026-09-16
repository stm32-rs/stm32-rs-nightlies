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
    macq0tx_fcr: MACQ0TX_FCR,
    _reserved11: [u8; 0x1c],
    macrx_fcr: MACRX_FCR,
    _reserved12: [u8; 0x04],
    mactx_qpmr: MACTX_QPMR,
    _reserved13: [u8; 0x04],
    macrx_qc0r: MACRX_QC0R,
    macrx_qc1r: MACRX_QC1R,
    macrx_qc2r: MACRX_QC2R,
    _reserved16: [u8; 0x04],
    macisr: MACISR,
    macier: MACIER,
    macrx_tx_sr: MACRX_TX_SR,
    _reserved19: [u8; 0x04],
    macpcsr: MACPCSR,
    macrwkpfr: MACRWKPFR,
    _reserved21: [u8; 0x08],
    maclcsr: MACLCSR,
    macltcr: MACLTCR,
    macletr: MACLETR,
    mac1ustcr: MAC1USTCR,
    _reserved25: [u8; 0x18],
    macphycsr: MACPHYCSR,
    _reserved26: [u8; 0x14],
    macvr: MACVR,
    macdr: MACDR,
    _reserved28: [u8; 0x08],
    machwf1r: MACHWF1R,
    machwf2r: MACHWF2R,
    _reserved30: [u8; 0xd8],
    macmdioar: MACMDIOAR,
    macmdiodr: MACMDIODR,
    _reserved32: [u8; 0xf8],
    maca0hr: MACA0HR,
    maca0lr: MACA0LR,
    maca1hr: MACA1HR,
    maca1lr: MACA1LR,
    maca2hr: MACA2HR,
    maca2lr: MACA2LR,
    maca3hr: MACA3HR,
    maca3lr: MACA3LR,
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
    macl3l4c0r: MACL3L4C0R,
    macl4a0r: MACL4A0R,
    _reserved57: [u8; 0x08],
    macl3a00r: MACL3A00R,
    macl3a10r: MACL3A10R,
    macl3a20: MACL3A20,
    macl3a30: MACL3A30,
    _reserved61: [u8; 0x10],
    macl3l4c1r: MACL3L4C1R,
    macl4a1r: MACL4A1R,
    _reserved63: [u8; 0x08],
    macl3a01r: MACL3A01R,
    macl3a11r: MACL3A11R,
    macl3a21r: MACL3A21R,
    macl3a31r: MACL3A31R,
    _reserved67: [u8; 0x0190],
    macarpar: MACARPAR,
    _reserved68: [u8; 0x1c],
    mactscr: MACTSCR,
    macssir: MACSSIR,
    macstsr: MACSTSR,
    macstnr: MACSTNR,
    macstsur: MACSTSUR,
    macstnur: MACSTNUR,
    mactsar: MACTSAR,
    _reserved75: [u8; 0x04],
    mactssr: MACTSSR,
    _reserved76: [u8; 0x0c],
    mactx_tssnr: MACTX_TSSNR,
    mactx_tsssr: MACTX_TSSSR,
    _reserved78: [u8; 0x08],
    macacr: MACACR,
    _reserved79: [u8; 0x04],
    macatsnr: MACATSNR,
    macatssr: MACATSSR,
    mactsiacr: MACTSIACR,
    mactseacr: MACTSEACR,
    mactsicnr: MACTSICNR,
    mactsecnr: MACTSECNR,
    _reserved85: [u8; 0x10],
    macppscr: MACPPSCR,
    _reserved86: [u8; 0x0c],
    macppsttsr: MACPPSTTSR,
    macppsttnr: MACPPSTTNR,
    macppsir: MACPPSIR,
    macppswr: MACPPSWR,
    _reserved90: [u8; 0x30],
    macpocr: MACPOCR,
    macspi0r: MACSPI0R,
    macspi1r: MACSPI1R,
    macspi2r: MACSPI2R,
    maclmir: MACLMIR,
}
impl RegisterBlock {
    ///0x00 - The MAC Configuration Register establishes the operating mode of the MAC.
    #[inline(always)]
    pub const fn maccr(&self) -> &MACCR {
        &self.maccr
    }
    ///0x04 - The MAC Extended Configuration Register establishes the operating mode of the MAC.
    #[inline(always)]
    pub const fn macecr(&self) -> &MACECR {
        &self.macecr
    }
    ///0x08 - The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.
    #[inline(always)]
    pub const fn macpfr(&self) -> &MACPFR {
        &self.macpfr
    }
    ///0x0c - The Watchdog Timeout register controls the watchdog timeout for received packets.
    #[inline(always)]
    pub const fn macwtr(&self) -> &MACWTR {
        &self.macwtr
    }
    ///0x10 - The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.
    #[inline(always)]
    pub const fn macht0r(&self) -> &MACHT0R {
        &self.macht0r
    }
    ///0x14 - The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.
    #[inline(always)]
    pub const fn macht1r(&self) -> &MACHT1R {
        &self.macht1r
    }
    ///0x50 - The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.
    #[inline(always)]
    pub const fn macvtr(&self) -> &MACVTR {
        &self.macvtr
    }
    ///0x58 - When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of this register are written.
    #[inline(always)]
    pub const fn macvhtr(&self) -> &MACVHTR {
        &self.macvhtr
    }
    ///0x60 - The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.
    #[inline(always)]
    pub const fn macvir(&self) -> &MACVIR {
        &self.macvir
    }
    ///0x64 - The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.
    #[inline(always)]
    pub const fn macivir(&self) -> &MACIVIR {
        &self.macivir
    }
    ///0x70 - The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.
    #[inline(always)]
    pub const fn macq0tx_fcr(&self) -> &MACQ0TX_FCR {
        &self.macq0tx_fcr
    }
    ///0x90 - The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.
    #[inline(always)]
    pub const fn macrx_fcr(&self) -> &MACRX_FCR {
        &self.macrx_fcr
    }
    ///0x98 - The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.
    #[inline(always)]
    pub const fn mactx_qpmr(&self) -> &MACTX_QPMR {
        &self.mactx_qpmr
    }
    ///0xa0 - The Receive Queue Control 0 register controls the queue management in the MAC Receiver.
    #[inline(always)]
    pub const fn macrx_qc0r(&self) -> &MACRX_QC0R {
        &self.macrx_qc0r
    }
    ///0xa4 - The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.
    #[inline(always)]
    pub const fn macrx_qc1r(&self) -> &MACRX_QC1R {
        &self.macrx_qc1r
    }
    ///0xa8 - This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.
    #[inline(always)]
    pub const fn macrx_qc2r(&self) -> &MACRX_QC2R {
        &self.macrx_qc2r
    }
    ///0xb0 - The Interrupt Status register contains the status of interrupts.
    #[inline(always)]
    pub const fn macisr(&self) -> &MACISR {
        &self.macisr
    }
    ///0xb4 - The Interrupt Enable register contains the masks for generating the interrupts.
    #[inline(always)]
    pub const fn macier(&self) -> &MACIER {
        &self.macier
    }
    ///0xb8 - The Receive Transmit Status register contains the Receive and Transmit Error status.
    #[inline(always)]
    pub const fn macrx_tx_sr(&self) -> &MACRX_TX_SR {
        &self.macrx_tx_sr
    }
    ///0xc0 - The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.
    #[inline(always)]
    pub const fn macpcsr(&self) -> &MACPCSR {
        &self.macpcsr
    }
    ///0xc4 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
    #[inline(always)]
    pub const fn macrwkpfr(&self) -> &MACRWKPFR {
        &self.macrwkpfr
    }
    ///0xd0 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
    #[inline(always)]
    pub const fn maclcsr(&self) -> &MACLCSR {
        &self.maclcsr
    }
    ///0xd4 - The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.
    #[inline(always)]
    pub const fn macltcr(&self) -> &MACLTCR {
        &self.macltcr
    }
    ///0xd8 - The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.
    #[inline(always)]
    pub const fn macletr(&self) -> &MACLETR {
        &self.macletr
    }
    ///0xdc - This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.
    #[inline(always)]
    pub const fn mac1ustcr(&self) -> &MAC1USTCR {
        &self.mac1ustcr
    }
    ///0xf8 - The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.
    #[inline(always)]
    pub const fn macphycsr(&self) -> &MACPHYCSR {
        &self.macphycsr
    }
    ///0x110 - The version register identifies the version of the Ethernet peripheral.
    #[inline(always)]
    pub const fn macvr(&self) -> &MACVR {
        &self.macvr
    }
    ///0x114 - The Debug register provides the debug status of various MAC blocks.
    #[inline(always)]
    pub const fn macdr(&self) -> &MACDR {
        &self.macdr
    }
    ///0x120 - This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
    #[inline(always)]
    pub const fn machwf1r(&self) -> &MACHWF1R {
        &self.machwf1r
    }
    ///0x124 - This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
    #[inline(always)]
    pub const fn machwf2r(&self) -> &MACHWF2R {
        &self.machwf2r
    }
    ///0x200 - The MDIO Address register controls the management cycles to external PHY through a management interface.
    #[inline(always)]
    pub const fn macmdioar(&self) -> &MACMDIOAR {
        &self.macmdioar
    }
    ///0x204 - The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.
    #[inline(always)]
    pub const fn macmdiodr(&self) -> &MACMDIODR {
        &self.macmdiodr
    }
    ///0x300 - The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\] is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.
    #[inline(always)]
    pub const fn maca0hr(&self) -> &MACA0HR {
        &self.maca0hr
    }
    ///0x304 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    #[inline(always)]
    pub const fn maca0lr(&self) -> &MACA0LR {
        &self.maca0lr
    }
    ///0x308 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    #[inline(always)]
    pub const fn maca1hr(&self) -> &MACA1HR {
        &self.maca1hr
    }
    ///0x30c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    #[inline(always)]
    pub const fn maca1lr(&self) -> &MACA1LR {
        &self.maca1lr
    }
    ///0x310 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    #[inline(always)]
    pub const fn maca2hr(&self) -> &MACA2HR {
        &self.maca2hr
    }
    ///0x314 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    #[inline(always)]
    pub const fn maca2lr(&self) -> &MACA2LR {
        &self.maca2lr
    }
    ///0x318 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
    #[inline(always)]
    pub const fn maca3hr(&self) -> &MACA3HR {
        &self.maca3hr
    }
    ///0x31c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
    #[inline(always)]
    pub const fn maca3lr(&self) -> &MACA3LR {
        &self.maca3lr
    }
    ///0x700 - This register configures the MMC operating mode.
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MMC_CONTROL {
        &self.mmc_control
    }
    ///0x704 - This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
    #[inline(always)]
    pub const fn mmc_rx_interrupt(&self) -> &MMC_RX_INTERRUPT {
        &self.mmc_rx_interrupt
    }
    ///0x708 - This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
    #[inline(always)]
    pub const fn mmc_tx_interrupt(&self) -> &MMC_TX_INTERRUPT {
        &self.mmc_tx_interrupt
    }
    ///0x70c - The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(&self) -> &MMC_RX_INTERRUPT_MASK {
        &self.mmc_rx_interrupt_mask
    }
    ///0x710 - This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(&self) -> &MMC_TX_INTERRUPT_MASK {
        &self.mmc_tx_interrupt_mask
    }
    ///0x74c - This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(&self) -> &TX_SINGLE_COLLISION_GOOD_PACKETS {
        &self.tx_single_collision_good_packets
    }
    ///0x750 - This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(&self) -> &TX_MULTIPLE_COLLISION_GOOD_PACKETS {
        &self.tx_multiple_collision_good_packets
    }
    ///0x768 - This register provides the number of good packets transmitted by Ethernet peripheral.
    #[inline(always)]
    pub const fn tx_packet_count_good(&self) -> &TX_PACKET_COUNT_GOOD {
        &self.tx_packet_count_good
    }
    ///0x794 - This register provides the number of packets received by Ethernet peripheral with CRC error.
    #[inline(always)]
    pub const fn rx_crc_error_packets(&self) -> &RX_CRC_ERROR_PACKETS {
        &self.rx_crc_error_packets
    }
    ///0x798 - This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.
    #[inline(always)]
    pub const fn rx_alignment_error_packets(&self) -> &RX_ALIGNMENT_ERROR_PACKETS {
        &self.rx_alignment_error_packets
    }
    ///0x7c4 - This register provides the number of good unicast packets received by Ethernet peripheral.
    #[inline(always)]
    pub const fn rx_unicast_packets_good(&self) -> &RX_UNICAST_PACKETS_GOOD {
        &self.rx_unicast_packets_good
    }
    ///0x7ec - This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(&self) -> &TX_LPI_USEC_CNTR {
        &self.tx_lpi_usec_cntr
    }
    ///0x7f0 - This register provides the number of times Ethernet peripheral has entered Tx LPI.
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(&self) -> &TX_LPI_TRAN_CNTR {
        &self.tx_lpi_tran_cntr
    }
    ///0x7f4 - This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(&self) -> &RX_LPI_USEC_CNTR {
        &self.rx_lpi_usec_cntr
    }
    ///0x7f8 - This register provides the number of times Ethernet peripheral has entered Rx LPI.
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(&self) -> &RX_LPI_TRAN_CNTR {
        &self.rx_lpi_tran_cntr
    }
    ///0x900 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.
    #[inline(always)]
    pub const fn macl3l4c0r(&self) -> &MACL3L4C0R {
        &self.macl3l4c0r
    }
    ///0x904 - Layer4 address filter 0 register
    #[inline(always)]
    pub const fn macl4a0r(&self) -> &MACL4A0R {
        &self.macl4a0r
    }
    ///0x910 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a00r(&self) -> &MACL3A00R {
        &self.macl3a00r
    }
    ///0x914 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a10r(&self) -> &MACL3A10R {
        &self.macl3a10r
    }
    ///0x918 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a20(&self) -> &MACL3A20 {
        &self.macl3a20
    }
    ///0x91c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a30(&self) -> &MACL3A30 {
        &self.macl3a30
    }
    ///0x930 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.
    #[inline(always)]
    pub const fn macl3l4c1r(&self) -> &MACL3L4C1R {
        &self.macl3l4c1r
    }
    ///0x934 - The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.
    #[inline(always)]
    pub const fn macl4a1r(&self) -> &MACL4A1R {
        &self.macl4a1r
    }
    ///0x940 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a01r(&self) -> &MACL3A01R {
        &self.macl3a01r
    }
    ///0x944 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a11r(&self) -> &MACL3A11R {
        &self.macl3a11r
    }
    ///0x948 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a21r(&self) -> &MACL3A21R {
        &self.macl3a21r
    }
    ///0x94c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.
    #[inline(always)]
    pub const fn macl3a31r(&self) -> &MACL3A31R {
        &self.macl3a31r
    }
    ///0xae0 - The ARP Address register contains the IPv4 Destination Address of the MAC.
    #[inline(always)]
    pub const fn macarpar(&self) -> &MACARPAR {
        &self.macarpar
    }
    ///0xb00 - This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.
    #[inline(always)]
    pub const fn mactscr(&self) -> &MACTSCR {
        &self.mactscr
    }
    ///0xb04 - The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.
    #[inline(always)]
    pub const fn macssir(&self) -> &MACSSIR {
        &self.macssir
    }
    ///0xb08 - The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    #[inline(always)]
    pub const fn macstsr(&self) -> &MACSTSR {
        &self.macstsr
    }
    ///0xb0c - The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    #[inline(always)]
    pub const fn macstnr(&self) -> &MACSTNR {
        &self.macstnr
    }
    ///0xb10 - The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
    #[inline(always)]
    pub const fn macstsur(&self) -> &MACSTSUR {
        &self.macstsur
    }
    ///0xb14 - This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.
    #[inline(always)]
    pub const fn macstnur(&self) -> &MACSTNUR {
        &self.macstnur
    }
    ///0xb18 - The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.
    #[inline(always)]
    pub const fn mactsar(&self) -> &MACTSAR {
        &self.mactsar
    }
    ///0xb20 - The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\[27:25\] gets cleared when the application reads this register.
    #[inline(always)]
    pub const fn mactssr(&self) -> &MACTSSR {
        &self.mactssr
    }
    ///0xb30 - This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.
    #[inline(always)]
    pub const fn mactx_tssnr(&self) -> &MACTX_TSSNR {
        &self.mactx_tssnr
    }
    ///0xb34 - The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.
    #[inline(always)]
    pub const fn mactx_tsssr(&self) -> &MACTX_TSSSR {
        &self.mactx_tsssr
    }
    ///0xb40 - The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.
    #[inline(always)]
    pub const fn macacr(&self) -> &MACACR {
        &self.macacr
    }
    ///0xb48 - The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\] in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\] are read and in big-endian mode, Bits\[7:0\] are read.
    #[inline(always)]
    pub const fn macatsnr(&self) -> &MACATSNR {
        &self.macatsnr
    }
    ///0xb4c - The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.
    #[inline(always)]
    pub const fn macatssr(&self) -> &MACATSSR {
        &self.macatssr
    }
    ///0xb50 - The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.
    #[inline(always)]
    pub const fn mactsiacr(&self) -> &MACTSIACR {
        &self.mactsiacr
    }
    ///0xb54 - The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.
    #[inline(always)]
    pub const fn mactseacr(&self) -> &MACTSEACR {
        &self.mactseacr
    }
    ///0xb58 - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.
    #[inline(always)]
    pub const fn mactsicnr(&self) -> &MACTSICNR {
        &self.mactsicnr
    }
    ///0xb5c - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.
    #[inline(always)]
    pub const fn mactsecnr(&self) -> &MACTSECNR {
        &self.mactsecnr
    }
    ///0xb70 - The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\] of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\] are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\] are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\] are valid only when Flexible PPS feature is selected.
    #[inline(always)]
    pub const fn macppscr(&self) -> &MACPPSCR {
        &self.macppscr
    }
    ///0xb80 - The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \[Bit 1 of ETH_MACTSSR\] when the system time exceeds the value programmed in these registers.
    #[inline(always)]
    pub const fn macppsttsr(&self) -> &MACPPSTTSR {
        &self.macppsttsr
    }
    ///0xb84 - The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.
    #[inline(always)]
    pub const fn macppsttnr(&self) -> &MACPPSTTNR {
        &self.macppsttnr
    }
    ///0xb88 - The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).
    #[inline(always)]
    pub const fn macppsir(&self) -> &MACPPSIR {
        &self.macppsir
    }
    ///0xb8c - The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).
    #[inline(always)]
    pub const fn macppswr(&self) -> &MACPPSWR {
        &self.macppswr
    }
    ///0xbc0 - This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    #[inline(always)]
    pub const fn macpocr(&self) -> &MACPOCR {
        &self.macpocr
    }
    ///0xbc4 - This register contains Bits\[31:0\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    #[inline(always)]
    pub const fn macspi0r(&self) -> &MACSPI0R {
        &self.macspi0r
    }
    ///0xbc8 - This register contains Bits\[63:32\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
    #[inline(always)]
    pub const fn macspi1r(&self) -> &MACSPI1R {
        &self.macspi1r
    }
    ///0xbcc - This register contains Bits\[79:64\] of the 80-bit Source Port Identity of the PTP node.
    #[inline(always)]
    pub const fn macspi2r(&self) -> &MACSPI2R {
        &self.macspi2r
    }
    ///0xbd0 - This register contains the periodic intervals for automatic PTP packet generation.
    #[inline(always)]
    pub const fn maclmir(&self) -> &MACLMIR {
        &self.maclmir
    }
}
/**MACCR (rw) register accessor: The MAC Configuration Register establishes the operating mode of the MAC.

You can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACCR)

For information about available fields see [`mod@maccr`] module*/
pub type MACCR = crate::Reg<maccr::MACCRrs>;
///The MAC Configuration Register establishes the operating mode of the MAC.
pub mod maccr;
/**MACECR (rw) register accessor: The MAC Extended Configuration Register establishes the operating mode of the MAC.

You can [`read`](crate::Reg::read) this register and get [`macecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACECR)

For information about available fields see [`mod@macecr`] module*/
pub type MACECR = crate::Reg<macecr::MACECRrs>;
///The MAC Extended Configuration Register establishes the operating mode of the MAC.
pub mod macecr;
/**MACPFR (rw) register accessor: The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.

You can [`read`](crate::Reg::read) this register and get [`macpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPFR)

For information about available fields see [`mod@macpfr`] module*/
pub type MACPFR = crate::Reg<macpfr::MACPFRrs>;
///The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.
pub mod macpfr;
/**MACWTR (rw) register accessor: The Watchdog Timeout register controls the watchdog timeout for received packets.

You can [`read`](crate::Reg::read) this register and get [`macwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACWTR)

For information about available fields see [`mod@macwtr`] module*/
pub type MACWTR = crate::Reg<macwtr::MACWTRrs>;
///The Watchdog Timeout register controls the watchdog timeout for received packets.
pub mod macwtr;
/**MACHT0R (rw) register accessor: The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.

You can [`read`](crate::Reg::read) this register and get [`macht0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHT0R)

For information about available fields see [`mod@macht0r`] module*/
pub type MACHT0R = crate::Reg<macht0r::MACHT0Rrs>;
///The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.
pub mod macht0r;
/**MACHT1R (rw) register accessor: The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.

You can [`read`](crate::Reg::read) this register and get [`macht1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHT1R)

For information about available fields see [`mod@macht1r`] module*/
pub type MACHT1R = crate::Reg<macht1r::MACHT1Rrs>;
///The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.
pub mod macht1r;
/**MACVTR (rw) register accessor: The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.

You can [`read`](crate::Reg::read) this register and get [`macvtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVTR)

For information about available fields see [`mod@macvtr`] module*/
pub type MACVTR = crate::Reg<macvtr::MACVTRrs>;
///The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.
pub mod macvtr;
/**MACVHTR (rw) register accessor: When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of this register are written.

You can [`read`](crate::Reg::read) this register and get [`macvhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVHTR)

For information about available fields see [`mod@macvhtr`] module*/
pub type MACVHTR = crate::Reg<macvhtr::MACVHTRrs>;
///When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of this register are written.
pub mod macvhtr;
/**MACVIR (rw) register accessor: The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.

You can [`read`](crate::Reg::read) this register and get [`macvir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVIR)

For information about available fields see [`mod@macvir`] module*/
pub type MACVIR = crate::Reg<macvir::MACVIRrs>;
///The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.
pub mod macvir;
/**MACIVIR (rw) register accessor: The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.

You can [`read`](crate::Reg::read) this register and get [`macivir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macivir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACIVIR)

For information about available fields see [`mod@macivir`] module*/
pub type MACIVIR = crate::Reg<macivir::MACIVIRrs>;
///The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.
pub mod macivir;
/**MACQ0TxFCR (rw) register accessor: The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.

You can [`read`](crate::Reg::read) this register and get [`macq0tx_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macq0tx_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACQ0TxFCR)

For information about available fields see [`mod@macq0tx_fcr`] module*/
#[doc(alias = "MACQ0TxFCR")]
pub type MACQ0TX_FCR = crate::Reg<macq0tx_fcr::MACQ0TX_FCRrs>;
///The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.
pub mod macq0tx_fcr;
/**MACRxFCR (rw) register accessor: The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.

You can [`read`](crate::Reg::read) this register and get [`macrx_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxFCR)

For information about available fields see [`mod@macrx_fcr`] module*/
#[doc(alias = "MACRxFCR")]
pub type MACRX_FCR = crate::Reg<macrx_fcr::MACRX_FCRrs>;
///The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.
pub mod macrx_fcr;
/**MACTxQPMR (r) register accessor: The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.

You can [`read`](crate::Reg::read) this register and get [`mactx_qpmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTxQPMR)

For information about available fields see [`mod@mactx_qpmr`] module*/
#[doc(alias = "MACTxQPMR")]
pub type MACTX_QPMR = crate::Reg<mactx_qpmr::MACTX_QPMRrs>;
///The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.
pub mod mactx_qpmr;
/**MACRxQC0R (rw) register accessor: The Receive Queue Control 0 register controls the queue management in the MAC Receiver.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxQC0R)

For information about available fields see [`mod@macrx_qc0r`] module*/
#[doc(alias = "MACRxQC0R")]
pub type MACRX_QC0R = crate::Reg<macrx_qc0r::MACRX_QC0Rrs>;
///The Receive Queue Control 0 register controls the queue management in the MAC Receiver.
pub mod macrx_qc0r;
/**MACRxQC1R (rw) register accessor: The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxQC1R)

For information about available fields see [`mod@macrx_qc1r`] module*/
#[doc(alias = "MACRxQC1R")]
pub type MACRX_QC1R = crate::Reg<macrx_qc1r::MACRX_QC1Rrs>;
///The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.
pub mod macrx_qc1r;
/**MACRxQC2R (rw) register accessor: This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxQC2R)

For information about available fields see [`mod@macrx_qc2r`] module*/
#[doc(alias = "MACRxQC2R")]
pub type MACRX_QC2R = crate::Reg<macrx_qc2r::MACRX_QC2Rrs>;
///This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.
pub mod macrx_qc2r;
/**MACISR (r) register accessor: The Interrupt Status register contains the status of interrupts.

You can [`read`](crate::Reg::read) this register and get [`macisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACISR)

For information about available fields see [`mod@macisr`] module*/
pub type MACISR = crate::Reg<macisr::MACISRrs>;
///The Interrupt Status register contains the status of interrupts.
pub mod macisr;
/**MACIER (rw) register accessor: The Interrupt Enable register contains the masks for generating the interrupts.

You can [`read`](crate::Reg::read) this register and get [`macier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACIER)

For information about available fields see [`mod@macier`] module*/
pub type MACIER = crate::Reg<macier::MACIERrs>;
///The Interrupt Enable register contains the masks for generating the interrupts.
pub mod macier;
/**MACRxTxSR (r) register accessor: The Receive Transmit Status register contains the Receive and Transmit Error status.

You can [`read`](crate::Reg::read) this register and get [`macrx_tx_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxTxSR)

For information about available fields see [`mod@macrx_tx_sr`] module*/
#[doc(alias = "MACRxTxSR")]
pub type MACRX_TX_SR = crate::Reg<macrx_tx_sr::MACRX_TX_SRrs>;
///The Receive Transmit Status register contains the Receive and Transmit Error status.
pub mod macrx_tx_sr;
/**MACPCSR (rw) register accessor: The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.

You can [`read`](crate::Reg::read) this register and get [`macpcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPCSR)

For information about available fields see [`mod@macpcsr`] module*/
pub type MACPCSR = crate::Reg<macpcsr::MACPCSRrs>;
///The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.
pub mod macpcsr;
/**MACRWKPFR (rw) register accessor: The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.

You can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRWKPFR)

For information about available fields see [`mod@macrwkpfr`] module*/
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFRrs>;
///The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
pub mod macrwkpfr;
/**MACLCSR (rw) register accessor: The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.

You can [`read`](crate::Reg::read) this register and get [`maclcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACLCSR)

For information about available fields see [`mod@maclcsr`] module*/
pub type MACLCSR = crate::Reg<maclcsr::MACLCSRrs>;
///The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.
pub mod maclcsr;
/**MACLTCR (rw) register accessor: The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.

You can [`read`](crate::Reg::read) this register and get [`macltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACLTCR)

For information about available fields see [`mod@macltcr`] module*/
pub type MACLTCR = crate::Reg<macltcr::MACLTCRrs>;
///The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.
pub mod macltcr;
/**MACLETR (rw) register accessor: The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.

You can [`read`](crate::Reg::read) this register and get [`macletr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACLETR)

For information about available fields see [`mod@macletr`] module*/
pub type MACLETR = crate::Reg<macletr::MACLETRrs>;
///The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.
pub mod macletr;
/**MAC1USTCR (rw) register accessor: This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.

You can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MAC1USTCR)

For information about available fields see [`mod@mac1ustcr`] module*/
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCRrs>;
///This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.
pub mod mac1ustcr;
/**MACPHYCSR (rw) register accessor: The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.

You can [`read`](crate::Reg::read) this register and get [`macphycsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macphycsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPHYCSR)

For information about available fields see [`mod@macphycsr`] module*/
pub type MACPHYCSR = crate::Reg<macphycsr::MACPHYCSRrs>;
///The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.
pub mod macphycsr;
/**MACVR (r) register accessor: The version register identifies the version of the Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`macvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVR)

For information about available fields see [`mod@macvr`] module*/
pub type MACVR = crate::Reg<macvr::MACVRrs>;
///The version register identifies the version of the Ethernet peripheral.
pub mod macvr;
/**MACDR (r) register accessor: The Debug register provides the debug status of various MAC blocks.

You can [`read`](crate::Reg::read) this register and get [`macdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACDR)

For information about available fields see [`mod@macdr`] module*/
pub type MACDR = crate::Reg<macdr::MACDRrs>;
///The Debug register provides the debug status of various MAC blocks.
pub mod macdr;
/**MACHWF1R (r) register accessor: This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.

You can [`read`](crate::Reg::read) this register and get [`machwf1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHWF1R)

For information about available fields see [`mod@machwf1r`] module*/
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1Rrs>;
///This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
pub mod machwf1r;
/**MACHWF2R (r) register accessor: This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.

You can [`read`](crate::Reg::read) this register and get [`machwf2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHWF2R)

For information about available fields see [`mod@machwf2r`] module*/
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2Rrs>;
///This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.
pub mod machwf2r;
/**MACMDIOAR (rw) register accessor: The MDIO Address register controls the management cycles to external PHY through a management interface.

You can [`read`](crate::Reg::read) this register and get [`macmdioar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACMDIOAR)

For information about available fields see [`mod@macmdioar`] module*/
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOARrs>;
///The MDIO Address register controls the management cycles to external PHY through a management interface.
pub mod macmdioar;
/**MACMDIODR (rw) register accessor: The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.

You can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACMDIODR)

For information about available fields see [`mod@macmdiodr`] module*/
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODRrs>;
///The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.
pub mod macmdiodr;
/**MACA0HR (rw) register accessor: The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\] is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.

You can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA0HR)

For information about available fields see [`mod@maca0hr`] module*/
pub type MACA0HR = crate::Reg<maca0hr::MACA0HRrs>;
///The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \[7:0\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \[47:0\] is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.
pub mod maca0hr;
/**MACA0LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA0LR)

For information about available fields see [`mod@maca0lr`] module*/
pub type MACA0LR = crate::Reg<maca0lr::MACA0LRrs>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod maca0lr;
/**MACA1HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA1HR)

For information about available fields see [`mod@maca1hr`] module*/
pub type MACA1HR = crate::Reg<maca1hr::MACA1HRrs>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod maca1hr;
/**MACA1LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA1LR)

For information about available fields see [`mod@maca1lr`] module*/
pub type MACA1LR = crate::Reg<maca1lr::MACA1LRrs>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod maca1lr;
/**MACA2HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA2HR)

For information about available fields see [`mod@maca2hr`] module*/
pub type MACA2HR = crate::Reg<maca2hr::MACA2HRrs>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod maca2hr;
/**MACA2LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA2LR)

For information about available fields see [`mod@maca2lr`] module*/
pub type MACA2LR = crate::Reg<maca2lr::MACA2LRrs>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod maca2lr;
/**MACA3HR (rw) register accessor: The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA3HR)

For information about available fields see [`mod@maca3hr`] module*/
pub type MACA3HR = crate::Reg<maca3hr::MACA3HRrs>;
///The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.
pub mod maca3hr;
/**MACA3LR (rw) register accessor: The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA3LR)

For information about available fields see [`mod@maca3lr`] module*/
pub type MACA3LR = crate::Reg<maca3lr::MACA3LRrs>;
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
pub mod maca3lr;
/**MMC_CONTROL (rw) register accessor: This register configures the MMC operating mode.

You can [`read`](crate::Reg::read) this register and get [`mmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MMC_CONTROL)

For information about available fields see [`mod@mmc_control`] module*/
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROLrs>;
///This register configures the MMC operating mode.
pub mod mmc_control;
/**MMC_RX_INTERRUPT (r) register accessor: This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MMC_RX_INTERRUPT)

For information about available fields see [`mod@mmc_rx_interrupt`] module*/
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPTrs>;
///This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
pub mod mmc_rx_interrupt;
/**MMC_TX_INTERRUPT (r) register accessor: This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MMC_TX_INTERRUPT)

For information about available fields see [`mod@mmc_tx_interrupt`] module*/
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPTrs>;
///This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.
pub mod mmc_tx_interrupt;
/**MMC_RX_INTERRUPT_MASK (rw) register accessor: The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MMC_RX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_rx_interrupt_mask`] module*/
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASKrs>;
///The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.
pub mod mmc_rx_interrupt_mask;
/**MMC_TX_INTERRUPT_MASK (rw) register accessor: This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MMC_TX_INTERRUPT_MASK)

For information about available fields see [`mod@mmc_tx_interrupt_mask`] module*/
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASKrs>;
///This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.
pub mod mmc_tx_interrupt_mask;
/**TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.

You can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:TX_SINGLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_single_collision_good_packets`] module*/
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
///This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.
pub mod tx_single_collision_good_packets;
/**TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.

You can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:TX_MULTIPLE_COLLISION_GOOD_PACKETS)

For information about available fields see [`mod@tx_multiple_collision_good_packets`] module*/
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETSrs>;
///This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.
pub mod tx_multiple_collision_good_packets;
/**TX_PACKET_COUNT_GOOD (r) register accessor: This register provides the number of good packets transmitted by Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:TX_PACKET_COUNT_GOOD)

For information about available fields see [`mod@tx_packet_count_good`] module*/
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOODrs>;
///This register provides the number of good packets transmitted by Ethernet peripheral.
pub mod tx_packet_count_good;
/**RX_CRC_ERROR_PACKETS (r) register accessor: This register provides the number of packets received by Ethernet peripheral with CRC error.

You can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:RX_CRC_ERROR_PACKETS)

For information about available fields see [`mod@rx_crc_error_packets`] module*/
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETSrs>;
///This register provides the number of packets received by Ethernet peripheral with CRC error.
pub mod rx_crc_error_packets;
/**RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.

You can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:RX_ALIGNMENT_ERROR_PACKETS)

For information about available fields see [`mod@rx_alignment_error_packets`] module*/
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETSrs>;
///This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.
pub mod rx_alignment_error_packets;
/**RX_UNICAST_PACKETS_GOOD (r) register accessor: This register provides the number of good unicast packets received by Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:RX_UNICAST_PACKETS_GOOD)

For information about available fields see [`mod@rx_unicast_packets_good`] module*/
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOODrs>;
///This register provides the number of good unicast packets received by Ethernet peripheral.
pub mod rx_unicast_packets_good;
/**TX_LPI_USEC_CNTR (r) register accessor: This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:TX_LPI_USEC_CNTR)

For information about available fields see [`mod@tx_lpi_usec_cntr`] module*/
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTRrs>;
///This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.
pub mod tx_lpi_usec_cntr;
/**TX_LPI_TRAN_CNTR (r) register accessor: This register provides the number of times Ethernet peripheral has entered Tx LPI.

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:TX_LPI_TRAN_CNTR)

For information about available fields see [`mod@tx_lpi_tran_cntr`] module*/
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTRrs>;
///This register provides the number of times Ethernet peripheral has entered Tx LPI.
pub mod tx_lpi_tran_cntr;
/**RX_LPI_USEC_CNTR (r) register accessor: This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:RX_LPI_USEC_CNTR)

For information about available fields see [`mod@rx_lpi_usec_cntr`] module*/
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTRrs>;
///This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.
pub mod rx_lpi_usec_cntr;
/**RX_LPI_TRAN_CNTR (r) register accessor: This register provides the number of times Ethernet peripheral has entered Rx LPI.

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:RX_LPI_TRAN_CNTR)

For information about available fields see [`mod@rx_lpi_tran_cntr`] module*/
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTRrs>;
///This register provides the number of times Ethernet peripheral has entered Rx LPI.
pub mod rx_lpi_tran_cntr;
/**MACL3L4C0R (rw) register accessor: The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.

You can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3L4C0R)

For information about available fields see [`mod@macl3l4c0r`] module*/
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0Rrs>;
///The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.
pub mod macl3l4c0r;
/**MACL4A0R (rw) register accessor: Layer4 address filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL4A0R)

For information about available fields see [`mod@macl4a0r`] module*/
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0Rrs>;
///Layer4 address filter 0 register
pub mod macl4a0r;
/**MACL3A00R (rw) register accessor: For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A00R)

For information about available fields see [`mod@macl3a00r`] module*/
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00Rrs>;
///For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.
pub mod macl3a00r;
/**MACL3A10R (rw) register accessor: For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A10R)

For information about available fields see [`mod@macl3a10r`] module*/
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10Rrs>;
///For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.
pub mod macl3a10r;
/**MACL3A20 (rw) register accessor: The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A20)

For information about available fields see [`mod@macl3a20`] module*/
pub type MACL3A20 = crate::Reg<macl3a20::MACL3A20rs>;
///The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.
pub mod macl3a20;
/**MACL3A30 (rw) register accessor: The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A30)

For information about available fields see [`mod@macl3a30`] module*/
pub type MACL3A30 = crate::Reg<macl3a30::MACL3A30rs>;
///The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.
pub mod macl3a30;
/**MACL3L4C1R (rw) register accessor: The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.

You can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3L4C1R)

For information about available fields see [`mod@macl3l4c1r`] module*/
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1Rrs>;
///The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.
pub mod macl3l4c1r;
/**MACL4A1R (rw) register accessor: The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.

You can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL4A1R)

For information about available fields see [`mod@macl4a1r`] module*/
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1Rrs>;
///The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.
pub mod macl4a1r;
/**MACL3A01R (rw) register accessor: For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A01R)

For information about available fields see [`mod@macl3a01r`] module*/
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01Rrs>;
///For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\[31:0\] of the 128-bit IP Source Address or Destination Address field.
pub mod macl3a01r;
/**MACL3A11R (rw) register accessor: For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A11R)

For information about available fields see [`mod@macl3a11r`] module*/
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11Rrs>;
///For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.
pub mod macl3a11r;
/**MACL3A21R (rw) register accessor: The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A21R)

For information about available fields see [`mod@macl3a21r`] module*/
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21Rrs>;
///The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[95:64\] of 128-bit IP Source Address or Destination Address field.
pub mod macl3a21r;
/**MACL3A31R (rw) register accessor: The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3A31R)

For information about available fields see [`mod@macl3a31r`] module*/
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31Rrs>;
///The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\] of 128-bit IP Source Address or Destination Address field.
pub mod macl3a31r;
/**MACARPAR (rw) register accessor: The ARP Address register contains the IPv4 Destination Address of the MAC.

You can [`read`](crate::Reg::read) this register and get [`macarpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACARPAR)

For information about available fields see [`mod@macarpar`] module*/
pub type MACARPAR = crate::Reg<macarpar::MACARPARrs>;
///The ARP Address register contains the IPv4 Destination Address of the MAC.
pub mod macarpar;
/**MACTSCR (rw) register accessor: This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.

You can [`read`](crate::Reg::read) this register and get [`mactscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSCR)

For information about available fields see [`mod@mactscr`] module*/
pub type MACTSCR = crate::Reg<mactscr::MACTSCRrs>;
///This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.
pub mod mactscr;
/**MACSSIR (rw) register accessor: The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.

You can [`read`](crate::Reg::read) this register and get [`macssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSSIR)

For information about available fields see [`mod@macssir`] module*/
pub type MACSSIR = crate::Reg<macssir::MACSSIRrs>;
///The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.
pub mod macssir;
/**MACSTSR (r) register accessor: The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`macstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSTSR)

For information about available fields see [`mod@macstsr`] module*/
pub type MACSTSR = crate::Reg<macstsr::MACSTSRrs>;
///The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod macstsr;
/**MACSTNR (r) register accessor: The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`macstnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSTNR)

For information about available fields see [`mod@macstnr`] module*/
pub type MACSTNR = crate::Reg<macstnr::MACSTNRrs>;
///The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod macstnr;
/**MACSTSUR (rw) register accessor: The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`macstsur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSTSUR)

For information about available fields see [`mod@macstsur`] module*/
pub type MACSTSUR = crate::Reg<macstsur::MACSTSURrs>;
///The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.
pub mod macstsur;
/**MACSTNUR (rw) register accessor: This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`macstnur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSTNUR)

For information about available fields see [`mod@macstnur`] module*/
pub type MACSTNUR = crate::Reg<macstnur::MACSTNURrs>;
///This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.
pub mod macstnur;
/**MACTSAR (rw) register accessor: The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.

You can [`read`](crate::Reg::read) this register and get [`mactsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSAR)

For information about available fields see [`mod@mactsar`] module*/
pub type MACTSAR = crate::Reg<mactsar::MACTSARrs>;
///The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.
pub mod mactsar;
/**MACTSSR (r) register accessor: The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\[27:25\] gets cleared when the application reads this register.

You can [`read`](crate::Reg::read) this register and get [`mactssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSSR)

For information about available fields see [`mod@mactssr`] module*/
pub type MACTSSR = crate::Reg<mactssr::MACTSSRrs>;
///The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\[27:25\] gets cleared when the application reads this register.
pub mod mactssr;
/**MACTxTSSNR (r) register accessor: This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.

You can [`read`](crate::Reg::read) this register and get [`mactx_tssnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTxTSSNR)

For information about available fields see [`mod@mactx_tssnr`] module*/
#[doc(alias = "MACTxTSSNR")]
pub type MACTX_TSSNR = crate::Reg<mactx_tssnr::MACTX_TSSNRrs>;
///This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.
pub mod mactx_tssnr;
/**MACTxTSSSR (r) register accessor: The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.

You can [`read`](crate::Reg::read) this register and get [`mactx_tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTxTSSSR)

For information about available fields see [`mod@mactx_tsssr`] module*/
#[doc(alias = "MACTxTSSSR")]
pub type MACTX_TSSSR = crate::Reg<mactx_tsssr::MACTX_TSSSRrs>;
///The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.
pub mod mactx_tsssr;
/**MACACR (rw) register accessor: The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.

You can [`read`](crate::Reg::read) this register and get [`macacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACACR)

For information about available fields see [`mod@macacr`] module*/
pub type MACACR = crate::Reg<macacr::MACACRrs>;
///The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.
pub mod macacr;
/**MACATSNR (r) register accessor: The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\] in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\] are read and in big-endian mode, Bits\[7:0\] are read.

You can [`read`](crate::Reg::read) this register and get [`macatsnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACATSNR)

For information about available fields see [`mod@macatsnr`] module*/
pub type MACATSNR = crate::Reg<macatsnr::MACATSNRrs>;
///The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\] in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\] are read and in big-endian mode, Bits\[7:0\] are read.
pub mod macatsnr;
/**MACATSSR (r) register accessor: The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.

You can [`read`](crate::Reg::read) this register and get [`macatssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACATSSR)

For information about available fields see [`mod@macatssr`] module*/
pub type MACATSSR = crate::Reg<macatssr::MACATSSRrs>;
///The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.
pub mod macatssr;
/**MACTSIACR (rw) register accessor: The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.

You can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSIACR)

For information about available fields see [`mod@mactsiacr`] module*/
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACRrs>;
///The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.
pub mod mactsiacr;
/**MACTSEACR (rw) register accessor: The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.

You can [`read`](crate::Reg::read) this register and get [`mactseacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSEACR)

For information about available fields see [`mod@mactseacr`] module*/
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACRrs>;
///The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.
pub mod mactseacr;
/**MACTSICNR (rw) register accessor: This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.

You can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSICNR)

For information about available fields see [`mod@mactsicnr`] module*/
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNRrs>;
///This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.
pub mod mactsicnr;
/**MACTSECNR (rw) register accessor: This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.

You can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTSECNR)

For information about available fields see [`mod@mactsecnr`] module*/
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNRrs>;
///This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.
pub mod mactsecnr;
/**MACPPSCR (rw) register accessor: The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\] of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\] are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\] are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\] are valid only when Flexible PPS feature is selected.

You can [`read`](crate::Reg::read) this register and get [`macppscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSCR)

For information about available fields see [`mod@macppscr`] module*/
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCRrs>;
///The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\[30:24\] of this register are valid only when four Flexible PPS outputs are selected. Bits\[22:16\] are valid only when three or more Flexible PPS outputs are selected. Bits\[14:8\] are valid only when two or more Flexible PPS outputs are selected. Bits\[6:4\] are valid only when Flexible PPS feature is selected.
pub mod macppscr;
/**MACPPSTTSR (rw) register accessor: The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \[Bit 1 of ETH_MACTSSR\] when the system time exceeds the value programmed in these registers.

You can [`read`](crate::Reg::read) this register and get [`macppsttsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSTTSR)

For information about available fields see [`mod@macppsttsr`] module*/
pub type MACPPSTTSR = crate::Reg<macppsttsr::MACPPSTTSRrs>;
///The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \[Bit 1 of ETH_MACTSSR\] when the system time exceeds the value programmed in these registers.
pub mod macppsttsr;
/**MACPPSTTNR (rw) register accessor: The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.

You can [`read`](crate::Reg::read) this register and get [`macppsttnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSTTNR)

For information about available fields see [`mod@macppsttnr`] module*/
pub type MACPPSTTNR = crate::Reg<macppsttnr::MACPPSTTNRrs>;
///The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.
pub mod macppsttnr;
/**MACPPSIR (rw) register accessor: The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).

You can [`read`](crate::Reg::read) this register and get [`macppsir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSIR)

For information about available fields see [`mod@macppsir`] module*/
pub type MACPPSIR = crate::Reg<macppsir::MACPPSIRrs>;
///The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\[0\]).
pub mod macppsir;
/**MACPPSWR (rw) register accessor: The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).

You can [`read`](crate::Reg::read) this register and get [`macppswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPPSWR)

For information about available fields see [`mod@macppswr`] module*/
pub type MACPPSWR = crate::Reg<macppswr::MACPPSWRrs>;
///The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).
pub mod macppswr;
/**MACPOCR (rw) register accessor: This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.

You can [`read`](crate::Reg::read) this register and get [`macpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACPOCR)

For information about available fields see [`mod@macpocr`] module*/
pub type MACPOCR = crate::Reg<macpocr::MACPOCRrs>;
///This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod macpocr;
/**MACSPI0R (rw) register accessor: This register contains Bits\[31:0\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.

You can [`read`](crate::Reg::read) this register and get [`macspi0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSPI0R)

For information about available fields see [`mod@macspi0r`] module*/
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0Rrs>;
///This register contains Bits\[31:0\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod macspi0r;
/**MACSPI1R (rw) register accessor: This register contains Bits\[63:32\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.

You can [`read`](crate::Reg::read) this register and get [`macspi1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSPI1R)

For information about available fields see [`mod@macspi1r`] module*/
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1Rrs>;
///This register contains Bits\[63:32\] of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.
pub mod macspi1r;
/**MACSPI2R (rw) register accessor: This register contains Bits\[79:64\] of the 80-bit Source Port Identity of the PTP node.

You can [`read`](crate::Reg::read) this register and get [`macspi2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSPI2R)

For information about available fields see [`mod@macspi2r`] module*/
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2Rrs>;
///This register contains Bits\[79:64\] of the 80-bit Source Port Identity of the PTP node.
pub mod macspi2r;
/**MACLMIR (rw) register accessor: This register contains the periodic intervals for automatic PTP packet generation.

You can [`read`](crate::Reg::read) this register and get [`maclmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACLMIR)

For information about available fields see [`mod@maclmir`] module*/
pub type MACLMIR = crate::Reg<maclmir::MACLMIRrs>;
///This register contains the periodic intervals for automatic PTP packet generation.
pub mod maclmir;
