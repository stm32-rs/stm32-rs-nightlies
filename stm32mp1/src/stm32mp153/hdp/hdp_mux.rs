#[doc = "Register `HDP_MUX` reader"]
pub struct R(crate::R<HDP_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDP_MUX` writer"]
pub struct W(crate::W<HDP_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP_MUX_SPEC>;
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
impl From<crate::W<HDP_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX0` reader - MUX0"]
pub struct MUX0_R(crate::FieldReader<u8, u8>);
impl MUX0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX0` writer - MUX0"]
pub struct MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `MUX1` reader - MUX1"]
pub struct MUX1_R(crate::FieldReader<u8, u8>);
impl MUX1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX1` writer - MUX1"]
pub struct MUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `MUX2` reader - MUX2"]
pub struct MUX2_R(crate::FieldReader<u8, u8>);
impl MUX2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX2` writer - MUX2"]
pub struct MUX2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MUX3` reader - MUX3"]
pub struct MUX3_R(crate::FieldReader<u8, u8>);
impl MUX3_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX3` writer - MUX3"]
pub struct MUX3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `MUX4` reader - MUX4"]
pub struct MUX4_R(crate::FieldReader<u8, u8>);
impl MUX4_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX4` writer - MUX4"]
pub struct MUX4_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MUX5` reader - MUX5"]
pub struct MUX5_R(crate::FieldReader<u8, u8>);
impl MUX5_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX5` writer - MUX5"]
pub struct MUX5_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `MUX6` reader - MUX6"]
pub struct MUX6_R(crate::FieldReader<u8, u8>);
impl MUX6_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX6` writer - MUX6"]
pub struct MUX6_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `MUX7` reader - MUX7"]
pub struct MUX7_R(crate::FieldReader<u8, u8>);
impl MUX7_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX7` writer - MUX7"]
pub struct MUX7_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - MUX0"]
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MUX1"]
    #[inline(always)]
    pub fn mux1(&self) -> MUX1_R {
        MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MUX2"]
    #[inline(always)]
    pub fn mux2(&self) -> MUX2_R {
        MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MUX3"]
    #[inline(always)]
    pub fn mux3(&self) -> MUX3_R {
        MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MUX4"]
    #[inline(always)]
    pub fn mux4(&self) -> MUX4_R {
        MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MUX5"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MUX6"]
    #[inline(always)]
    pub fn mux6(&self) -> MUX6_R {
        MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - MUX7"]
    #[inline(always)]
    pub fn mux7(&self) -> MUX7_R {
        MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MUX0"]
    #[inline(always)]
    pub fn mux0(&mut self) -> MUX0_W {
        MUX0_W { w: self }
    }
    #[doc = "Bits 4:7 - MUX1"]
    #[inline(always)]
    pub fn mux1(&mut self) -> MUX1_W {
        MUX1_W { w: self }
    }
    #[doc = "Bits 8:11 - MUX2"]
    #[inline(always)]
    pub fn mux2(&mut self) -> MUX2_W {
        MUX2_W { w: self }
    }
    #[doc = "Bits 12:15 - MUX3"]
    #[inline(always)]
    pub fn mux3(&mut self) -> MUX3_W {
        MUX3_W { w: self }
    }
    #[doc = "Bits 16:19 - MUX4"]
    #[inline(always)]
    pub fn mux4(&mut self) -> MUX4_W {
        MUX4_W { w: self }
    }
    #[doc = "Bits 20:23 - MUX5"]
    #[inline(always)]
    pub fn mux5(&mut self) -> MUX5_W {
        MUX5_W { w: self }
    }
    #[doc = "Bits 24:27 - MUX6"]
    #[inline(always)]
    pub fn mux6(&mut self) -> MUX6_W {
        MUX6_W { w: self }
    }
    #[doc = "Bits 28:31 - MUX7"]
    #[inline(always)]
    pub fn mux7(&mut self) -> MUX7_W {
        MUX7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDP multiplexing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_mux](index.html) module"]
pub struct HDP_MUX_SPEC;
impl crate::RegisterSpec for HDP_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp_mux::R](R) reader structure"]
impl crate::Readable for HDP_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hdp_mux::W](W) writer structure"]
impl crate::Writable for HDP_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDP_MUX to value 0"]
impl crate::Resettable for HDP_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
