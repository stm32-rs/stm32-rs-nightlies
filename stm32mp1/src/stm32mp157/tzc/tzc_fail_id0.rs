#[doc = "Register `TZC_FAIL_ID0` reader"]
pub type R = crate::R<TZC_FAIL_ID0rs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_fail_id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_FAIL_ID0rs;
impl crate::RegisterSpec for TZC_FAIL_ID0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_fail_id0::R`](R) reader structure"]
impl crate::Readable for TZC_FAIL_ID0rs {}
#[doc = "`reset()` method sets TZC_FAIL_ID0 to value 0"]
impl crate::Resettable for TZC_FAIL_ID0rs {
    const RESET_VALUE: u32 = 0;
}
