#[doc = "Register `TIMx_AF1` reader"]
pub struct R(crate::R<TIMX_AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_AF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_AF1` writer"]
pub struct W(crate::W<TIMX_AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_AF1_SPEC>;
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
impl From<crate::W<TIMX_AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_AF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BKINE"]
pub struct BKINE_R(crate::FieldReader<bool, bool>);
impl BKINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKINE` writer - BKINE"]
pub struct BKINE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINE_W<'a> {
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
#[doc = "Field `BKDF1BK2E` reader - BKDF1BK2E"]
pub struct BKDF1BK2E_R(crate::FieldReader<bool, bool>);
impl BKDF1BK2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKDF1BK2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKDF1BK2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKDF1BK2E` writer - BKDF1BK2E"]
pub struct BKDF1BK2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKDF1BK2E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BKINP` reader - BKINP"]
pub struct BKINP_R(crate::FieldReader<bool, bool>);
impl BKINP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKINP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKINP` writer - BKINP"]
pub struct BKINP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKDF1BK2E"]
    #[inline(always)]
    pub fn bkdf1bk2e(&self) -> BKDF1BK2E_R {
        BKDF1BK2E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BKINE"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W {
        BKINE_W { w: self }
    }
    #[doc = "Bit 8 - BKDF1BK2E"]
    #[inline(always)]
    pub fn bkdf1bk2e(&mut self) -> BKDF1BK2E_W {
        BKDF1BK2E_W { w: self }
    }
    #[doc = "Bit 9 - BKINP"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W {
        BKINP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_af1](index.html) module"]
pub struct TIMX_AF1_SPEC;
impl crate::RegisterSpec for TIMX_AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_af1::R](R) reader structure"]
impl crate::Readable for TIMX_AF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_af1::W](W) writer structure"]
impl crate::Writable for TIMX_AF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_AF1 to value 0x01"]
impl crate::Resettable for TIMX_AF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
