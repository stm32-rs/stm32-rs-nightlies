///Register `FDCAN_RXF1A` reader
pub type R = crate::R<FDCAN_RXF1Ars>;
///Register `FDCAN_RXF1A` writer
pub type W = crate::W<FDCAN_RXF1Ars>;
///Field `F1AI` reader - Rx FIFO 1 acknowledge index
pub type F1AI_R = crate::FieldReader;
///Field `F1AI` writer - Rx FIFO 1 acknowledge index
pub type F1AI_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1A")
            .field("f1ai", &self.f1ai())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1AI_W<'_, FDCAN_RXF1Ars> {
        F1AI_W::new(self, 0)
    }
}
/**FDCAN Rx FIFO 1 acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_RXF1A)*/
pub struct FDCAN_RXF1Ars;
impl crate::RegisterSpec for FDCAN_RXF1Ars {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf1a::R`](R) reader structure
impl crate::Readable for FDCAN_RXF1Ars {}
///`write(|w| ..)` method takes [`fdcan_rxf1a::W`](W) writer structure
impl crate::Writable for FDCAN_RXF1Ars {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_RXF1A to value 0
impl crate::Resettable for FDCAN_RXF1Ars {}
