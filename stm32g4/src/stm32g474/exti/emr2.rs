#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event mask on external/internal line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM32_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<EM32_A> for bool {
    #[inline(always)]
    fn from(variant: EM32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM32` reader - Event mask on external/internal line 32"]
pub struct EM32_R(crate::FieldReader<bool, EM32_A>);
impl EM32_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM32_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM32_A {
        match self.bits {
            false => EM32_A::MASKED,
            true => EM32_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == EM32_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == EM32_A::UNMASKED
    }
}
impl core::ops::Deref for EM32_R {
    type Target = crate::FieldReader<bool, EM32_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM32` writer - Event mask on external/internal line 32"]
pub struct EM32_W<'a> {
    w: &'a mut W,
}
impl<'a> EM32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM32_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM32_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM32_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 33"]
pub type EM33_A = EM32_A;
#[doc = "Field `EM33` reader - Event mask on external/internal line 33"]
pub type EM33_R = EM32_R;
#[doc = "Field `EM33` writer - Event mask on external/internal line 33"]
pub struct EM33_W<'a> {
    w: &'a mut W,
}
impl<'a> EM33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM33_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM33_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM33_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 34"]
pub type EM34_A = EM32_A;
#[doc = "Field `EM34` reader - Event mask on external/internal line 34"]
pub type EM34_R = EM32_R;
#[doc = "Field `EM34` writer - Event mask on external/internal line 34"]
pub struct EM34_W<'a> {
    w: &'a mut W,
}
impl<'a> EM34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM34_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM34_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 35"]
pub type EM35_A = EM32_A;
#[doc = "Field `EM35` reader - Event mask on external/internal line 35"]
pub type EM35_R = EM32_R;
#[doc = "Field `EM35` writer - Event mask on external/internal line 35"]
pub struct EM35_W<'a> {
    w: &'a mut W,
}
impl<'a> EM35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM35_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM35_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM35_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 36"]
pub type EM36_A = EM32_A;
#[doc = "Field `EM36` reader - Event mask on external/internal line 36"]
pub type EM36_R = EM32_R;
#[doc = "Field `EM36` writer - Event mask on external/internal line 36"]
pub struct EM36_W<'a> {
    w: &'a mut W,
}
impl<'a> EM36_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM36_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM36_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM36_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 37"]
pub type EM37_A = EM32_A;
#[doc = "Field `EM37` reader - Event mask on external/internal line 37"]
pub type EM37_R = EM32_R;
#[doc = "Field `EM37` writer - Event mask on external/internal line 37"]
pub struct EM37_W<'a> {
    w: &'a mut W,
}
impl<'a> EM37_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM37_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM37_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM37_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 40"]
pub type EM40_A = EM32_A;
#[doc = "Field `EM40` reader - Event mask on external/internal line 40"]
pub type EM40_R = EM32_R;
#[doc = "Field `EM40` writer - Event mask on external/internal line 40"]
pub struct EM40_W<'a> {
    w: &'a mut W,
}
impl<'a> EM40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM40_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM40_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 41"]
pub type EM41_A = EM32_A;
#[doc = "Field `EM41` reader - Event mask on external/internal line 41"]
pub type EM41_R = EM32_R;
#[doc = "Field `EM41` writer - Event mask on external/internal line 41"]
pub struct EM41_W<'a> {
    w: &'a mut W,
}
impl<'a> EM41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM41_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM41_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 42"]
pub type EM42_A = EM32_A;
#[doc = "Field `EM42` reader - Event mask on external/internal line 42"]
pub type EM42_R = EM32_R;
#[doc = "Field `EM42` writer - Event mask on external/internal line 42"]
pub struct EM42_W<'a> {
    w: &'a mut W,
}
impl<'a> EM42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM42_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM42_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM42_A::UNMASKED)
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
#[doc = "Event mask on external/internal line 43"]
pub type EM43_A = EM32_A;
#[doc = "Field `EM43` reader - Event mask on external/internal line 43"]
pub type EM43_R = EM32_R;
#[doc = "Field `EM43` writer - Event mask on external/internal line 43"]
pub struct EM43_W<'a> {
    w: &'a mut W,
}
impl<'a> EM43_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM43_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM43_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM43_A::UNMASKED)
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
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event mask on external/internal line 41"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Event mask on external/internal line 42"]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Event mask on external/internal line 43"]
    #[inline(always)]
    pub fn em43(&self) -> EM43_R {
        EM43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn em32(&mut self) -> EM32_W {
        EM32_W { w: self }
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn em33(&mut self) -> EM33_W {
        EM33_W { w: self }
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W {
        EM34_W { w: self }
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn em35(&mut self) -> EM35_W {
        EM35_W { w: self }
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn em36(&mut self) -> EM36_W {
        EM36_W { w: self }
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn em37(&mut self) -> EM37_W {
        EM37_W { w: self }
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W {
        EM40_W { w: self }
    }
    #[doc = "Bit 9 - Event mask on external/internal line 41"]
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W {
        EM41_W { w: self }
    }
    #[doc = "Bit 10 - Event mask on external/internal line 42"]
    #[inline(always)]
    pub fn em42(&mut self) -> EM42_W {
        EM42_W { w: self }
    }
    #[doc = "Bit 11 - Event mask on external/internal line 43"]
    #[inline(always)]
    pub fn em43(&mut self) -> EM43_W {
        EM43_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
