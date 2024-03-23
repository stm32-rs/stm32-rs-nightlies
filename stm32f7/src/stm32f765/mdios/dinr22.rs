#[doc = "Register `DINR22` reader"]
pub type R = crate::R<DINR22rs>;
#[doc = "Field `DIN22` reader - Input data received from MDIO Master during write frames"]
pub type DIN22_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din22(&self) -> DIN22_R {
        DIN22_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR22rs;
impl crate::RegisterSpec for DINR22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr22::R`](R) reader structure"]
impl crate::Readable for DINR22rs {}
#[doc = "`reset()` method sets DINR22 to value 0"]
impl crate::Resettable for DINR22rs {
    const RESET_VALUE: u32 = 0;
}
