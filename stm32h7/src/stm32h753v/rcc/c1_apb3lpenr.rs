#[doc = "Register `C1_APB3LPENR` reader"]
pub struct R(crate::R<C1_APB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB3LPENR` writer"]
pub struct W(crate::W<C1_APB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB3LPENR_SPEC>;
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
impl From<crate::W<C1_APB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LTDC peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<LTDCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode"]
pub struct LTDCLPEN_R(crate::FieldReader<bool, LTDCLPEN_A>);
impl LTDCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCLPEN_A {
        match self.bits {
            false => LTDCLPEN_A::DISABLED,
            true => LTDCLPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LTDCLPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LTDCLPEN_A::ENABLED
    }
}
impl core::ops::Deref for LTDCLPEN_R {
    type Target = crate::FieldReader<bool, LTDCLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode"]
pub struct LTDCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::ENABLED)
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
#[doc = "WWDG1 Clock Enable During CSleep Mode"]
pub type WWDG1LPEN_A = LTDCLPEN_A;
#[doc = "Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode"]
pub type WWDG1LPEN_R = LTDCLPEN_R;
#[doc = "Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode"]
pub struct WWDG1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDG1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDG1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDG1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> WWDG1LPEN_R {
        WWDG1LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W {
        LTDCLPEN_W { w: self }
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&mut self) -> WWDG1LPEN_W {
        WWDG1LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb3lpenr](index.html) module"]
pub struct C1_APB3LPENR_SPEC;
impl crate::RegisterSpec for C1_APB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb3lpenr::R](R) reader structure"]
impl crate::Readable for C1_APB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb3lpenr::W](W) writer structure"]
impl crate::Writable for C1_APB3LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB3LPENR to value 0"]
impl crate::Resettable for C1_APB3LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
