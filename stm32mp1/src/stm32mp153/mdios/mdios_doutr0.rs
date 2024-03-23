#[doc = "Register `MDIOS_DOUTR0` reader"]
pub type R = crate::R<MDIOS_DOUTR0rs>;
#[doc = "Field `DOUT` reader - DOUT"]
pub type DOUT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR0rs;
impl crate::RegisterSpec for MDIOS_DOUTR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr0::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR0rs {}
#[doc = "`reset()` method sets MDIOS_DOUTR0 to value 0"]
impl crate::Resettable for MDIOS_DOUTR0rs {
    const RESET_VALUE: u32 = 0;
}
