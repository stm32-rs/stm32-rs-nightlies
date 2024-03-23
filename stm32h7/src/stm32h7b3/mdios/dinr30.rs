#[doc = "Register `DINR30` reader"]
pub type R = crate::R<DINR30rs>;
#[doc = "Field `DIN30` reader - Input data received from MDIO Master during write frames"]
pub type DIN30_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din30(&self) -> DIN30_R {
        DIN30_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr30::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR30rs;
impl crate::RegisterSpec for DINR30rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr30::R`](R) reader structure"]
impl crate::Readable for DINR30rs {}
#[doc = "`reset()` method sets DINR30 to value 0"]
impl crate::Resettable for DINR30rs {
    const RESET_VALUE: u32 = 0;
}
