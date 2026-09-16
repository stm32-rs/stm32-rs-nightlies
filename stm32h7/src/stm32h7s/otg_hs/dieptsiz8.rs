///Register `DIEPTSIZ8` reader
pub type R = crate::R<DIEPTSIZ8rs>;
///Register `DIEPTSIZ8` writer
pub type W = crate::W<DIEPTSIZ8rs>;
///Field `XFRSIZ` reader - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `MCNT` reader - Multi count For periodic IN endpoints, this field indicates the number of packets that must be transmitted per frame on the USB. The core uses this field to calculate the data PID for isochronous IN endpoints.
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - Multi count For periodic IN endpoints, this field indicates the number of packets that must be transmitted per frame on the USB. The core uses this field to calculate the data PID for isochronous IN endpoints.
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Multi count For periodic IN endpoints, this field indicates the number of packets that must be transmitted per frame on the USB. The core uses this field to calculate the data PID for isochronous IN endpoints.
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ8")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("mcnt", &self.mcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, DIEPTSIZ8rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DIEPTSIZ8rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Multi count For periodic IN endpoints, this field indicates the number of packets that must be transmitted per frame on the USB. The core uses this field to calculate the data PID for isochronous IN endpoints.
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<'_, DIEPTSIZ8rs> {
        MCNT_W::new(self, 29)
    }
}
/**OTG device IN endpoint 8 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DIEPTSIZ8)*/
pub struct DIEPTSIZ8rs;
impl crate::RegisterSpec for DIEPTSIZ8rs {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz8::R`](R) reader structure
impl crate::Readable for DIEPTSIZ8rs {}
///`write(|w| ..)` method takes [`dieptsiz8::W`](W) writer structure
impl crate::Writable for DIEPTSIZ8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTSIZ8 to value 0
impl crate::Resettable for DIEPTSIZ8rs {}
