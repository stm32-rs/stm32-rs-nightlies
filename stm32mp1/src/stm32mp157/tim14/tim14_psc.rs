///Register `TIM14_PSC` reader
pub type R = crate::R<TIM14_PSCrs>;
///Register `TIM14_PSC` writer
pub type W = crate::W<TIM14_PSCrs>;
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
        f.debug_struct("TIM14_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - PSC
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIM14_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM14 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim14_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:TIM14_PSC)*/
pub struct TIM14_PSCrs;
impl crate::RegisterSpec for TIM14_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim14_psc::R`](R) reader structure
impl crate::Readable for TIM14_PSCrs {}
///`write(|w| ..)` method takes [`tim14_psc::W`](W) writer structure
impl crate::Writable for TIM14_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM14_PSC to value 0
impl crate::Resettable for TIM14_PSCrs {
    const RESET_VALUE: u16 = 0;
}
