///Register `TDR` writer
pub type W = crate::W<TDRrs>;
///Field `TDB0` writer - 8-bit data to transmit on I3C bus.
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<TDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - 8-bit data to transmit on I3C bus.
    #[inline(always)]
    pub fn tdb0(&mut self) -> TDB0_W<'_, TDRrs> {
        TDB0_W::new(self, 0)
    }
}
/**I3C transmit data byte register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#I3C1:TDR)*/
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0
impl crate::Resettable for TDRrs {}
