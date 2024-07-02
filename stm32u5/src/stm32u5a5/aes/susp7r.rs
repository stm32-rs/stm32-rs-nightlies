///Register `SUSP7R` writer
pub type W = crate::W<SUSP7Rrs>;
///Field `SUSP7` writer - AES suspend
pub type SUSP7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SUSP7Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    #[must_use]
    pub fn susp7(&mut self) -> SUSP7_W<SUSP7Rrs> {
        SUSP7_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp7r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#AES:SUSP7R)*/
pub struct SUSP7Rrs;
impl crate::RegisterSpec for SUSP7Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp7r::W`](W) writer structure
impl crate::Writable for SUSP7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP7R to value 0
impl crate::Resettable for SUSP7Rrs {
    const RESET_VALUE: u32 = 0;
}
