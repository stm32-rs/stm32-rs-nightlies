#[doc = "Register `BSLOTR` reader"]
pub struct R(crate::R<BSLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSLOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSLOTR` writer"]
pub struct W(crate::W<BSLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSLOTR_SPEC>;
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
impl From<crate::W<BSLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSLOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOTEN` reader - Slot enable"]
pub struct SLOTEN_R(crate::FieldReader<u16, u16>);
impl SLOTEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        SLOTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTEN` writer - Slot enable"]
pub struct SLOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `NBSLOT` reader - Number of slots in an audio frame"]
pub struct NBSLOT_R(crate::FieldReader<u8, u8>);
impl NBSLOT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBSLOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBSLOT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBSLOT` writer - Number of slots in an audio frame"]
pub struct NBSLOT_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSLOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SLOTSZ` reader - Slot size"]
pub struct SLOTSZ_R(crate::FieldReader<u8, u8>);
impl SLOTSZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLOTSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTSZ` writer - Slot size"]
pub struct SLOTSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `FBOFF` reader - First bit offset"]
pub struct FBOFF_R(crate::FieldReader<u8, u8>);
impl FBOFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        FBOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBOFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBOFF` writer - First bit offset"]
pub struct FBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FBOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W {
        SLOTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W {
        NBSLOT_W { w: self }
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W {
        SLOTSZ_W { w: self }
    }
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W {
        FBOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSlot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bslotr](index.html) module"]
pub struct BSLOTR_SPEC;
impl crate::RegisterSpec for BSLOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bslotr::R](R) reader structure"]
impl crate::Readable for BSLOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bslotr::W](W) writer structure"]
impl crate::Writable for BSLOTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSLOTR to value 0"]
impl crate::Resettable for BSLOTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
