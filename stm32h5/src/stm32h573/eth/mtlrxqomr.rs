#[doc = "Register `MTLRXQOMR` reader"]
pub type R = crate::R<MTLRXQOMRrs>;
#[doc = "Register `MTLRXQOMR` writer"]
pub type W = crate::W<MTLRXQOMRrs>;
#[doc = "Field `RTC` reader - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold. In addition, full packets with length less than the threshold are automatically transferred. This field is valid only when the RSF bit is zero. This field is ignored when the RSF bit is set to 1."]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold. In addition, full packets with length less than the threshold are automatically transferred. This field is valid only when the RSF bit is zero. This field is ignored when the RSF bit is set to 1."]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUP` reader - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC. When this bit is reset, the Rx queue drops all packets of less than 64 bytes, unless a packet is already transferred because of the lower value of Rx Threshold, for example, RTC = 01."]
pub type FUP_R = crate::BitReader;
#[doc = "Field `FUP` writer - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC. When this bit is reset, the Rx queue drops all packets of less than 64 bytes, unless a packet is already transferred because of the lower value of Rx Threshold, for example, RTC = 01."]
pub type FUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEP` reader - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, receive error, watchdog timeout, or overflow). However, if the start byte (write) pointer of a packet is already transferred to the read controller side (in Threshold mode), the packet is not dropped. When this bit is set, all packets except the runt error packets are forwarded to the application or DMA. If the RSF bit is set and the Rx queue overflows when a partial packet is written, the packet is dropped irrespective of the setting of this bit. However, if the RSF bit is reset and the Rx queue overflows when a partial packet is written, a partial packet may be forwarded to the application or DMA."]
pub type FEP_R = crate::BitReader;
#[doc = "Field `FEP` writer - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, receive error, watchdog timeout, or overflow). However, if the start byte (write) pointer of a packet is already transferred to the read controller side (in Threshold mode), the packet is not dropped. When this bit is set, all packets except the runt error packets are forwarded to the application or DMA. If the RSF bit is set and the Rx queue overflows when a partial packet is written, the packet is dropped irrespective of the setting of this bit. However, if the RSF bit is reset and the Rx queue overflows when a partial packet is written, a partial packet may be forwarded to the application or DMA."]
pub type FEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Queue Store and Forward When this bit is set, the Ethernet peripheral reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register. When this bit is reset, the Rx queue operates in the Threshold (cut-through) mode, subject to the threshold specified by the RTC field of this register."]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Queue Store and Forward When this bit is set, the Ethernet peripheral reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register. When this bit is reset, the Rx queue operates in the Threshold (cut-through) mode, subject to the threshold specified by the RTC field of this register."]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine. Such packets have errors only in the encapsulated payload. There are no errors (including FCS error) in the Ethernet packet received by the MAC. When this bit is reset, all error packets are dropped if the FEP bit is reset."]
pub type DIS_TCP_EF_R = crate::BitReader;
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine. Such packets have errors only in the encapsulated payload. There are no errors (including FCS error) in the Ethernet packet received by the MAC. When this bit is reset, all error packets are dropped if the FEP bit is reset."]
pub type DIS_TCP_EF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RQS` reader - Receive Queue Size This field is read-only and the configured Rx FIFO size in blocks of 256 bytes is reflected in the reset value. The size of the Queue is (RQS�+�1)�*�256 bytes."]
pub type RQS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold. In addition, full packets with length less than the threshold are automatically transferred. This field is valid only when the RSF bit is zero. This field is ignored when the RSF bit is set to 1."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC. When this bit is reset, the Rx queue drops all packets of less than 64 bytes, unless a packet is already transferred because of the lower value of Rx Threshold, for example, RTC = 01."]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, receive error, watchdog timeout, or overflow). However, if the start byte (write) pointer of a packet is already transferred to the read controller side (in Threshold mode), the packet is not dropped. When this bit is set, all packets except the runt error packets are forwarded to the application or DMA. If the RSF bit is set and the Rx queue overflows when a partial packet is written, the packet is dropped irrespective of the setting of this bit. However, if the RSF bit is reset and the Rx queue overflows when a partial packet is written, a partial packet may be forwarded to the application or DMA."]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the Ethernet peripheral reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register. When this bit is reset, the Rx queue operates in the Threshold (cut-through) mode, subject to the threshold specified by the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine. Such packets have errors only in the encapsulated payload. There are no errors (including FCS error) in the Ethernet packet received by the MAC. When this bit is reset, all error packets are dropped if the FEP bit is reset."]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Receive Queue Size This field is read-only and the configured Rx FIFO size in blocks of 256 bytes is reflected in the reset value. The size of the Queue is (RQS�+�1)�*�256 bytes."]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold. In addition, full packets with length less than the threshold are automatically transferred. This field is valid only when the RSF bit is zero. This field is ignored when the RSF bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<MTLRXQOMRrs> {
        RTC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC. When this bit is reset, the Rx queue drops all packets of less than 64 bytes, unless a packet is already transferred because of the lower value of Rx Threshold, for example, RTC = 01."]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<MTLRXQOMRrs> {
        FUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, receive error, watchdog timeout, or overflow). However, if the start byte (write) pointer of a packet is already transferred to the read controller side (in Threshold mode), the packet is not dropped. When this bit is set, all packets except the runt error packets are forwarded to the application or DMA. If the RSF bit is set and the Rx queue overflows when a partial packet is written, the packet is dropped irrespective of the setting of this bit. However, if the RSF bit is reset and the Rx queue overflows when a partial packet is written, a partial packet may be forwarded to the application or DMA."]
    #[inline(always)]
    #[must_use]
    pub fn fep(&mut self) -> FEP_W<MTLRXQOMRrs> {
        FEP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the Ethernet peripheral reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register. When this bit is reset, the Rx queue operates in the Threshold (cut-through) mode, subject to the threshold specified by the RTC field of this register."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<MTLRXQOMRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine. Such packets have errors only in the encapsulated payload. There are no errors (including FCS error) in the Ethernet packet received by the MAC. When this bit is reset, all error packets are dropped if the FEP bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<MTLRXQOMRrs> {
        DIS_TCP_EF_W::new(self, 6)
    }
}
#[doc = "Rx queue operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrxqomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrxqomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRXQOMRrs;
impl crate::RegisterSpec for MTLRXQOMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrxqomr::R`](R) reader structure"]
impl crate::Readable for MTLRXQOMRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlrxqomr::W`](W) writer structure"]
impl crate::Writable for MTLRXQOMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLRXQOMR to value 0x0070_0000"]
impl crate::Resettable for MTLRXQOMRrs {
    const RESET_VALUE: u32 = 0x0070_0000;
}
