#[doc = "Register `DINR7` reader"]
pub type R = crate::R<DINR7rs>;
#[doc = "Field `DIN7` reader - Input data received from MDIO Master during write frames"]
pub type DIN7_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR7rs;
impl crate::RegisterSpec for DINR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr7::R`](R) reader structure"]
impl crate::Readable for DINR7rs {}
#[doc = "`reset()` method sets DINR7 to value 0"]
impl crate::Resettable for DINR7rs {
    const RESET_VALUE: u32 = 0;
}
