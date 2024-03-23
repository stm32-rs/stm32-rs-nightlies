#[doc = "Register `IPCC_ID` reader"]
pub type R = crate::R<IPCC_IDrs>;
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "IPCC IP Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_IDrs;
impl crate::RegisterSpec for IPCC_IDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_id::R`](R) reader structure"]
impl crate::Readable for IPCC_IDrs {}
#[doc = "`reset()` method sets IPCC_ID to value 0x0010_0071"]
impl crate::Resettable for IPCC_IDrs {
    const RESET_VALUE: u32 = 0x0010_0071;
}
