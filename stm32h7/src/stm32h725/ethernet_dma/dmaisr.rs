///Register `DMAISR` reader
pub type R = crate::R<DMAISRrs>;
///Register `DMAISR` writer
pub type W = crate::W<DMAISRrs>;
///Field `DC0IS` reader - DMA Channel Interrupt Status
pub type DC0IS_R = crate::BitReader;
///Field `DC0IS` writer - DMA Channel Interrupt Status
pub type DC0IS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MTLIS` reader - MTL Interrupt Status
pub type MTLIS_R = crate::BitReader;
///Field `MTLIS` writer - MTL Interrupt Status
pub type MTLIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MACIS` reader - MAC Interrupt Status
pub type MACIS_R = crate::BitReader;
///Field `MACIS` writer - MAC Interrupt Status
pub type MACIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA Channel Interrupt Status
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - MTL Interrupt Status
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MAC Interrupt Status
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAISR")
            .field("macis", &self.macis())
            .field("mtlis", &self.mtlis())
            .field("dc0is", &self.dc0is())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA Channel Interrupt Status
    #[inline(always)]
    pub fn dc0is(&mut self) -> DC0IS_W<'_, DMAISRrs> {
        DC0IS_W::new(self, 0)
    }
    ///Bit 16 - MTL Interrupt Status
    #[inline(always)]
    pub fn mtlis(&mut self) -> MTLIS_W<'_, DMAISRrs> {
        MTLIS_W::new(self, 16)
    }
    ///Bit 17 - MAC Interrupt Status
    #[inline(always)]
    pub fn macis(&mut self) -> MACIS_W<'_, DMAISRrs> {
        MACIS_W::new(self, 17)
    }
}
/**Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#Ethernet_DMA:DMAISR)*/
pub struct DMAISRrs;
impl crate::RegisterSpec for DMAISRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaisr::R`](R) reader structure
impl crate::Readable for DMAISRrs {}
///`write(|w| ..)` method takes [`dmaisr::W`](W) writer structure
impl crate::Writable for DMAISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAISR to value 0
impl crate::Resettable for DMAISRrs {}
