#[doc = "Register `I3C_TDR` writer"]
pub type W = crate::W<I3C_TDRrs>;
#[doc = "Field `TDB0` writer - 8-bit data to transmit on I3C bus."]
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - 8-bit data to transmit on I3C bus."]
    #[inline(always)]
    #[must_use]
    pub fn tdb0(&mut self) -> TDB0_W<I3C_TDRrs> {
        TDB0_W::new(self, 0)
    }
}
#[doc = "I3C transmit data byte register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_TDRrs;
impl crate::RegisterSpec for I3C_TDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i3c_tdr::W`](W) writer structure"]
impl crate::Writable for I3C_TDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_TDR to value 0"]
impl crate::Resettable for I3C_TDRrs {
    const RESET_VALUE: u32 = 0;
}
