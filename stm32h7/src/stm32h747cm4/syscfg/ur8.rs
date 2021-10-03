#[doc = "Register `UR8` reader"]
pub struct R(crate::R<UR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEPAD_2` reader - Mass erase protected area disabled for bank 2"]
pub struct MEPAD_2_R(crate::FieldReader<bool, bool>);
impl MEPAD_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEPAD_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEPAD_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MESAD_2` reader - Mass erase secured area disabled for bank 2"]
pub struct MESAD_2_R(crate::FieldReader<bool, bool>);
impl MESAD_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MESAD_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MESAD_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Mass erase protected area disabled for bank 2"]
    #[inline(always)]
    pub fn mepad_2(&self) -> MEPAD_2_R {
        MEPAD_2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mass erase secured area disabled for bank 2"]
    #[inline(always)]
    pub fn mesad_2(&self) -> MESAD_2_R {
        MESAD_2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur8](index.html) module"]
pub struct UR8_SPEC;
impl crate::RegisterSpec for UR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur8::R](R) reader structure"]
impl crate::Readable for UR8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR8 to value 0"]
impl crate::Resettable for UR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
