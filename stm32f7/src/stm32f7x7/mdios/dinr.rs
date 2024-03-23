#[doc = "Register `DINR%s` reader"]
pub type R = crate::R<DINRrs>;
#[doc = "Field `DIN` reader - Input data received from MDIO Master during write frames"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr::R`](R) reader structure"]
impl crate::Readable for DINRrs {}
#[doc = "`reset()` method sets DINR%s to value 0"]
impl crate::Resettable for DINRrs {
    const RESET_VALUE: u32 = 0;
}
