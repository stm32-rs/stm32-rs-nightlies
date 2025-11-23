///Register `ALRL` writer
pub type W = crate::W<ALRLrs>;
///Field `ALRL` writer - RTC alarm register low
pub type ALRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<ALRLrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - RTC alarm register low
    #[inline(always)]
    pub fn alrl(&mut self) -> ALRL_W<'_, ALRLrs> {
        ALRL_W::new(self, 0)
    }
}
/**RTC Alarm Register Low

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#RTC:ALRL)*/
pub struct ALRLrs;
impl crate::RegisterSpec for ALRLrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`alrl::W`](W) writer structure
impl crate::Writable for ALRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRL to value 0xffff
impl crate::Resettable for ALRLrs {
    const RESET_VALUE: u32 = 0xffff;
}
