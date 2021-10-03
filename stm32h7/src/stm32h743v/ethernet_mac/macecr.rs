#[doc = "Register `MACECR` reader"]
pub struct R(crate::R<MACECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACECR` writer"]
pub struct W(crate::W<MACECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACECR_SPEC>;
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
impl From<crate::W<MACECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPSL` reader - Giant Packet Size Limit"]
pub struct GPSL_R(crate::FieldReader<u16, u16>);
impl GPSL_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPSL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPSL` writer - Giant Packet Size Limit"]
pub struct GPSL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `DCRCC` reader - Disable CRC Checking for Received Packets"]
pub struct DCRCC_R(crate::FieldReader<bool, bool>);
impl DCRCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCC` writer - Disable CRC Checking for Received Packets"]
pub struct DCRCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCC_W<'a> {
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
#[doc = "Field `SPEN` reader - Slow Protocol Detection Enable"]
pub struct SPEN_R(crate::FieldReader<bool, bool>);
impl SPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEN` writer - Slow Protocol Detection Enable"]
pub struct SPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `USP` reader - Unicast Slow Protocol Packet Detect"]
pub struct USP_R(crate::FieldReader<bool, bool>);
impl USP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USP` writer - Unicast Slow Protocol Packet Detect"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `EIPGEN` reader - Extended Inter-Packet Gap Enable"]
pub struct EIPGEN_R(crate::FieldReader<bool, bool>);
impl EIPGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIPGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIPGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIPGEN` writer - Extended Inter-Packet Gap Enable"]
pub struct EIPGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EIPGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EIPG` reader - Extended Inter-Packet Gap"]
pub struct EIPG_R(crate::FieldReader<u8, u8>);
impl EIPG_R {
    pub(crate) fn new(bits: u8) -> Self {
        EIPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIPG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIPG` writer - Extended Inter-Packet Gap"]
pub struct EIPG_W<'a> {
    w: &'a mut W,
}
impl<'a> EIPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    pub fn gpsl(&mut self) -> GPSL_W {
        GPSL_W { w: self }
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DCRCC_W {
        DCRCC_W { w: self }
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W {
        SPEN_W { w: self }
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub fn eipgen(&mut self) -> EIPGEN_W {
        EIPGEN_W { w: self }
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    pub fn eipg(&mut self) -> EIPG_W {
        EIPG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended operating mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macecr](index.html) module"]
pub struct MACECR_SPEC;
impl crate::RegisterSpec for MACECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macecr::R](R) reader structure"]
impl crate::Readable for MACECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macecr::W](W) writer structure"]
impl crate::Writable for MACECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACECR to value 0"]
impl crate::Resettable for MACECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
