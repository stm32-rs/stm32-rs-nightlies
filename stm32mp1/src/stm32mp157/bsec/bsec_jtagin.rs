#[doc = "Register `BSEC_JTAGIN` reader"]
pub type R = crate::R<BSEC_JTAGINrs>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BSEC JTAG input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_jtagin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_JTAGINrs;
impl crate::RegisterSpec for BSEC_JTAGINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_jtagin::R`](R) reader structure"]
impl crate::Readable for BSEC_JTAGINrs {}
#[doc = "`reset()` method sets BSEC_JTAGIN to value 0"]
impl crate::Resettable for BSEC_JTAGINrs {
    const RESET_VALUE: u32 = 0;
}
