#[doc = "Register `ODISR` reader"]
pub struct R(crate::R<ODISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODISR` writer"]
pub struct W(crate::W<ODISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODISR_SPEC>;
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
impl From<crate::W<ODISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TE2ODIS"]
pub type TE2ODIS_A = TA1ODIS_A;
#[doc = "Field `TE2ODIS` reader - TE2ODIS"]
pub type TE2ODIS_R = TA1ODIS_R;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub struct TE2ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TE2ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE2ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TE2ODIS_A::DISABLE)
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
#[doc = "TE1ODIS"]
pub type TE1ODIS_A = TA1ODIS_A;
#[doc = "Field `TE1ODIS` reader - TE1ODIS"]
pub type TE1ODIS_R = TA1ODIS_R;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub struct TE1ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE1ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TE1ODIS_A::DISABLE)
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
#[doc = "TD2ODIS"]
pub type TD2ODIS_A = TA1ODIS_A;
#[doc = "Field `TD2ODIS` reader - TD2ODIS"]
pub type TD2ODIS_R = TA1ODIS_R;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub struct TD2ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TD2ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TD2ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TD2ODIS_A::DISABLE)
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
#[doc = "TD1ODIS"]
pub type TD1ODIS_A = TA1ODIS_A;
#[doc = "Field `TD1ODIS` reader - TD1ODIS"]
pub type TD1ODIS_R = TA1ODIS_R;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub struct TD1ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TD1ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TD1ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TD1ODIS_A::DISABLE)
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
#[doc = "TC2ODIS"]
pub type TC2ODIS_A = TA1ODIS_A;
#[doc = "Field `TC2ODIS` reader - TC2ODIS"]
pub type TC2ODIS_R = TA1ODIS_R;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub struct TC2ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TC2ODIS_A::DISABLE)
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
#[doc = "TC1ODIS"]
pub type TC1ODIS_A = TA1ODIS_A;
#[doc = "Field `TC1ODIS` reader - TC1ODIS"]
pub type TC1ODIS_R = TA1ODIS_R;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub struct TC1ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TC1ODIS_A::DISABLE)
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
#[doc = "TB2ODIS"]
pub type TB2ODIS_A = TA1ODIS_A;
#[doc = "Field `TB2ODIS` reader - TB2ODIS"]
pub type TB2ODIS_R = TA1ODIS_R;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub struct TB2ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TB2ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB2ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TB2ODIS_A::DISABLE)
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
#[doc = "TB1ODIS"]
pub type TB1ODIS_A = TA1ODIS_A;
#[doc = "Field `TB1ODIS` reader - TB1ODIS"]
pub type TB1ODIS_R = TA1ODIS_R;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub struct TB1ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB1ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TB1ODIS_A::DISABLE)
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
#[doc = "TA2ODIS"]
pub type TA2ODIS_A = TA1ODIS_A;
#[doc = "Field `TA2ODIS` reader - TA2ODIS"]
pub type TA2ODIS_R = TA1ODIS_R;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub struct TA2ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TA2ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TA2ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TA2ODIS_A::DISABLE)
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
#[doc = "TA1ODIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TA1ODIS_A {
    #[doc = "1: Disable output"]
    DISABLE = 1,
}
impl From<TA1ODIS_A> for bool {
    #[inline(always)]
    fn from(variant: TA1ODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1ODIS` reader - TA1ODIS"]
pub struct TA1ODIS_R(crate::FieldReader<bool, TA1ODIS_A>);
impl TA1ODIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA1ODIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TA1ODIS_A> {
        match self.bits {
            true => Some(TA1ODIS_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TA1ODIS_A::DISABLE
    }
}
impl core::ops::Deref for TA1ODIS_R {
    type Target = crate::FieldReader<bool, TA1ODIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub struct TA1ODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1ODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TA1ODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TA1ODIS_A::DISABLE)
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
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&self) -> TE2ODIS_R {
        TE2ODIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&self) -> TE1ODIS_R {
        TE1ODIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&self) -> TD2ODIS_R {
        TD2ODIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&self) -> TD1ODIS_R {
        TD1ODIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&self) -> TC2ODIS_R {
        TC2ODIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&self) -> TC1ODIS_R {
        TC1ODIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&self) -> TB2ODIS_R {
        TB2ODIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&self) -> TB1ODIS_R {
        TB1ODIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&self) -> TA2ODIS_R {
        TA2ODIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&self) -> TA1ODIS_R {
        TA1ODIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&mut self) -> TE2ODIS_W {
        TE2ODIS_W { w: self }
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&mut self) -> TE1ODIS_W {
        TE1ODIS_W { w: self }
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&mut self) -> TD2ODIS_W {
        TD2ODIS_W { w: self }
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&mut self) -> TD1ODIS_W {
        TD1ODIS_W { w: self }
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&mut self) -> TC2ODIS_W {
        TC2ODIS_W { w: self }
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&mut self) -> TC1ODIS_W {
        TC1ODIS_W { w: self }
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&mut self) -> TB2ODIS_W {
        TB2ODIS_W { w: self }
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&mut self) -> TB1ODIS_W {
        TB1ODIS_W { w: self }
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&mut self) -> TA2ODIS_W {
        TA2ODIS_W { w: self }
    }
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&mut self) -> TA1ODIS_W {
        TA1ODIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odisr](index.html) module"]
pub struct ODISR_SPEC;
impl crate::RegisterSpec for ODISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odisr::R](R) reader structure"]
impl crate::Readable for ODISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odisr::W](W) writer structure"]
impl crate::Writable for ODISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODISR to value 0"]
impl crate::Resettable for ODISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
