#[doc = "Register `UR17` reader"]
pub struct R(crate::R<UR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IO_HSLV` reader - I/O high speed / low voltage"]
pub struct IO_HSLV_R(crate::FieldReader<bool, bool>);
impl IO_HSLV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IO_HSLV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_HSLV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - I/O high speed / low voltage"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 17\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur17](index.html) module"]
pub struct UR17_SPEC;
impl crate::RegisterSpec for UR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur17::R](R) reader structure"]
impl crate::Readable for UR17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR17 to value 0"]
impl crate::Resettable for UR17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
