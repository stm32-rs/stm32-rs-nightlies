#[doc = "Register `DINR23` reader"]
pub type R = crate::R<DINR23rs>;
#[doc = "Field `DIN23` reader - Input data received from MDIO Master during write frames"]
pub type DIN23_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din23(&self) -> DIN23_R {
        DIN23_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR23rs;
impl crate::RegisterSpec for DINR23rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr23::R`](R) reader structure"]
impl crate::Readable for DINR23rs {}
#[doc = "`reset()` method sets DINR23 to value 0"]
impl crate::Resettable for DINR23rs {
    const RESET_VALUE: u32 = 0;
}
