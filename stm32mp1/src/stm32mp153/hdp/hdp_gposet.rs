#[doc = "Register `HDP_GPOSET` writer"]
pub type W = crate::W<HDP_GPOSETrs>;
#[doc = "Field `HDPGPOSET` writer - HDPGPOSET"]
pub type HDPGPOSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - HDPGPOSET"]
    #[inline(always)]
    #[must_use]
    pub fn hdpgposet(&mut self) -> HDPGPOSET_W<HDP_GPOSETrs> {
        HDPGPOSET_W::new(self, 0)
    }
}
#[doc = "HDP GPO set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp_gposet::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_GPOSETrs;
impl crate::RegisterSpec for HDP_GPOSETrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdp_gposet::W`](W) writer structure"]
impl crate::Writable for HDP_GPOSETrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDP_GPOSET to value 0"]
impl crate::Resettable for HDP_GPOSETrs {
    const RESET_VALUE: u32 = 0;
}
