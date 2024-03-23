#[doc = "Register `DMAMUX_IPIDR` reader"]
pub type R = crate::R<DMAMUX_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "This register identifies the IP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_IPIDRrs;
impl crate::RegisterSpec for DMAMUX_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_ipidr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_IPIDRrs {}
#[doc = "`reset()` method sets DMAMUX_IPIDR to value 0x0010_0011"]
impl crate::Resettable for DMAMUX_IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0011;
}
