#[doc = "Register `DINR14` reader"]
pub type R = crate::R<DINR14rs>;
#[doc = "Field `DIN14` reader - Input data received from MDIO Master during write frames"]
pub type DIN14_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR14rs;
impl crate::RegisterSpec for DINR14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr14::R`](R) reader structure"]
impl crate::Readable for DINR14rs {}
#[doc = "`reset()` method sets DINR14 to value 0"]
impl crate::Resettable for DINR14rs {
    const RESET_VALUE: u32 = 0;
}
