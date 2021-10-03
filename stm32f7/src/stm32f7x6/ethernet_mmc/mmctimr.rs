#[doc = "Register `MMCTIMR` reader"]
pub struct R(crate::R<MMCTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTIMR` writer"]
pub struct W(crate::W<MMCTIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTIMR_SPEC>;
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
impl From<crate::W<MMCTIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitted good frames single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFSCM_A {
    #[doc = "0: Transmitted-good-single-collision half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Transmitted-good-single-collision half-full interrupt disabled"]
    MASKED = 1,
}
impl From<TGFSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub struct TGFSCM_R(crate::FieldReader<bool, TGFSCM_A>);
impl TGFSCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFSCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFSCM_A {
        match self.bits {
            false => TGFSCM_A::UNMASKED,
            true => TGFSCM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == TGFSCM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TGFSCM_A::MASKED
    }
}
impl core::ops::Deref for TGFSCM_R {
    type Target = crate::FieldReader<bool, TGFSCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub struct TGFSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFSCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFSCM_A::UNMASKED)
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFSCM_A::MASKED)
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
#[doc = "Transmitted good frames more than single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMSCM_A {
    #[doc = "0: Transmitted-good-multiple-collision half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Transmitted-good-multiple-collision half-full interrupt disabled"]
    MASKED = 1,
}
impl From<TGFMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more than single collision mask"]
pub struct TGFMSCM_R(crate::FieldReader<bool, TGFMSCM_A>);
impl TGFMSCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFMSCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFMSCM_A {
        match self.bits {
            false => TGFMSCM_A::UNMASKED,
            true => TGFMSCM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == TGFMSCM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TGFMSCM_A::MASKED
    }
}
impl core::ops::Deref for TGFMSCM_R {
    type Target = crate::FieldReader<bool, TGFMSCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more than single collision mask"]
pub struct TGFMSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFMSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFMSCM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::UNMASKED)
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::MASKED)
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
#[doc = "Transmitted good frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFM_A {
    #[doc = "0: Transmitted-good counter half-full interrupt enabled"]
    UNMASKED = 0,
    #[doc = "1: Transmitted-good counter half-full interrupt disabled"]
    MASKED = 1,
}
impl From<TGFM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub struct TGFM_R(crate::FieldReader<bool, TGFM_A>);
impl TGFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFM_A {
        match self.bits {
            false => TGFM_A::UNMASKED,
            true => TGFM_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == TGFM_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TGFM_A::MASKED
    }
}
impl core::ops::Deref for TGFM_R {
    type Target = crate::FieldReader<bool, TGFM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub struct TGFM_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TGFM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFM_A::UNMASKED)
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFM_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W {
        TGFSCM_W { w: self }
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W {
        TGFMSCM_W { w: self }
    }
    #[doc = "Bit 16 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W {
        TGFM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctimr](index.html) module"]
pub struct MMCTIMR_SPEC;
impl crate::RegisterSpec for MMCTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctimr::R](R) reader structure"]
impl crate::Readable for MMCTIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctimr::W](W) writer structure"]
impl crate::Writable for MMCTIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MMCTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
