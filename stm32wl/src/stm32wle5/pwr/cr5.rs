#[doc = "Register `CR5` reader"]
pub struct R(crate::R<CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR5` writer"]
pub struct W(crate::W<CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR5_SPEC>;
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
impl From<crate::W<CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable SMPS Step Down converter SMPS mode enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEN_A {
    #[doc = "0: SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    DISABLED = 0,
    #[doc = "1: SMPS step-down converter SMPS mode enabled"]
    ENABLED = 1,
}
impl From<SMPSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEN` reader - Enable SMPS Step Down converter SMPS mode enabled."]
pub struct SMPSEN_R(crate::FieldReader<bool, SMPSEN_A>);
impl SMPSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEN_A {
        match self.bits {
            false => SMPSEN_A::DISABLED,
            true => SMPSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SMPSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SMPSEN_A::ENABLED
    }
}
impl core::ops::Deref for SMPSEN_R {
    type Target = crate::FieldReader<bool, SMPSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSEN` writer - Enable SMPS Step Down converter SMPS mode enabled."]
pub struct SMPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMPSEN_A::DISABLED)
    }
    #[doc = "SMPS step-down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMPSEN_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable Radio End Of Life detector enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEOLEN_A {
    #[doc = "0: Radio end-of-life detector disabled"]
    DISABLED = 0,
    #[doc = "1: Radio end-of-life detector enabled"]
    ENABLED = 1,
}
impl From<RFEOLEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEOLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEOLEN` reader - Enable Radio End Of Life detector enabled"]
pub struct RFEOLEN_R(crate::FieldReader<bool, RFEOLEN_A>);
impl RFEOLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFEOLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEOLEN_A {
        match self.bits {
            false => RFEOLEN_A::DISABLED,
            true => RFEOLEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RFEOLEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RFEOLEN_A::ENABLED
    }
}
impl core::ops::Deref for RFEOLEN_R {
    type Target = crate::FieldReader<bool, RFEOLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEOLEN` writer - Enable Radio End Of Life detector enabled"]
pub struct RFEOLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEOLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEOLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Radio end-of-life detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFEOLEN_A::DISABLED)
    }
    #[doc = "Radio end-of-life detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFEOLEN_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&self) -> RFEOLEN_R {
        RFEOLEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&mut self) -> SMPSEN_W {
        SMPSEN_W { w: self }
    }
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&mut self) -> RFEOLEN_W {
        RFEOLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](index.html) module"]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr5::R](R) reader structure"]
impl crate::Readable for CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr5::W](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR5 to value 0"]
impl crate::Resettable for CR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
