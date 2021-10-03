#[doc = "Register `PRIVCFGR2` reader"]
pub struct R(crate::R<PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR2` writer"]
pub struct W(crate::W<PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR2_SPEC>;
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
impl From<crate::W<PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV32` reader - PRIV32"]
pub struct PRIV32_R(crate::FieldReader<bool, bool>);
impl PRIV32_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV32` writer - PRIV32"]
pub struct PRIV32_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV32_W<'a> {
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
#[doc = "Field `PRIV33` reader - PRIV33"]
pub struct PRIV33_R(crate::FieldReader<bool, bool>);
impl PRIV33_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV33` writer - PRIV33"]
pub struct PRIV33_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV33_W<'a> {
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
#[doc = "Field `PRIV34` reader - PRIV34"]
pub struct PRIV34_R(crate::FieldReader<bool, bool>);
impl PRIV34_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV34` writer - PRIV34"]
pub struct PRIV34_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV34_W<'a> {
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
#[doc = "Field `PRIV35` reader - PRIV35"]
pub struct PRIV35_R(crate::FieldReader<bool, bool>);
impl PRIV35_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV35` writer - PRIV35"]
pub struct PRIV35_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV35_W<'a> {
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
#[doc = "Field `PRIV36` reader - PRIV36"]
pub struct PRIV36_R(crate::FieldReader<bool, bool>);
impl PRIV36_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV36` writer - PRIV36"]
pub struct PRIV36_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV36_W<'a> {
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
#[doc = "Field `PRIV37` reader - PRIV37"]
pub struct PRIV37_R(crate::FieldReader<bool, bool>);
impl PRIV37_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV37` writer - PRIV37"]
pub struct PRIV37_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV37_W<'a> {
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
#[doc = "Field `PRIV38` reader - PRIV38"]
pub struct PRIV38_R(crate::FieldReader<bool, bool>);
impl PRIV38_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV38` writer - PRIV38"]
pub struct PRIV38_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV38_W<'a> {
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
#[doc = "Field `PRIV39` reader - PRIV39"]
pub struct PRIV39_R(crate::FieldReader<bool, bool>);
impl PRIV39_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV39` writer - PRIV39"]
pub struct PRIV39_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV39_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PRIV40` reader - PRIV40"]
pub struct PRIV40_R(crate::FieldReader<bool, bool>);
impl PRIV40_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV40` writer - PRIV40"]
pub struct PRIV40_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV40_W<'a> {
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
#[doc = "Field `PRIV41` reader - PRIV41"]
pub struct PRIV41_R(crate::FieldReader<bool, bool>);
impl PRIV41_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV41` writer - PRIV41"]
pub struct PRIV41_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV41_W<'a> {
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
#[doc = "Field `PRIV42` reader - PRIV42"]
pub struct PRIV42_R(crate::FieldReader<bool, bool>);
impl PRIV42_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV42` writer - PRIV42"]
pub struct PRIV42_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV42_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    pub fn priv32(&mut self) -> PRIV32_W {
        PRIV32_W { w: self }
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    pub fn priv33(&mut self) -> PRIV33_W {
        PRIV33_W { w: self }
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    pub fn priv34(&mut self) -> PRIV34_W {
        PRIV34_W { w: self }
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    pub fn priv35(&mut self) -> PRIV35_W {
        PRIV35_W { w: self }
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    pub fn priv36(&mut self) -> PRIV36_W {
        PRIV36_W { w: self }
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    pub fn priv37(&mut self) -> PRIV37_W {
        PRIV37_W { w: self }
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    pub fn priv38(&mut self) -> PRIV38_W {
        PRIV38_W { w: self }
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    pub fn priv39(&mut self) -> PRIV39_W {
        PRIV39_W { w: self }
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    pub fn priv40(&mut self) -> PRIV40_W {
        PRIV40_W { w: self }
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    pub fn priv41(&mut self) -> PRIV41_W {
        PRIV41_W { w: self }
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    pub fn priv42(&mut self) -> PRIV42_W {
        PRIV42_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI security enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr2](index.html) module"]
pub struct PRIVCFGR2_SPEC;
impl crate::RegisterSpec for PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr2::R](R) reader structure"]
impl crate::Readable for PRIVCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr2::W](W) writer structure"]
impl crate::Writable for PRIVCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
