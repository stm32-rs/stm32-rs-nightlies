///Register `TIM4_PSC` reader
pub type R = crate::R<TIM4_PSCrs>;
///Register `TIM4_PSC` writer
pub type W = crate::W<TIM4_PSCrs>;
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
        f.debug_struct("TIM4_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - PSC
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIM4_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM4 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim4_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim4_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM4:TIM4_PSC)*/
pub struct TIM4_PSCrs;
impl crate::RegisterSpec for TIM4_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim4_psc::R`](R) reader structure
impl crate::Readable for TIM4_PSCrs {}
///`write(|w| ..)` method takes [`tim4_psc::W`](W) writer structure
impl crate::Writable for TIM4_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM4_PSC to value 0
impl crate::Resettable for TIM4_PSCrs {
    const RESET_VALUE: u16 = 0;
}
