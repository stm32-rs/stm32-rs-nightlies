///Register `SWREG139` writer
pub type W = crate::W<SWREG139rs>;
///Field `SWREG_FIELD` writer - DMV qpel penalty values (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SWREG139rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - DMV qpel penalty values (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<SWREG139rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg139::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG139)*/
pub struct SWREG139rs;
impl crate::RegisterSpec for SWREG139rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swreg139::W`](W) writer structure
impl crate::Writable for SWREG139rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG139 to value 0
impl crate::Resettable for SWREG139rs {}
