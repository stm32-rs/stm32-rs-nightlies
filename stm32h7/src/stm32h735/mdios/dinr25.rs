#[doc = "Register `DINR25` reader"]
pub type R = crate::R<DINR25rs>;
#[doc = "Field `DIN25` reader - Input data received from MDIO Master during write frames"]
pub type DIN25_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din25(&self) -> DIN25_R {
        DIN25_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr25::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR25rs;
impl crate::RegisterSpec for DINR25rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr25::R`](R) reader structure"]
impl crate::Readable for DINR25rs {}
#[doc = "`reset()` method sets DINR25 to value 0"]
impl crate::Resettable for DINR25rs {
    const RESET_VALUE: u32 = 0;
}
