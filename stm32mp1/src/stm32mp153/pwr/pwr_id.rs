#[doc = "Register `PWR_ID` reader"]
pub type R = crate::R<PWR_IDrs>;
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "PWR IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_IDrs;
impl crate::RegisterSpec for PWR_IDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_id::R`](R) reader structure"]
impl crate::Readable for PWR_IDrs {}
#[doc = "`reset()` method sets PWR_ID to value 0x0001_0001"]
impl crate::Resettable for PWR_IDrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
