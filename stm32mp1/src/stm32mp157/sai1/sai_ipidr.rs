#[doc = "Register `SAI_IPIDR` reader"]
pub type R = crate::R<SAI_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "SAI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_IPIDRrs;
impl crate::RegisterSpec for SAI_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_ipidr::R`](R) reader structure"]
impl crate::Readable for SAI_IPIDRrs {}
#[doc = "`reset()` method sets SAI_IPIDR to value 0x0013_0031"]
impl crate::Resettable for SAI_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0031;
}
