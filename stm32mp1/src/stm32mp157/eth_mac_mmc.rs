#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The MAC Configuration Register establishes the operating mode of the MAC."]
    pub eth_maccr: ETH_MACCR,
    #[doc = "0x04 - The MAC Extended Configuration Register establishes the operating mode of the MAC."]
    pub eth_macecr: ETH_MACECR,
    #[doc = "0x08 - The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets."]
    pub eth_macpfr: ETH_MACPFR,
    #[doc = "0x0c - The Watchdog Timeout register controls the watchdog timeout for received packets."]
    pub eth_macwtr: ETH_MACWTR,
    #[doc = "0x10 - The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
    pub eth_macht0r: ETH_MACHT0R,
    #[doc = "0x14 - The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
    pub eth_macht1r: ETH_MACHT1R,
    _reserved6: [u8; 56usize],
    #[doc = "0x50 - The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets."]
    pub eth_macvtr: ETH_MACVTR,
    _reserved7: [u8; 4usize],
    #[doc = "0x58 - When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written."]
    pub eth_macvhtr: ETH_MACVHTR,
    _reserved8: [u8; 4usize],
    #[doc = "0x60 - The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls."]
    pub eth_macvir: ETH_MACVIR,
    #[doc = "0x64 - The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls."]
    pub eth_macivir: ETH_MACIVIR,
    _reserved10: [u8; 8usize],
    #[doc = "0x70 - The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register."]
    pub eth_macq0tx_fcr: ETH_MACQ0TXFCR,
    _reserved11: [u8; 28usize],
    #[doc = "0x90 - The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet."]
    pub eth_macrx_fcr: ETH_MACRXFCR,
    _reserved12: [u8; 4usize],
    #[doc = "0x98 - The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1."]
    pub eth_mactx_qpmr: ETH_MACTXQPMR,
    _reserved13: [u8; 4usize],
    #[doc = "0xa0 - The Receive Queue Control 0 register controls the queue management in the MAC Receiver."]
    pub eth_macrx_qc0r: ETH_MACRXQC0R,
    #[doc = "0xa4 - The Receive Queue Control 1 register controls queue 1 management in the MAC receiver."]
    pub eth_macrx_qc1r: ETH_MACRXQC1R,
    #[doc = "0xa8 - This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1."]
    pub eth_macrx_qc2r: ETH_MACRXQC2R,
    _reserved16: [u8; 4usize],
    #[doc = "0xb0 - The Interrupt Status register contains the status of interrupts."]
    pub eth_macisr: ETH_MACISR,
    #[doc = "0xb4 - The Interrupt Enable register contains the masks for generating the interrupts."]
    pub eth_macier: ETH_MACIER,
    #[doc = "0xb8 - The Receive Transmit Status register contains the Receive and Transmit Error status."]
    pub eth_macrx_tx_sr: ETH_MACRXTXSR,
    _reserved19: [u8; 4usize],
    #[doc = "0xc0 - The PMT Control and Status Register is present only when you select the PMT module in coreConsultant."]
    pub eth_macpcsr: ETH_MACPCSR,
    #[doc = "0xc4 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
    pub eth_macrwkpfr: ETH_MACRWKPFR,
    _reserved21: [u8; 8usize],
    #[doc = "0xd0 - The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
    pub eth_maclcsr: ETH_MACLCSR,
    #[doc = "0xd4 - The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission."]
    pub eth_macltcr: ETH_MACLTCR,
    #[doc = "0xd8 - The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds."]
    pub eth_macletr: ETH_MACLETR,
    #[doc = "0xdc - This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially."]
    pub eth_mac1ustcr: ETH_MAC1USTCR,
    _reserved25: [u8; 24usize],
    #[doc = "0xf8 - The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY."]
    pub eth_macphycsr: ETH_MACPHYCSR,
    _reserved26: [u8; 20usize],
    #[doc = "0x110 - The version register identifies the version of the Ethernet peripheral."]
    pub eth_macvr: ETH_MACVR,
    #[doc = "0x114 - The Debug register provides the debug status of various MAC blocks."]
    pub eth_macdr: ETH_MACDR,
    _reserved28: [u8; 8usize],
    #[doc = "0x120 - This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
    pub eth_machwf1r: ETH_MACHWF1R,
    #[doc = "0x124 - This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
    pub eth_machwf2r: ETH_MACHWF2R,
    _reserved30: [u8; 216usize],
    #[doc = "0x200 - The MDIO Address register controls the management cycles to external PHY through a management interface."]
    pub eth_macmdioar: ETH_MACMDIOAR,
    #[doc = "0x204 - The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register."]
    pub eth_macmdiodr: ETH_MACMDIODR,
    _reserved32: [u8; 248usize],
    #[doc = "0x300 - The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain."]
    pub eth_maca0hr: ETH_MACA0HR,
    #[doc = "0x304 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    pub eth_maca0lr: ETH_MACA0LR,
    #[doc = "0x308 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    pub eth_maca1hr: ETH_MACA1HR,
    #[doc = "0x30c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    pub eth_maca1lr: ETH_MACA1LR,
    #[doc = "0x310 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    pub eth_maca2hr: ETH_MACA2HR,
    #[doc = "0x314 - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    pub eth_maca2lr: ETH_MACA2LR,
    #[doc = "0x318 - The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
    pub eth_maca3hr: ETH_MACA3HR,
    #[doc = "0x31c - The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
    pub eth_maca3lr: ETH_MACA3LR,
    _reserved40: [u8; 992usize],
    #[doc = "0x700 - This register configures the MMC operating mode."]
    pub mmc_control: MMC_CONTROL,
    #[doc = "0x704 - This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
    pub mmc_rx_interrupt: MMC_RX_INTERRUPT,
    #[doc = "0x708 - This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
    pub mmc_tx_interrupt: MMC_TX_INTERRUPT,
    #[doc = "0x70c - The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values."]
    pub mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    #[doc = "0x710 - This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration."]
    pub mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved45: [u8; 56usize],
    #[doc = "0x74c - This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode."]
    pub tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    #[doc = "0x750 - This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode."]
    pub tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved47: [u8; 20usize],
    #[doc = "0x768 - This register provides the number of good packets transmitted by Ethernet peripheral."]
    pub tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved48: [u8; 40usize],
    #[doc = "0x794 - This register provides the number of packets received by Ethernet peripheral with CRC error."]
    pub rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    #[doc = "0x798 - This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode."]
    pub rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved50: [u8; 40usize],
    #[doc = "0x7c4 - This register provides the number of good unicast packets received by Ethernet peripheral."]
    pub rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved51: [u8; 36usize],
    #[doc = "0x7ec - This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral."]
    pub tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    #[doc = "0x7f0 - This register provides the number of times Ethernet peripheral has entered Tx LPI."]
    pub tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    #[doc = "0x7f4 - This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral."]
    pub rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    #[doc = "0x7f8 - This register provides the number of times Ethernet peripheral has entered Rx LPI."]
    pub rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved55: [u8; 260usize],
    #[doc = "0x900 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration."]
    pub eth_macl3l4c0r: ETH_MACL3L4C0R,
    #[doc = "0x904 - Layer4 address filter 0 register"]
    pub eth_macl4a0r: ETH_MACL4A0R,
    _reserved57: [u8; 8usize],
    #[doc = "0x910 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a00r: ETH_MACL3A00R,
    #[doc = "0x914 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a10r: ETH_MACL3A10R,
    #[doc = "0x918 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a20: ETH_MACL3A20,
    #[doc = "0x91c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a30: ETH_MACL3A30,
    _reserved61: [u8; 16usize],
    #[doc = "0x930 - The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4."]
    pub eth_macl3l4c1r: ETH_MACL3L4C1R,
    #[doc = "0x934 - The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock."]
    pub eth_macl4a1r: ETH_MACL4A1R,
    _reserved63: [u8; 8usize],
    #[doc = "0x940 - For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a01r: ETH_MACL3A01R,
    #[doc = "0x944 - For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a11r: ETH_MACL3A11R,
    #[doc = "0x948 - The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a21r: ETH_MACL3A21R,
    #[doc = "0x94c - The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
    pub eth_macl3a31r: ETH_MACL3A31R,
    _reserved67: [u8; 400usize],
    #[doc = "0xae0 - The ARP Address register contains the IPv4 Destination Address of the MAC."]
    pub eth_macarpar: ETH_MACARPAR,
    _reserved68: [u8; 28usize],
    #[doc = "0xb00 - This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver."]
    pub eth_mactscr: ETH_MACTSCR,
    #[doc = "0xb04 - The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow."]
    pub eth_macssir: ETH_MACSSIR,
    #[doc = "0xb08 - The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    pub eth_macstsr: ETH_MACSTSR,
    #[doc = "0xb0c - The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    pub eth_macstnr: ETH_MACSTNR,
    #[doc = "0xb10 - The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
    pub eth_macstsur: ETH_MACSTSUR,
    #[doc = "0xb14 - This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input."]
    pub eth_macstnur: ETH_MACSTNUR,
    #[doc = "0xb18 - The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows."]
    pub eth_mactsar: ETH_MACTSAR,
    _reserved75: [u8; 4usize],
    #[doc = "0xb20 - The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register."]
    pub eth_mactssr: ETH_MACTSSR,
    _reserved76: [u8; 12usize],
    #[doc = "0xb30 - This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled."]
    pub eth_mactx_tssnr: ETH_MACTXTSSNR,
    #[doc = "0xb34 - The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted."]
    pub eth_mactx_tsssr: ETH_MACTXTSSSR,
    _reserved78: [u8; 8usize],
    #[doc = "0xb40 - The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot."]
    pub eth_macacr: ETH_MACACR,
    _reserved79: [u8; 4usize],
    #[doc = "0xb48 - The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read."]
    pub eth_macatsnr: ETH_MACATSNR,
    #[doc = "0xb4c - The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register."]
    pub eth_macatssr: ETH_MACATSSR,
    #[doc = "0xb50 - The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages."]
    pub eth_mactsiacr: ETH_MACTSIACR,
    #[doc = "0xb54 - The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages."]
    pub eth_mactseacr: ETH_MACTSEACR,
    #[doc = "0xb58 - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path."]
    pub eth_mactsicnr: ETH_MACTSICNR,
    #[doc = "0xb5c - This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path."]
    pub eth_mactsecnr: ETH_MACTSECNR,
    _reserved85: [u8; 16usize],
    #[doc = "0xb70 - The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected."]
    pub eth_macppscr: ETH_MACPPSCR,
    _reserved86: [u8; 12usize],
    #[doc = "0xb80 - The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers."]
    pub eth_macppsttsr: ETH_MACPPSTTSR,
    #[doc = "0xb84 - The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected."]
    pub eth_macppsttnr: ETH_MACPPSTTNR,
    #[doc = "0xb88 - The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\])."]
    pub eth_macppsir: ETH_MACPPSIR,
    #[doc = "0xb8c - The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o)."]
    pub eth_macppswr: ETH_MACPPSWR,
    _reserved90: [u8; 48usize],
    #[doc = "0xbc0 - This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    pub eth_macpocr: ETH_MACPOCR,
    #[doc = "0xbc4 - This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    pub eth_macspi0r: ETH_MACSPI0R,
    #[doc = "0xbc8 - This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
    pub eth_macspi1r: ETH_MACSPI1R,
    #[doc = "0xbcc - This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node."]
    pub eth_macspi2r: ETH_MACSPI2R,
    #[doc = "0xbd0 - This register contains the periodic intervals for automatic PTP packet generation."]
    pub eth_maclmir: ETH_MACLMIR,
}
#[doc = "The MAC Configuration Register establishes the operating mode of the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maccr](eth_maccr) module"]
pub type ETH_MACCR = crate::Reg<u32, _ETH_MACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACCR;
#[doc = "`read()` method returns [eth_maccr::R](eth_maccr::R) reader structure"]
impl crate::Readable for ETH_MACCR {}
#[doc = "`write(|w| ..)` method takes [eth_maccr::W](eth_maccr::W) writer structure"]
impl crate::Writable for ETH_MACCR {}
#[doc = "The MAC Configuration Register establishes the operating mode of the MAC."]
pub mod eth_maccr;
#[doc = "The MAC Extended Configuration Register establishes the operating mode of the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macecr](eth_macecr) module"]
pub type ETH_MACECR = crate::Reg<u32, _ETH_MACECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACECR;
#[doc = "`read()` method returns [eth_macecr::R](eth_macecr::R) reader structure"]
impl crate::Readable for ETH_MACECR {}
#[doc = "`write(|w| ..)` method takes [eth_macecr::W](eth_macecr::W) writer structure"]
impl crate::Writable for ETH_MACECR {}
#[doc = "The MAC Extended Configuration Register establishes the operating mode of the MAC."]
pub mod eth_macecr;
#[doc = "The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpfr](eth_macpfr) module"]
pub type ETH_MACPFR = crate::Reg<u32, _ETH_MACPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPFR;
#[doc = "`read()` method returns [eth_macpfr::R](eth_macpfr::R) reader structure"]
impl crate::Readable for ETH_MACPFR {}
#[doc = "`write(|w| ..)` method takes [eth_macpfr::W](eth_macpfr::W) writer structure"]
impl crate::Writable for ETH_MACPFR {}
#[doc = "The MAC Packet Filter register contains the filter controls for receiving packets. Some of the controls from this register go to the address check block of the MAC which performs the first level of address filtering. The second level of filtering is performed on the incoming packet based on other controls such as Pass Bad Packets and Pass Control Packets."]
pub mod eth_macpfr;
#[doc = "The Watchdog Timeout register controls the watchdog timeout for received packets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macwtr](eth_macwtr) module"]
pub type ETH_MACWTR = crate::Reg<u32, _ETH_MACWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACWTR;
#[doc = "`read()` method returns [eth_macwtr::R](eth_macwtr::R) reader structure"]
impl crate::Readable for ETH_MACWTR {}
#[doc = "`write(|w| ..)` method takes [eth_macwtr::W](eth_macwtr::W) writer structure"]
impl crate::Writable for ETH_MACWTR {}
#[doc = "The Watchdog Timeout register controls the watchdog timeout for received packets."]
pub mod eth_macwtr;
#[doc = "The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macht0r](eth_macht0r) module"]
pub type ETH_MACHT0R = crate::Reg<u32, _ETH_MACHT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACHT0R;
#[doc = "`read()` method returns [eth_macht0r::R](eth_macht0r::R) reader structure"]
impl crate::Readable for ETH_MACHT0R {}
#[doc = "`write(|w| ..)` method takes [eth_macht0r::W](eth_macht0r::W) writer structure"]
impl crate::Writable for ETH_MACHT0R {}
#[doc = "The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
pub mod eth_macht0r;
#[doc = "The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macht1r](eth_macht1r) module"]
pub type ETH_MACHT1R = crate::Reg<u32, _ETH_MACHT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACHT1R;
#[doc = "`read()` method returns [eth_macht1r::R](eth_macht1r::R) reader structure"]
impl crate::Readable for ETH_MACHT1R {}
#[doc = "`write(|w| ..)` method takes [eth_macht1r::W](eth_macht1r::W) writer structure"]
impl crate::Writable for ETH_MACHT1R {}
#[doc = "The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written."]
pub mod eth_macht1r;
#[doc = "The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvtr](eth_macvtr) module"]
pub type ETH_MACVTR = crate::Reg<u32, _ETH_MACVTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACVTR;
#[doc = "`read()` method returns [eth_macvtr::R](eth_macvtr::R) reader structure"]
impl crate::Readable for ETH_MACVTR {}
#[doc = "`write(|w| ..)` method takes [eth_macvtr::W](eth_macvtr::W) writer structure"]
impl crate::Writable for ETH_MACVTR {}
#[doc = "The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets."]
pub mod eth_macvtr;
#[doc = "When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvhtr](eth_macvhtr) module"]
pub type ETH_MACVHTR = crate::Reg<u32, _ETH_MACVHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACVHTR;
#[doc = "`read()` method returns [eth_macvhtr::R](eth_macvhtr::R) reader structure"]
impl crate::Readable for ETH_MACVHTR {}
#[doc = "`write(|w| ..)` method takes [eth_macvhtr::W](eth_macvhtr::W) writer structure"]
impl crate::Writable for ETH_MACVHTR {}
#[doc = "When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written."]
pub mod eth_macvhtr;
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvir](eth_macvir) module"]
pub type ETH_MACVIR = crate::Reg<u32, _ETH_MACVIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACVIR;
#[doc = "`read()` method returns [eth_macvir::R](eth_macvir::R) reader structure"]
impl crate::Readable for ETH_MACVIR {}
#[doc = "`write(|w| ..)` method takes [eth_macvir::W](eth_macvir::W) writer structure"]
impl crate::Writable for ETH_MACVIR {}
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls."]
pub mod eth_macvir;
#[doc = "The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macivir](eth_macivir) module"]
pub type ETH_MACIVIR = crate::Reg<u32, _ETH_MACIVIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACIVIR;
#[doc = "`read()` method returns [eth_macivir::R](eth_macivir::R) reader structure"]
impl crate::Readable for ETH_MACIVIR {}
#[doc = "`write(|w| ..)` method takes [eth_macivir::W](eth_macivir::W) writer structure"]
impl crate::Writable for ETH_MACIVIR {}
#[doc = "The Inner VLAN Tag Inclusion or Replacement register contains the inner VLAN tag to be inserted or replaced in the Transmit packet. It also contains the inner VLAN tag insertion controls."]
pub mod eth_macivir;
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macq0tx_fcr](eth_macq0tx_fcr) module"]
pub type ETH_MACQ0TXFCR = crate::Reg<u32, _ETH_MACQ0TXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACQ0TXFCR;
#[doc = "`read()` method returns [eth_macq0tx_fcr::R](eth_macq0tx_fcr::R) reader structure"]
impl crate::Readable for ETH_MACQ0TXFCR {}
#[doc = "`write(|w| ..)` method takes [eth_macq0tx_fcr::W](eth_macq0tx_fcr::W) writer structure"]
impl crate::Writable for ETH_MACQ0TXFCR {}
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register."]
pub mod eth_macq0tx_fcr;
#[doc = "The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_fcr](eth_macrx_fcr) module"]
pub type ETH_MACRXFCR = crate::Reg<u32, _ETH_MACRXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRXFCR;
#[doc = "`read()` method returns [eth_macrx_fcr::R](eth_macrx_fcr::R) reader structure"]
impl crate::Readable for ETH_MACRXFCR {}
#[doc = "`write(|w| ..)` method takes [eth_macrx_fcr::W](eth_macrx_fcr::W) writer structure"]
impl crate::Writable for ETH_MACRXFCR {}
#[doc = "The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet."]
pub mod eth_macrx_fcr;
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_qpmr](eth_mactx_qpmr) module"]
pub type ETH_MACTXQPMR = crate::Reg<u32, _ETH_MACTXQPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTXQPMR;
#[doc = "`read()` method returns [eth_mactx_qpmr::R](eth_mactx_qpmr::R) reader structure"]
impl crate::Readable for ETH_MACTXQPMR {}
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1."]
pub mod eth_mactx_qpmr;
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc0r](eth_macrx_qc0r) module"]
pub type ETH_MACRXQC0R = crate::Reg<u32, _ETH_MACRXQC0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRXQC0R;
#[doc = "`read()` method returns [eth_macrx_qc0r::R](eth_macrx_qc0r::R) reader structure"]
impl crate::Readable for ETH_MACRXQC0R {}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc0r::W](eth_macrx_qc0r::W) writer structure"]
impl crate::Writable for ETH_MACRXQC0R {}
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver."]
pub mod eth_macrx_qc0r;
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc1r](eth_macrx_qc1r) module"]
pub type ETH_MACRXQC1R = crate::Reg<u32, _ETH_MACRXQC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRXQC1R;
#[doc = "`read()` method returns [eth_macrx_qc1r::R](eth_macrx_qc1r::R) reader structure"]
impl crate::Readable for ETH_MACRXQC1R {}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc1r::W](eth_macrx_qc1r::W) writer structure"]
impl crate::Writable for ETH_MACRXQC1R {}
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver."]
pub mod eth_macrx_qc1r;
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc2r](eth_macrx_qc2r) module"]
pub type ETH_MACRXQC2R = crate::Reg<u32, _ETH_MACRXQC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRXQC2R;
#[doc = "`read()` method returns [eth_macrx_qc2r::R](eth_macrx_qc2r::R) reader structure"]
impl crate::Readable for ETH_MACRXQC2R {}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc2r::W](eth_macrx_qc2r::W) writer structure"]
impl crate::Writable for ETH_MACRXQC2R {}
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1."]
pub mod eth_macrx_qc2r;
#[doc = "The Interrupt Status register contains the status of interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macisr](eth_macisr) module"]
pub type ETH_MACISR = crate::Reg<u32, _ETH_MACISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACISR;
#[doc = "`read()` method returns [eth_macisr::R](eth_macisr::R) reader structure"]
impl crate::Readable for ETH_MACISR {}
#[doc = "The Interrupt Status register contains the status of interrupts."]
pub mod eth_macisr;
#[doc = "The Interrupt Enable register contains the masks for generating the interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macier](eth_macier) module"]
pub type ETH_MACIER = crate::Reg<u32, _ETH_MACIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACIER;
#[doc = "`read()` method returns [eth_macier::R](eth_macier::R) reader structure"]
impl crate::Readable for ETH_MACIER {}
#[doc = "`write(|w| ..)` method takes [eth_macier::W](eth_macier::W) writer structure"]
impl crate::Writable for ETH_MACIER {}
#[doc = "The Interrupt Enable register contains the masks for generating the interrupts."]
pub mod eth_macier;
#[doc = "The Receive Transmit Status register contains the Receive and Transmit Error status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_tx_sr](eth_macrx_tx_sr) module"]
pub type ETH_MACRXTXSR = crate::Reg<u32, _ETH_MACRXTXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRXTXSR;
#[doc = "`read()` method returns [eth_macrx_tx_sr::R](eth_macrx_tx_sr::R) reader structure"]
impl crate::Readable for ETH_MACRXTXSR {}
#[doc = "The Receive Transmit Status register contains the Receive and Transmit Error status."]
pub mod eth_macrx_tx_sr;
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpcsr](eth_macpcsr) module"]
pub type ETH_MACPCSR = crate::Reg<u32, _ETH_MACPCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPCSR;
#[doc = "`read()` method returns [eth_macpcsr::R](eth_macpcsr::R) reader structure"]
impl crate::Readable for ETH_MACPCSR {}
#[doc = "`write(|w| ..)` method takes [eth_macpcsr::W](eth_macpcsr::W) writer structure"]
impl crate::Writable for ETH_MACPCSR {}
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant."]
pub mod eth_macpcsr;
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrwkpfr](eth_macrwkpfr) module"]
pub type ETH_MACRWKPFR = crate::Reg<u32, _ETH_MACRWKPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACRWKPFR;
#[doc = "`read()` method returns [eth_macrwkpfr::R](eth_macrwkpfr::R) reader structure"]
impl crate::Readable for ETH_MACRWKPFR {}
#[doc = "`write(|w| ..)` method takes [eth_macrwkpfr::W](eth_macrwkpfr::W) writer structure"]
impl crate::Writable for ETH_MACRWKPFR {}
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
pub mod eth_macrwkpfr;
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclcsr](eth_maclcsr) module"]
pub type ETH_MACLCSR = crate::Reg<u32, _ETH_MACLCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACLCSR;
#[doc = "`read()` method returns [eth_maclcsr::R](eth_maclcsr::R) reader structure"]
impl crate::Readable for ETH_MACLCSR {}
#[doc = "`write(|w| ..)` method takes [eth_maclcsr::W](eth_maclcsr::W) writer structure"]
impl crate::Writable for ETH_MACLCSR {}
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read."]
pub mod eth_maclcsr;
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macltcr](eth_macltcr) module"]
pub type ETH_MACLTCR = crate::Reg<u32, _ETH_MACLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACLTCR;
#[doc = "`read()` method returns [eth_macltcr::R](eth_macltcr::R) reader structure"]
impl crate::Readable for ETH_MACLTCR {}
#[doc = "`write(|w| ..)` method takes [eth_macltcr::W](eth_macltcr::W) writer structure"]
impl crate::Writable for ETH_MACLTCR {}
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission."]
pub mod eth_macltcr;
#[doc = "The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macletr](eth_macletr) module"]
pub type ETH_MACLETR = crate::Reg<u32, _ETH_MACLETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACLETR;
#[doc = "`read()` method returns [eth_macletr::R](eth_macletr::R) reader structure"]
impl crate::Readable for ETH_MACLETR {}
#[doc = "`write(|w| ..)` method takes [eth_macletr::W](eth_macletr::W) writer structure"]
impl crate::Writable for ETH_MACLETR {}
#[doc = "The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds."]
pub mod eth_macletr;
#[doc = "This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mac1ustcr](eth_mac1ustcr) module"]
pub type ETH_MAC1USTCR = crate::Reg<u32, _ETH_MAC1USTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MAC1USTCR;
#[doc = "`read()` method returns [eth_mac1ustcr::R](eth_mac1ustcr::R) reader structure"]
impl crate::Readable for ETH_MAC1USTCR {}
#[doc = "`write(|w| ..)` method takes [eth_mac1ustcr::W](eth_mac1ustcr::W) writer structure"]
impl crate::Writable for ETH_MAC1USTCR {}
#[doc = "This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially."]
pub mod eth_mac1ustcr;
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macphycsr](eth_macphycsr) module"]
pub type ETH_MACPHYCSR = crate::Reg<u32, _ETH_MACPHYCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPHYCSR;
#[doc = "`read()` method returns [eth_macphycsr::R](eth_macphycsr::R) reader structure"]
impl crate::Readable for ETH_MACPHYCSR {}
#[doc = "`write(|w| ..)` method takes [eth_macphycsr::W](eth_macphycsr::W) writer structure"]
impl crate::Writable for ETH_MACPHYCSR {}
#[doc = "The PHY Interface Control and Status register indicates the status signals received by the, RGMII interface from the PHY."]
pub mod eth_macphycsr;
#[doc = "The version register identifies the version of the Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvr](eth_macvr) module"]
pub type ETH_MACVR = crate::Reg<u32, _ETH_MACVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACVR;
#[doc = "`read()` method returns [eth_macvr::R](eth_macvr::R) reader structure"]
impl crate::Readable for ETH_MACVR {}
#[doc = "The version register identifies the version of the Ethernet peripheral."]
pub mod eth_macvr;
#[doc = "The Debug register provides the debug status of various MAC blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macdr](eth_macdr) module"]
pub type ETH_MACDR = crate::Reg<u32, _ETH_MACDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACDR;
#[doc = "`read()` method returns [eth_macdr::R](eth_macdr::R) reader structure"]
impl crate::Readable for ETH_MACDR {}
#[doc = "The Debug register provides the debug status of various MAC blocks."]
pub mod eth_macdr;
#[doc = "This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_machwf1r](eth_machwf1r) module"]
pub type ETH_MACHWF1R = crate::Reg<u32, _ETH_MACHWF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACHWF1R;
#[doc = "`read()` method returns [eth_machwf1r::R](eth_machwf1r::R) reader structure"]
impl crate::Readable for ETH_MACHWF1R {}
#[doc = "This register indicates the presence of second set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
pub mod eth_machwf1r;
#[doc = "This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_machwf2r](eth_machwf2r) module"]
pub type ETH_MACHWF2R = crate::Reg<u32, _ETH_MACHWF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACHWF2R;
#[doc = "`read()` method returns [eth_machwf2r::R](eth_machwf2r::R) reader structure"]
impl crate::Readable for ETH_MACHWF2R {}
#[doc = "This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks."]
pub mod eth_machwf2r;
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdioar](eth_macmdioar) module"]
pub type ETH_MACMDIOAR = crate::Reg<u32, _ETH_MACMDIOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACMDIOAR;
#[doc = "`read()` method returns [eth_macmdioar::R](eth_macmdioar::R) reader structure"]
impl crate::Readable for ETH_MACMDIOAR {}
#[doc = "`write(|w| ..)` method takes [eth_macmdioar::W](eth_macmdioar::W) writer structure"]
impl crate::Writable for ETH_MACMDIOAR {}
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface."]
pub mod eth_macmdioar;
#[doc = "The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdiodr](eth_macmdiodr) module"]
pub type ETH_MACMDIODR = crate::Reg<u32, _ETH_MACMDIODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACMDIODR;
#[doc = "`read()` method returns [eth_macmdiodr::R](eth_macmdiodr::R) reader structure"]
impl crate::Readable for ETH_MACMDIODR {}
#[doc = "`write(|w| ..)` method takes [eth_macmdiodr::W](eth_macmdiodr::W) writer structure"]
impl crate::Writable for ETH_MACMDIODR {}
#[doc = "The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register."]
pub mod eth_macmdiodr;
#[doc = "The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca0hr](eth_maca0hr) module"]
pub type ETH_MACA0HR = crate::Reg<u32, _ETH_MACA0HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA0HR;
#[doc = "`read()` method returns [eth_maca0hr::R](eth_maca0hr::R) reader structure"]
impl crate::Readable for ETH_MACA0HR {}
#[doc = "`write(|w| ..)` method takes [eth_maca0hr::W](eth_maca0hr::W) writer structure"]
impl crate::Writable for ETH_MACA0HR {}
#[doc = "The MAC Address0 High register holds the upper 16 bits of the first 6-byte MAC address of the station. The first DA byte that is received on the GMII interface corresponds to the LS byte (Bits \\[7:0\\]) of the MAC Address Low register. For example, if 0x112233445566 is received (0x11 in lane 0 of the first column) on the GMII as the destination address, then the MacAddress0 Register \\[47:0\\]
is compared with 0x665544332211. If the MAC address registers are configured to be double-synchronized to the GMII clock domains, then the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the MAC Address0 Low Register are written. For proper synchronization updates, the consecutive writes to this Address Low Register should be performed after at least four clock cycles in the destination clock domain."]
pub mod eth_maca0hr;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca0lr](eth_maca0lr) module"]
pub type ETH_MACA0LR = crate::Reg<u32, _ETH_MACA0LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA0LR;
#[doc = "`read()` method returns [eth_maca0lr::R](eth_maca0lr::R) reader structure"]
impl crate::Readable for ETH_MACA0LR {}
#[doc = "`write(|w| ..)` method takes [eth_maca0lr::W](eth_maca0lr::W) writer structure"]
impl crate::Writable for ETH_MACA0LR {}
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca0lr;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca1hr](eth_maca1hr) module"]
pub type ETH_MACA1HR = crate::Reg<u32, _ETH_MACA1HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA1HR;
#[doc = "`read()` method returns [eth_maca1hr::R](eth_maca1hr::R) reader structure"]
impl crate::Readable for ETH_MACA1HR {}
#[doc = "`write(|w| ..)` method takes [eth_maca1hr::W](eth_maca1hr::W) writer structure"]
impl crate::Writable for ETH_MACA1HR {}
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca1hr;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca1lr](eth_maca1lr) module"]
pub type ETH_MACA1LR = crate::Reg<u32, _ETH_MACA1LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA1LR;
#[doc = "`read()` method returns [eth_maca1lr::R](eth_maca1lr::R) reader structure"]
impl crate::Readable for ETH_MACA1LR {}
#[doc = "`write(|w| ..)` method takes [eth_maca1lr::W](eth_maca1lr::W) writer structure"]
impl crate::Writable for ETH_MACA1LR {}
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca1lr;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca2hr](eth_maca2hr) module"]
pub type ETH_MACA2HR = crate::Reg<u32, _ETH_MACA2HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA2HR;
#[doc = "`read()` method returns [eth_maca2hr::R](eth_maca2hr::R) reader structure"]
impl crate::Readable for ETH_MACA2HR {}
#[doc = "`write(|w| ..)` method takes [eth_maca2hr::W](eth_maca2hr::W) writer structure"]
impl crate::Writable for ETH_MACA2HR {}
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca2hr;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca2lr](eth_maca2lr) module"]
pub type ETH_MACA2LR = crate::Reg<u32, _ETH_MACA2LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA2LR;
#[doc = "`read()` method returns [eth_maca2lr::R](eth_maca2lr::R) reader structure"]
impl crate::Readable for ETH_MACA2LR {}
#[doc = "`write(|w| ..)` method takes [eth_maca2lr::W](eth_maca2lr::W) writer structure"]
impl crate::Writable for ETH_MACA2LR {}
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca2lr;
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca3hr](eth_maca3hr) module"]
pub type ETH_MACA3HR = crate::Reg<u32, _ETH_MACA3HR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA3HR;
#[doc = "`read()` method returns [eth_maca3hr::R](eth_maca3hr::R) reader structure"]
impl crate::Readable for ETH_MACA3HR {}
#[doc = "`write(|w| ..)` method takes [eth_maca3hr::W](eth_maca3hr::W) writer structure"]
impl crate::Writable for ETH_MACA3HR {}
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station."]
pub mod eth_maca3hr;
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca3lr](eth_maca3lr) module"]
pub type ETH_MACA3LR = crate::Reg<u32, _ETH_MACA3LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACA3LR;
#[doc = "`read()` method returns [eth_maca3lr::R](eth_maca3lr::R) reader structure"]
impl crate::Readable for ETH_MACA3LR {}
#[doc = "`write(|w| ..)` method takes [eth_maca3lr::W](eth_maca3lr::W) writer structure"]
impl crate::Writable for ETH_MACA3LR {}
#[doc = "The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station."]
pub mod eth_maca3lr;
#[doc = "This register configures the MMC operating mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_control](mmc_control) module"]
pub type MMC_CONTROL = crate::Reg<u32, _MMC_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_CONTROL;
#[doc = "`read()` method returns [mmc_control::R](mmc_control::R) reader structure"]
impl crate::Readable for MMC_CONTROL {}
#[doc = "`write(|w| ..)` method takes [mmc_control::W](mmc_control::W) writer structure"]
impl crate::Writable for MMC_CONTROL {}
#[doc = "This register configures the MMC operating mode."]
pub mod mmc_control;
#[doc = "This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt](mmc_rx_interrupt) module"]
pub type MMC_RX_INTERRUPT = crate::Reg<u32, _MMC_RX_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RX_INTERRUPT;
#[doc = "`read()` method returns [mmc_rx_interrupt::R](mmc_rx_interrupt::R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT {}
#[doc = "This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
pub mod mmc_rx_interrupt;
#[doc = "This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt](mmc_tx_interrupt) module"]
pub type MMC_TX_INTERRUPT = crate::Reg<u32, _MMC_TX_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TX_INTERRUPT;
#[doc = "`read()` method returns [mmc_tx_interrupt::R](mmc_tx_interrupt::R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT {}
#[doc = "This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit."]
pub mod mmc_tx_interrupt;
#[doc = "The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt_mask](mmc_rx_interrupt_mask) module"]
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<u32, _MMC_RX_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RX_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_rx_interrupt_mask::R](mmc_rx_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_rx_interrupt_mask::W](mmc_rx_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPT_MASK {}
#[doc = "The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values."]
pub mod mmc_rx_interrupt_mask;
#[doc = "This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt_mask](mmc_tx_interrupt_mask) module"]
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<u32, _MMC_TX_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TX_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_tx_interrupt_mask::R](mmc_tx_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_tx_interrupt_mask::W](mmc_tx_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASK {}
#[doc = "This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration."]
pub mod mmc_tx_interrupt_mask;
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_single_collision_good_packets](tx_single_collision_good_packets) module"]
pub type TX_SINGLE_COLLISION_GOOD_PACKETS = crate::Reg<u32, _TX_SINGLE_COLLISION_GOOD_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_SINGLE_COLLISION_GOOD_PACKETS;
#[doc = "`read()` method returns [tx_single_collision_good_packets::R](tx_single_collision_good_packets::R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETS {}
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode."]
pub mod tx_single_collision_good_packets;
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_multiple_collision_good_packets](tx_multiple_collision_good_packets) module"]
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS = crate::Reg<u32, _TX_MULTIPLE_COLLISION_GOOD_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_MULTIPLE_COLLISION_GOOD_PACKETS;
#[doc = "`read()` method returns [tx_multiple_collision_good_packets::R](tx_multiple_collision_good_packets::R) reader structure"]
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_PACKETS {}
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode."]
pub mod tx_multiple_collision_good_packets;
#[doc = "This register provides the number of good packets transmitted by Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_packet_count_good](tx_packet_count_good) module"]
pub type TX_PACKET_COUNT_GOOD = crate::Reg<u32, _TX_PACKET_COUNT_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PACKET_COUNT_GOOD;
#[doc = "`read()` method returns [tx_packet_count_good::R](tx_packet_count_good::R) reader structure"]
impl crate::Readable for TX_PACKET_COUNT_GOOD {}
#[doc = "This register provides the number of good packets transmitted by Ethernet peripheral."]
pub mod tx_packet_count_good;
#[doc = "This register provides the number of packets received by Ethernet peripheral with CRC error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_crc_error_packets](rx_crc_error_packets) module"]
pub type RX_CRC_ERROR_PACKETS = crate::Reg<u32, _RX_CRC_ERROR_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CRC_ERROR_PACKETS;
#[doc = "`read()` method returns [rx_crc_error_packets::R](rx_crc_error_packets::R) reader structure"]
impl crate::Readable for RX_CRC_ERROR_PACKETS {}
#[doc = "This register provides the number of packets received by Ethernet peripheral with CRC error."]
pub mod rx_crc_error_packets;
#[doc = "This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_alignment_error_packets](rx_alignment_error_packets) module"]
pub type RX_ALIGNMENT_ERROR_PACKETS = crate::Reg<u32, _RX_ALIGNMENT_ERROR_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ALIGNMENT_ERROR_PACKETS;
#[doc = "`read()` method returns [rx_alignment_error_packets::R](rx_alignment_error_packets::R) reader structure"]
impl crate::Readable for RX_ALIGNMENT_ERROR_PACKETS {}
#[doc = "This register provides the number of packets received by Ethernet peripheral with alignment (dribble) error. It is valid only in 10/100 mode."]
pub mod rx_alignment_error_packets;
#[doc = "This register provides the number of good unicast packets received by Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_unicast_packets_good](rx_unicast_packets_good) module"]
pub type RX_UNICAST_PACKETS_GOOD = crate::Reg<u32, _RX_UNICAST_PACKETS_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_UNICAST_PACKETS_GOOD;
#[doc = "`read()` method returns [rx_unicast_packets_good::R](rx_unicast_packets_good::R) reader structure"]
impl crate::Readable for RX_UNICAST_PACKETS_GOOD {}
#[doc = "This register provides the number of good unicast packets received by Ethernet peripheral."]
pub mod rx_unicast_packets_good;
#[doc = "This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lpi_usec_cntr](tx_lpi_usec_cntr) module"]
pub type TX_LPI_USEC_CNTR = crate::Reg<u32, _TX_LPI_USEC_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_LPI_USEC_CNTR;
#[doc = "`read()` method returns [tx_lpi_usec_cntr::R](tx_lpi_usec_cntr::R) reader structure"]
impl crate::Readable for TX_LPI_USEC_CNTR {}
#[doc = "This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral."]
pub mod tx_lpi_usec_cntr;
#[doc = "This register provides the number of times Ethernet peripheral has entered Tx LPI.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lpi_tran_cntr](tx_lpi_tran_cntr) module"]
pub type TX_LPI_TRAN_CNTR = crate::Reg<u32, _TX_LPI_TRAN_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_LPI_TRAN_CNTR;
#[doc = "`read()` method returns [tx_lpi_tran_cntr::R](tx_lpi_tran_cntr::R) reader structure"]
impl crate::Readable for TX_LPI_TRAN_CNTR {}
#[doc = "This register provides the number of times Ethernet peripheral has entered Tx LPI."]
pub mod tx_lpi_tran_cntr;
#[doc = "This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi_usec_cntr](rx_lpi_usec_cntr) module"]
pub type RX_LPI_USEC_CNTR = crate::Reg<u32, _RX_LPI_USEC_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_LPI_USEC_CNTR;
#[doc = "`read()` method returns [rx_lpi_usec_cntr::R](rx_lpi_usec_cntr::R) reader structure"]
impl crate::Readable for RX_LPI_USEC_CNTR {}
#[doc = "This register provides the number of microseconds Rx LPI is sampled by Ethernet peripheral."]
pub mod rx_lpi_usec_cntr;
#[doc = "This register provides the number of times Ethernet peripheral has entered Rx LPI.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi_tran_cntr](rx_lpi_tran_cntr) module"]
pub type RX_LPI_TRAN_CNTR = crate::Reg<u32, _RX_LPI_TRAN_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_LPI_TRAN_CNTR;
#[doc = "`read()` method returns [rx_lpi_tran_cntr::R](rx_lpi_tran_cntr::R) reader structure"]
impl crate::Readable for RX_LPI_TRAN_CNTR {}
#[doc = "This register provides the number of times Ethernet peripheral has entered Rx LPI."]
pub mod rx_lpi_tran_cntr;
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3l4c0r](eth_macl3l4c0r) module"]
pub type ETH_MACL3L4C0R = crate::Reg<u32, _ETH_MACL3L4C0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3L4C0R;
#[doc = "`read()` method returns [eth_macl3l4c0r::R](eth_macl3l4c0r::R) reader structure"]
impl crate::Readable for ETH_MACL3L4C0R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3l4c0r::W](eth_macl3l4c0r::W) writer structure"]
impl crate::Writable for ETH_MACL3L4C0R {}
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration."]
pub mod eth_macl3l4c0r;
#[doc = "Layer4 address filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl4a0r](eth_macl4a0r) module"]
pub type ETH_MACL4A0R = crate::Reg<u32, _ETH_MACL4A0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL4A0R;
#[doc = "`read()` method returns [eth_macl4a0r::R](eth_macl4a0r::R) reader structure"]
impl crate::Readable for ETH_MACL4A0R {}
#[doc = "`write(|w| ..)` method takes [eth_macl4a0r::W](eth_macl4a0r::W) writer structure"]
impl crate::Writable for ETH_MACL4A0R {}
#[doc = "Layer4 address filter 0 register"]
pub mod eth_macl4a0r;
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a00r](eth_macl3a00r) module"]
pub type ETH_MACL3A00R = crate::Reg<u32, _ETH_MACL3A00R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A00R;
#[doc = "`read()` method returns [eth_macl3a00r::R](eth_macl3a00r::R) reader structure"]
impl crate::Readable for ETH_MACL3A00R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a00r::W](eth_macl3a00r::W) writer structure"]
impl crate::Writable for ETH_MACL3A00R {}
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a00r;
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a10r](eth_macl3a10r) module"]
pub type ETH_MACL3A10R = crate::Reg<u32, _ETH_MACL3A10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A10R;
#[doc = "`read()` method returns [eth_macl3a10r::R](eth_macl3a10r::R) reader structure"]
impl crate::Readable for ETH_MACL3A10R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a10r::W](eth_macl3a10r::W) writer structure"]
impl crate::Writable for ETH_MACL3A10R {}
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a10r;
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a20](eth_macl3a20) module"]
pub type ETH_MACL3A20 = crate::Reg<u32, _ETH_MACL3A20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A20;
#[doc = "`read()` method returns [eth_macl3a20::R](eth_macl3a20::R) reader structure"]
impl crate::Readable for ETH_MACL3A20 {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a20::W](eth_macl3a20::W) writer structure"]
impl crate::Writable for ETH_MACL3A20 {}
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a20;
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a30](eth_macl3a30) module"]
pub type ETH_MACL3A30 = crate::Reg<u32, _ETH_MACL3A30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A30;
#[doc = "`read()` method returns [eth_macl3a30::R](eth_macl3a30::R) reader structure"]
impl crate::Readable for ETH_MACL3A30 {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a30::W](eth_macl3a30::W) writer structure"]
impl crate::Writable for ETH_MACL3A30 {}
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a30;
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3l4c1r](eth_macl3l4c1r) module"]
pub type ETH_MACL3L4C1R = crate::Reg<u32, _ETH_MACL3L4C1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3L4C1R;
#[doc = "`read()` method returns [eth_macl3l4c1r::R](eth_macl3l4c1r::R) reader structure"]
impl crate::Readable for ETH_MACL3L4C1R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3l4c1r::W](eth_macl3l4c1r::W) writer structure"]
impl crate::Writable for ETH_MACL3L4C1R {}
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4."]
pub mod eth_macl3l4c1r;
#[doc = "The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl4a1r](eth_macl4a1r) module"]
pub type ETH_MACL4A1R = crate::Reg<u32, _ETH_MACL4A1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL4A1R;
#[doc = "`read()` method returns [eth_macl4a1r::R](eth_macl4a1r::R) reader structure"]
impl crate::Readable for ETH_MACL4A1R {}
#[doc = "`write(|w| ..)` method takes [eth_macl4a1r::W](eth_macl4a1r::W) writer structure"]
impl crate::Writable for ETH_MACL4A1R {}
#[doc = "The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock."]
pub mod eth_macl4a1r;
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a01r](eth_macl3a01r) module"]
pub type ETH_MACL3A01R = crate::Reg<u32, _ETH_MACL3A01R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A01R;
#[doc = "`read()` method returns [eth_macl3a01r::R](eth_macl3a01r::R) reader structure"]
impl crate::Readable for ETH_MACL3A01R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a01r::W](eth_macl3a01r::W) writer structure"]
impl crate::Writable for ETH_MACL3A01R {}
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a01r;
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a11r](eth_macl3a11r) module"]
pub type ETH_MACL3A11R = crate::Reg<u32, _ETH_MACL3A11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A11R;
#[doc = "`read()` method returns [eth_macl3a11r::R](eth_macl3a11r::R) reader structure"]
impl crate::Readable for ETH_MACL3A11R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a11r::W](eth_macl3a11r::W) writer structure"]
impl crate::Writable for ETH_MACL3A11R {}
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a11r;
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a21r](eth_macl3a21r) module"]
pub type ETH_MACL3A21R = crate::Reg<u32, _ETH_MACL3A21R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A21R;
#[doc = "`read()` method returns [eth_macl3a21r::R](eth_macl3a21r::R) reader structure"]
impl crate::Readable for ETH_MACL3A21R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a21r::W](eth_macl3a21r::W) writer structure"]
impl crate::Writable for ETH_MACL3A21R {}
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a21r;
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a31r](eth_macl3a31r) module"]
pub type ETH_MACL3A31R = crate::Reg<u32, _ETH_MACL3A31R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACL3A31R;
#[doc = "`read()` method returns [eth_macl3a31r::R](eth_macl3a31r::R) reader structure"]
impl crate::Readable for ETH_MACL3A31R {}
#[doc = "`write(|w| ..)` method takes [eth_macl3a31r::W](eth_macl3a31r::W) writer structure"]
impl crate::Writable for ETH_MACL3A31R {}
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field."]
pub mod eth_macl3a31r;
#[doc = "The ARP Address register contains the IPv4 Destination Address of the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macarpar](eth_macarpar) module"]
pub type ETH_MACARPAR = crate::Reg<u32, _ETH_MACARPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACARPAR;
#[doc = "`read()` method returns [eth_macarpar::R](eth_macarpar::R) reader structure"]
impl crate::Readable for ETH_MACARPAR {}
#[doc = "`write(|w| ..)` method takes [eth_macarpar::W](eth_macarpar::W) writer structure"]
impl crate::Writable for ETH_MACARPAR {}
#[doc = "The ARP Address register contains the IPv4 Destination Address of the MAC."]
pub mod eth_macarpar;
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactscr](eth_mactscr) module"]
pub type ETH_MACTSCR = crate::Reg<u32, _ETH_MACTSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSCR;
#[doc = "`read()` method returns [eth_mactscr::R](eth_mactscr::R) reader structure"]
impl crate::Readable for ETH_MACTSCR {}
#[doc = "`write(|w| ..)` method takes [eth_mactscr::W](eth_mactscr::W) writer structure"]
impl crate::Writable for ETH_MACTSCR {}
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver."]
pub mod eth_mactscr;
#[doc = "The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macssir](eth_macssir) module"]
pub type ETH_MACSSIR = crate::Reg<u32, _ETH_MACSSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSSIR;
#[doc = "`read()` method returns [eth_macssir::R](eth_macssir::R) reader structure"]
impl crate::Readable for ETH_MACSSIR {}
#[doc = "`write(|w| ..)` method takes [eth_macssir::W](eth_macssir::W) writer structure"]
impl crate::Writable for ETH_MACSSIR {}
#[doc = "The Sub-second Increment register is present only when the IEEE 1588 timestamp feature is selected without an external timestamp input. In Coarse Update mode \\[Bit 1 in ETH_MACTSCR register, the value in this register is added to the system time every clock cycle of HCLK. In Fine Update mode, the value in this register is added to the system time whenever the Accumulator gets an overflow."]
pub mod eth_macssir;
#[doc = "The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstsr](eth_macstsr) module"]
pub type ETH_MACSTSR = crate::Reg<u32, _ETH_MACSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSTSR;
#[doc = "`read()` method returns [eth_macstsr::R](eth_macstsr::R) reader structure"]
impl crate::Readable for ETH_MACSTSR {}
#[doc = "The System Time Seconds register, along with System Time Nanoseconds register, indicates the current value of the system time maintained by the MAC. Though it is updated on a continuous basis, there is some delay from the actual time because of clock domain transfer latencies (from HCLK to CSR clock). This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstsr;
#[doc = "The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstnr](eth_macstnr) module"]
pub type ETH_MACSTNR = crate::Reg<u32, _ETH_MACSTNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSTNR;
#[doc = "`read()` method returns [eth_macstnr::R](eth_macstnr::R) reader structure"]
impl crate::Readable for ETH_MACSTNR {}
#[doc = "The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstnr;
#[doc = "The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstsur](eth_macstsur) module"]
pub type ETH_MACSTSUR = crate::Reg<u32, _ETH_MACSTSUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSTSUR;
#[doc = "`read()` method returns [eth_macstsur::R](eth_macstsur::R) reader structure"]
impl crate::Readable for ETH_MACSTSUR {}
#[doc = "`write(|w| ..)` method takes [eth_macstsur::W](eth_macstsur::W) writer structure"]
impl crate::Writable for ETH_MACSTSUR {}
#[doc = "The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input."]
pub mod eth_macstsur;
#[doc = "This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstnur](eth_macstnur) module"]
pub type ETH_MACSTNUR = crate::Reg<u32, _ETH_MACSTNUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSTNUR;
#[doc = "`read()` method returns [eth_macstnur::R](eth_macstnur::R) reader structure"]
impl crate::Readable for ETH_MACSTNUR {}
#[doc = "`write(|w| ..)` method takes [eth_macstnur::W](eth_macstnur::W) writer structure"]
impl crate::Writable for ETH_MACSTNUR {}
#[doc = "This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input."]
pub mod eth_macstnur;
#[doc = "The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsar](eth_mactsar) module"]
pub type ETH_MACTSAR = crate::Reg<u32, _ETH_MACTSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSAR;
#[doc = "`read()` method returns [eth_mactsar::R](eth_mactsar::R) reader structure"]
impl crate::Readable for ETH_MACTSAR {}
#[doc = "`write(|w| ..)` method takes [eth_mactsar::W](eth_mactsar::W) writer structure"]
impl crate::Writable for ETH_MACTSAR {}
#[doc = "The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows."]
pub mod eth_mactsar;
#[doc = "The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactssr](eth_mactssr) module"]
pub type ETH_MACTSSR = crate::Reg<u32, _ETH_MACTSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSSR;
#[doc = "`read()` method returns [eth_mactssr::R](eth_mactssr::R) reader structure"]
impl crate::Readable for ETH_MACTSSR {}
#[doc = "The Timestamp Status register is present only when the IEEE 1588 Timestamp feature is selected. All bits except Bits\\[27:25\\]
gets cleared when the application reads this register."]
pub mod eth_mactssr;
#[doc = "This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_tssnr](eth_mactx_tssnr) module"]
pub type ETH_MACTXTSSNR = crate::Reg<u32, _ETH_MACTXTSSNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTXTSSNR;
#[doc = "`read()` method returns [eth_mactx_tssnr::R](eth_mactx_tssnr::R) reader structure"]
impl crate::Readable for ETH_MACTXTSSNR {}
#[doc = "This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled."]
pub mod eth_mactx_tssnr;
#[doc = "The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_tsssr](eth_mactx_tsssr) module"]
pub type ETH_MACTXTSSSR = crate::Reg<u32, _ETH_MACTXTSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTXTSSSR;
#[doc = "`read()` method returns [eth_mactx_tsssr::R](eth_mactx_tsssr::R) reader structure"]
impl crate::Readable for ETH_MACTXTSSSR {}
#[doc = "The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted."]
pub mod eth_mactx_tsssr;
#[doc = "The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macacr](eth_macacr) module"]
pub type ETH_MACACR = crate::Reg<u32, _ETH_MACACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACACR;
#[doc = "`read()` method returns [eth_macacr::R](eth_macacr::R) reader structure"]
impl crate::Readable for ETH_MACACR {}
#[doc = "`write(|w| ..)` method takes [eth_macacr::W](eth_macacr::W) writer structure"]
impl crate::Writable for ETH_MACACR {}
#[doc = "The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot."]
pub mod eth_macacr;
#[doc = "The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macatsnr](eth_macatsnr) module"]
pub type ETH_MACATSNR = crate::Reg<u32, _ETH_MACATSNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACATSNR;
#[doc = "`read()` method returns [eth_macatsnr::R](eth_macatsnr::R) reader structure"]
impl crate::Readable for ETH_MACATSNR {}
#[doc = "The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read."]
pub mod eth_macatsnr;
#[doc = "The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macatssr](eth_macatssr) module"]
pub type ETH_MACATSSR = crate::Reg<u32, _ETH_MACATSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACATSSR;
#[doc = "`read()` method returns [eth_macatssr::R](eth_macatssr::R) reader structure"]
impl crate::Readable for ETH_MACATSSR {}
#[doc = "The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register."]
pub mod eth_macatssr;
#[doc = "The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsiacr](eth_mactsiacr) module"]
pub type ETH_MACTSIACR = crate::Reg<u32, _ETH_MACTSIACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSIACR;
#[doc = "`read()` method returns [eth_mactsiacr::R](eth_mactsiacr::R) reader structure"]
impl crate::Readable for ETH_MACTSIACR {}
#[doc = "`write(|w| ..)` method takes [eth_mactsiacr::W](eth_mactsiacr::W) writer structure"]
impl crate::Writable for ETH_MACTSIACR {}
#[doc = "The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages."]
pub mod eth_mactsiacr;
#[doc = "The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactseacr](eth_mactseacr) module"]
pub type ETH_MACTSEACR = crate::Reg<u32, _ETH_MACTSEACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSEACR;
#[doc = "`read()` method returns [eth_mactseacr::R](eth_mactseacr::R) reader structure"]
impl crate::Readable for ETH_MACTSEACR {}
#[doc = "`write(|w| ..)` method takes [eth_mactseacr::W](eth_mactseacr::W) writer structure"]
impl crate::Writable for ETH_MACTSEACR {}
#[doc = "The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages."]
pub mod eth_mactseacr;
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsicnr](eth_mactsicnr) module"]
pub type ETH_MACTSICNR = crate::Reg<u32, _ETH_MACTSICNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSICNR;
#[doc = "`read()` method returns [eth_mactsicnr::R](eth_mactsicnr::R) reader structure"]
impl crate::Readable for ETH_MACTSICNR {}
#[doc = "`write(|w| ..)` method takes [eth_mactsicnr::W](eth_mactsicnr::W) writer structure"]
impl crate::Writable for ETH_MACTSICNR {}
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path."]
pub mod eth_mactsicnr;
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsecnr](eth_mactsecnr) module"]
pub type ETH_MACTSECNR = crate::Reg<u32, _ETH_MACTSECNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACTSECNR;
#[doc = "`read()` method returns [eth_mactsecnr::R](eth_mactsecnr::R) reader structure"]
impl crate::Readable for ETH_MACTSECNR {}
#[doc = "`write(|w| ..)` method takes [eth_mactsecnr::W](eth_mactsecnr::W) writer structure"]
impl crate::Writable for ETH_MACTSECNR {}
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path."]
pub mod eth_mactsecnr;
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppscr](eth_macppscr) module"]
pub type ETH_MACPPSCR = crate::Reg<u32, _ETH_MACPPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPPSCR;
#[doc = "`read()` method returns [eth_macppscr::R](eth_macppscr::R) reader structure"]
impl crate::Readable for ETH_MACPPSCR {}
#[doc = "`write(|w| ..)` method takes [eth_macppscr::W](eth_macppscr::W) writer structure"]
impl crate::Writable for ETH_MACPPSCR {}
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected."]
pub mod eth_macppscr;
#[doc = "The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppsttsr](eth_macppsttsr) module"]
pub type ETH_MACPPSTTSR = crate::Reg<u32, _ETH_MACPPSTTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPPSTTSR;
#[doc = "`read()` method returns [eth_macppsttsr::R](eth_macppsttsr::R) reader structure"]
impl crate::Readable for ETH_MACPPSTTSR {}
#[doc = "`write(|w| ..)` method takes [eth_macppsttsr::W](eth_macppsttsr::W) writer structure"]
impl crate::Writable for ETH_MACPPSTTSR {}
#[doc = "The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers."]
pub mod eth_macppsttsr;
#[doc = "The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppsttnr](eth_macppsttnr) module"]
pub type ETH_MACPPSTTNR = crate::Reg<u32, _ETH_MACPPSTTNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPPSTTNR;
#[doc = "`read()` method returns [eth_macppsttnr::R](eth_macppsttnr::R) reader structure"]
impl crate::Readable for ETH_MACPPSTTNR {}
#[doc = "`write(|w| ..)` method takes [eth_macppsttnr::W](eth_macppsttnr::W) writer structure"]
impl crate::Writable for ETH_MACPPSTTNR {}
#[doc = "The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected."]
pub mod eth_macppsttnr;
#[doc = "The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppsir](eth_macppsir) module"]
pub type ETH_MACPPSIR = crate::Reg<u32, _ETH_MACPPSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPPSIR;
#[doc = "`read()` method returns [eth_macppsir::R](eth_macppsir::R) reader structure"]
impl crate::Readable for ETH_MACPPSIR {}
#[doc = "`write(|w| ..)` method takes [eth_macppsir::W](eth_macppsir::W) writer structure"]
impl crate::Writable for ETH_MACPPSIR {}
#[doc = "The PPS Interval register contains the number of units of sub-second increment value between the rising edges of PPS signal output (ptp_pps_o\\[0\\])."]
pub mod eth_macppsir;
#[doc = "The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppswr](eth_macppswr) module"]
pub type ETH_MACPPSWR = crate::Reg<u32, _ETH_MACPPSWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPPSWR;
#[doc = "`read()` method returns [eth_macppswr::R](eth_macppswr::R) reader structure"]
impl crate::Readable for ETH_MACPPSWR {}
#[doc = "`write(|w| ..)` method takes [eth_macppswr::W](eth_macppswr::W) writer structure"]
impl crate::Writable for ETH_MACPPSWR {}
#[doc = "The PPS Width register contains the number of units of sub-second increment value between the rising and corresponding falling edges of PPS signal output (ptp_pps_o)."]
pub mod eth_macppswr;
#[doc = "This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpocr](eth_macpocr) module"]
pub type ETH_MACPOCR = crate::Reg<u32, _ETH_MACPOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACPOCR;
#[doc = "`read()` method returns [eth_macpocr::R](eth_macpocr::R) reader structure"]
impl crate::Readable for ETH_MACPOCR {}
#[doc = "`write(|w| ..)` method takes [eth_macpocr::W](eth_macpocr::W) writer structure"]
impl crate::Writable for ETH_MACPOCR {}
#[doc = "This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macpocr;
#[doc = "This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi0r](eth_macspi0r) module"]
pub type ETH_MACSPI0R = crate::Reg<u32, _ETH_MACSPI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSPI0R;
#[doc = "`read()` method returns [eth_macspi0r::R](eth_macspi0r::R) reader structure"]
impl crate::Readable for ETH_MACSPI0R {}
#[doc = "`write(|w| ..)` method takes [eth_macspi0r::W](eth_macspi0r::W) writer structure"]
impl crate::Writable for ETH_MACSPI0R {}
#[doc = "This register contains Bits\\[31:0\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macspi0r;
#[doc = "This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi1r](eth_macspi1r) module"]
pub type ETH_MACSPI1R = crate::Reg<u32, _ETH_MACSPI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSPI1R;
#[doc = "`read()` method returns [eth_macspi1r::R](eth_macspi1r::R) reader structure"]
impl crate::Readable for ETH_MACSPI1R {}
#[doc = "`write(|w| ..)` method takes [eth_macspi1r::W](eth_macspi1r::W) writer structure"]
impl crate::Writable for ETH_MACSPI1R {}
#[doc = "This register contains Bits\\[63:32\\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected."]
pub mod eth_macspi1r;
#[doc = "This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macspi2r](eth_macspi2r) module"]
pub type ETH_MACSPI2R = crate::Reg<u32, _ETH_MACSPI2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACSPI2R;
#[doc = "`read()` method returns [eth_macspi2r::R](eth_macspi2r::R) reader structure"]
impl crate::Readable for ETH_MACSPI2R {}
#[doc = "`write(|w| ..)` method takes [eth_macspi2r::W](eth_macspi2r::W) writer structure"]
impl crate::Writable for ETH_MACSPI2R {}
#[doc = "This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node."]
pub mod eth_macspi2r;
#[doc = "This register contains the periodic intervals for automatic PTP packet generation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclmir](eth_maclmir) module"]
pub type ETH_MACLMIR = crate::Reg<u32, _ETH_MACLMIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MACLMIR;
#[doc = "`read()` method returns [eth_maclmir::R](eth_maclmir::R) reader structure"]
impl crate::Readable for ETH_MACLMIR {}
#[doc = "`write(|w| ..)` method takes [eth_maclmir::W](eth_maclmir::W) writer structure"]
impl crate::Writable for ETH_MACLMIR {}
#[doc = "This register contains the periodic intervals for automatic PTP packet generation."]
pub mod eth_maclmir;
