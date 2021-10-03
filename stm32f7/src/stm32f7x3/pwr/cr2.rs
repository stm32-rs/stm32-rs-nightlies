#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWUPF1` reader - Clear Wakeup Pin flag for PA0"]
pub struct CWUPF1_R(crate::FieldReader<bool, bool>);
impl CWUPF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUPF2` reader - Clear Wakeup Pin flag for PA2"]
pub struct CWUPF2_R(crate::FieldReader<bool, bool>);
impl CWUPF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUPF3` reader - Clear Wakeup Pin flag for PC1"]
pub struct CWUPF3_R(crate::FieldReader<bool, bool>);
impl CWUPF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUPF4` reader - Clear Wakeup Pin flag for PC13"]
pub struct CWUPF4_R(crate::FieldReader<bool, bool>);
impl CWUPF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUPF5` reader - Clear Wakeup Pin flag for PI8"]
pub struct CWUPF5_R(crate::FieldReader<bool, bool>);
impl CWUPF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUPF6` reader - Clear Wakeup Pin flag for PI11"]
pub struct CWUPF6_R(crate::FieldReader<bool, bool>);
impl CWUPF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUPF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUPF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP1` reader - Wakeup pin polarity bit for PA0"]
pub struct WUPP1_R(crate::FieldReader<bool, bool>);
impl WUPP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP1` writer - Wakeup pin polarity bit for PA0"]
pub struct WUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP1_W<'a> {
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
#[doc = "Field `WUPP2` reader - Wakeup pin polarity bit for PA2"]
pub struct WUPP2_R(crate::FieldReader<bool, bool>);
impl WUPP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP2` writer - Wakeup pin polarity bit for PA2"]
pub struct WUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP2_W<'a> {
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
#[doc = "Field `WUPP3` reader - Wakeup pin polarity bit for PC1"]
pub struct WUPP3_R(crate::FieldReader<bool, bool>);
impl WUPP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP3` writer - Wakeup pin polarity bit for PC1"]
pub struct WUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP3_W<'a> {
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
#[doc = "Field `WUPP4` reader - Wakeup pin polarity bit for PC13"]
pub struct WUPP4_R(crate::FieldReader<bool, bool>);
impl WUPP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP4` writer - Wakeup pin polarity bit for PC13"]
pub struct WUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `WUPP5` reader - Wakeup pin polarity bit for PI8"]
pub struct WUPP5_R(crate::FieldReader<bool, bool>);
impl WUPP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP5` writer - Wakeup pin polarity bit for PI8"]
pub struct WUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `WUPP6` reader - Wakeup pin polarity bit for PI11"]
pub struct WUPP6_R(crate::FieldReader<bool, bool>);
impl WUPP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPP6` writer - Wakeup pin polarity bit for PI11"]
pub struct WUPP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn cwupf1(&self) -> CWUPF1_R {
        CWUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn cwupf2(&self) -> CWUPF2_R {
        CWUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn cwupf3(&self) -> CWUPF3_R {
        CWUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn cwupf4(&self) -> CWUPF4_R {
        CWUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn cwupf5(&self) -> CWUPF5_R {
        CWUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn cwupf6(&self) -> CWUPF6_R {
        CWUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W {
        WUPP1_W { w: self }
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W {
        WUPP2_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W {
        WUPP3_W { w: self }
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W {
        WUPP4_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W {
        WUPP5_W { w: self }
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W {
        WUPP6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
