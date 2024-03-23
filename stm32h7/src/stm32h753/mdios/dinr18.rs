#[doc = "Register `DINR18` reader"]
pub type R = crate::R<DINR18rs>;
#[doc = "Field `DIN18` reader - Input data received from MDIO Master during write frames"]
pub type DIN18_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din18(&self) -> DIN18_R {
        DIN18_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR18rs;
impl crate::RegisterSpec for DINR18rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr18::R`](R) reader structure"]
impl crate::Readable for DINR18rs {}
#[doc = "`reset()` method sets DINR18 to value 0"]
impl crate::Resettable for DINR18rs {
    const RESET_VALUE: u32 = 0;
}
