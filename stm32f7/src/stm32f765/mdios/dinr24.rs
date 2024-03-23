#[doc = "Register `DINR24` reader"]
pub type R = crate::R<DINR24rs>;
#[doc = "Field `DIN24` reader - Input data received from MDIO Master during write frames"]
pub type DIN24_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din24(&self) -> DIN24_R {
        DIN24_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr24::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR24rs;
impl crate::RegisterSpec for DINR24rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr24::R`](R) reader structure"]
impl crate::Readable for DINR24rs {}
#[doc = "`reset()` method sets DINR24 to value 0"]
impl crate::Resettable for DINR24rs {
    const RESET_VALUE: u32 = 0;
}
