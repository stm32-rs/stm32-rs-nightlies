#[doc = "Register `GPIOZ_HWCFGR9` reader"]
pub struct R(crate::R<GPIOZ_HWCFGR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOZ_HWCFGR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOZ_HWCFGR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOZ_HWCFGR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EN_IO` reader - EN_IO"]
pub struct EN_IO_R(crate::FieldReader<u16, u16>);
impl EN_IO_R {
    pub(crate) fn new(bits: u16) -> Self {
        EN_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_IO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - EN_IO"]
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr9](index.html) module"]
pub struct GPIOZ_HWCFGR9_SPEC;
impl crate::RegisterSpec for GPIOZ_HWCFGR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioz_hwcfgr9::R](R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOZ_HWCFGR9 to value 0xff"]
impl crate::Resettable for GPIOZ_HWCFGR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
