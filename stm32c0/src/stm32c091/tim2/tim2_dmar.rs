///Register `TIM2_DMAR` reader
pub type R = crate::R<TIM2_DMARrs>;
///Register `TIM2_DMAR` writer
pub type W = crate::W<TIM2_DMARrs>;
///Field `DMAB` reader - DMA register for burst accesses
pub type DMAB_R = crate::FieldReader<u16>;
///Field `DMAB` writer - DMA register for burst accesses
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W<'_, TIM2_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM2 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim2_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_DMAR)*/
pub struct TIM2_DMARrs;
impl crate::RegisterSpec for TIM2_DMARrs {
    type Ux = u16;
}
///`read()` method returns [`tim2_dmar::R`](R) reader structure
impl crate::Readable for TIM2_DMARrs {}
///`write(|w| ..)` method takes [`tim2_dmar::W`](W) writer structure
impl crate::Writable for TIM2_DMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_DMAR to value 0
impl crate::Resettable for TIM2_DMARrs {}
