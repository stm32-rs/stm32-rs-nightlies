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
#[doc = "Timer E counter software reset"]
pub type TERST_A = MRST_A;
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub type TERST_R = MRST_R;
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub struct TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> TERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TERST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Timer D counter software reset"]
pub type TDRST_A = MRST_A;
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub type TDRST_R = MRST_R;
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub struct TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TDRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Timer C counter software reset"]
pub type TCRST_A = MRST_A;
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub type TCRST_R = MRST_R;
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub struct TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Timer B counter software reset"]
pub type TBRST_A = MRST_A;
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub type TBRST_R = MRST_R;
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub struct TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TBRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Timer A counter software reset"]
pub type TARST_A = MRST_A;
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub type TARST_R = MRST_R;
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub struct TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> TARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TARST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Master Counter software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRST_A {
    #[doc = "1: Reset timer"]
    RESET = 1,
}
impl From<MRST_A> for bool {
    #[inline(always)]
    fn from(variant: MRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub struct MRST_R(crate::FieldReader<bool, MRST_A>);
impl MRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MRST_A> {
        match self.bits {
            true => Some(MRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == MRST_A::RESET
    }
}
impl core::ops::Deref for MRST_R {
    type Target = crate::FieldReader<bool, MRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub struct MRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Timer E Software Update"]
pub type TESWU_A = MSWU_A;
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub type TESWU_R = MSWU_R;
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub struct TESWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TESWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TESWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(TESWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer D Software Update"]
pub type TDSWU_A = MSWU_A;
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub type TDSWU_R = MSWU_R;
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub struct TDSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TDSWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDSWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(TDSWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Timer C Software Update"]
pub type TCSWU_A = MSWU_A;
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub type TCSWU_R = MSWU_R;
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub struct TCSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCSWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(TCSWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Timer B Software Update"]
pub type TBSWU_A = MSWU_A;
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub type TBSWU_R = MSWU_R;
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub struct TBSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBSWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(TBSWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer A Software update"]
pub type TASWU_A = MSWU_A;
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub type TASWU_R = MSWU_R;
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub struct TASWU_W<'a> {
    w: &'a mut W,
}
impl<'a> TASWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(TASWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Master Timer Software update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSWU_A {
    #[doc = "1: Force immediate update"]
    UPDATE = 1,
}
impl From<MSWU_A> for bool {
    #[inline(always)]
    fn from(variant: MSWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub struct MSWU_R(crate::FieldReader<bool, MSWU_A>);
impl MSWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSWU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSWU_A> {
        match self.bits {
            true => Some(MSWU_A::UPDATE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == MSWU_A::UPDATE
    }
}
impl core::ops::Deref for MSWU_R {
    type Target = crate::FieldReader<bool, MSWU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub struct MSWU_W<'a> {
    w: &'a mut W,
}
impl<'a> MSWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MSWU_A::UPDATE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&mut self) -> TERST_W {
        TERST_W { w: self }
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&mut self) -> TDRST_W {
        TDRST_W { w: self }
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&mut self) -> TCRST_W {
        TCRST_W { w: self }
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&mut self) -> TBRST_W {
        TBRST_W { w: self }
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&mut self) -> TARST_W {
        TARST_W { w: self }
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&mut self) -> MRST_W {
        MRST_W { w: self }
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&mut self) -> TESWU_W {
        TESWU_W { w: self }
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&mut self) -> TDSWU_W {
        TDSWU_W { w: self }
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&mut self) -> TCSWU_W {
        TCSWU_W { w: self }
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&mut self) -> TBSWU_W {
        TBSWU_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&mut self) -> TASWU_W {
        TASWU_W { w: self }
    }
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&mut self) -> MSWU_W {
        MSWU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
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
