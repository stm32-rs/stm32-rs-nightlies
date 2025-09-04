///Register `SWREG134` writer
pub type W = crate::W<SWREG134rs>;
///Field `SWREG_FIELD` writer - DMV qpel penalty values (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG134rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV qpel penalty values (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG134rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg134::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG134)*/
pub struct SWREG134rs;
impl crate::RegisterSpec for SWREG134rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg134::W`](W) writer structure
impl crate::Writable for SWREG134rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG134 to value 0
impl crate::Resettable for SWREG134rs {}
