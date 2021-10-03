#[doc = "Register `ETZPC_DECPROT3` reader"]
pub struct R(crate::R<ETZPC_DECPROT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_DECPROT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_DECPROT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_DECPROT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_DECPROT3` writer"]
pub struct W(crate::W<ETZPC_DECPROT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_DECPROT3_SPEC>;
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
impl From<crate::W<ETZPC_DECPROT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_DECPROT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECPROT0` reader - DECPROT0"]
pub struct DECPROT0_R(crate::FieldReader<u8, u8>);
impl DECPROT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT0` writer - DECPROT0"]
pub struct DECPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DECPROT1` reader - DECPROT1"]
pub struct DECPROT1_R(crate::FieldReader<u8, u8>);
impl DECPROT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT1` writer - DECPROT1"]
pub struct DECPROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DECPROT2` reader - DECPROT2"]
pub struct DECPROT2_R(crate::FieldReader<u8, u8>);
impl DECPROT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT2` writer - DECPROT2"]
pub struct DECPROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DECPROT3` reader - DECPROT3"]
pub struct DECPROT3_R(crate::FieldReader<u8, u8>);
impl DECPROT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT3` writer - DECPROT3"]
pub struct DECPROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DECPROT4` reader - DECPROT4"]
pub struct DECPROT4_R(crate::FieldReader<u8, u8>);
impl DECPROT4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT4` writer - DECPROT4"]
pub struct DECPROT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DECPROT5` reader - DECPROT5"]
pub struct DECPROT5_R(crate::FieldReader<u8, u8>);
impl DECPROT5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT5` writer - DECPROT5"]
pub struct DECPROT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `DECPROT6` reader - DECPROT6"]
pub struct DECPROT6_R(crate::FieldReader<u8, u8>);
impl DECPROT6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT6` writer - DECPROT6"]
pub struct DECPROT6_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DECPROT7` reader - DECPROT7"]
pub struct DECPROT7_R(crate::FieldReader<u8, u8>);
impl DECPROT7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT7` writer - DECPROT7"]
pub struct DECPROT7_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DECPROT8` reader - DECPROT8"]
pub struct DECPROT8_R(crate::FieldReader<u8, u8>);
impl DECPROT8_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT8` writer - DECPROT8"]
pub struct DECPROT8_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DECPROT9` reader - DECPROT9"]
pub struct DECPROT9_R(crate::FieldReader<u8, u8>);
impl DECPROT9_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT9` writer - DECPROT9"]
pub struct DECPROT9_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `DECPROT10` reader - DECPROT10"]
pub struct DECPROT10_R(crate::FieldReader<u8, u8>);
impl DECPROT10_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT10` writer - DECPROT10"]
pub struct DECPROT10_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DECPROT11` reader - DECPROT11"]
pub struct DECPROT11_R(crate::FieldReader<u8, u8>);
impl DECPROT11_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT11` writer - DECPROT11"]
pub struct DECPROT11_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `DECPROT12` reader - DECPROT12"]
pub struct DECPROT12_R(crate::FieldReader<u8, u8>);
impl DECPROT12_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT12` writer - DECPROT12"]
pub struct DECPROT12_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `DECPROT13` reader - DECPROT13"]
pub struct DECPROT13_R(crate::FieldReader<u8, u8>);
impl DECPROT13_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT13` writer - DECPROT13"]
pub struct DECPROT13_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `DECPROT14` reader - DECPROT14"]
pub struct DECPROT14_R(crate::FieldReader<u8, u8>);
impl DECPROT14_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT14` writer - DECPROT14"]
pub struct DECPROT14_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DECPROT15` reader - DECPROT15"]
pub struct DECPROT15_R(crate::FieldReader<u8, u8>);
impl DECPROT15_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECPROT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECPROT15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECPROT15` writer - DECPROT15"]
pub struct DECPROT15_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&self) -> DECPROT0_R {
        DECPROT0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&self) -> DECPROT1_R {
        DECPROT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&self) -> DECPROT2_R {
        DECPROT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&self) -> DECPROT3_R {
        DECPROT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&self) -> DECPROT4_R {
        DECPROT4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&self) -> DECPROT5_R {
        DECPROT5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&self) -> DECPROT6_R {
        DECPROT6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&self) -> DECPROT7_R {
        DECPROT7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&self) -> DECPROT8_R {
        DECPROT8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&self) -> DECPROT9_R {
        DECPROT9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&self) -> DECPROT10_R {
        DECPROT10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&self) -> DECPROT11_R {
        DECPROT11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&self) -> DECPROT12_R {
        DECPROT12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&self) -> DECPROT13_R {
        DECPROT13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&self) -> DECPROT14_R {
        DECPROT14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&self) -> DECPROT15_R {
        DECPROT15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&mut self) -> DECPROT0_W {
        DECPROT0_W { w: self }
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&mut self) -> DECPROT1_W {
        DECPROT1_W { w: self }
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&mut self) -> DECPROT2_W {
        DECPROT2_W { w: self }
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&mut self) -> DECPROT3_W {
        DECPROT3_W { w: self }
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&mut self) -> DECPROT4_W {
        DECPROT4_W { w: self }
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&mut self) -> DECPROT5_W {
        DECPROT5_W { w: self }
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&mut self) -> DECPROT6_W {
        DECPROT6_W { w: self }
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&mut self) -> DECPROT7_W {
        DECPROT7_W { w: self }
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&mut self) -> DECPROT8_W {
        DECPROT8_W { w: self }
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&mut self) -> DECPROT9_W {
        DECPROT9_W { w: self }
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&mut self) -> DECPROT10_W {
        DECPROT10_W { w: self }
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&mut self) -> DECPROT11_W {
        DECPROT11_W { w: self }
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&mut self) -> DECPROT12_W {
        DECPROT12_W { w: self }
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&mut self) -> DECPROT13_W {
        DECPROT13_W { w: self }
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&mut self) -> DECPROT14_W {
        DECPROT14_W { w: self }
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&mut self) -> DECPROT15_W {
        DECPROT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot3](index.html) module"]
pub struct ETZPC_DECPROT3_SPEC;
impl crate::RegisterSpec for ETZPC_DECPROT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_decprot3::R](R) reader structure"]
impl crate::Readable for ETZPC_DECPROT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot3::W](W) writer structure"]
impl crate::Writable for ETZPC_DECPROT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETZPC_DECPROT3 to value 0"]
impl crate::Resettable for ETZPC_DECPROT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
