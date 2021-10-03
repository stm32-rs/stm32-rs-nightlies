#[doc = "Register `SECWM1R2` reader"]
pub struct R(crate::R<SECWM1R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM1R2` writer"]
pub struct W(crate::W<SECWM1R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM1R2_SPEC>;
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
impl From<crate::W<SECWM1R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM1R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1_PSTRT` reader - PCROP1_PSTRT"]
pub struct PCROP1_PSTRT_R(crate::FieldReader<u8, u8>);
impl PCROP1_PSTRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1_PSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1_PSTRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP1_PSTRT` writer - PCROP1_PSTRT"]
pub struct PCROP1_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PCROP1EN` reader - PCROP1EN"]
pub struct PCROP1EN_R(crate::FieldReader<bool, bool>);
impl PCROP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCROP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP1EN` writer - PCROP1EN"]
pub struct PCROP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1EN_W<'a> {
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
#[doc = "Field `HDP1_PEND` reader - HDP1_PEND"]
pub struct HDP1_PEND_R(crate::FieldReader<u8, u8>);
impl HDP1_PEND_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDP1_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP1_PEND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP1_PEND` writer - HDP1_PEND"]
pub struct HDP1_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `HDP1EN` reader - HDP1EN"]
pub struct HDP1EN_R(crate::FieldReader<bool, bool>);
impl HDP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP1EN` writer - HDP1EN"]
pub struct HDP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    pub fn pcrop1_pstrt(&self) -> PCROP1_PSTRT_R {
        PCROP1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    pub fn pcrop1en(&self) -> PCROP1EN_R {
        PCROP1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP1_PSTRT"]
    #[inline(always)]
    pub fn pcrop1_pstrt(&mut self) -> PCROP1_PSTRT_W {
        PCROP1_PSTRT_W { w: self }
    }
    #[doc = "Bit 15 - PCROP1EN"]
    #[inline(always)]
    pub fn pcrop1en(&mut self) -> PCROP1EN_W {
        PCROP1EN_W { w: self }
    }
    #[doc = "Bits 16:22 - HDP1_PEND"]
    #[inline(always)]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W {
        HDP1_PEND_W { w: self }
    }
    #[doc = "Bit 31 - HDP1EN"]
    #[inline(always)]
    pub fn hdp1en(&mut self) -> HDP1EN_W {
        HDP1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure watermak1 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r2](index.html) module"]
pub struct SECWM1R2_SPEC;
impl crate::RegisterSpec for SECWM1R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm1r2::R](R) reader structure"]
impl crate::Readable for SECWM1R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm1r2::W](W) writer structure"]
impl crate::Writable for SECWM1R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECWM1R2 to value 0x0f00_0f00"]
impl crate::Resettable for SECWM1R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_0f00
    }
}
