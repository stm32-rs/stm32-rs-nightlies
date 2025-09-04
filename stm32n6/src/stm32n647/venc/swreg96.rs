///Register `SWREG96` writer
pub type W = crate::W<SWREG96rs>;
///Field `SWREG_FIELD` writer - DMV 4p/1p penalty values 0-3 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG96rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV 4p/1p penalty values 0-3 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG96rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV 4p/1p penalty values 0-3 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg96::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG96)*/
pub struct SWREG96rs;
impl crate::RegisterSpec for SWREG96rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg96::W`](W) writer structure
impl crate::Writable for SWREG96rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG96 to value 0
impl crate::Resettable for SWREG96rs {}
