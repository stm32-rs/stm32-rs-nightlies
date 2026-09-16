///Register `ALRH` writer
pub type W = crate::W<ALRHrs>;
///Field `ALRH` writer - RTC alarm register high
pub type ALRH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<ALRHrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - RTC alarm register high
    #[inline(always)]
    pub fn alrh(&mut self) -> ALRH_W<'_, ALRHrs> {
        ALRH_W::new(self, 0)
    }
}
/**RTC Alarm Register High

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#RTC:ALRH)*/
pub struct ALRHrs;
impl crate::RegisterSpec for ALRHrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`alrh::W`](W) writer structure
impl crate::Writable for ALRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRH to value 0xffff
impl crate::Resettable for ALRHrs {
    const RESET_VALUE: u32 = 0xffff;
}
