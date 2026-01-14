///Register `DOEPTSIZ0` reader
pub type R = crate::R<DOEPTSIZ0rs>;
///Register `DOEPTSIZ0` writer
pub type W = crate::W<DOEPTSIZ0rs>;
///Field `XFRSIZ` reader - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
pub type XFRSIZ_R = crate::FieldReader;
///Field `XFRSIZ` writer - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKTCNT` reader - Packet count This field is decremented to zero after a packet is written into the Rx FIFO.
pub type PKTCNT_R = crate::BitReader;
///Field `PKTCNT` writer - Packet count This field is decremented to zero after a packet is written into the Rx FIFO.
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUPCNT` reader - SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
pub type STUPCNT_R = crate::FieldReader;
///Field `STUPCNT` writer - SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
pub type STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6 - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 19 - Packet count This field is decremented to zero after a packet is written into the Rx FIFO.
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 29:30 - SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ0")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("stupcnt", &self.stupcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Transfer size Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, DOEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bit 19 - Packet count This field is decremented to zero after a packet is written into the Rx FIFO.
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
    #[inline(always)]
    pub fn stupcnt(&mut self) -> STUPCNT_W<'_, DOEPTSIZ0rs> {
        STUPCNT_W::new(self, 29)
    }
}
/**OTG device OUT endpoint 0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DOEPTSIZ0)*/
pub struct DOEPTSIZ0rs;
impl crate::RegisterSpec for DOEPTSIZ0rs {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz0::R`](R) reader structure
impl crate::Readable for DOEPTSIZ0rs {}
///`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure
impl crate::Writable for DOEPTSIZ0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPTSIZ0 to value 0
impl crate::Resettable for DOEPTSIZ0rs {}
