///Register `SUSP5R` writer
pub type W = crate::W<SUSP5Rrs>;
///Field `SUSP5` writer - AES suspend
pub type SUSP5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SUSP5Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    #[must_use]
    pub fn susp5(&mut self) -> SUSP5_W<SUSP5Rrs> {
        SUSP5_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp5r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#AES:SUSP5R)*/
pub struct SUSP5Rrs;
impl crate::RegisterSpec for SUSP5Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp5r::W`](W) writer structure
impl crate::Writable for SUSP5Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP5R to value 0
impl crate::Resettable for SUSP5Rrs {
    const RESET_VALUE: u32 = 0;
}
