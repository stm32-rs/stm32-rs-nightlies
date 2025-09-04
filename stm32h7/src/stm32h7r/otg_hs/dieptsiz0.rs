///Register `DIEPTSIZ0` reader
pub type R = crate::R<DIEPTSIZ0rs>;
///Register `DIEPTSIZ0` writer
pub type W = crate::W<DIEPTSIZ0rs>;
///Field `XFRSIZ` reader - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
pub type XFRSIZ_R = crate::FieldReader;
///Field `XFRSIZ` writer - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKTCNT` reader - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for endpoint 0. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
pub type PKTCNT_R = crate::FieldReader;
///Field `PKTCNT` writer - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for endpoint 0. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6 - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 19:20 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for endpoint 0. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ0")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet from the external memory is written to the Tx FIFO.
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:20 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for endpoint 0. This field is decremented every time a packet (maximum size or short packet) is read from the Tx FIFO.
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
}
/**OTG device IN endpoint 0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DIEPTSIZ0)*/
pub struct DIEPTSIZ0rs;
impl crate::RegisterSpec for DIEPTSIZ0rs {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz0::R`](R) reader structure
impl crate::Readable for DIEPTSIZ0rs {}
///`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure
impl crate::Writable for DIEPTSIZ0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTSIZ0 to value 0
impl crate::Resettable for DIEPTSIZ0rs {}
