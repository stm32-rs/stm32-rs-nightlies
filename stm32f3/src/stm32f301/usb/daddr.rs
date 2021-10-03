#[doc = "Register `DADDR` reader"]
pub struct R(crate::R<DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR` writer"]
pub struct W(crate::W<DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR_SPEC>;
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
impl From<crate::W<DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - Device address"]
pub struct ADD_R(crate::FieldReader<bool, bool>);
impl ADD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD` writer - Device address"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ADD1` reader - Device address"]
pub struct ADD1_R(crate::FieldReader<bool, bool>);
impl ADD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD1` writer - Device address"]
pub struct ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1_W<'a> {
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
#[doc = "Field `ADD2` reader - Device address"]
pub struct ADD2_R(crate::FieldReader<bool, bool>);
impl ADD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD2` writer - Device address"]
pub struct ADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD2_W<'a> {
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
#[doc = "Field `ADD3` reader - Device address"]
pub struct ADD3_R(crate::FieldReader<bool, bool>);
impl ADD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD3` writer - Device address"]
pub struct ADD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD3_W<'a> {
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
#[doc = "Field `ADD4` reader - Device address"]
pub struct ADD4_R(crate::FieldReader<bool, bool>);
impl ADD4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD4` writer - Device address"]
pub struct ADD4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD4_W<'a> {
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
#[doc = "Field `ADD5` reader - Device address"]
pub struct ADD5_R(crate::FieldReader<bool, bool>);
impl ADD5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD5` writer - Device address"]
pub struct ADD5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD5_W<'a> {
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
#[doc = "Field `ADD6` reader - Device address"]
pub struct ADD6_R(crate::FieldReader<bool, bool>);
impl ADD6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD6` writer - Device address"]
pub struct ADD6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD6_W<'a> {
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
#[doc = "Enable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EF_A {
    #[doc = "0: USB device disabled"]
    DISABLED = 0,
    #[doc = "1: USB device enabled"]
    ENABLED = 1,
}
impl From<EF_A> for bool {
    #[inline(always)]
    fn from(variant: EF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EF` reader - Enable function"]
pub struct EF_R(crate::FieldReader<bool, EF_A>);
impl EF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EF_A {
        match self.bits {
            false => EF_A::DISABLED,
            true => EF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EF_A::ENABLED
    }
}
impl core::ops::Deref for EF_R {
    type Target = crate::FieldReader<bool, EF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EF` writer - Enable function"]
pub struct EF_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB device disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EF_A::DISABLED)
    }
    #[doc = "USB device enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EF_A::ENABLED)
    }
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
impl R {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&self) -> ADD3_R {
        ADD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&self) -> ADD4_R {
        ADD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&self) -> ADD5_R {
        ADD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&self) -> ADD6_R {
        ADD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&mut self) -> ADD1_W {
        ADD1_W { w: self }
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W { w: self }
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&mut self) -> ADD3_W {
        ADD3_W { w: self }
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&mut self) -> ADD4_W {
        ADD4_W { w: self }
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&mut self) -> ADD5_W {
        ADD5_W { w: self }
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&mut self) -> ADD6_W {
        ADD6_W { w: self }
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W {
        EF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](index.html) module"]
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daddr::R](R) reader structure"]
impl crate::Readable for DADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr::W](W) writer structure"]
impl crate::Writable for DADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
