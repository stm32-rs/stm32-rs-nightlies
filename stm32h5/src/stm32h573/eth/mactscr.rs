#[doc = "Register `MACTSCR` reader"]
pub type R = crate::R<MACTSCRrs>;
#[doc = "Register `MACTSCR` writer"]
pub type W = crate::W<MACTSCRrs>;
#[doc = "Field `TSENA` reader - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets. When disabled, timestamp is not added for transmit and receive packets and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the Receive side, the MAC processes the 1588 packets only if this bit is set."]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets. When disabled, timestamp is not added for transmit and receive packets and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the Receive side, the MAC processes the 1588 packets only if this bit is set."]
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp. When this bit is reset, Coarse method is used to update the system timestamp."]
pub type TSCFUPDT_R = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp. When this bit is reset, Coarse method is used to update the system timestamp."]
pub type TSCFUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before it is updated. This bit is reset when the initialization is complete."]
pub type TSINIT_R = crate::BitReader;
#[doc = "Field `TSINIT` writer - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before it is updated. This bit is reset when the initialization is complete."]
pub type TSINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before updating it. This bit is reset when the update is complete in hardware."]
pub type TSUPDT_R = crate::BitReader;
#[doc = "Field `TSUPDT` writer - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before updating it. This bit is reset when the update is complete in hardware."]
pub type TSUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This bit is cleared when the update is complete. This bit should be zero before it is set."]
pub type TSADDREG_R = crate::BitReader;
#[doc = "Field `TSADDREG` writer - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This bit is cleared when the update is complete. This bit should be zero before it is set."]
pub type TSADDREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub type TSENALL_R = crate::BitReader;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub type TSENALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When this bit is reset, the rollover value of subsecond register is 0x7FFF_FFFF. The subsecond increment must be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
pub type TSCTRLSSR_R = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When this bit is reset, the rollover value of subsecond register is 0x7FFF_FFFF. The subsecond increment must be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
pub type TSCTRLSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets. When this bit is reset, the IEEE 1588 version 1 format is used to process the PTP packets. The IEEE 1588 formats are described in 'PTP Processing and Control'."]
pub type TSVER2ENA_R = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets. When this bit is reset, the IEEE 1588 version 1 format is used to process the PTP packets. The IEEE 1588 formats are described in 'PTP Processing and Control'."]
pub type TSVER2ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets. When this bit is reset, the MAC ignores the PTP over Ethernet packets."]
pub type TSIPENA_R = crate::BitReader;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets. When this bit is reset, the MAC ignores the PTP over Ethernet packets."]
pub type TSIPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets. When this bit is clear, the MAC ignores the PTP transported over IPv6-UDP packets."]
pub type TSIPV6ENA_R = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets. When this bit is clear, the MAC ignores the PTP transported over IPv6-UDP packets."]
pub type TSIPV6ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets. When this bit is reset, the MAC ignores the PTP transported over IPv4-UDP packets. This bit is set by default."]
pub type TSIPV4ENA_R = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets. When this bit is reset, the MAC ignores the PTP transported over IPv4-UDP packets. This bit is set by default."]
pub type TSIPV4ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When this bit is reset, the snapshot is taken for all messages except Announce, Management, and Signaling. For more information about the timestamp snapshots, see Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
pub type TSEVNTENA_R = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When this bit is reset, the snapshot is taken for all messages except Announce, Management, and Signaling. For more information about the timestamp snapshots, see Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
pub type TSEVNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
pub type TSMSTRENA_R = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
pub type TSMSTRENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, define the set of PTP packet types for which snapshot needs to be taken. The encoding is given in Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
pub type SNAPTYPSEL_R = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, define the set of PTP packet types for which snapshot needs to be taken. The encoding is given in Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
pub type SNAPTYPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet. When this bit is set, received PTP packets with DA containing a special multicast or unicast address that matches the one programmed in MAC address registers are considered for processing as indicated below, when PTP is directly sent over Ethernet. For normal timestamping operation, MAC address registers 0 to 31 is considered for unicast destination address matching. For PTP offload, only MAC address register 0 is considered for unicast destination address matching."]
pub type TSENMACADDR_R = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet. When this bit is set, received PTP packets with DA containing a special multicast or unicast address that matches the one programmed in MAC address registers are considered for processing as indicated below, when PTP is directly sent over Ethernet. For normal timestamping operation, MAC address registers 0 to 31 is considered for unicast destination address matching. For PTP offload, only MAC address register 0 is considered for unicast destination address matching."]
pub type TSENMACADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTSSTSM` reader - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) register. When this bit is reset, the MAC ignores the timestamp status of current packet if the timestamp status of previous packet is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR)."]
pub type TXTSSTSM_R = crate::BitReader;
#[doc = "Field `TXTSSTSM` writer - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) register. When this bit is reset, the MAC ignores the timestamp status of current packet if the timestamp status of previous packet is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR)."]
pub type TXTSSTSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AV8021ASMEN` reader - AV 802.1AS Mode Enable When this bit is set, the MAC processes only untagged PTP over Ethernet packets for providing PTP status and capturing timestamp snapshots, that is, IEEE 802.1AS operating mode. When PTP offload feature is enabled, for the purpose of PTP offload, the transport specific field in the PTP header is generated and checked based on the value of this bit."]
pub type AV8021ASMEN_R = crate::BitReader;
#[doc = "Field `AV8021ASMEN` writer - AV 802.1AS Mode Enable When this bit is set, the MAC processes only untagged PTP over Ethernet packets for providing PTP status and capturing timestamp snapshots, that is, IEEE 802.1AS operating mode. When PTP offload feature is enabled, for the purpose of PTP offload, the transport specific field in the PTP header is generated and checked based on the value of this bit."]
pub type AV8021ASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets. When disabled, timestamp is not added for transmit and receive packets and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the Receive side, the MAC processes the 1588 packets only if this bit is set."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp. When this bit is reset, Coarse method is used to update the system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before it is updated. This bit is reset when the initialization is complete."]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before updating it. This bit is reset when the update is complete in hardware."]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This bit is cleared when the update is complete. This bit should be zero before it is set."]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When this bit is reset, the rollover value of subsecond register is 0x7FFF_FFFF. The subsecond increment must be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets. When this bit is reset, the IEEE 1588 version 1 format is used to process the PTP packets. The IEEE 1588 formats are described in 'PTP Processing and Control'."]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets. When this bit is reset, the MAC ignores the PTP over Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets. When this bit is clear, the MAC ignores the PTP transported over IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets. When this bit is reset, the MAC ignores the PTP transported over IPv4-UDP packets. This bit is set by default."]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When this bit is reset, the snapshot is taken for all messages except Announce, Management, and Signaling. For more information about the timestamp snapshots, see Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, define the set of PTP packet types for which snapshot needs to be taken. The encoding is given in Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet. When this bit is set, received PTP packets with DA containing a special multicast or unicast address that matches the one programmed in MAC address registers are considered for processing as indicated below, when PTP is directly sent over Ethernet. For normal timestamping operation, MAC address registers 0 to 31 is considered for unicast destination address matching. For PTP offload, only MAC address register 0 is considered for unicast destination address matching."]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) register. When this bit is reset, the MAC ignores the timestamp status of current packet if the timestamp status of previous packet is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR)."]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - AV 802.1AS Mode Enable When this bit is set, the MAC processes only untagged PTP over Ethernet packets for providing PTP status and capturing timestamp snapshots, that is, IEEE 802.1AS operating mode. When PTP offload feature is enabled, for the purpose of PTP offload, the transport specific field in the PTP header is generated and checked based on the value of this bit."]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets. When disabled, timestamp is not added for transmit and receive packets and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the Receive side, the MAC processes the 1588 packets only if this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<MACTSCRrs> {
        TSENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp. When this bit is reset, Coarse method is used to update the system timestamp."]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<MACTSCRrs> {
        TSCFUPDT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before it is updated. This bit is reset when the initialization is complete."]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TSINIT_W<MACTSCRrs> {
        TSINIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in System time seconds update register (ETH_MACSTSUR) and System time nanoseconds update register (ETH_MACSTNUR). This bit should be zero before updating it. This bit is reset when the update is complete in hardware."]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TSUPDT_W<MACTSCRrs> {
        TSUPDT_W::new(self, 3)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This bit is cleared when the update is complete. This bit should be zero before it is set."]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<MACTSCRrs> {
        TSADDREG_W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TSENALL_W<MACTSCRrs> {
        TSENALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When this bit is reset, the rollover value of subsecond register is 0x7FFF_FFFF. The subsecond increment must be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<MACTSCRrs> {
        TSCTRLSSR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets. When this bit is reset, the IEEE 1588 version 1 format is used to process the PTP packets. The IEEE 1588 formats are described in 'PTP Processing and Control'."]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<MACTSCRrs> {
        TSVER2ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets. When this bit is reset, the MAC ignores the PTP over Ethernet packets."]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TSIPENA_W<MACTSCRrs> {
        TSIPENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets. When this bit is clear, the MAC ignores the PTP transported over IPv6-UDP packets."]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<MACTSCRrs> {
        TSIPV6ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets. When this bit is reset, the MAC ignores the PTP transported over IPv4-UDP packets. This bit is set by default."]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<MACTSCRrs> {
        TSIPV4ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When this bit is reset, the snapshot is taken for all messages except Announce, Management, and Signaling. For more information about the timestamp snapshots, see Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<MACTSCRrs> {
        TSEVNTENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<MACTSCRrs> {
        TSMSTRENA_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, define the set of PTP packet types for which snapshot needs to be taken. The encoding is given in Table�634: Timestamp Snapshot Dependency on ETH_MACTSCR bits."]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<MACTSCRrs> {
        SNAPTYPSEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet. When this bit is set, received PTP packets with DA containing a special multicast or unicast address that matches the one programmed in MAC address registers are considered for processing as indicated below, when PTP is directly sent over Ethernet. For normal timestamping operation, MAC address registers 0 to 31 is considered for unicast destination address matching. For PTP offload, only MAC address register 0 is considered for unicast destination address matching."]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<MACTSCRrs> {
        TSENMACADDR_W::new(self, 18)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) register. When this bit is reset, the MAC ignores the timestamp status of current packet if the timestamp status of previous packet is not read by the software. The MAC indicates this by setting the TXTSSMIS bit of the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR)."]
    #[inline(always)]
    #[must_use]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W<MACTSCRrs> {
        TXTSSTSM_W::new(self, 24)
    }
    #[doc = "Bit 28 - AV 802.1AS Mode Enable When this bit is set, the MAC processes only untagged PTP over Ethernet packets for providing PTP status and capturing timestamp snapshots, that is, IEEE 802.1AS operating mode. When PTP offload feature is enabled, for the purpose of PTP offload, the transport specific field in the PTP header is generated and checked based on the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W<MACTSCRrs> {
        AV8021ASMEN_W::new(self, 28)
    }
}
#[doc = "Timestamp control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSCRrs;
impl crate::RegisterSpec for MACTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactscr::R`](R) reader structure"]
impl crate::Readable for MACTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`mactscr::W`](W) writer structure"]
impl crate::Writable for MACTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSCR to value 0x2000"]
impl crate::Resettable for MACTSCRrs {
    const RESET_VALUE: u32 = 0x2000;
}
