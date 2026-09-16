///Register `DMAA4TXACR` reader
pub type R = crate::R<DMAA4TXACRrs>;
///Register `DMAA4TXACR` writer
pub type W = crate::W<DMAA4TXACRrs>;
///Field `TDRC` reader - Transmit DMA Read Descriptor Cache Control
pub type TDRC_R = crate::FieldReader;
///Field `TDRC` writer - Transmit DMA Read Descriptor Cache Control
pub type TDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TEC` reader - Transmit DMA Extended Packet Buffer or TSO Payload Cache Control
pub type TEC_R = crate::FieldReader;
///Field `TEC` writer - Transmit DMA Extended Packet Buffer or TSO Payload Cache Control
pub type TEC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `THC` reader - Transmit DMA First Packet Buffer or TSO Header Cache Control
pub type THC_R = crate::FieldReader;
///Field `THC` writer - Transmit DMA First Packet Buffer or TSO Header Cache Control
pub type THC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Transmit DMA Read Descriptor Cache Control
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Transmit DMA Extended Packet Buffer or TSO Payload Cache Control
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Transmit DMA First Packet Buffer or TSO Header Cache Control
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAA4TXACR")
            .field("tdrc", &self.tdrc())
            .field("tec", &self.tec())
            .field("thc", &self.thc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Transmit DMA Read Descriptor Cache Control
    #[inline(always)]
    pub fn tdrc(&mut self) -> TDRC_W<'_, DMAA4TXACRrs> {
        TDRC_W::new(self, 0)
    }
    ///Bits 8:11 - Transmit DMA Extended Packet Buffer or TSO Payload Cache Control
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W<'_, DMAA4TXACRrs> {
        TEC_W::new(self, 8)
    }
    ///Bits 16:19 - Transmit DMA First Packet Buffer or TSO Header Cache Control
    #[inline(always)]
    pub fn thc(&mut self) -> THC_W<'_, DMAA4TXACRrs> {
        THC_W::new(self, 16)
    }
}
/**AXI4 transmit channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4txacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4txacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAA4TXACR)*/
pub struct DMAA4TXACRrs;
impl crate::RegisterSpec for DMAA4TXACRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaa4txacr::R`](R) reader structure
impl crate::Readable for DMAA4TXACRrs {}
///`write(|w| ..)` method takes [`dmaa4txacr::W`](W) writer structure
impl crate::Writable for DMAA4TXACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAA4TXACR to value 0
impl crate::Resettable for DMAA4TXACRrs {}
