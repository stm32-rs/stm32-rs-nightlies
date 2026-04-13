///Register `TDWR` writer
pub type W = crate::W<TDWRrs>;
///Field `TDB0` writer - 8-bit transmit data (earliest byte on I3C bus)
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TDB1` writer - 8-bit transmit data (next byte after TDB0\[7:0\] on I3C bus).
pub type TDB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TDB2` writer - 8-bit transmit data (next byte after TDB1\[7:0\] on I3C bus).
pub type TDB2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TDB3` writer - 8-bit transmit data (latest byte on I3C bus).
pub type TDB3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<TDWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - 8-bit transmit data (earliest byte on I3C bus)
    #[inline(always)]
    pub fn tdb0(&mut self) -> TDB0_W<'_, TDWRrs> {
        TDB0_W::new(self, 0)
    }
    ///Bits 8:15 - 8-bit transmit data (next byte after TDB0\[7:0\] on I3C bus).
    #[inline(always)]
    pub fn tdb1(&mut self) -> TDB1_W<'_, TDWRrs> {
        TDB1_W::new(self, 8)
    }
    ///Bits 16:23 - 8-bit transmit data (next byte after TDB1\[7:0\] on I3C bus).
    #[inline(always)]
    pub fn tdb2(&mut self) -> TDB2_W<'_, TDWRrs> {
        TDB2_W::new(self, 16)
    }
    ///Bits 24:31 - 8-bit transmit data (latest byte on I3C bus).
    #[inline(always)]
    pub fn tdb3(&mut self) -> TDB3_W<'_, TDWRrs> {
        TDB3_W::new(self, 24)
    }
}
/**I3C transmit data word register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#I3C1:TDWR)*/
pub struct TDWRrs;
impl crate::RegisterSpec for TDWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tdwr::W`](W) writer structure
impl crate::Writable for TDWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDWR to value 0
impl crate::Resettable for TDWRrs {}
