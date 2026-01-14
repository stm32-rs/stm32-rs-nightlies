///Register `TIM1_PSC` reader
pub type R = crate::R<TIM1_PSCrs>;
///Register `TIM1_PSC` writer
pub type W = crate::W<TIM1_PSCrs>;
///Field `PSC` reader - Prescaler value The counter clock frequency (CK_CNT) is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode ).
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - Prescaler value The counter clock frequency (CK_CNT) is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode ).
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode ).
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode ).
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<'_, TIM1_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM1 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim1_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM1:TIM1_PSC)*/
pub struct TIM1_PSCrs;
impl crate::RegisterSpec for TIM1_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim1_psc::R`](R) reader structure
impl crate::Readable for TIM1_PSCrs {}
///`write(|w| ..)` method takes [`tim1_psc::W`](W) writer structure
impl crate::Writable for TIM1_PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_PSC to value 0
impl crate::Resettable for TIM1_PSCrs {}
