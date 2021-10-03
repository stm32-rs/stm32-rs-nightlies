#[doc = "Register `CFG3` reader"]
pub struct R(crate::R<CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG3` writer"]
pub struct W(crate::W<CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG3_SPEC>;
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
impl From<crate::W<CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM1_NG_CCRPD` reader - TRIM1_NG_CCRPD"]
pub struct TRIM1_NG_CCRPD_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CCRPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CCRPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CCRPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM1_NG_CCRPD` writer - TRIM1_NG_CCRPD"]
pub struct TRIM1_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CCRPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TRIM1_NG_CC1A5` reader - TRIM1_NG_CC1A5"]
pub struct TRIM1_NG_CC1A5_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CC1A5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CC1A5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CC1A5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM1_NG_CC1A5` writer - TRIM1_NG_CC1A5"]
pub struct TRIM1_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC1A5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `TRIM1_NG_CC3A0` reader - TRIM1_NG_CC3A0"]
pub struct TRIM1_NG_CC3A0_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CC3A0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CC3A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CC3A0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM1_NG_CC3A0` writer - TRIM1_NG_CC3A0"]
pub struct TRIM1_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC3A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `TRIM2_NG_CCRPD` reader - TRIM2_NG_CCRPD"]
pub struct TRIM2_NG_CCRPD_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CCRPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CCRPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CCRPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM2_NG_CCRPD` writer - TRIM2_NG_CCRPD"]
pub struct TRIM2_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CCRPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TRIM2_NG_CC1A5` reader - TRIM2_NG_CC1A5"]
pub struct TRIM2_NG_CC1A5_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CC1A5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CC1A5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CC1A5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM2_NG_CC1A5` writer - TRIM2_NG_CC1A5"]
pub struct TRIM2_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC1A5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `TRIM2_NG_CC3A0` reader - TRIM2_NG_CC3A0"]
pub struct TRIM2_NG_CC3A0_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CC3A0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CC3A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CC3A0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM2_NG_CC3A0` writer - TRIM2_NG_CC3A0"]
pub struct TRIM2_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC3A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W {
        TRIM1_NG_CCRPD_W { w: self }
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W {
        TRIM1_NG_CC1A5_W { w: self }
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W {
        TRIM1_NG_CC3A0_W { w: self }
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W {
        TRIM2_NG_CCRPD_W { w: self }
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W {
        TRIM2_NG_CC1A5_W { w: self }
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W {
        TRIM2_NG_CC3A0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg3](index.html) module"]
pub struct CFG3_SPEC;
impl crate::RegisterSpec for CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg3::R](R) reader structure"]
impl crate::Readable for CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg3::W](W) writer structure"]
impl crate::Writable for CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
