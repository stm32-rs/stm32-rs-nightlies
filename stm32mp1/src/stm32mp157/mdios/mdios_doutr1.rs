#[doc = "Register `MDIOS_DOUTR1` reader"]
pub type R = crate::R<MDIOS_DOUTR1rs>;
#[doc = "Field `DOUT` reader - DOUT"]
pub type DOUT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR1rs;
impl crate::RegisterSpec for MDIOS_DOUTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr1::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR1rs {}
#[doc = "`reset()` method sets MDIOS_DOUTR1 to value 0"]
impl crate::Resettable for MDIOS_DOUTR1rs {
    const RESET_VALUE: u32 = 0;
}
