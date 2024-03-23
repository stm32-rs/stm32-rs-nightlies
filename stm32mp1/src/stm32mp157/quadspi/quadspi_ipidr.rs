#[doc = "Register `QUADSPI_IPIDR` reader"]
pub type R = crate::R<QUADSPI_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "QUADSPI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_IPIDRrs;
impl crate::RegisterSpec for QUADSPI_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_ipidr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_IPIDRrs {}
#[doc = "`reset()` method sets QUADSPI_IPIDR to value 0x0014_0031"]
impl crate::Resettable for QUADSPI_IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0031;
}
