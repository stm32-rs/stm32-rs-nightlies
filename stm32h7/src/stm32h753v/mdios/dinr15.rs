#[doc = "Register `DINR15` reader"]
pub type R = crate::R<DINR15rs>;
#[doc = "Field `DIN15` reader - Input data received from MDIO Master during write frames"]
pub type DIN15_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din15(&self) -> DIN15_R {
        DIN15_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR15rs;
impl crate::RegisterSpec for DINR15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr15::R`](R) reader structure"]
impl crate::Readable for DINR15rs {}
#[doc = "`reset()` method sets DINR15 to value 0"]
impl crate::Resettable for DINR15rs {
    const RESET_VALUE: u32 = 0;
}
