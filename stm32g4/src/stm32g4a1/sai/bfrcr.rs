#[doc = "Register `BFRCR` reader"]
pub struct R(crate::R<BFRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFRCR` writer"]
pub struct W(crate::W<BFRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFRCR_SPEC>;
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
impl From<crate::W<BFRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub struct FSOFF_R(crate::FieldReader<bool, bool>);
impl FSOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub struct FSOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFF_W<'a> {
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
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub struct FSPOL_R(crate::FieldReader<bool, bool>);
impl FSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub struct FSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPOL_W<'a> {
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
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub struct FSDEF_R(crate::FieldReader<bool, bool>);
impl FSDEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSDEF` writer - Frame synchronization definition"]
pub struct FSDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDEF_W<'a> {
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
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub struct FSALL_R(crate::FieldReader<u8, u8>);
impl FSALL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSALL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub struct FSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `FRL` reader - Frame length"]
pub struct FRL_R(crate::FieldReader<u8, u8>);
impl FRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRL` writer - Frame length"]
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W {
        FSOFF_W { w: self }
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W {
        FSPOL_W { w: self }
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W {
        FSDEF_W { w: self }
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W {
        FSALL_W { w: self }
    }
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfrcr](index.html) module"]
pub struct BFRCR_SPEC;
impl crate::RegisterSpec for BFRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfrcr::R](R) reader structure"]
impl crate::Readable for BFRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfrcr::W](W) writer structure"]
impl crate::Writable for BFRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFRCR to value 0x07"]
impl crate::Resettable for BFRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
