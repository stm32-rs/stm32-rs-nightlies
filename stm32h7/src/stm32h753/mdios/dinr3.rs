#[doc = "Register `DINR3` reader"]
pub type R = crate::R<DINR3rs>;
#[doc = "Field `DIN3` reader - Input data received from MDIO Master during write frames"]
pub type DIN3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR3rs;
impl crate::RegisterSpec for DINR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr3::R`](R) reader structure"]
impl crate::Readable for DINR3rs {}
#[doc = "`reset()` method sets DINR3 to value 0"]
impl crate::Resettable for DINR3rs {
    const RESET_VALUE: u32 = 0;
}
