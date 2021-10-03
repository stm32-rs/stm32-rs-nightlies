#[doc = "Register `SECWM2R2` reader"]
pub struct R(crate::R<SECWM2R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM2R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM2R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM2R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM2R2` writer"]
pub struct W(crate::W<SECWM2R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM2R2_SPEC>;
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
impl From<crate::W<SECWM2R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM2R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2_PSTRT` reader - PCROP2_PSTRT"]
pub struct PCROP2_PSTRT_R(crate::FieldReader<u8, u8>);
impl PCROP2_PSTRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP2_PSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP2_PSTRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP2_PSTRT` writer - PCROP2_PSTRT"]
pub struct PCROP2_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PCROP2EN` reader - PCROP2EN"]
pub struct PCROP2EN_R(crate::FieldReader<bool, bool>);
impl PCROP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCROP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP2EN` writer - PCROP2EN"]
pub struct PCROP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2EN_W<'a> {
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
#[doc = "Field `HDP2_PEND` reader - HDP2_PEND"]
pub struct HDP2_PEND_R(crate::FieldReader<u8, u8>);
impl HDP2_PEND_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDP2_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP2_PEND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP2_PEND` writer - HDP2_PEND"]
pub struct HDP2_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `HDP2EN` reader - HDP2EN"]
pub struct HDP2EN_R(crate::FieldReader<bool, bool>);
impl HDP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP2EN` writer - HDP2EN"]
pub struct HDP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2EN_W<'a> {
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
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    pub fn pcrop2_pstrt(&self) -> PCROP2_PSTRT_R {
        PCROP2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    pub fn pcrop2en(&self) -> PCROP2EN_R {
        PCROP2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    pub fn pcrop2_pstrt(&mut self) -> PCROP2_PSTRT_W {
        PCROP2_PSTRT_W { w: self }
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    pub fn pcrop2en(&mut self) -> PCROP2EN_W {
        PCROP2EN_W { w: self }
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W {
        HDP2_PEND_W { w: self }
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    pub fn hdp2en(&mut self) -> HDP2EN_W {
        HDP2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure watermak2 register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r2](index.html) module"]
pub struct SECWM2R2_SPEC;
impl crate::RegisterSpec for SECWM2R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm2r2::R](R) reader structure"]
impl crate::Readable for SECWM2R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm2r2::W](W) writer structure"]
impl crate::Writable for SECWM2R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECWM2R2 to value 0x0f00_0f00"]
impl crate::Resettable for SECWM2R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_0f00
    }
}
