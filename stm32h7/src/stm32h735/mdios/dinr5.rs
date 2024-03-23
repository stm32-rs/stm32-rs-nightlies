#[doc = "Register `DINR5` reader"]
pub type R = crate::R<DINR5rs>;
#[doc = "Field `DIN5` reader - Input data received from MDIO Master during write frames"]
pub type DIN5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din5(&self) -> DIN5_R {
        DIN5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR5rs;
impl crate::RegisterSpec for DINR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr5::R`](R) reader structure"]
impl crate::Readable for DINR5rs {}
#[doc = "`reset()` method sets DINR5 to value 0"]
impl crate::Resettable for DINR5rs {
    const RESET_VALUE: u32 = 0;
}
