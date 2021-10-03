#[doc = "Register `DDRPHYC_DX3DQTR` reader"]
pub struct R(crate::R<DDRPHYC_DX3DQTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3DQTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3DQTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3DQTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DX3DQTR` writer"]
pub struct W(crate::W<DDRPHYC_DX3DQTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX3DQTR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX3DQTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX3DQTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQDLY0` reader - DQDLY0"]
pub struct DQDLY0_R(crate::FieldReader<u8, u8>);
impl DQDLY0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY0` writer - DQDLY0"]
pub struct DQDLY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DQDLY1` reader - DQDLY1"]
pub struct DQDLY1_R(crate::FieldReader<u8, u8>);
impl DQDLY1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY1` writer - DQDLY1"]
pub struct DQDLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DQDLY2` reader - DQDLY2"]
pub struct DQDLY2_R(crate::FieldReader<u8, u8>);
impl DQDLY2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY2` writer - DQDLY2"]
pub struct DQDLY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DQDLY3` reader - DQDLY3"]
pub struct DQDLY3_R(crate::FieldReader<u8, u8>);
impl DQDLY3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY3` writer - DQDLY3"]
pub struct DQDLY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `DQDLY4` reader - DQDLY4"]
pub struct DQDLY4_R(crate::FieldReader<u8, u8>);
impl DQDLY4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY4` writer - DQDLY4"]
pub struct DQDLY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DQDLY5` reader - DQDLY5"]
pub struct DQDLY5_R(crate::FieldReader<u8, u8>);
impl DQDLY5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY5` writer - DQDLY5"]
pub struct DQDLY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `DQDLY6` reader - DQDLY6"]
pub struct DQDLY6_R(crate::FieldReader<u8, u8>);
impl DQDLY6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY6` writer - DQDLY6"]
pub struct DQDLY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `DQDLY7` reader - DQDLY7"]
pub struct DQDLY7_R(crate::FieldReader<u8, u8>);
impl DQDLY7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQDLY7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQDLY7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQDLY7` writer - DQDLY7"]
pub struct DQDLY7_W<'a> {
    w: &'a mut W,
}
impl<'a> DQDLY7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    pub fn dqdly0(&self) -> DQDLY0_R {
        DQDLY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    pub fn dqdly1(&self) -> DQDLY1_R {
        DQDLY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    pub fn dqdly2(&self) -> DQDLY2_R {
        DQDLY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    pub fn dqdly3(&self) -> DQDLY3_R {
        DQDLY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    pub fn dqdly4(&self) -> DQDLY4_R {
        DQDLY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    pub fn dqdly5(&self) -> DQDLY5_R {
        DQDLY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    pub fn dqdly6(&self) -> DQDLY6_R {
        DQDLY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    pub fn dqdly7(&self) -> DQDLY7_R {
        DQDLY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    pub fn dqdly0(&mut self) -> DQDLY0_W {
        DQDLY0_W { w: self }
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    pub fn dqdly1(&mut self) -> DQDLY1_W {
        DQDLY1_W { w: self }
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    pub fn dqdly2(&mut self) -> DQDLY2_W {
        DQDLY2_W { w: self }
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    pub fn dqdly3(&mut self) -> DQDLY3_W {
        DQDLY3_W { w: self }
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    pub fn dqdly4(&mut self) -> DQDLY4_W {
        DQDLY4_W { w: self }
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    pub fn dqdly5(&mut self) -> DQDLY5_W {
        DQDLY5_W { w: self }
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    pub fn dqdly6(&mut self) -> DQDLY6_W {
        DQDLY6_W { w: self }
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    pub fn dqdly7(&mut self) -> DQDLY7_W {
        DQDLY7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC byte lane 3 DQT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqtr](index.html) module"]
pub struct DDRPHYC_DX3DQTR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3DQTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx3dqtr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqtr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DX3DQTR to value 0xffff_ffff"]
impl crate::Resettable for DDRPHYC_DX3DQTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
