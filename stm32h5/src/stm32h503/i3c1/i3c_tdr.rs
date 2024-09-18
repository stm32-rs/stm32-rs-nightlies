///Register `I3C_TDR` writer
pub type W = crate::W<I3C_TDRrs>;
///Field `TDB0` writer - 8-bit data to transmit on I3C bus.
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<I3C_TDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - 8-bit data to transmit on I3C bus.
    #[inline(always)]
    #[must_use]
    pub fn tdb0(&mut self) -> TDB0_W<I3C_TDRrs> {
        TDB0_W::new(self, 0)
    }
}
/**I3C transmit data byte register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TDR)*/
pub struct I3C_TDRrs;
impl crate::RegisterSpec for I3C_TDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`i3c_tdr::W`](W) writer structure
impl crate::Writable for I3C_TDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_TDR to value 0
impl crate::Resettable for I3C_TDRrs {
    const RESET_VALUE: u32 = 0;
}
