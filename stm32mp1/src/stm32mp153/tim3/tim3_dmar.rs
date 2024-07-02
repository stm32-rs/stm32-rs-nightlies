///Register `TIM3_DMAR` reader
pub type R = crate::R<TIM3_DMARrs>;
///Register `TIM3_DMAR` writer
pub type W = crate::W<TIM3_DMARrs>;
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
        f.debug_struct("TIM3_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMAB
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<TIM3_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM3 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim3_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM3:TIM3_DMAR)*/
pub struct TIM3_DMARrs;
impl crate::RegisterSpec for TIM3_DMARrs {
    type Ux = u32;
}
///`read()` method returns [`tim3_dmar::R`](R) reader structure
impl crate::Readable for TIM3_DMARrs {}
///`write(|w| ..)` method takes [`tim3_dmar::W`](W) writer structure
impl crate::Writable for TIM3_DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM3_DMAR to value 0
impl crate::Resettable for TIM3_DMARrs {
    const RESET_VALUE: u32 = 0;
}
