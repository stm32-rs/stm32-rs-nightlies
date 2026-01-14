///Register `PRLH` writer
pub type W = crate::W<PRLHrs>;
///Field `PRLH` writer - RTC Prescaler Load Register High
pub type PRLH_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<PRLHrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:3 - RTC Prescaler Load Register High
    #[inline(always)]
    pub fn prlh(&mut self) -> PRLH_W<'_, PRLHrs> {
        PRLH_W::new(self, 0)
    }
}
/**RTC Prescaler Load Register High

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prlh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#RTC:PRLH)*/
pub struct PRLHrs;
impl crate::RegisterSpec for PRLHrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`prlh::W`](W) writer structure
impl crate::Writable for PRLHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRLH to value 0
impl crate::Resettable for PRLHrs {}
