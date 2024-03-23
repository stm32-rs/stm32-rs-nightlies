#[doc = "Register `DINR21` reader"]
pub type R = crate::R<DINR21rs>;
#[doc = "Field `DIN21` reader - Input data received from MDIO Master during write frames"]
pub type DIN21_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din21(&self) -> DIN21_R {
        DIN21_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR21rs;
impl crate::RegisterSpec for DINR21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr21::R`](R) reader structure"]
impl crate::Readable for DINR21rs {}
#[doc = "`reset()` method sets DINR21 to value 0"]
impl crate::Resettable for DINR21rs {
    const RESET_VALUE: u32 = 0;
}
