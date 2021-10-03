#[doc = "Register `C2APB3ENR` reader"]
pub struct R(crate::R<C2APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB3ENR` writer"]
pub struct W(crate::W<C2APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB3ENR_SPEC>;
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
impl From<crate::W<C2APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU2 sub-GHz radio SPI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBGHZSPIEN_A {
    #[doc = "0: Clock disabled"]
    DISABLED = 0,
    #[doc = "1: Clock enabled"]
    ENABLED = 1,
}
impl From<SUBGHZSPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHZSPIEN` reader - CPU2 sub-GHz radio SPI clock enable"]
pub struct SUBGHZSPIEN_R(crate::FieldReader<bool, SUBGHZSPIEN_A>);
impl SUBGHZSPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUBGHZSPIEN_A {
        match self.bits {
            false => SUBGHZSPIEN_A::DISABLED,
            true => SUBGHZSPIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SUBGHZSPIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SUBGHZSPIEN_A::ENABLED
    }
}
impl core::ops::Deref for SUBGHZSPIEN_R {
    type Target = crate::FieldReader<bool, SUBGHZSPIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPIEN` writer - CPU2 sub-GHz radio SPI clock enable"]
pub struct SUBGHZSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBGHZSPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUBGHZSPIEN_A::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUBGHZSPIEN_A::ENABLED)
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
    #[doc = "Bit 0 - CPU2 sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&self) -> SUBGHZSPIEN_R {
        SUBGHZSPIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&mut self) -> SUBGHZSPIEN_W {
        SUBGHZSPIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB3 peripheral clock enable register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3enr](index.html) module"]
pub struct C2APB3ENR_SPEC;
impl crate::RegisterSpec for C2APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb3enr::R](R) reader structure"]
impl crate::Readable for C2APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb3enr::W](W) writer structure"]
impl crate::Writable for C2APB3ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB3ENR to value 0"]
impl crate::Resettable for C2APB3ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
