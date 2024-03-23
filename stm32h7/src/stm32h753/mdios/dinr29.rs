#[doc = "Register `DINR29` reader"]
pub type R = crate::R<DINR29rs>;
#[doc = "Field `DIN29` reader - Input data received from MDIO Master during write frames"]
pub type DIN29_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din29(&self) -> DIN29_R {
        DIN29_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR29rs;
impl crate::RegisterSpec for DINR29rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr29::R`](R) reader structure"]
impl crate::Readable for DINR29rs {}
#[doc = "`reset()` method sets DINR29 to value 0"]
impl crate::Resettable for DINR29rs {
    const RESET_VALUE: u32 = 0;
}
