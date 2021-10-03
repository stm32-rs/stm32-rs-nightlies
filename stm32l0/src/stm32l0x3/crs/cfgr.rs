#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection"]
pub struct SYNCPOL_R(crate::FieldReader<bool, bool>);
impl SYNCPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection"]
pub struct SYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCPOL_W<'a> {
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
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection"]
pub struct SYNCSRC_R(crate::FieldReader<u8, u8>);
impl SYNCSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection"]
pub struct SYNCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `SYNCDIV` reader - SYNC divider"]
pub struct SYNCDIV_R(crate::FieldReader<u8, u8>);
impl SYNCDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCDIV` writer - SYNC divider"]
pub struct SYNCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FELIM` reader - Frequency error limit"]
pub struct FELIM_R(crate::FieldReader<u8, u8>);
impl FELIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FELIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FELIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FELIM` writer - Frequency error limit"]
pub struct FELIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FELIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RELOAD` reader - Counter reload value"]
pub struct RELOAD_R(crate::FieldReader<u16, u16>);
impl RELOAD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD` writer - Counter reload value"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W {
        SYNCPOL_W { w: self }
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W {
        SYNCSRC_W { w: self }
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W {
        SYNCDIV_W { w: self }
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W {
        FELIM_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_bb7f
    }
}
