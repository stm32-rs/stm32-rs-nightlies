#[doc = "Register `HSECR` reader"]
pub struct R(crate::R<HSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSECR` writer"]
pub struct W(crate::W<HSECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSECR_SPEC>;
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
impl From<crate::W<HSECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSETUNE` reader - HSE capacitor tuning"]
pub struct HSETUNE_R(crate::FieldReader<u8, u8>);
impl HSETUNE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSETUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSETUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEGMC` reader - HSE current control"]
pub struct HSEGMC_R(crate::FieldReader<u8, u8>);
impl HSEGMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSEGMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEGMC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEGMC` writer - HSE current control"]
pub struct HSEGMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEGMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `HSES` reader - HSE Sense amplifier threshold"]
pub struct HSES_R(crate::FieldReader<bool, bool>);
impl HSES_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSES` writer - HSE Sense amplifier threshold"]
pub struct HSES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UNLOCKED` reader - Register lock system"]
pub struct UNLOCKED_R(crate::FieldReader<bool, bool>);
impl UNLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCKED` writer - Register lock system"]
pub struct UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCKED_W<'a> {
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
impl R {
    #[doc = "Bits 8:13 - HSE capacitor tuning"]
    #[inline(always)]
    pub fn hsetune(&self) -> HSETUNE_R {
        HSETUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    pub fn hsegmc(&self) -> HSEGMC_R {
        HSEGMC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    pub fn hsegmc(&mut self) -> HSEGMC_W {
        HSEGMC_W { w: self }
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    pub fn hses(&mut self) -> HSES_W {
        HSES_W { w: self }
    }
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    pub fn unlocked(&mut self) -> UNLOCKED_W {
        UNLOCKED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock HSE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsecr](index.html) module"]
pub struct HSECR_SPEC;
impl crate::RegisterSpec for HSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsecr::R](R) reader structure"]
impl crate::Readable for HSECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsecr::W](W) writer structure"]
impl crate::Writable for HSECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSECR to value 0x30"]
impl crate::Resettable for HSECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
