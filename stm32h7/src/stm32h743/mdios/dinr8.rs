#[doc = "Register `DINR8` reader"]
pub type R = crate::R<DINR8rs>;
#[doc = "Field `DIN8` reader - Input data received from MDIO Master during write frames"]
pub type DIN8_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR8rs;
impl crate::RegisterSpec for DINR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr8::R`](R) reader structure"]
impl crate::Readable for DINR8rs {}
#[doc = "`reset()` method sets DINR8 to value 0"]
impl crate::Resettable for DINR8rs {
    const RESET_VALUE: u32 = 0;
}
