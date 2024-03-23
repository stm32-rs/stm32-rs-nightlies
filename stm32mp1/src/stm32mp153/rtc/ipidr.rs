#[doc = "Register `IPIDR` reader"]
pub type R = crate::R<IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "RTC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipidr::R`](R) reader structure"]
impl crate::Readable for IPIDRrs {}
#[doc = "`reset()` method sets IPIDR to value 0x0012_0033"]
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x0012_0033;
}
