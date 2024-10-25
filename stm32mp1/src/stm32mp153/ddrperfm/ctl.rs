///Register `CTL` writer
pub type W = crate::W<CTLrs>;
///Field `START` writer - START
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` writer - STOP
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CTLrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - START
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTLrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - STOP
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTLrs> {
        STOP_W::new(self, 1)
    }
}
/**Write-only register. A read request returns all zeros.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:CTL)*/
pub struct CTLrs;
impl crate::RegisterSpec for CTLrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ctl::W`](W) writer structure
impl crate::Writable for CTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTL to value 0
impl crate::Resettable for CTLrs {
    const RESET_VALUE: u32 = 0;
}
