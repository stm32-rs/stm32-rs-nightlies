#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSPEEDR15` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR15_R(crate::FieldReader<u8, u8>);
impl OSPEEDR15_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR15` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `OSPEEDR14` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR14_R(crate::FieldReader<u8, u8>);
impl OSPEEDR14_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR14` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `OSPEEDR13` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR13_R(crate::FieldReader<u8, u8>);
impl OSPEEDR13_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR13` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `OSPEEDR12` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR12_R(crate::FieldReader<u8, u8>);
impl OSPEEDR12_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR12` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `OSPEEDR11` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR11_R(crate::FieldReader<u8, u8>);
impl OSPEEDR11_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR11` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `OSPEEDR10` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR10_R(crate::FieldReader<u8, u8>);
impl OSPEEDR10_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR10` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `OSPEEDR9` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR9_R(crate::FieldReader<u8, u8>);
impl OSPEEDR9_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR9` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `OSPEEDR8` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR8_R(crate::FieldReader<u8, u8>);
impl OSPEEDR8_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR8` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OSPEEDR7` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR7_R(crate::FieldReader<u8, u8>);
impl OSPEEDR7_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR7` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `OSPEEDR6` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR6_R(crate::FieldReader<u8, u8>);
impl OSPEEDR6_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR6` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `OSPEEDR5` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR5_R(crate::FieldReader<u8, u8>);
impl OSPEEDR5_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR5` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR4_R(crate::FieldReader<u8, u8>);
impl OSPEEDR4_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR3_R(crate::FieldReader<u8, u8>);
impl OSPEEDR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR2_R(crate::FieldReader<u8, u8>);
impl OSPEEDR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR1_R(crate::FieldReader<u8, u8>);
impl OSPEEDR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR0_R(crate::FieldReader<u8, u8>);
impl OSPEEDR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEEDR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
        OSPEEDR15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
        OSPEEDR14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
        OSPEEDR13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
        OSPEEDR12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
        OSPEEDR11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
        OSPEEDR10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
        OSPEEDR9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
        OSPEEDR8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
        OSPEEDR7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
        OSPEEDR6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
        OSPEEDR5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
        OSPEEDR4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
        OSPEEDR3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
        OSPEEDR2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
        OSPEEDR1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
        OSPEEDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
