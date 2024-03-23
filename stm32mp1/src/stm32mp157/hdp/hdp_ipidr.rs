#[doc = "Register `HDP_IPIDR` reader"]
pub type R = crate::R<HDP_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "HDP IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_IPIDRrs;
impl crate::RegisterSpec for HDP_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp_ipidr::R`](R) reader structure"]
impl crate::Readable for HDP_IPIDRrs {}
#[doc = "`reset()` method sets HDP_IPIDR to value 0x0003_0002"]
impl crate::Resettable for HDP_IPIDRrs {
    const RESET_VALUE: u32 = 0x0003_0002;
}
