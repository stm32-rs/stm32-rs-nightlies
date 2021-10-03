#[doc = "Register `MDIOS_DOUTR7` reader"]
pub struct R(crate::R<MDIOS_DOUTR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DOUTR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DOUTR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DOUTR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOUT` reader - DOUT"]
pub struct DOUT_R(crate::FieldReader<u16, u16>);
impl DOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS output data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr7](index.html) module"]
pub struct MDIOS_DOUTR7_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_doutr7::R](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DOUTR7 to value 0"]
impl crate::Resettable for MDIOS_DOUTR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
