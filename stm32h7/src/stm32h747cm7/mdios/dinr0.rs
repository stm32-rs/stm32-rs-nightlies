#[doc = "Register `DINR0` reader"]
pub type R = crate::R<DINR0rs>;
#[doc = "Field `DIN0` reader - Input data received from MDIO Master during write frames"]
pub type DIN0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR0rs;
impl crate::RegisterSpec for DINR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr0::R`](R) reader structure"]
impl crate::Readable for DINR0rs {}
#[doc = "`reset()` method sets DINR0 to value 0"]
impl crate::Resettable for DINR0rs {
    const RESET_VALUE: u32 = 0;
}
