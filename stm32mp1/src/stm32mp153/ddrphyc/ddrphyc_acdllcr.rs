#[doc = "Register `DDRPHYC_ACDLLCR` reader"]
pub struct R(crate::R<DDRPHYC_ACDLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ACDLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ACDLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ACDLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ACDLLCR` writer"]
pub struct W(crate::W<DDRPHYC_ACDLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ACDLLCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ACDLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ACDLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFBDLY` reader - MFBDLY"]
pub struct MFBDLY_R(crate::FieldReader<u8, u8>);
impl MFBDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        MFBDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFBDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFBDLY` writer - MFBDLY"]
pub struct MFBDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFBDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `MFWDLY` reader - MFWDLY"]
pub struct MFWDLY_R(crate::FieldReader<u8, u8>);
impl MFWDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        MFWDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFWDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFWDLY` writer - MFWDLY"]
pub struct MFWDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MFWDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `ATESTEN` reader - ATESTEN"]
pub struct ATESTEN_R(crate::FieldReader<bool, bool>);
impl ATESTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATESTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATESTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATESTEN` writer - ATESTEN"]
pub struct ATESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTEN_W<'a> {
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
#[doc = "Field `DLLSRST` reader - DLLSRST"]
pub struct DLLSRST_R(crate::FieldReader<bool, bool>);
impl DLLSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLSRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLSRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub struct DLLSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DLLDIS` reader - DLLDIS"]
pub struct DLLDIS_R(crate::FieldReader<bool, bool>);
impl DLLDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLLDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLDIS` writer - DLLDIS"]
pub struct DLLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLDIS_W<'a> {
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
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&mut self) -> MFBDLY_W {
        MFBDLY_W { w: self }
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&mut self) -> MFWDLY_W {
        MFWDLY_W { w: self }
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&mut self) -> ATESTEN_W {
        ATESTEN_W { w: self }
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W {
        DLLSRST_W { w: self }
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&mut self) -> DLLDIS_W {
        DLLDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC AC DLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_acdllcr](index.html) module"]
pub struct DDRPHYC_ACDLLCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ACDLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_acdllcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ACDLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_acdllcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ACDLLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_ACDLLCR to value 0x4000_0000"]
impl crate::Resettable for DDRPHYC_ACDLLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
