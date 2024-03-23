#[doc = "Register `HDP_GPOCLR` writer"]
pub type W = crate::W<HDP_GPOCLRrs>;
#[doc = "Field `HDPGPOCLR` writer - HDPGPOCLR"]
pub type HDPGPOCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - HDPGPOCLR"]
    #[inline(always)]
    #[must_use]
    pub fn hdpgpoclr(&mut self) -> HDPGPOCLR_W<HDP_GPOCLRrs> {
        HDPGPOCLR_W::new(self, 0)
    }
}
#[doc = "HDP GPO clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp_gpoclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_GPOCLRrs;
impl crate::RegisterSpec for HDP_GPOCLRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdp_gpoclr::W`](W) writer structure"]
impl crate::Writable for HDP_GPOCLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDP_GPOCLR to value 0"]
impl crate::Resettable for HDP_GPOCLRrs {
    const RESET_VALUE: u32 = 0;
}
