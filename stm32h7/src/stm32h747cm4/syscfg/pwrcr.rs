#[doc = "Register `PWRCR` reader"]
pub struct R(crate::R<PWRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCR` writer"]
pub struct W(crate::W<PWRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCR_SPEC>;
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
impl From<crate::W<PWRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Overdrive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODEN_A {
    #[doc = "0: Overdrive mode disabled"]
    DISABLED = 0,
    #[doc = "1: Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    ENABLED = 1,
}
impl From<ODEN_A> for bool {
    #[inline(always)]
    fn from(variant: ODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub struct ODEN_R(crate::FieldReader<bool, ODEN_A>);
impl ODEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODEN_A {
        match self.bits {
            false => ODEN_A::DISABLED,
            true => ODEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ODEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ODEN_A::ENABLED
    }
}
impl core::ops::Deref for ODEN_R {
    type Target = crate::FieldReader<bool, ODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub struct ODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overdrive mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ODEN_A::DISABLED)
    }
    #[doc = "Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W {
        ODEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcr](index.html) module"]
pub struct PWRCR_SPEC;
impl crate::RegisterSpec for PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcr::R](R) reader structure"]
impl crate::Readable for PWRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcr::W](W) writer structure"]
impl crate::Writable for PWRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
