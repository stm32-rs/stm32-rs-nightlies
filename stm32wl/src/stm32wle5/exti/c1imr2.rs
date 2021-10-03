#[doc = "Register `C1IMR2` reader"]
pub struct R(crate::R<C1IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IMR2` writer"]
pub struct W(crate::W<C1IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IMR2_SPEC>;
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
impl From<crate::W<C1IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPUm Wakeup with interrupt Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM34_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<IM34_A> for bool {
    #[inline(always)]
    fn from(variant: IM34_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM34` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM34_R(crate::FieldReader<bool, IM34_A>);
impl IM34_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM34_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM34_A {
        match self.bits {
            false => IM34_A::MASKED,
            true => IM34_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == IM34_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == IM34_A::UNMASKED
    }
}
impl core::ops::Deref for IM34_R {
    type Target = crate::FieldReader<bool, IM34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM34` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM34_W<'a> {
    w: &'a mut W,
}
impl<'a> IM34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM34_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM34_A::UNMASKED)
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
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM38_A = IM34_A;
#[doc = "Field `IM38` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM38_R = IM34_R;
#[doc = "Field `IM38` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM38_W<'a> {
    w: &'a mut W,
}
impl<'a> IM38_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM38_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM38_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM38_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM42_A = IM34_A;
#[doc = "Field `IM42` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM42_R = IM34_R;
#[doc = "Field `IM42` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM42_W<'a> {
    w: &'a mut W,
}
impl<'a> IM42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM42_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM42_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM42_A::UNMASKED)
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
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM43_A = IM34_A;
#[doc = "Field `IM43` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM43_R = IM34_R;
#[doc = "Field `IM43` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM43_W<'a> {
    w: &'a mut W,
}
impl<'a> IM43_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM43_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM43_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM43_A::UNMASKED)
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
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM44_A = IM34_A;
#[doc = "Field `IM44` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM44_R = IM34_R;
#[doc = "Field `IM44` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM44_W<'a> {
    w: &'a mut W,
}
impl<'a> IM44_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM44_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM44_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM44_A::UNMASKED)
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
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM45_A = IM34_A;
#[doc = "Field `IM45` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM45_R = IM34_R;
#[doc = "Field `IM45` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM45_W<'a> {
    w: &'a mut W,
}
impl<'a> IM45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM45_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM45_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM45_A::UNMASKED)
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
#[doc = "CPUm Wakeup with interrupt Mask on Event input"]
pub type IM46_A = IM34_A;
#[doc = "Field `IM46` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM46_R = IM34_R;
#[doc = "Field `IM46` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM46_W<'a> {
    w: &'a mut W,
}
impl<'a> IM46_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM46_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM46_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM46_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W {
        IM34_W { w: self }
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W {
        IM38_W { w: self }
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W {
        IM42_W { w: self }
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W {
        IM43_W { w: self }
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W {
        IM44_W { w: self }
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W {
        IM45_W { w: self }
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W {
        IM46_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr2](index.html) module"]
pub struct C1IMR2_SPEC;
impl crate::RegisterSpec for C1IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1imr2::R](R) reader structure"]
impl crate::Readable for C1IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1imr2::W](W) writer structure"]
impl crate::Writable for C1IMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1IMR2 to value 0"]
impl crate::Resettable for C1IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
