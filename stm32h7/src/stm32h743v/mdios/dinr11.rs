#[doc = "Register `DINR11` reader"]
pub type R = crate::R<DINR11rs>;
#[doc = "Field `DIN11` reader - Input data received from MDIO Master during write frames"]
pub type DIN11_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR11rs;
impl crate::RegisterSpec for DINR11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr11::R`](R) reader structure"]
impl crate::Readable for DINR11rs {}
#[doc = "`reset()` method sets DINR11 to value 0"]
impl crate::Resettable for DINR11rs {
    const RESET_VALUE: u32 = 0;
}
