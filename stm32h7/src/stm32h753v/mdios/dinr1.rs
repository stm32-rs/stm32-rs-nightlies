#[doc = "Register `DINR1` reader"]
pub type R = crate::R<DINR1rs>;
#[doc = "Field `DIN1` reader - Input data received from MDIO Master during write frames"]
pub type DIN1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR1rs;
impl crate::RegisterSpec for DINR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr1::R`](R) reader structure"]
impl crate::Readable for DINR1rs {}
#[doc = "`reset()` method sets DINR1 to value 0"]
impl crate::Resettable for DINR1rs {
    const RESET_VALUE: u32 = 0;
}
