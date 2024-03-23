#[doc = "Register `TZC_FAIL_ADDRESS_HIGH1` reader"]
pub type R = crate::R<TZC_FAIL_ADDRESS_HIGH1rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TZC_FAIL_ADDRESS_HIGH1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_high1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_FAIL_ADDRESS_HIGH1rs;
impl crate::RegisterSpec for TZC_FAIL_ADDRESS_HIGH1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_fail_address_high1::R`](R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_HIGH1rs {}
#[doc = "`reset()` method sets TZC_FAIL_ADDRESS_HIGH1 to value 0"]
impl crate::Resettable for TZC_FAIL_ADDRESS_HIGH1rs {
    const RESET_VALUE: u32 = 0;
}
