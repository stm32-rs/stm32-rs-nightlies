#[doc = "Register `PDCRD` reader"]
pub struct R(crate::R<PDCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRD` writer"]
pub struct W(crate::W<PDCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRD_SPEC>;
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
impl From<crate::W<PDCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD9` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD9_R(crate::FieldReader<bool, bool>);
impl PD9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD9` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_W<'a> {
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
#[doc = "Field `PD8` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD8_R(crate::FieldReader<bool, bool>);
impl PD8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD8` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_W<'a> {
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
#[doc = "Field `PD6` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD6_R(crate::FieldReader<bool, bool>);
impl PD6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD6` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PD5` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD5_R(crate::FieldReader<bool, bool>);
impl PD5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD5` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PD4` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD4_R(crate::FieldReader<bool, bool>);
impl PD4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD4` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PD3` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD3_R(crate::FieldReader<bool, bool>);
impl PD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD3` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PD2` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD2_R(crate::FieldReader<bool, bool>);
impl PD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD2` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
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
#[doc = "Field `PD1` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD1_R(crate::FieldReader<bool, bool>);
impl PD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD1` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PD0` reader - Port D pull-down bit y (y=0..15)"]
pub struct PD0_R(crate::FieldReader<bool, bool>);
impl PD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD0` writer - Port D pull-down bit y (y=0..15)"]
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W { w: self }
    }
    #[doc = "Bit 8 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W { w: self }
    }
    #[doc = "Bit 6 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    #[doc = "Bit 5 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    #[doc = "Bit 4 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    #[doc = "Bit 3 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    #[doc = "Bit 2 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    #[doc = "Bit 1 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    #[doc = "Bit 0 - Port D pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port D pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrd](index.html) module"]
pub struct PDCRD_SPEC;
impl crate::RegisterSpec for PDCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrd::R](R) reader structure"]
impl crate::Readable for PDCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrd::W](W) writer structure"]
impl crate::Writable for PDCRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDCRD to value 0"]
impl crate::Resettable for PDCRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
