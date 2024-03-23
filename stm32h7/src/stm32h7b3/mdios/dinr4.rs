#[doc = "Register `DINR4` reader"]
pub type R = crate::R<DINR4rs>;
#[doc = "Field `DIN4` reader - Input data received from MDIO Master during write frames"]
pub type DIN4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR4rs;
impl crate::RegisterSpec for DINR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr4::R`](R) reader structure"]
impl crate::Readable for DINR4rs {}
#[doc = "`reset()` method sets DINR4 to value 0"]
impl crate::Resettable for DINR4rs {
    const RESET_VALUE: u32 = 0;
}
