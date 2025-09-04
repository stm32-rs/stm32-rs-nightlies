///Register `PSC` reader
pub type R = crate::R<PSCrs>;
///Register `PSC` writer
pub type W = crate::W<PSCrs>;
///Field `PSC` reader - PSC\[15:0\]: Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode').
pub type PSC_R = crate::FieldReader<u16>;
///Field `PSC` writer - PSC\[15:0\]: Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode').
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - PSC\[15:0\]: Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode').
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSC").field("psc", &self.psc()).finish()
    }
}
impl W {
    ///Bits 0:15 - PSC\[15:0\]: Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\[15:0\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode').
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<PSCrs> {
        PSC_W::new(self, 0)
    }
}
/**PSC register

You can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM16:PSC)*/
pub struct PSCrs;
impl crate::RegisterSpec for PSCrs {
    type Ux = u32;
}
///`read()` method returns [`psc::R`](R) reader structure
impl crate::Readable for PSCrs {}
///`write(|w| ..)` method takes [`psc::W`](W) writer structure
impl crate::Writable for PSCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSC to value 0
impl crate::Resettable for PSCrs {}
