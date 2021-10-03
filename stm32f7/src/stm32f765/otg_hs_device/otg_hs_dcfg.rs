#[doc = "Register `OTG_HS_DCFG` reader"]
pub struct R(crate::R<OTG_HS_DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_DCFG` writer"]
pub struct W(crate::W<OTG_HS_DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_DCFG_SPEC>;
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
impl From<crate::W<OTG_HS_DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPD` reader - Device speed"]
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
#[doc = "Field `DSPD` writer - Device speed"]
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
#[doc = "Field `NZLSOHSK` reader - Nonzero-length status OUT handshake"]
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
#[doc = "Field `NZLSOHSK` writer - Nonzero-length status OUT handshake"]
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
#[doc = "Field `DAD` reader - Device address"]
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
#[doc = "Field `DAD` writer - Device address"]
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
#[doc = "Field `PFIVL` reader - Periodic (micro)frame interval"]
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
#[doc = "Field `PFIVL` writer - Periodic (micro)frame interval"]
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
#[doc = "Field `PERSCHIVL` reader - Periodic scheduling interval"]
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
#[doc = "Field `PERSCHIVL` writer - Periodic scheduling interval"]
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
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W {
        DSPD_W { w: self }
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W {
        NZLSOHSK_W { w: self }
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W {
        DAD_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W {
        PFIVL_W { w: self }
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
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
#[doc = "OTG_HS device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dcfg](index.html) module"]
pub struct OTG_HS_DCFG_SPEC;
impl crate::RegisterSpec for OTG_HS_DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_dcfg::R](R) reader structure"]
impl crate::Readable for OTG_HS_DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_dcfg::W](W) writer structure"]
impl crate::Writable for OTG_HS_DCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_DCFG to value 0x0220_0000"]
impl crate::Resettable for OTG_HS_DCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0220_0000
    }
}
