#[doc = "Register `MACQTXFCR` reader"]
pub type R = crate::R<MACQTXFCRrs>;
#[doc = "Register `MACQTXFCR` writer"]
pub type W = crate::W<MACQTXFCRrs>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate This bit initiates a Pause packet in the Full-duplex mode and activates the backpressure function in the Half-duplex mode if the TFE bit is set. Full-Duplex mode: this bit should be read as 0 before writing to this register. To initiate a Pause packet, the application must set this bit to 1. During Control packet transfer, this bit continues to be set to indicate that a packet transmission is in progress. When Pause packet transmission is complete, the MAC resets this bit to 0. You should not write to this register until this bit is cleared. Half-duplex mode: When this bit is set (and TFE bit is set) in the Half-duplex mode, the MAC asserts the backpressure. During backpressure, when the MAC receives a new packet, the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the Full-duplex mode, the BPA is automatically disabled."]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate This bit initiates a Pause packet in the Full-duplex mode and activates the backpressure function in the Half-duplex mode if the TFE bit is set. Full-Duplex mode: this bit should be read as 0 before writing to this register. To initiate a Pause packet, the application must set this bit to 1. During Control packet transfer, this bit continues to be set to indicate that a packet transmission is in progress. When Pause packet transmission is complete, the MAC resets this bit to 0. You should not write to this register until this bit is cleared. Half-duplex mode: When this bit is set (and TFE bit is set) in the Half-duplex mode, the MAC asserts the backpressure. During backpressure, when the MAC receives a new packet, the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the Full-duplex mode, the BPA is automatically disabled."]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable Full-duplex mode: when this bit is set, the MAC enables the flow control operation to Tx Pause packets. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause packets. Half-duplex mode: when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable Full-duplex mode: when this bit is set, the MAC enables the flow control operation to Tx Pause packets. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause packets. Half-duplex mode: when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow is checked for automatic retransmission of the Pause packet. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot times), and PLT = 001, a second Pause packet is automatically transmitted at 228 (256-28) slot times after the first Pause packet is transmitted. The following list provides the threshold values for different values: 110 to 111: Reserved, must not be used The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface. This (approximate) computation is based on the packet size (64, 1518, 2000, 9018, 16384, or 32768) + 2 Pause Packet Size + IPG in Slot Times."]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow is checked for automatic retransmission of the Pause packet. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot times), and PLT = 001, a second Pause packet is automatically transmitted at 228 (256-28) slot times after the first Pause packet is transmitted. The following list provides the threshold values for different values: 110 to 111: Reserved, must not be used The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface. This (approximate) computation is based on the packet size (64, 1518, 2000, 9018, 16384, or 32768) + 2 Pause Packet Size + IPG in Slot Times."]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the zero-quanta Pause packets. When this bit is reset, normal operation with automatic zero-quanta Pause packet generation is enabled."]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the zero-quanta Pause packets. When this bit is reset, normal operation with automatic zero-quanta Pause packet generation is enabled."]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time This field holds the value to be used in the Pause Time field in the Tx control packet. I"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time This field holds the value to be used in the Pause Time field in the Tx control packet. I"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause packet in the Full-duplex mode and activates the backpressure function in the Half-duplex mode if the TFE bit is set. Full-Duplex mode: this bit should be read as 0 before writing to this register. To initiate a Pause packet, the application must set this bit to 1. During Control packet transfer, this bit continues to be set to indicate that a packet transmission is in progress. When Pause packet transmission is complete, the MAC resets this bit to 0. You should not write to this register until this bit is cleared. Half-duplex mode: When this bit is set (and TFE bit is set) in the Half-duplex mode, the MAC asserts the backpressure. During backpressure, when the MAC receives a new packet, the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the Full-duplex mode, the BPA is automatically disabled."]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable Full-duplex mode: when this bit is set, the MAC enables the flow control operation to Tx Pause packets. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause packets. Half-duplex mode: when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow is checked for automatic retransmission of the Pause packet. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot times), and PLT = 001, a second Pause packet is automatically transmitted at 228 (256-28) slot times after the first Pause packet is transmitted. The following list provides the threshold values for different values: 110 to 111: Reserved, must not be used The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface. This (approximate) computation is based on the packet size (64, 1518, 2000, 9018, 16384, or 32768) + 2 Pause Packet Size + IPG in Slot Times."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the zero-quanta Pause packets. When this bit is reset, normal operation with automatic zero-quanta Pause packet generation is enabled."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the Tx control packet. I"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause packet in the Full-duplex mode and activates the backpressure function in the Half-duplex mode if the TFE bit is set. Full-Duplex mode: this bit should be read as 0 before writing to this register. To initiate a Pause packet, the application must set this bit to 1. During Control packet transfer, this bit continues to be set to indicate that a packet transmission is in progress. When Pause packet transmission is complete, the MAC resets this bit to 0. You should not write to this register until this bit is cleared. Half-duplex mode: When this bit is set (and TFE bit is set) in the Half-duplex mode, the MAC asserts the backpressure. During backpressure, when the MAC receives a new packet, the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the Full-duplex mode, the BPA is automatically disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<MACQTXFCRrs> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable Full-duplex mode: when this bit is set, the MAC enables the flow control operation to Tx Pause packets. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause packets. Half-duplex mode: when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<MACQTXFCRrs> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow is checked for automatic retransmission of the Pause packet. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot times), and PLT = 001, a second Pause packet is automatically transmitted at 228 (256-28) slot times after the first Pause packet is transmitted. The following list provides the threshold values for different values: 110 to 111: Reserved, must not be used The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface. This (approximate) computation is based on the packet size (64, 1518, 2000, 9018, 16384, or 32768) + 2 Pause Packet Size + IPG in Slot Times."]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACQTXFCRrs> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the zero-quanta Pause packets. When this bit is reset, normal operation with automatic zero-quanta Pause packet generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<MACQTXFCRrs> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the Tx control packet. I"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACQTXFCRrs> {
        PT_W::new(self, 16)
    }
}
#[doc = "Tx Queue flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macqtxfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macqtxfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACQTXFCRrs;
impl crate::RegisterSpec for MACQTXFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macqtxfcr::R`](R) reader structure"]
impl crate::Readable for MACQTXFCRrs {}
#[doc = "`write(|w| ..)` method takes [`macqtxfcr::W`](W) writer structure"]
impl crate::Writable for MACQTXFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACQTXFCR to value 0"]
impl crate::Resettable for MACQTXFCRrs {
    const RESET_VALUE: u32 = 0;
}
