///Register `TIM2_PSC` reader
pub type R = crate::R<TIM2_PSCrs>;
///Register `TIM2_PSC` writer
pub type W = crate::W<TIM2_PSCrs>;
///Field `PSC` reader - Prescaler value
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - Prescaler value
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<'_, TIM2_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM2 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim2_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM2:TIM2_PSC)*/
pub struct TIM2_PSCrs;
impl crate::RegisterSpec for TIM2_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim2_psc::R`](R) reader structure
impl crate::Readable for TIM2_PSCrs {}
///`write(|w| ..)` method takes [`tim2_psc::W`](W) writer structure
impl crate::Writable for TIM2_PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_PSC to value 0
impl crate::Resettable for TIM2_PSCrs {}
