///Register `TIMx_PSC` reader
pub type R = crate::R<TIMX_PSCrs>;
///Register `TIMx_PSC` writer
pub type W = crate::W<TIMX_PSCrs>;
///Field `PSC` reader - PSC
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - PSC
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PSC
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMx_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - PSC
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIMX_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM16/TIM17 prescaler

You can [`read`](crate::Reg::read) this register and get [`timx_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM16:TIMx_PSC)*/
pub struct TIMX_PSCrs;
impl crate::RegisterSpec for TIMX_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`timx_psc::R`](R) reader structure
impl crate::Readable for TIMX_PSCrs {}
///`write(|w| ..)` method takes [`timx_psc::W`](W) writer structure
impl crate::Writable for TIMX_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIMx_PSC to value 0
impl crate::Resettable for TIMX_PSCrs {
    const RESET_VALUE: u16 = 0;
}
