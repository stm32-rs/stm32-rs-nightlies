///Register `TIMx_DMAR` reader
pub type R = crate::R<TIMX_DMARrs>;
///Register `TIMx_DMAR` writer
pub type W = crate::W<TIMX_DMARrs>;
///Field `DMAB` reader - DMAB
pub type DMAB_R = crate::FieldReader<u16>;
///Field `DMAB` writer - DMAB
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DMAB
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMx_DMAR")
            .field("dmab", &self.dmab())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DMAB
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<TIMX_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
/**TIM16/TIM17 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`timx_dmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_dmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM16:TIMx_DMAR)*/
pub struct TIMX_DMARrs;
impl crate::RegisterSpec for TIMX_DMARrs {
    type Ux = u16;
}
///`read()` method returns [`timx_dmar::R`](R) reader structure
impl crate::Readable for TIMX_DMARrs {}
///`write(|w| ..)` method takes [`timx_dmar::W`](W) writer structure
impl crate::Writable for TIMX_DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIMx_DMAR to value 0
impl crate::Resettable for TIMX_DMARrs {
    const RESET_VALUE: u16 = 0;
}
