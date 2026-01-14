///Register `MACCR` reader
pub type R = crate::R<MACCRrs>;
///Register `MACCR` writer
pub type W = crate::W<MACCRrs>;
///Field `RE` reader - Receiver Enable When this bit is set, the Rx state machine of the MAC is enabled for receiving packets from the MII interface. When this bit is reset, the MAC Rx state machine is disabled after it completes the reception of the current packet. The Rx state machine does not receive any more packets from the MII interface.
pub type RE_R = crate::BitReader;
///Field `RE` writer - Receiver Enable When this bit is set, the Rx state machine of the MAC is enabled for receiving packets from the MII interface. When this bit is reset, the MAC Rx state machine is disabled after it completes the reception of the current packet. The Rx state machine does not receive any more packets from the MII interface.
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - Transmitter Enable When this bit is set, the Tx state machine of the MAC is enabled for transmission on the MII interface. When this bit is reset, the MAC Tx state machine is disabled after it completes the transmission of the current packet. The Tx state machine does not transmit any more packets.
pub type TE_R = crate::BitReader;
///Field `TE` writer - Transmitter Enable When this bit is set, the Tx state machine of the MAC is enabled for transmission on the MII interface. When this bit is reset, the MAC Tx state machine is disabled after it completes the transmission of the current packet. The Tx state machine does not transmit any more packets.
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRELEN` reader - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.
pub type PRELEN_R = crate::FieldReader;
///Field `PRELEN` writer - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.
pub type PRELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DC` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Packet Abort status, along with the excessive deferral error bit set in the Tx packet status, when the Tx state machine is deferred for more than 24,288 bit times in 10 or 100 Mbps mode. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0, and it is restarted. When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive. This bit is applicable only in the half-duplex mode.
pub type DC_R = crate::BitReader;
///Field `DC` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Packet Abort status, along with the excessive deferral error bit set in the Tx packet status, when the Tx state machine is deferred for more than 24,288 bit times in 10 or 100 Mbps mode. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0, and it is restarted. When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive. This bit is applicable only in the half-duplex mode.
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BL` reader - Back-Off Limit
pub type BL_R = crate::FieldReader;
///Field `BL` writer - Back-Off Limit
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DR` reader - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the MII interface, the MAC ignores the current packet transmission and reports a Packet Abort with excessive collision error in the Tx packet status. When this bit is reset, the MAC retries based on the settings of the BL field. This bit is applicable only in the half-duplex mode.
pub type DR_R = crate::BitReader;
///Field `DR` writer - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the MII interface, the MAC ignores the current packet transmission and reports a Packet Abort with excessive collision error in the Tx packet status. When this bit is reset, the MAC retries based on the settings of the BL field. This bit is applicable only in the half-duplex mode.
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRS` reader - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the (G)MII CRS signal during packet transmission in the half-duplex mode. As a result, no errors are generated because of Loss of Carrier or No Carrier during transmission. When this bit is reset, the MAC transmitter generates errors because of Carrier Sense. The MAC can even abort the transmission.
pub type DCRS_R = crate::BitReader;
///Field `DCRS` writer - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the (G)MII CRS signal during packet transmission in the half-duplex mode. As a result, no errors are generated because of Loss of Carrier or No Carrier during transmission. When this bit is reset, the MAC transmitter generates errors because of Carrier Sense. The MAC can even abort the transmission.
pub type DCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DO` reader - Disable Receive Own When this bit is set, the MAC disables the reception of packets when the ETH_TX_EN is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets given by the PHY. This bit is not applicable in the full-duplex mode. This bit is reserved and read-only (RO) with default value in the full-duplex-only configurations.
pub type DO_R = crate::BitReader;
///Field `DO` writer - Disable Receive Own When this bit is set, the MAC disables the reception of packets when the ETH_TX_EN is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets given by the PHY. This bit is not applicable in the full-duplex mode. This bit is reserved and read-only (RO) with default value in the full-duplex-only configurations.
pub type DO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECRSFD` reader - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode. The MAC starts the transmission only when the CRS signal is low. When this bit is reset, the MAC transmitter ignores the status of the CRS signal.
pub type ECRSFD_R = crate::BitReader;
///Field `ECRSFD` writer - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode. The MAC starts the transmission only when the CRS signal is low. When this bit is reset, the MAC transmitter ignores the status of the CRS signal.
pub type ECRSFD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LM` reader - Loopback Mode When this bit is set, the MAC operates in the loopback mode at MII. The MII Rx clock input (eth_mii_rx_clk) is required for the loopback to work properly. This is because the Tx clock is not internally looped back.
pub type LM_R = crate::BitReader;
///Field `LM` writer - Loopback Mode When this bit is set, the MAC operates in the loopback mode at MII. The MII Rx clock input (eth_mii_rx_clk) is required for the loopback to work properly. This is because the Tx clock is not internally looped back.
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DM` reader - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode in which it can transmit and receive simultaneously.
pub type DM_R = crate::BitReader;
///Field `DM` writer - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode in which it can transmit and receive simultaneously.
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FES` reader - MAC Speed This bit selects the speed in the 10/100 Mbps mode:
pub type FES_R = crate::BitReader;
///Field `FES` writer - MAC Speed This bit selects the speed in the 10/100 Mbps mode:
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JE` reader - Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see .
pub type JE_R = crate::BitReader;
///Field `JE` writer - Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see .
pub type JE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JD` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer packets of up to 16,383 bytes. When this bit is reset, if the application sends more than 2,048 bytes of data (10,240 if JE is set high) during transmission, the MAC does not send rest of the bytes in that packet.
pub type JD_R = crate::BitReader;
///Field `JD` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer packets of up to 16,383 bytes. When this bit is reset, if the application sends more than 2,048 bytes of data (10,240 if JE is set high) during transmission, the MAC does not send rest of the bytes in that packet.
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WD` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive packets of up to 16,383 bytes. When this bit is reset, the MAC does not allow more than 2,048 bytes (10,240 if JE is set high) of the packet being received. The MAC cuts off any bytes received after 2,048 bytes.
pub type WD_R = crate::BitReader;
///Field `WD` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive packets of up to 16,383 bytes. When this bit is reset, the MAC does not allow more than 2,048 bytes (10,240 if JE is set high) of the packet being received. The MAC cuts off any bytes received after 2,048 bytes.
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACS` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes. All received packets with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming packets to the application, without any modification. Note: For information about how the settings of CST bit and this bit impact the packet length, see .
pub type ACS_R = crate::BitReader;
///Field `ACS` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes. All received packets with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming packets to the application, without any modification. Note: For information about how the settings of CST bit and this bit impact the packet length, see .
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CST` reader - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled. Note: For information about how the settings of the ACS bit and this bit impact the packet length, see .
pub type CST_R = crate::BitReader;
///Field `CST` writer - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled. Note: For information about how the settings of the ACS bit and this bit impact the packet length, see .
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S2KP` reader - IEEE 802.3as Support for 2K Packets When this bit is set, the MAC considers all packets with up to 2,000 bytes length as normal packets. When the JE bit is not set, the MAC considers all received packets of size more than 2K bytes as Giant packets. When this bit is reset and the JE bit is not set, the MAC considers all received packets of size more than 1,518 bytes (1,522 bytes for tagged) as giant packets. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see Packet Status based on S2KP and JE Bits. Note: When the JE bit is set, setting this bit has no effect on the giant packet status.
pub type S2KP_R = crate::BitReader;
///Field `S2KP` writer - IEEE 802.3as Support for 2K Packets When this bit is set, the MAC considers all packets with up to 2,000 bytes length as normal packets. When the JE bit is not set, the MAC considers all received packets of size more than 2K bytes as Giant packets. When this bit is reset and the JE bit is not set, the MAC considers all received packets of size more than 1,518 bytes (1,522 bytes for tagged) as giant packets. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see Packet Status based on S2KP and JE Bits. Note: When the JE bit is set, setting this bit has no effect on the giant packet status.
pub type S2KP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPSLCE` reader - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in ETH_MACECR register to declare a received packet as Giant packet. This field must be programmed to more than 1,518 bytes. Otherwise, the MAC considers 1,518 bytes as giant packet limit. When this bit is reset, the MAC considers a received packet as Giant packet when its size is greater than 1,518 bytes (1522 bytes for tagged packet). The watchdog timeout limit, Jumbo Packet Enable and 2K Packet Enable have higher precedence over this bit, that is the MAC considers a received packet as Giant packet when its size is greater than 9,018 bytes (9,022 bytes for tagged packet) with Jumbo Packet Enabled and greater than 2,000 bytes with 2K Packet Enabled. The watchdog timeout, if enabled, terminates the received packet when watchdog limit is reached. Therefore, the programmed giant packet limit should be less than the watchdog limit to get the giant packet status.
pub type GPSLCE_R = crate::BitReader;
///Field `GPSLCE` writer - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in ETH_MACECR register to declare a received packet as Giant packet. This field must be programmed to more than 1,518 bytes. Otherwise, the MAC considers 1,518 bytes as giant packet limit. When this bit is reset, the MAC considers a received packet as Giant packet when its size is greater than 1,518 bytes (1522 bytes for tagged packet). The watchdog timeout limit, Jumbo Packet Enable and 2K Packet Enable have higher precedence over this bit, that is the MAC considers a received packet as Giant packet when its size is greater than 9,018 bytes (9,022 bytes for tagged packet) with Jumbo Packet Enabled and greater than 2,000 bytes with 2K Packet Enabled. The watchdog timeout, if enabled, terminates the received packet when watchdog limit is reached. Therefore, the programmed giant packet limit should be less than the watchdog limit to get the giant packet status.
pub type GPSLCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPG` reader - Inter-Packet Gap These bits control the minimum IPG between packets during transmission. ... This range of minimum IPG is valid in full-duplex mode. In the half-duplex mode, the minimum IPG can be configured only for 64-bit times (IPG = 100). Lower values are not considered. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IPG. The above function (IPG less than 96 bit times) is valid only when EIPGEN bit in ETH_MACECR register is reset. When EIPGEN is set, then the minimum IPG (greater than 96 bit times) is controlled as per the description given in EIPG field in ETH_MACECR register.
pub type IPG_R = crate::FieldReader;
///Field `IPG` writer - Inter-Packet Gap These bits control the minimum IPG between packets during transmission. ... This range of minimum IPG is valid in full-duplex mode. In the half-duplex mode, the minimum IPG can be configured only for 64-bit times (IPG = 100). Lower values are not considered. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IPG. The above function (IPG less than 96 bit times) is valid only when EIPGEN bit in ETH_MACECR register is reset. When EIPGEN is set, then the minimum IPG (greater than 96 bit times) is controlled as per the description given in EIPG field in ETH_MACECR register.
pub type IPG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IPC` reader - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking. When this bit is reset, the COE function in the receiver is disabled. The Layer 3 and Layer 4 Packet Filter feature automatically selects the IPC Full Checksum Offload Engine on the Receive side. When this feature is enabled, you must set the IPC bit.
pub type IPC_R = crate::BitReader;
///Field `IPC` writer - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking. When this bit is reset, the COE function in the receiver is disabled. The Layer 3 and Layer 4 Packet Filter feature automatically selects the IPC Full Checksum Offload Engine on the Receive side. When this feature is enabled, you must set the IPC bit.
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARC` reader - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted packets. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits\[29:28\]: Others: Reserved, must not be used. Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
pub type SARC_R = crate::FieldReader;
///Field `SARC` writer - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted packets. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits\[29:28\]: Others: Reserved, must not be used. Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
pub type SARC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ARPEN` reader - ARP Offload Enable When this bit is set, the MAC can recognize an incoming ARP request packet and schedules the ARP packet for transmission. It will forward the ARP packet to the application and also indicate the events in the RxStatus. When this bit is reset, the MAC receiver does not recognize any ARP packet and indicates them as Type frame in the RxStatus.
pub type ARPEN_R = crate::BitReader;
///Field `ARPEN` writer - ARP Offload Enable When this bit is set, the MAC can recognize an incoming ARP request packet and schedules the ARP packet for transmission. It will forward the ARP packet to the application and also indicate the events in the RxStatus. When this bit is reset, the MAC receiver does not recognize any ARP packet and indicates them as Type frame in the RxStatus.
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Receiver Enable When this bit is set, the Rx state machine of the MAC is enabled for receiving packets from the MII interface. When this bit is reset, the MAC Rx state machine is disabled after it completes the reception of the current packet. The Rx state machine does not receive any more packets from the MII interface.
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmitter Enable When this bit is set, the Tx state machine of the MAC is enabled for transmission on the MII interface. When this bit is reset, the MAC Tx state machine is disabled after it completes the transmission of the current packet. The Tx state machine does not transmit any more packets.
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Packet Abort status, along with the excessive deferral error bit set in the Tx packet status, when the Tx state machine is deferred for more than 24,288 bit times in 10 or 100 Mbps mode. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0, and it is restarted. When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive. This bit is applicable only in the half-duplex mode.
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Back-Off Limit
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 8 - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the MII interface, the MAC ignores the current packet transmission and reports a Packet Abort with excessive collision error in the Tx packet status. When this bit is reset, the MAC retries based on the settings of the BL field. This bit is applicable only in the half-duplex mode.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the (G)MII CRS signal during packet transmission in the half-duplex mode. As a result, no errors are generated because of Loss of Carrier or No Carrier during transmission. When this bit is reset, the MAC transmitter generates errors because of Carrier Sense. The MAC can even abort the transmission.
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of packets when the ETH_TX_EN is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets given by the PHY. This bit is not applicable in the full-duplex mode. This bit is reserved and read-only (RO) with default value in the full-duplex-only configurations.
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode. The MAC starts the transmission only when the CRS signal is low. When this bit is reset, the MAC transmitter ignores the status of the CRS signal.
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at MII. The MII Rx clock input (eth_mii_rx_clk) is required for the loopback to work properly. This is because the Tx clock is not internally looped back.
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode in which it can transmit and receive simultaneously.
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MAC Speed This bit selects the speed in the 10/100 Mbps mode:
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see .
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer packets of up to 16,383 bytes. When this bit is reset, if the application sends more than 2,048 bytes of data (10,240 if JE is set high) during transmission, the MAC does not send rest of the bytes in that packet.
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive packets of up to 16,383 bytes. When this bit is reset, the MAC does not allow more than 2,048 bytes (10,240 if JE is set high) of the packet being received. The MAC cuts off any bytes received after 2,048 bytes.
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes. All received packets with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming packets to the application, without any modification. Note: For information about how the settings of CST bit and this bit impact the packet length, see .
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled. Note: For information about how the settings of the ACS bit and this bit impact the packet length, see .
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - IEEE 802.3as Support for 2K Packets When this bit is set, the MAC considers all packets with up to 2,000 bytes length as normal packets. When the JE bit is not set, the MAC considers all received packets of size more than 2K bytes as Giant packets. When this bit is reset and the JE bit is not set, the MAC considers all received packets of size more than 1,518 bytes (1,522 bytes for tagged) as giant packets. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see Packet Status based on S2KP and JE Bits. Note: When the JE bit is set, setting this bit has no effect on the giant packet status.
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in ETH_MACECR register to declare a received packet as Giant packet. This field must be programmed to more than 1,518 bytes. Otherwise, the MAC considers 1,518 bytes as giant packet limit. When this bit is reset, the MAC considers a received packet as Giant packet when its size is greater than 1,518 bytes (1522 bytes for tagged packet). The watchdog timeout limit, Jumbo Packet Enable and 2K Packet Enable have higher precedence over this bit, that is the MAC considers a received packet as Giant packet when its size is greater than 9,018 bytes (9,022 bytes for tagged packet) with Jumbo Packet Enabled and greater than 2,000 bytes with 2K Packet Enabled. The watchdog timeout, if enabled, terminates the received packet when watchdog limit is reached. Therefore, the programmed giant packet limit should be less than the watchdog limit to get the giant packet status.
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission. ... This range of minimum IPG is valid in full-duplex mode. In the half-duplex mode, the minimum IPG can be configured only for 64-bit times (IPG = 100). Lower values are not considered. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IPG. The above function (IPG less than 96 bit times) is valid only when EIPGEN bit in ETH_MACECR register is reset. When EIPGEN is set, then the minimum IPG (greater than 96 bit times) is controlled as per the description given in EIPG field in ETH_MACECR register.
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking. When this bit is reset, the COE function in the receiver is disabled. The Layer 3 and Layer 4 Packet Filter feature automatically selects the IPC Full Checksum Offload Engine on the Receive side. When this feature is enabled, you must set the IPC bit.
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted packets. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits\[29:28\]: Others: Reserved, must not be used. Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - ARP Offload Enable When this bit is set, the MAC can recognize an incoming ARP request packet and schedules the ARP packet for transmission. It will forward the ARP packet to the application and also indicate the events in the RxStatus. When this bit is reset, the MAC receiver does not recognize any ARP packet and indicates them as Type frame in the RxStatus.
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCR")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("prelen", &self.prelen())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("dr", &self.dr())
            .field("dcrs", &self.dcrs())
            .field("do_", &self.do_())
            .field("ecrsfd", &self.ecrsfd())
            .field("lm", &self.lm())
            .field("dm", &self.dm())
            .field("fes", &self.fes())
            .field("je", &self.je())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("acs", &self.acs())
            .field("cst", &self.cst())
            .field("s2kp", &self.s2kp())
            .field("gpslce", &self.gpslce())
            .field("ipg", &self.ipg())
            .field("ipc", &self.ipc())
            .field("sarc", &self.sarc())
            .field("arpen", &self.arpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Receiver Enable When this bit is set, the Rx state machine of the MAC is enabled for receiving packets from the MII interface. When this bit is reset, the MAC Rx state machine is disabled after it completes the reception of the current packet. The Rx state machine does not receive any more packets from the MII interface.
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, MACCRrs> {
        RE_W::new(self, 0)
    }
    ///Bit 1 - Transmitter Enable When this bit is set, the Tx state machine of the MAC is enabled for transmission on the MII interface. When this bit is reset, the MAC Tx state machine is disabled after it completes the transmission of the current packet. The Tx state machine does not transmit any more packets.
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, MACCRrs> {
        TE_W::new(self, 1)
    }
    ///Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W<'_, MACCRrs> {
        PRELEN_W::new(self, 2)
    }
    ///Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Packet Abort status, along with the excessive deferral error bit set in the Tx packet status, when the Tx state machine is deferred for more than 24,288 bit times in 10 or 100 Mbps mode. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0, and it is restarted. When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive. This bit is applicable only in the half-duplex mode.
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, MACCRrs> {
        DC_W::new(self, 4)
    }
    ///Bits 5:6 - Back-Off Limit
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, MACCRrs> {
        BL_W::new(self, 5)
    }
    ///Bit 8 - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the MII interface, the MAC ignores the current packet transmission and reports a Packet Abort with excessive collision error in the Tx packet status. When this bit is reset, the MAC retries based on the settings of the BL field. This bit is applicable only in the half-duplex mode.
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, MACCRrs> {
        DR_W::new(self, 8)
    }
    ///Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the (G)MII CRS signal during packet transmission in the half-duplex mode. As a result, no errors are generated because of Loss of Carrier or No Carrier during transmission. When this bit is reset, the MAC transmitter generates errors because of Carrier Sense. The MAC can even abort the transmission.
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W<'_, MACCRrs> {
        DCRS_W::new(self, 9)
    }
    ///Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of packets when the ETH_TX_EN is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets given by the PHY. This bit is not applicable in the full-duplex mode. This bit is reserved and read-only (RO) with default value in the full-duplex-only configurations.
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W<'_, MACCRrs> {
        DO_W::new(self, 10)
    }
    ///Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode. The MAC starts the transmission only when the CRS signal is low. When this bit is reset, the MAC transmitter ignores the status of the CRS signal.
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<'_, MACCRrs> {
        ECRSFD_W::new(self, 11)
    }
    ///Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at MII. The MII Rx clock input (eth_mii_rx_clk) is required for the loopback to work properly. This is because the Tx clock is not internally looped back.
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, MACCRrs> {
        LM_W::new(self, 12)
    }
    ///Bit 13 - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode in which it can transmit and receive simultaneously.
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, MACCRrs> {
        DM_W::new(self, 13)
    }
    ///Bit 14 - MAC Speed This bit selects the speed in the 10/100 Mbps mode:
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, MACCRrs> {
        FES_W::new(self, 14)
    }
    ///Bit 16 - Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see .
    #[inline(always)]
    pub fn je(&mut self) -> JE_W<'_, MACCRrs> {
        JE_W::new(self, 16)
    }
    ///Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer packets of up to 16,383 bytes. When this bit is reset, if the application sends more than 2,048 bytes of data (10,240 if JE is set high) during transmission, the MAC does not send rest of the bytes in that packet.
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, MACCRrs> {
        JD_W::new(self, 17)
    }
    ///Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive packets of up to 16,383 bytes. When this bit is reset, the MAC does not allow more than 2,048 bytes (10,240 if JE is set high) of the packet being received. The MAC cuts off any bytes received after 2,048 bytes.
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, MACCRrs> {
        WD_W::new(self, 19)
    }
    ///Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes. All received packets with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming packets to the application, without any modification. Note: For information about how the settings of CST bit and this bit impact the packet length, see .
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<'_, MACCRrs> {
        ACS_W::new(self, 20)
    }
    ///Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled. Note: For information about how the settings of the ACS bit and this bit impact the packet length, see .
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<'_, MACCRrs> {
        CST_W::new(self, 21)
    }
    ///Bit 22 - IEEE 802.3as Support for 2K Packets When this bit is set, the MAC considers all packets with up to 2,000 bytes length as normal packets. When the JE bit is not set, the MAC considers all received packets of size more than 2K bytes as Giant packets. When this bit is reset and the JE bit is not set, the MAC considers all received packets of size more than 1,518 bytes (1,522 bytes for tagged) as giant packets. For more information about how the setting of this bit and the JE bit impact the Giant packet status, see Packet Status based on S2KP and JE Bits. Note: When the JE bit is set, setting this bit has no effect on the giant packet status.
    #[inline(always)]
    pub fn s2kp(&mut self) -> S2KP_W<'_, MACCRrs> {
        S2KP_W::new(self, 22)
    }
    ///Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in ETH_MACECR register to declare a received packet as Giant packet. This field must be programmed to more than 1,518 bytes. Otherwise, the MAC considers 1,518 bytes as giant packet limit. When this bit is reset, the MAC considers a received packet as Giant packet when its size is greater than 1,518 bytes (1522 bytes for tagged packet). The watchdog timeout limit, Jumbo Packet Enable and 2K Packet Enable have higher precedence over this bit, that is the MAC considers a received packet as Giant packet when its size is greater than 9,018 bytes (9,022 bytes for tagged packet) with Jumbo Packet Enabled and greater than 2,000 bytes with 2K Packet Enabled. The watchdog timeout, if enabled, terminates the received packet when watchdog limit is reached. Therefore, the programmed giant packet limit should be less than the watchdog limit to get the giant packet status.
    #[inline(always)]
    pub fn gpslce(&mut self) -> GPSLCE_W<'_, MACCRrs> {
        GPSLCE_W::new(self, 23)
    }
    ///Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission. ... This range of minimum IPG is valid in full-duplex mode. In the half-duplex mode, the minimum IPG can be configured only for 64-bit times (IPG = 100). Lower values are not considered. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IPG. The above function (IPG less than 96 bit times) is valid only when EIPGEN bit in ETH_MACECR register is reset. When EIPGEN is set, then the minimum IPG (greater than 96 bit times) is controlled as per the description given in EIPG field in ETH_MACECR register.
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W<'_, MACCRrs> {
        IPG_W::new(self, 24)
    }
    ///Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking. When this bit is reset, the COE function in the receiver is disabled. The Layer 3 and Layer 4 Packet Filter feature automatically selects the IPC Full Checksum Offload Engine on the Receive side. When this feature is enabled, you must set the IPC bit.
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<'_, MACCRrs> {
        IPC_W::new(self, 27)
    }
    ///Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted packets. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits\[29:28\]: Others: Reserved, must not be used. Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
    #[inline(always)]
    pub fn sarc(&mut self) -> SARC_W<'_, MACCRrs> {
        SARC_W::new(self, 28)
    }
    ///Bit 31 - ARP Offload Enable When this bit is set, the MAC can recognize an incoming ARP request packet and schedules the ARP packet for transmission. It will forward the ARP packet to the application and also indicate the events in the RxStatus. When this bit is reset, the MAC receiver does not recognize any ARP packet and indicates them as Type frame in the RxStatus.
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W<'_, MACCRrs> {
        ARPEN_W::new(self, 31)
    }
}
/**Operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`maccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACCR)*/
pub struct MACCRrs;
impl crate::RegisterSpec for MACCRrs {
    type Ux = u32;
}
///`read()` method returns [`maccr::R`](R) reader structure
impl crate::Readable for MACCRrs {}
///`write(|w| ..)` method takes [`maccr::W`](W) writer structure
impl crate::Writable for MACCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACCR to value 0
impl crate::Resettable for MACCRrs {}
