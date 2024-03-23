#[doc = "Register `MDIOS_DOUTR7` reader"]
pub type R = crate::R<MDIOS_DOUTR7rs>;
#[doc = "Field `DOUT` reader - DOUT"]
pub type DOUT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR7rs;
impl crate::RegisterSpec for MDIOS_DOUTR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr7::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR7rs {}
#[doc = "`reset()` method sets MDIOS_DOUTR7 to value 0"]
impl crate::Resettable for MDIOS_DOUTR7rs {
    const RESET_VALUE: u32 = 0;
}
