#[doc = "Register `FDCAN_TTMLM` reader"]
pub struct R(crate::R<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTMLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTMLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTMLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTMLM` writer"]
pub struct W(crate::W<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTMLM_SPEC>;
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
impl From<crate::W<FDCAN_TTMLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTMLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCM` reader - Cycle Count Max"]
pub struct CCM_R(crate::FieldReader<u8, u8>);
impl CCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM` writer - Cycle Count Max"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CSS` reader - Cycle Start Synchronization"]
pub struct CSS_R(crate::FieldReader<u8, u8>);
impl CSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSS` writer - Cycle Start Synchronization"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TXEW` reader - Tx Enable Window"]
pub struct TXEW_R(crate::FieldReader<u8, u8>);
impl TXEW_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEW` writer - Tx Enable Window"]
pub struct TXEW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ENTT` reader - Expected Number of Tx Triggers"]
pub struct ENTT_R(crate::FieldReader<u16, u16>);
impl ENTT_R {
    pub(crate) fn new(bits: u16) -> Self {
        ENTT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTT` writer - Expected Number of Tx Triggers"]
pub struct ENTT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    pub fn txew(&mut self) -> TXEW_W {
        TXEW_W { w: self }
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    pub fn entt(&mut self) -> ENTT_W {
        ENTT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Matrix Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttmlm](index.html) module"]
pub struct FDCAN_TTMLM_SPEC;
impl crate::RegisterSpec for FDCAN_TTMLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttmlm::R](R) reader structure"]
impl crate::Readable for FDCAN_TTMLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttmlm::W](W) writer structure"]
impl crate::Writable for FDCAN_TTMLM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTMLM to value 0"]
impl crate::Resettable for FDCAN_TTMLM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
