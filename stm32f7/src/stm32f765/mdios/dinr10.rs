#[doc = "Register `DINR10` reader"]
pub type R = crate::R<DINR10rs>;
#[doc = "Field `DIN10` reader - Input data received from MDIO Master during write frames"]
pub type DIN10_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din10(&self) -> DIN10_R {
        DIN10_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR10rs;
impl crate::RegisterSpec for DINR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr10::R`](R) reader structure"]
impl crate::Readable for DINR10rs {}
#[doc = "`reset()` method sets DINR10 to value 0"]
impl crate::Resettable for DINR10rs {
    const RESET_VALUE: u32 = 0;
}
