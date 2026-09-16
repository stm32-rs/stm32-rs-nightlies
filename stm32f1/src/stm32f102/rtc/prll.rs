///Register `PRLL` writer
pub type W = crate::W<PRLLrs>;
///Field `PRLL` writer - RTC Prescaler Divider Register Low
pub type PRLL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<PRLLrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - RTC Prescaler Divider Register Low
    #[inline(always)]
    pub fn prll(&mut self) -> PRLL_W<'_, PRLLrs> {
        PRLL_W::new(self, 0)
    }
}
/**RTC Prescaler Load Register Low

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prll::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#RTC:PRLL)*/
pub struct PRLLrs;
impl crate::RegisterSpec for PRLLrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`prll::W`](W) writer structure
impl crate::Writable for PRLLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRLL to value 0x8000
impl crate::Resettable for PRLLrs {
    const RESET_VALUE: u32 = 0x8000;
}
