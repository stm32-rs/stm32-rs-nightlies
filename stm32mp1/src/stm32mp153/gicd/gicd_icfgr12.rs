#[doc = "Register `GICD_ICFGR12` reader"]
pub struct R(crate::R<GICD_ICFGR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR12` writer"]
pub struct W(crate::W<GICD_ICFGR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR12_SPEC>;
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
impl From<crate::W<GICD_ICFGR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_CONFIG0` reader - INT_CONFIG0"]
pub struct INT_CONFIG0_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG0_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG0` writer - INT_CONFIG0"]
pub struct INT_CONFIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `INT_CONFIG1` reader - INT_CONFIG1"]
pub struct INT_CONFIG1_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG1` writer - INT_CONFIG1"]
pub struct INT_CONFIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `INT_CONFIG2` reader - INT_CONFIG2"]
pub struct INT_CONFIG2_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG2` writer - INT_CONFIG2"]
pub struct INT_CONFIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `INT_CONFIG3` reader - INT_CONFIG3"]
pub struct INT_CONFIG3_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG3` writer - INT_CONFIG3"]
pub struct INT_CONFIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `INT_CONFIG4` reader - INT_CONFIG4"]
pub struct INT_CONFIG4_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG4_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG4` writer - INT_CONFIG4"]
pub struct INT_CONFIG4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `INT_CONFIG5` reader - INT_CONFIG5"]
pub struct INT_CONFIG5_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG5_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG5` writer - INT_CONFIG5"]
pub struct INT_CONFIG5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `INT_CONFIG6` reader - INT_CONFIG6"]
pub struct INT_CONFIG6_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG6_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG6` writer - INT_CONFIG6"]
pub struct INT_CONFIG6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `INT_CONFIG7` reader - INT_CONFIG7"]
pub struct INT_CONFIG7_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG7_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG7` writer - INT_CONFIG7"]
pub struct INT_CONFIG7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `INT_CONFIG8` reader - INT_CONFIG8"]
pub struct INT_CONFIG8_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG8_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG8` writer - INT_CONFIG8"]
pub struct INT_CONFIG8_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `INT_CONFIG9` reader - INT_CONFIG9"]
pub struct INT_CONFIG9_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG9_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG9` writer - INT_CONFIG9"]
pub struct INT_CONFIG9_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `INT_CONFIG10` reader - INT_CONFIG10"]
pub struct INT_CONFIG10_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG10_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG10` writer - INT_CONFIG10"]
pub struct INT_CONFIG10_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `INT_CONFIG11` reader - INT_CONFIG11"]
pub struct INT_CONFIG11_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG11_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG11` writer - INT_CONFIG11"]
pub struct INT_CONFIG11_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `INT_CONFIG12` reader - INT_CONFIG12"]
pub struct INT_CONFIG12_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG12_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG12` writer - INT_CONFIG12"]
pub struct INT_CONFIG12_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT_CONFIG13` reader - INT_CONFIG13"]
pub struct INT_CONFIG13_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG13_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG13` writer - INT_CONFIG13"]
pub struct INT_CONFIG13_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `INT_CONFIG14` reader - INT_CONFIG14"]
pub struct INT_CONFIG14_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG14_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG14` writer - INT_CONFIG14"]
pub struct INT_CONFIG14_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `INT_CONFIG15` reader - INT_CONFIG15"]
pub struct INT_CONFIG15_R(crate::FieldReader<u8, u8>);
impl INT_CONFIG15_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_CONFIG15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_CONFIG15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_CONFIG15` writer - INT_CONFIG15"]
pub struct INT_CONFIG15_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CONFIG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&self) -> INT_CONFIG0_R {
        INT_CONFIG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&self) -> INT_CONFIG1_R {
        INT_CONFIG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&self) -> INT_CONFIG2_R {
        INT_CONFIG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&self) -> INT_CONFIG3_R {
        INT_CONFIG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&self) -> INT_CONFIG4_R {
        INT_CONFIG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&self) -> INT_CONFIG5_R {
        INT_CONFIG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&self) -> INT_CONFIG6_R {
        INT_CONFIG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&self) -> INT_CONFIG7_R {
        INT_CONFIG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&self) -> INT_CONFIG8_R {
        INT_CONFIG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&self) -> INT_CONFIG9_R {
        INT_CONFIG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&self) -> INT_CONFIG10_R {
        INT_CONFIG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&self) -> INT_CONFIG11_R {
        INT_CONFIG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&self) -> INT_CONFIG12_R {
        INT_CONFIG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&self) -> INT_CONFIG13_R {
        INT_CONFIG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&self) -> INT_CONFIG14_R {
        INT_CONFIG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&self) -> INT_CONFIG15_R {
        INT_CONFIG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - INT_CONFIG0"]
    #[inline(always)]
    pub fn int_config0(&mut self) -> INT_CONFIG0_W {
        INT_CONFIG0_W { w: self }
    }
    #[doc = "Bits 2:3 - INT_CONFIG1"]
    #[inline(always)]
    pub fn int_config1(&mut self) -> INT_CONFIG1_W {
        INT_CONFIG1_W { w: self }
    }
    #[doc = "Bits 4:5 - INT_CONFIG2"]
    #[inline(always)]
    pub fn int_config2(&mut self) -> INT_CONFIG2_W {
        INT_CONFIG2_W { w: self }
    }
    #[doc = "Bits 6:7 - INT_CONFIG3"]
    #[inline(always)]
    pub fn int_config3(&mut self) -> INT_CONFIG3_W {
        INT_CONFIG3_W { w: self }
    }
    #[doc = "Bits 8:9 - INT_CONFIG4"]
    #[inline(always)]
    pub fn int_config4(&mut self) -> INT_CONFIG4_W {
        INT_CONFIG4_W { w: self }
    }
    #[doc = "Bits 10:11 - INT_CONFIG5"]
    #[inline(always)]
    pub fn int_config5(&mut self) -> INT_CONFIG5_W {
        INT_CONFIG5_W { w: self }
    }
    #[doc = "Bits 12:13 - INT_CONFIG6"]
    #[inline(always)]
    pub fn int_config6(&mut self) -> INT_CONFIG6_W {
        INT_CONFIG6_W { w: self }
    }
    #[doc = "Bits 14:15 - INT_CONFIG7"]
    #[inline(always)]
    pub fn int_config7(&mut self) -> INT_CONFIG7_W {
        INT_CONFIG7_W { w: self }
    }
    #[doc = "Bits 16:17 - INT_CONFIG8"]
    #[inline(always)]
    pub fn int_config8(&mut self) -> INT_CONFIG8_W {
        INT_CONFIG8_W { w: self }
    }
    #[doc = "Bits 18:19 - INT_CONFIG9"]
    #[inline(always)]
    pub fn int_config9(&mut self) -> INT_CONFIG9_W {
        INT_CONFIG9_W { w: self }
    }
    #[doc = "Bits 20:21 - INT_CONFIG10"]
    #[inline(always)]
    pub fn int_config10(&mut self) -> INT_CONFIG10_W {
        INT_CONFIG10_W { w: self }
    }
    #[doc = "Bits 22:23 - INT_CONFIG11"]
    #[inline(always)]
    pub fn int_config11(&mut self) -> INT_CONFIG11_W {
        INT_CONFIG11_W { w: self }
    }
    #[doc = "Bits 24:25 - INT_CONFIG12"]
    #[inline(always)]
    pub fn int_config12(&mut self) -> INT_CONFIG12_W {
        INT_CONFIG12_W { w: self }
    }
    #[doc = "Bits 26:27 - INT_CONFIG13"]
    #[inline(always)]
    pub fn int_config13(&mut self) -> INT_CONFIG13_W {
        INT_CONFIG13_W { w: self }
    }
    #[doc = "Bits 28:29 - INT_CONFIG14"]
    #[inline(always)]
    pub fn int_config14(&mut self) -> INT_CONFIG14_W {
        INT_CONFIG14_W { w: self }
    }
    #[doc = "Bits 30:31 - INT_CONFIG15"]
    #[inline(always)]
    pub fn int_config15(&mut self) -> INT_CONFIG15_W {
        INT_CONFIG15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD interrupt configuration register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr12](index.html) module"]
pub struct GICD_ICFGR12_SPEC;
impl crate::RegisterSpec for GICD_ICFGR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr12::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr12::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICFGR12 to value 0x5555_5555"]
impl crate::Resettable for GICD_ICFGR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5555_5555
    }
}
