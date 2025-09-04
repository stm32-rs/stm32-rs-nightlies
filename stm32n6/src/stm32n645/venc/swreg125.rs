///Register `SWREG125` writer
pub type W = crate::W<SWREG125rs>;
///Field `SWREG_FIELD` writer - DMV 4p/1p penalty values (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG125rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV 4p/1p penalty values (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG125rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg125::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG125)*/
pub struct SWREG125rs;
impl crate::RegisterSpec for SWREG125rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg125::W`](W) writer structure
impl crate::Writable for SWREG125rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG125 to value 0
impl crate::Resettable for SWREG125rs {}
