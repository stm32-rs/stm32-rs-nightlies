///Register `TIM15_PSC` reader
pub type R = crate::R<TIM15_PSCrs>;
///Register `TIM15_PSC` writer
pub type W = crate::W<TIM15_PSCrs>;
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
        f.debug_struct("TIM15_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<'_, TIM15_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM15 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim15_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_PSC)*/
pub struct TIM15_PSCrs;
impl crate::RegisterSpec for TIM15_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_psc::R`](R) reader structure
impl crate::Readable for TIM15_PSCrs {}
///`write(|w| ..)` method takes [`tim15_psc::W`](W) writer structure
impl crate::Writable for TIM15_PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_PSC to value 0
impl crate::Resettable for TIM15_PSCrs {}
