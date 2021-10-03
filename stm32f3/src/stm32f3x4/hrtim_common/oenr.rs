#[doc = "Register `OENR` writer"]
pub struct W(crate::W<OENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OENR_SPEC>;
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
impl From<crate::W<OENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer E Output 2 Enable"]
pub type TE2OEN_AW = TA1OEN_AW;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub struct TE2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE2OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE2OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TE2OEN_AW::ENABLE)
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
#[doc = "Timer E Output 1 Enable"]
pub type TE1OEN_AW = TA1OEN_AW;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub struct TE1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE1OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TE1OEN_AW::ENABLE)
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
#[doc = "Timer D Output 2 Enable"]
pub type TD2OEN_AW = TA1OEN_AW;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub struct TD2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TD2OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TD2OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TD2OEN_AW::ENABLE)
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
#[doc = "Timer D Output 1 Enable"]
pub type TD1OEN_AW = TA1OEN_AW;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub struct TD1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TD1OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TD1OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TD1OEN_AW::ENABLE)
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
#[doc = "Timer C Output 2 Enable"]
pub type TC2OEN_AW = TA1OEN_AW;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub struct TC2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TC2OEN_AW::ENABLE)
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
#[doc = "Timer C Output 1 Enable"]
pub type TC1OEN_AW = TA1OEN_AW;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub struct TC1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TC1OEN_AW::ENABLE)
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
#[doc = "Timer B Output 2 Enable"]
pub type TB2OEN_AW = TA1OEN_AW;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub struct TB2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TB2OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB2OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TB2OEN_AW::ENABLE)
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
#[doc = "Timer B Output 1 Enable"]
pub type TB1OEN_AW = TA1OEN_AW;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub struct TB1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB1OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TB1OEN_AW::ENABLE)
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
#[doc = "Timer A Output 2 Enable"]
pub type TA2OEN_AW = TA1OEN_AW;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub struct TA2OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TA2OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TA2OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TA2OEN_AW::ENABLE)
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
#[doc = "Timer A Output 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TA1OEN_AW {
    #[doc = "1: Enable output"]
    ENABLE = 1,
}
impl From<TA1OEN_AW> for bool {
    #[inline(always)]
    fn from(variant: TA1OEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub struct TA1OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TA1OEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TA1OEN_AW::ENABLE)
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
impl W {
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&mut self) -> TE2OEN_W {
        TE2OEN_W { w: self }
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&mut self) -> TE1OEN_W {
        TE1OEN_W { w: self }
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&mut self) -> TD2OEN_W {
        TD2OEN_W { w: self }
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&mut self) -> TD1OEN_W {
        TD1OEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&mut self) -> TC2OEN_W {
        TC2OEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&mut self) -> TC1OEN_W {
        TC1OEN_W { w: self }
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&mut self) -> TB2OEN_W {
        TB2OEN_W { w: self }
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&mut self) -> TB1OEN_W {
        TB1OEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&mut self) -> TA2OEN_W {
        TA2OEN_W { w: self }
    }
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&mut self) -> TA1OEN_W {
        TA1OEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oenr](index.html) module"]
pub struct OENR_SPEC;
impl crate::RegisterSpec for OENR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oenr::W](W) writer structure"]
impl crate::Writable for OENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
