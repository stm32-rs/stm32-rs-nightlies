#[doc = "Register `PUCRH` reader"]
pub struct R(crate::R<PUCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRH` writer"]
pub struct W(crate::W<PUCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRH_SPEC>;
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
impl From<crate::W<PUCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "pull-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PU3_A {
    #[doc = "0: Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    DISABLED = 0,
    #[doc = "1: Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    ENABLED = 1,
}
impl From<PU3_A> for bool {
    #[inline(always)]
    fn from(variant: PU3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU3` reader - pull-up"]
pub struct PU3_R(crate::FieldReader<bool, PU3_A>);
impl PU3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU3_A {
        match self.bits {
            false => PU3_A::DISABLED,
            true => PU3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PU3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PU3_A::ENABLED
    }
}
impl core::ops::Deref for PU3_R {
    type Target = crate::FieldReader<bool, PU3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU3` writer - pull-up"]
pub struct PU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PU3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PU3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU3_A::DISABLED)
    }
    #[doc = "Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W {
        PU3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port H pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrh](index.html) module"]
pub struct PUCRH_SPEC;
impl crate::RegisterSpec for PUCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrh::R](R) reader structure"]
impl crate::Readable for PUCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrh::W](W) writer structure"]
impl crate::Writable for PUCRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
