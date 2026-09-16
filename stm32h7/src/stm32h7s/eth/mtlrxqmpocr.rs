///Register `MTLRXQMPOCR` reader
pub type R = crate::R<MTLRXQMPOCRrs>;
///Register `MTLRXQMPOCR` writer
pub type W = crate::W<MTLRXQMPOCRrs>;
/**Field `OVFPKTCNT` reader - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
///Field `OVFPKTCNT` writer - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read.
pub type OVFPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type OVFCNTOVF_R = crate::BitReader;
///Field `OVFCNTOVF` writer - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
pub type OVFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `MISPKTCNT` reader - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MISPKTCNT_R = crate::FieldReader<u16>;
///Field `MISPKTCNT` writer - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability.
pub type MISPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MISCNTOVF_R = crate::BitReader;
///Field `MISCNTOVF` writer - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.
pub type MISCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read.
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:26 - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability.
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 27 - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQMPOCR").finish()
    }
}
impl W {
    ///Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet peripheral because of Receive queue overflow. This counter is incremented each time the Ethernet peripheral discards an incoming packet because of overflow. This counter is reset when this register is read.
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W<'_, MTLRXQMPOCRrs> {
        OVFPKTCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W<'_, MTLRXQMPOCRrs> {
        OVFCNTOVF_W::new(self, 11)
    }
    ///Bits 16:26 - Missed Packet Counter This field indicates the number of packets missed by the Ethernet peripheral because the application requested to flush the packets for this queue. This counter is reset when this register is read. This counter is incremented by 1 when the DMA discards the packet because of buffer unavailability.
    #[inline(always)]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W<'_, MTLRXQMPOCRrs> {
        MISPKTCNT_W::new(self, 16)
    }
    ///Bit 27 - Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W<'_, MTLRXQMPOCRrs> {
        MISCNTOVF_W::new(self, 27)
    }
}
/**Rx queue missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrxqmpocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxqmpocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MTLRXQMPOCR)*/
pub struct MTLRXQMPOCRrs;
impl crate::RegisterSpec for MTLRXQMPOCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxqmpocr::R`](R) reader structure
impl crate::Readable for MTLRXQMPOCRrs {}
///`write(|w| ..)` method takes [`mtlrxqmpocr::W`](W) writer structure
impl crate::Writable for MTLRXQMPOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRXQMPOCR to value 0
impl crate::Resettable for MTLRXQMPOCRrs {}
