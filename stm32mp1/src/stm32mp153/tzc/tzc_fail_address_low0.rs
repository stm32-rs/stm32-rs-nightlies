#[doc = "Register `TZC_FAIL_ADDRESS_LOW0` reader"]
pub type R = crate::R<TZC_FAIL_ADDRESS_LOW0rs>;
#[doc = "Field `ADDR_STATUS_LOW` reader - ADDR_STATUS_LOW"]
pub type ADDR_STATUS_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR_STATUS_LOW"]
    #[inline(always)]
    pub fn addr_status_low(&self) -> ADDR_STATUS_LOW_R {
        ADDR_STATUS_LOW_R::new(self.bits)
    }
}
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_address_low0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_FAIL_ADDRESS_LOW0rs;
impl crate::RegisterSpec for TZC_FAIL_ADDRESS_LOW0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_fail_address_low0::R`](R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_LOW0rs {}
#[doc = "`reset()` method sets TZC_FAIL_ADDRESS_LOW0 to value 0"]
impl crate::Resettable for TZC_FAIL_ADDRESS_LOW0rs {
    const RESET_VALUE: u32 = 0;
}
