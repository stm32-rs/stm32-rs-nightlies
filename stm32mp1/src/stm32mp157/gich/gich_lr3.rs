#[doc = "Register `GICH_LR3` reader"]
pub struct R(crate::R<GICH_LR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_LR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_LR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_LR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICH_LR3` writer"]
pub struct W(crate::W<GICH_LR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_LR3_SPEC>;
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
impl From<crate::W<GICH_LR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_LR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VIRTUALID` reader - VIRTUALID"]
pub struct VIRTUALID_R(crate::FieldReader<u16, u16>);
impl VIRTUALID_R {
    pub(crate) fn new(bits: u16) -> Self {
        VIRTUALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIRTUALID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIRTUALID` writer - VIRTUALID"]
pub struct VIRTUALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VIRTUALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PHYSICALID` reader - PHYSICALID"]
pub struct PHYSICALID_R(crate::FieldReader<u16, u16>);
impl PHYSICALID_R {
    pub(crate) fn new(bits: u16) -> Self {
        PHYSICALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYSICALID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYSICALID` writer - PHYSICALID"]
pub struct PHYSICALID_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSICALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub struct PRIORITY_R(crate::FieldReader<u8, u8>);
impl PRIORITY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY` writer - PRIORITY"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | ((value as u32 & 0x1f) << 23);
        self.w
    }
}
#[doc = "Field `STATE` reader - STATE"]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` writer - STATE"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `GRP1` reader - GRP1"]
pub struct GRP1_R(crate::FieldReader<bool, bool>);
impl GRP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GRP1` writer - GRP1"]
pub struct GRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GRP1_W<'a> {
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
#[doc = "Field `HW` reader - HW"]
pub struct HW_R(crate::FieldReader<bool, bool>);
impl HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW` writer - HW"]
pub struct HW_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_W<'a> {
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
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    pub fn virtualid(&mut self) -> VIRTUALID_W {
        VIRTUALID_W { w: self }
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    pub fn physicalid(&mut self) -> PHYSICALID_W {
        PHYSICALID_W { w: self }
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    pub fn grp1(&mut self) -> GRP1_W {
        GRP1_W { w: self }
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    pub fn hw(&mut self) -> HW_W {
        HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICH list register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_lr3](index.html) module"]
pub struct GICH_LR3_SPEC;
impl crate::RegisterSpec for GICH_LR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_lr3::R](R) reader structure"]
impl crate::Readable for GICH_LR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gich_lr3::W](W) writer structure"]
impl crate::Writable for GICH_LR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICH_LR3 to value 0"]
impl crate::Resettable for GICH_LR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
