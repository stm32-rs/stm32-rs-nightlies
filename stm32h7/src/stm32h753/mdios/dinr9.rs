#[doc = "Register `DINR9` reader"]
pub type R = crate::R<DINR9rs>;
#[doc = "Field `DIN9` reader - Input data received from MDIO Master during write frames"]
pub type DIN9_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR9rs;
impl crate::RegisterSpec for DINR9rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr9::R`](R) reader structure"]
impl crate::Readable for DINR9rs {}
#[doc = "`reset()` method sets DINR9 to value 0"]
impl crate::Resettable for DINR9rs {
    const RESET_VALUE: u32 = 0;
}
