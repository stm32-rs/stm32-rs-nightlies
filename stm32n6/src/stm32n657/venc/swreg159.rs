///Register `SWREG159` writer
pub type W = crate::W<SWREG159rs>;
///Field `SWREG_FIELD` writer - DMV qpel penalty values 124-127 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG159rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV qpel penalty values 124-127 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG159rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV qpel penalty values 124-127 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg159::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG159)*/
pub struct SWREG159rs;
impl crate::RegisterSpec for SWREG159rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg159::W`](W) writer structure
impl crate::Writable for SWREG159rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG159 to value 0
impl crate::Resettable for SWREG159rs {}
