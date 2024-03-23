#[doc = "Register `DINR26` reader"]
pub type R = crate::R<DINR26rs>;
#[doc = "Field `DIN26` reader - Input data received from MDIO Master during write frames"]
pub type DIN26_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din26(&self) -> DIN26_R {
        DIN26_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr26::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR26rs;
impl crate::RegisterSpec for DINR26rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr26::R`](R) reader structure"]
impl crate::Readable for DINR26rs {}
#[doc = "`reset()` method sets DINR26 to value 0"]
impl crate::Resettable for DINR26rs {
    const RESET_VALUE: u32 = 0;
}
