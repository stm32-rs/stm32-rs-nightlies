#[doc = "Register `DINR16` reader"]
pub type R = crate::R<DINR16rs>;
#[doc = "Field `DIN16` reader - Input data received from MDIO Master during write frames"]
pub type DIN16_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din16(&self) -> DIN16_R {
        DIN16_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR16rs;
impl crate::RegisterSpec for DINR16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr16::R`](R) reader structure"]
impl crate::Readable for DINR16rs {}
#[doc = "`reset()` method sets DINR16 to value 0"]
impl crate::Resettable for DINR16rs {
    const RESET_VALUE: u32 = 0;
}
