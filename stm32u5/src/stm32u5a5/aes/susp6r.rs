///Register `SUSP6R` writer
pub type W = crate::W<SUSP6Rrs>;
///Field `SUSP6` writer - AES suspend
pub type SUSP6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SUSP6Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    #[must_use]
    pub fn susp6(&mut self) -> SUSP6_W<SUSP6Rrs> {
        SUSP6_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp6r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:SUSP6R)*/
pub struct SUSP6Rrs;
impl crate::RegisterSpec for SUSP6Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp6r::W`](W) writer structure
impl crate::Writable for SUSP6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP6R to value 0
impl crate::Resettable for SUSP6Rrs {
    const RESET_VALUE: u32 = 0;
}
