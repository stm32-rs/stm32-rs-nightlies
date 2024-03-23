#[doc = "Register `DINR27` reader"]
pub type R = crate::R<DINR27rs>;
#[doc = "Field `DIN27` reader - Input data received from MDIO Master during write frames"]
pub type DIN27_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din27(&self) -> DIN27_R {
        DIN27_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr27::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR27rs;
impl crate::RegisterSpec for DINR27rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr27::R`](R) reader structure"]
impl crate::Readable for DINR27rs {}
#[doc = "`reset()` method sets DINR27 to value 0"]
impl crate::Resettable for DINR27rs {
    const RESET_VALUE: u32 = 0;
}
