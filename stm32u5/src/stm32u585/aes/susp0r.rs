///Register `SUSP0R` writer
pub type W = crate::W<SUSP0Rrs>;
///Field `SUSP0` writer - AES suspend
pub type SUSP0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SUSP0Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp0(&mut self) -> SUSP0_W<SUSP0Rrs> {
        SUSP0_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp0r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#AES:SUSP0R)*/
pub struct SUSP0Rrs;
impl crate::RegisterSpec for SUSP0Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp0r::W`](W) writer structure
impl crate::Writable for SUSP0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP0R to value 0
impl crate::Resettable for SUSP0Rrs {
    const RESET_VALUE: u32 = 0;
}
