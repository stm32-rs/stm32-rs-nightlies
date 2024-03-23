#[doc = "Register `I3C_TDWR` writer"]
pub type W = crate::W<I3C_TDWRrs>;
#[doc = "Field `TDB0` writer - 8-bit transmit data (earliest byte on I3C bus)"]
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB1` writer - 8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
pub type TDB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB2` writer - 8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
pub type TDB2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDB3` writer - 8-bit transmit data (latest byte on I3C bus)."]
pub type TDB3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data (earliest byte on I3C bus)"]
    #[inline(always)]
    #[must_use]
    pub fn tdb0(&mut self) -> TDB0_W<I3C_TDWRrs> {
        TDB0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb1(&mut self) -> TDB1_W<I3C_TDWRrs> {
        TDB1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb2(&mut self) -> TDB2_W<I3C_TDWRrs> {
        TDB2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit transmit data (latest byte on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb3(&mut self) -> TDB3_W<I3C_TDWRrs> {
        TDB3_W::new(self, 24)
    }
}
#[doc = "I3C transmit data word register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_tdwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_TDWRrs;
impl crate::RegisterSpec for I3C_TDWRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i3c_tdwr::W`](W) writer structure"]
impl crate::Writable for I3C_TDWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_TDWR to value 0"]
impl crate::Resettable for I3C_TDWRrs {
    const RESET_VALUE: u32 = 0;
}
