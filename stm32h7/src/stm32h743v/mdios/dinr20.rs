#[doc = "Register `DINR20` reader"]
pub type R = crate::R<DINR20rs>;
#[doc = "Field `DIN20` reader - Input data received from MDIO Master during write frames"]
pub type DIN20_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din20(&self) -> DIN20_R {
        DIN20_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR20rs;
impl crate::RegisterSpec for DINR20rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr20::R`](R) reader structure"]
impl crate::Readable for DINR20rs {}
#[doc = "`reset()` method sets DINR20 to value 0"]
impl crate::Resettable for DINR20rs {
    const RESET_VALUE: u32 = 0;
}
