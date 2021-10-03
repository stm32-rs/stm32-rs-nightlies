#[doc = "Register `ITCMCR` reader"]
pub struct R(crate::R<ITCMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITCMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITCMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITCMCR` writer"]
pub struct W(crate::W<ITCMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITCMCR_SPEC>;
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
impl From<crate::W<ITCMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITCMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `RMW` reader - RMW"]
pub struct RMW_R(crate::FieldReader<bool, bool>);
impl RMW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMW` writer - RMW"]
pub struct RMW_W<'a> {
    w: &'a mut W,
}
impl<'a> RMW_W<'a> {
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
#[doc = "Field `RETEN` reader - RETEN"]
pub struct RETEN_R(crate::FieldReader<bool, bool>);
impl RETEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETEN` writer - RETEN"]
pub struct RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETEN_W<'a> {
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
#[doc = "Field `SZ` reader - SZ"]
pub struct SZ_R(crate::FieldReader<u8, u8>);
impl SZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SZ` writer - SZ"]
pub struct SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    pub fn rmw(&mut self) -> RMW_W {
        RMW_W { w: self }
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    pub fn reten(&mut self) -> RETEN_W {
        RETEN_W { w: self }
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W {
        SZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itcmcr](index.html) module"]
pub struct ITCMCR_SPEC;
impl crate::RegisterSpec for ITCMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itcmcr::R](R) reader structure"]
impl crate::Readable for ITCMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itcmcr::W](W) writer structure"]
impl crate::Writable for ITCMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITCMCR to value 0"]
impl crate::Resettable for ITCMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
