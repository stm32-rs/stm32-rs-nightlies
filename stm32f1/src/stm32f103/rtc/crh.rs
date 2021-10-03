#[doc = "Register `CRH` reader"]
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRH` writer"]
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Second interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECIE_A {
    #[doc = "0: Second interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Second interrupt is enabled"]
    ENABLED = 1,
}
impl From<SECIE_A> for bool {
    #[inline(always)]
    fn from(variant: SECIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECIE` reader - Second interrupt Enable"]
pub struct SECIE_R(crate::FieldReader<bool, SECIE_A>);
impl SECIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECIE_A {
        match self.bits {
            false => SECIE_A::DISABLED,
            true => SECIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SECIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SECIE_A::ENABLED
    }
}
impl core::ops::Deref for SECIE_R {
    type Target = crate::FieldReader<bool, SECIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECIE` writer - Second interrupt Enable"]
pub struct SECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Second interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECIE_A::DISABLED)
    }
    #[doc = "Second interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Alarm interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRIE_A {
    #[doc = "0: Alarm interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Alarm interrupt is enabled"]
    ENABLED = 1,
}
impl From<ALRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRIE` reader - Alarm interrupt Enable"]
pub struct ALRIE_R(crate::FieldReader<bool, ALRIE_A>);
impl ALRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRIE_A {
        match self.bits {
            false => ALRIE_A::DISABLED,
            true => ALRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRIE_A::ENABLED
    }
}
impl core::ops::Deref for ALRIE_R {
    type Target = crate::FieldReader<bool, ALRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRIE` writer - Alarm interrupt Enable"]
pub struct ALRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRIE_A::DISABLED)
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Overflow interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWIE_A {
    #[doc = "0: Overflow interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    ENABLED = 1,
}
impl From<OWIE_A> for bool {
    #[inline(always)]
    fn from(variant: OWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWIE` reader - Overflow interrupt Enable"]
pub struct OWIE_R(crate::FieldReader<bool, OWIE_A>);
impl OWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWIE_A {
        match self.bits {
            false => OWIE_A::DISABLED,
            true => OWIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OWIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OWIE_A::ENABLED
    }
}
impl core::ops::Deref for OWIE_R {
    type Target = crate::FieldReader<bool, OWIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWIE` writer - Overflow interrupt Enable"]
pub struct OWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overflow interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OWIE_A::DISABLED)
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OWIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&mut self) -> SECIE_W {
        SECIE_W { w: self }
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&mut self) -> ALRIE_W {
        ALRIE_W { w: self }
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&mut self) -> OWIE_W {
        OWIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](index.html) module"]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crh::R](R) reader structure"]
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crh::W](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRH to value 0"]
impl crate::Resettable for CRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
