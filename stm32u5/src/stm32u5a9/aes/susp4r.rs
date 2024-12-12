///Register `SUSP4R` writer
pub type W = crate::W<SUSP4Rrs>;
///Field `SUSP4` writer - AES suspend
pub type SUSP4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SUSP4Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp4(&mut self) -> SUSP4_W<SUSP4Rrs> {
        SUSP4_W::new(self, 0)
    }
}
/**suspend registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp4r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#AES:SUSP4R)*/
pub struct SUSP4Rrs;
impl crate::RegisterSpec for SUSP4Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`susp4r::W`](W) writer structure
impl crate::Writable for SUSP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUSP4R to value 0
impl crate::Resettable for SUSP4Rrs {
    const RESET_VALUE: u32 = 0;
}
