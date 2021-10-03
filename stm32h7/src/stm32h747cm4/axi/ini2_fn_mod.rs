#[doc = "Register `INI2_FN_MOD` reader"]
pub struct R(crate::R<INI2_FN_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI2_FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI2_FN_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI2_FN_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI2_FN_MOD` writer"]
pub struct W(crate::W<INI2_FN_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI2_FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INI2_FN_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI2_FN_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub struct READ_ISS_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl READ_ISS_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_ISS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_ISS_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub struct READ_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ISS_OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub struct WRITE_ISS_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl WRITE_ISS_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_ISS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_ISS_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub struct WRITE_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ISS_OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W {
        READ_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W {
        WRITE_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini2_fn_mod](index.html) module"]
pub struct INI2_FN_MOD_SPEC;
impl crate::RegisterSpec for INI2_FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini2_fn_mod::R](R) reader structure"]
impl crate::Readable for INI2_FN_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini2_fn_mod::W](W) writer structure"]
impl crate::Writable for INI2_FN_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INI2_FN_MOD to value 0x04"]
impl crate::Resettable for INI2_FN_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}