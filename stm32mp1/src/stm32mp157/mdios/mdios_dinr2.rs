#[doc = "Register `MDIOS_DINR2` reader"]
pub type R = crate::R<MDIOS_DINR2rs>;
#[doc = "Field `DIN` reader - DIN"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DIN"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR2rs;
impl crate::RegisterSpec for MDIOS_DINR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr2::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR2rs {}
#[doc = "`reset()` method sets MDIOS_DINR2 to value 0"]
impl crate::Resettable for MDIOS_DINR2rs {
    const RESET_VALUE: u32 = 0;
}
