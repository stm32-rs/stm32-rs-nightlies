///Register `SWREG97` writer
pub type W = crate::W<SWREG97rs>;
///Field `SWREG_FIELD` writer - DMV 4p/1p penalty values 4-7 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG97rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV 4p/1p penalty values 4-7 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG97rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV 4p/1p penalty values 4-7 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg97::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG97)*/
pub struct SWREG97rs;
impl crate::RegisterSpec for SWREG97rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg97::W`](W) writer structure
impl crate::Writable for SWREG97rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG97 to value 0
impl crate::Resettable for SWREG97rs {}
