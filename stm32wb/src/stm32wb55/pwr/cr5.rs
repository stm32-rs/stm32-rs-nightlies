#[doc = "Register `CR5` reader"]
pub struct R(crate::R<CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR5` writer"]
pub struct W(crate::W<CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR5_SPEC>;
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
impl From<crate::W<CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDEB` reader - Enable Step Down converter SMPS mode enabled"]
pub struct SDEB_R(crate::FieldReader<bool, bool>);
impl SDEB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDEB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDEB` writer - Enable Step Down converter SMPS mode enabled"]
pub struct SDEB_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEB_W<'a> {
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
#[doc = "Field `SDBEN` reader - Enable Step Down converter Bypass mode enabled"]
pub struct SDBEN_R(crate::FieldReader<bool, bool>);
impl SDBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDBEN` writer - Enable Step Down converter Bypass mode enabled"]
pub struct SDBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SMPSCFG` reader - VOS configuration selection (non user)"]
pub struct SMPSCFG_R(crate::FieldReader<bool, bool>);
impl SMPSCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSCFG` writer - VOS configuration selection (non user)"]
pub struct SMPSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSCFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BORHC` reader - BORH configuration selection"]
pub struct BORHC_R(crate::FieldReader<bool, bool>);
impl BORHC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORHC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORHC` writer - BORH configuration selection"]
pub struct BORHC_W<'a> {
    w: &'a mut W,
}
impl<'a> BORHC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SDSC` reader - Step Down converter supplt startup current selection"]
pub struct SDSC_R(crate::FieldReader<u8, u8>);
impl SDSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDSC` writer - Step Down converter supplt startup current selection"]
pub struct SDSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SDVOS` reader - Step Down converter voltage output scaling"]
pub struct SDVOS_R(crate::FieldReader<u8, u8>);
impl SDVOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDVOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDVOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDVOS` writer - Step Down converter voltage output scaling"]
pub struct SDVOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDVOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&self) -> SDEB_R {
        SDEB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&self) -> SDBEN_R {
        SDBEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&self) -> SMPSCFG_R {
        SMPSCFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&self) -> BORHC_R {
        BORHC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&self) -> SDSC_R {
        SDSC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&self) -> SDVOS_R {
        SDVOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&mut self) -> SDEB_W {
        SDEB_W { w: self }
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&mut self) -> SDBEN_W {
        SDBEN_W { w: self }
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&mut self) -> SMPSCFG_W {
        SMPSCFG_W { w: self }
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&mut self) -> BORHC_W {
        BORHC_W { w: self }
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&mut self) -> SDSC_W {
        SDSC_W { w: self }
    }
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&mut self) -> SDVOS_W {
        SDVOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](index.html) module"]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr5::R](R) reader structure"]
impl crate::Readable for CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr5::W](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR5 to value 0x4270"]
impl crate::Resettable for CR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4270
    }
}
