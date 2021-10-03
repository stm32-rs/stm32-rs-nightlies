#[doc = "Register `OTG_DCFG` reader"]
pub struct R(crate::R<OTG_DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DCFG` writer"]
pub struct W(crate::W<OTG_DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DCFG_SPEC>;
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
impl From<crate::W<OTG_DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPD` reader - DSPD"]
pub struct DSPD_R(crate::FieldReader<u8, u8>);
impl DSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSPD` writer - DSPD"]
pub struct DSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `NZLSOHSK` reader - NZLSOHSK"]
pub struct NZLSOHSK_R(crate::FieldReader<bool, bool>);
impl NZLSOHSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NZLSOHSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NZLSOHSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NZLSOHSK` writer - NZLSOHSK"]
pub struct NZLSOHSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZLSOHSK_W<'a> {
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
#[doc = "Field `DAD` reader - DAD"]
pub struct DAD_R(crate::FieldReader<u8, u8>);
impl DAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAD` writer - DAD"]
pub struct DAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | ((value as u32 & 0x7f) << 4);
        self.w
    }
}
#[doc = "Field `PFIVL` reader - PFIVL"]
pub struct PFIVL_R(crate::FieldReader<u8, u8>);
impl PFIVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PFIVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFIVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFIVL` writer - PFIVL"]
pub struct PFIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `XCVRDLY` reader - XCVRDLY"]
pub struct XCVRDLY_R(crate::FieldReader<bool, bool>);
impl XCVRDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCVRDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCVRDLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCVRDLY` writer - XCVRDLY"]
pub struct XCVRDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> XCVRDLY_W<'a> {
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
#[doc = "Field `ERRATIM` reader - ERRATIM"]
pub struct ERRATIM_R(crate::FieldReader<bool, bool>);
impl ERRATIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRATIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRATIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRATIM` writer - ERRATIM"]
pub struct ERRATIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRATIM_W<'a> {
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
#[doc = "Field `PERSCHIVL` reader - PERSCHIVL"]
pub struct PERSCHIVL_R(crate::FieldReader<u8, u8>);
impl PERSCHIVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERSCHIVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERSCHIVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERSCHIVL` writer - PERSCHIVL"]
pub struct PERSCHIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSPD"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W {
        DSPD_W { w: self }
    }
    #[doc = "Bit 2 - NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W {
        NZLSOHSK_W { w: self }
    }
    #[doc = "Bits 4:10 - DAD"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W {
        DAD_W { w: self }
    }
    #[doc = "Bits 11:12 - PFIVL"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W {
        PFIVL_W { w: self }
    }
    #[doc = "Bit 14 - XCVRDLY"]
    #[inline(always)]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W {
        XCVRDLY_W { w: self }
    }
    #[doc = "Bit 15 - ERRATIM"]
    #[inline(always)]
    pub fn erratim(&mut self) -> ERRATIM_W {
        ERRATIM_W { w: self }
    }
    #[doc = "Bits 24:25 - PERSCHIVL"]
    #[inline(always)]
    pub fn perschivl(&mut self) -> PERSCHIVL_W {
        PERSCHIVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dcfg](index.html) module"]
pub struct OTG_DCFG_SPEC;
impl crate::RegisterSpec for OTG_DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dcfg::R](R) reader structure"]
impl crate::Readable for OTG_DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dcfg::W](W) writer structure"]
impl crate::Writable for OTG_DCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DCFG to value 0x0220_0000"]
impl crate::Resettable for OTG_DCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0220_0000
    }
}
