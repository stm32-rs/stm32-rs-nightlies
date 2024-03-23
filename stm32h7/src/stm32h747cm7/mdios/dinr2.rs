#[doc = "Register `DINR2` reader"]
pub type R = crate::R<DINR2rs>;
#[doc = "Field `DIN2` reader - Input data received from MDIO Master during write frames"]
pub type DIN2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR2rs;
impl crate::RegisterSpec for DINR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr2::R`](R) reader structure"]
impl crate::Readable for DINR2rs {}
#[doc = "`reset()` method sets DINR2 to value 0"]
impl crate::Resettable for DINR2rs {
    const RESET_VALUE: u32 = 0;
}
