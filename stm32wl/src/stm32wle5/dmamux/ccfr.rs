#[doc = "Register `CCFR` writer"]
pub struct W(crate::W<CCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFR_SPEC>;
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
impl From<crate::W<CCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CSOF13"]
pub type CSOF13_AW = CSOF10_AW;
#[doc = "Field `CSOF13` writer - CSOF13"]
pub struct CSOF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF13_AW::CLEAR)
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
#[doc = "CSOF12"]
pub type CSOF12_AW = CSOF10_AW;
#[doc = "Field `CSOF12` writer - CSOF12"]
pub struct CSOF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF12_AW::CLEAR)
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
#[doc = "CSOF11"]
pub type CSOF11_AW = CSOF10_AW;
#[doc = "Field `CSOF11` writer - CSOF11"]
pub struct CSOF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF11_AW::CLEAR)
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
#[doc = "CSOF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOF10_AW {
    #[doc = "1: Clear synchronization flag"]
    CLEAR = 1,
}
impl From<CSOF10_AW> for bool {
    #[inline(always)]
    fn from(variant: CSOF10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOF10` writer - CSOF10"]
pub struct CSOF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF10_AW::CLEAR)
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
#[doc = "CSOF9"]
pub type CSOF9_AW = CSOF0_AW;
#[doc = "Field `CSOF9` writer - CSOF9"]
pub struct CSOF9_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF9_AW::CLEAR)
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
#[doc = "CSOF8"]
pub type CSOF8_AW = CSOF0_AW;
#[doc = "Field `CSOF8` writer - CSOF8"]
pub struct CSOF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF8_AW::CLEAR)
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
#[doc = "CSOF7"]
pub type CSOF7_AW = CSOF0_AW;
#[doc = "Field `CSOF7` writer - CSOF7"]
pub struct CSOF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF7_AW::CLEAR)
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
#[doc = "CSOF6"]
pub type CSOF6_AW = CSOF0_AW;
#[doc = "Field `CSOF6` writer - CSOF6"]
pub struct CSOF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF6_AW::CLEAR)
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
#[doc = "CSOF5"]
pub type CSOF5_AW = CSOF0_AW;
#[doc = "Field `CSOF5` writer - CSOF5"]
pub struct CSOF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF5_AW::CLEAR)
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
#[doc = "CSOF4"]
pub type CSOF4_AW = CSOF0_AW;
#[doc = "Field `CSOF4` writer - CSOF4"]
pub struct CSOF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF4_AW::CLEAR)
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
#[doc = "CSOF3"]
pub type CSOF3_AW = CSOF0_AW;
#[doc = "Field `CSOF3` writer - CSOF3"]
pub struct CSOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF3_AW::CLEAR)
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
#[doc = "CSOF2"]
pub type CSOF2_AW = CSOF0_AW;
#[doc = "Field `CSOF2` writer - CSOF2"]
pub struct CSOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF2_AW::CLEAR)
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
#[doc = "CSOF1"]
pub type CSOF1_AW = CSOF0_AW;
#[doc = "Field `CSOF1` writer - CSOF1"]
pub struct CSOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF1_AW::CLEAR)
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
#[doc = "CSOF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOF0_AW {
    #[doc = "1: Clear synchronization flag"]
    CLEAR = 1,
}
impl From<CSOF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CSOF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOF0` writer - CSOF0"]
pub struct CSOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF0_AW::CLEAR)
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
    #[doc = "Bit 13 - CSOF13"]
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF13_W {
        CSOF13_W { w: self }
    }
    #[doc = "Bit 12 - CSOF12"]
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF12_W {
        CSOF12_W { w: self }
    }
    #[doc = "Bit 11 - CSOF11"]
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W {
        CSOF11_W { w: self }
    }
    #[doc = "Bit 10 - CSOF10"]
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W {
        CSOF10_W { w: self }
    }
    #[doc = "Bit 9 - CSOF9"]
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W {
        CSOF9_W { w: self }
    }
    #[doc = "Bit 8 - CSOF8"]
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W {
        CSOF8_W { w: self }
    }
    #[doc = "Bit 7 - CSOF7"]
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W {
        CSOF7_W { w: self }
    }
    #[doc = "Bit 6 - CSOF6"]
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W {
        CSOF6_W { w: self }
    }
    #[doc = "Bit 5 - CSOF5"]
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W {
        CSOF5_W { w: self }
    }
    #[doc = "Bit 4 - CSOF4"]
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W {
        CSOF4_W { w: self }
    }
    #[doc = "Bit 3 - CSOF3"]
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W {
        CSOF3_W { w: self }
    }
    #[doc = "Bit 2 - CSOF2"]
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W {
        CSOF2_W { w: self }
    }
    #[doc = "Bit 1 - CSOF1"]
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W {
        CSOF1_W { w: self }
    }
    #[doc = "Bit 0 - CSOF0"]
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W {
        CSOF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "request line multiplexer interrupt channel clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfr](index.html) module"]
pub struct CCFR_SPEC;
impl crate::RegisterSpec for CCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ccfr::W](W) writer structure"]
impl crate::Writable for CCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFR to value 0"]
impl crate::Resettable for CCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
