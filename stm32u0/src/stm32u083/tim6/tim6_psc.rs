///Register `TIM6_PSC` reader
pub type R = crate::R<TIM6_PSCrs>;
///Register `TIM6_PSC` writer
pub type W = crate::W<TIM6_PSCrs>;
/**Field `PSC` reader - Prescaler value The counter clock frequency CK_CNT is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\]
+ 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode).*/
pub type PSC_R = crate::FieldReader<u16>;
/**Field `PSC` writer - Prescaler value The counter clock frequency CK_CNT is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\]
+ 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode).*/
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:15 - Prescaler value The counter clock frequency CK_CNT is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\]
    + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode).*/
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6_PSC")
            .field("psc", &self.psc())
            .finish()
    }
}
impl W {
    /**Bits 0:15 - Prescaler value The counter clock frequency CK_CNT is equal to f<sub>CK_PSC</sub> / (PSC\[15:0\]
    + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in reset mode).*/
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<TIM6_PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**TIM6 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim6_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM6:TIM6_PSC)*/
pub struct TIM6_PSCrs;
impl crate::RegisterSpec for TIM6_PSCrs {
    type Ux = u16;
}
///`read()` method returns [`tim6_psc::R`](R) reader structure
impl crate::Readable for TIM6_PSCrs {}
///`write(|w| ..)` method takes [`tim6_psc::W`](W) writer structure
impl crate::Writable for TIM6_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM6_PSC to value 0
impl crate::Resettable for TIM6_PSCrs {
    const RESET_VALUE: u16 = 0;
}