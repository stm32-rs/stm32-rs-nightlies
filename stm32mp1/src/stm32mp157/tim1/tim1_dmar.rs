///Register `TIM1_DMAR` reader
pub type R = crate::R<TIM1_DMARrs>;
///Register `TIM1_DMAR` writer
pub type W = crate::W<TIM1_DMARrs>;
///Field `DMAB` reader - DMAB
pub type DMAB_R = crate::FieldReader<u32>;
///Field `DMAB` writer - DMAB
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMAB
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAB
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<TIM1_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM1 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim1_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM1:TIM1_DMAR)*/
pub struct TIM1_DMARrs;
impl crate::RegisterSpec for TIM1_DMARrs {
    type Ux = u32;
}
///`read()` method returns [`tim1_dmar::R`](R) reader structure
impl crate::Readable for TIM1_DMARrs {}
///`write(|w| ..)` method takes [`tim1_dmar::W`](W) writer structure
impl crate::Writable for TIM1_DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM1_DMAR to value 0
impl crate::Resettable for TIM1_DMARrs {
    const RESET_VALUE: u32 = 0;
}
