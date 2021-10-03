#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Repetition register update OK clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPOKCF_AW {
    #[doc = "1: Clear REPOK flag"]
    CLEAR = 1,
}
impl From<REPOKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: REPOKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag"]
pub struct REPOKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> REPOKCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPOKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear REPOK flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPOKCF_AW::CLEAR)
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
#[doc = "Update event clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UECF_AW {
    #[doc = "1: Clear update event flag"]
    CLEAR = 1,
}
impl From<UECF_AW> for bool {
    #[inline(always)]
    fn from(variant: UECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UECF` writer - Update event clear flag"]
pub struct UECF_W<'a> {
    w: &'a mut W,
}
impl<'a> UECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear update event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UECF_AW::CLEAR)
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
#[doc = "Direction change to down Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNCF_AW {
    #[doc = "1: Direction change to down Clear Flag"]
    CLEAR = 1,
}
impl From<DOWNCF_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWNCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub struct DOWNCF_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWNCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DOWNCF_AW::CLEAR)
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
#[doc = "Direction change to UP Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPCF_AW {
    #[doc = "1: Direction change to up Clear Flag"]
    CLEAR = 1,
}
impl From<UPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: UPCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub struct UPCF_W<'a> {
    w: &'a mut W,
}
impl<'a> UPCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Direction change to up Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPCF_AW::CLEAR)
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
#[doc = "Autoreload register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROKCF_AW {
    #[doc = "1: Autoreload register update OK Clear Flag"]
    CLEAR = 1,
}
impl From<ARROKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARROKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub struct ARROKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARROKCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARROKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARROKCF_AW::CLEAR)
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
#[doc = "Compare register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOKCF_AW {
    #[doc = "1: Compare register update OK Clear Flag"]
    CLEAR = 1,
}
impl From<CMPOKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPOKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub struct CMPOKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOKCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPOKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPOKCF_AW::CLEAR)
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
#[doc = "External trigger valid edge Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIGCF_AW {
    #[doc = "1: External trigger valid edge Clear Flag"]
    CLEAR = 1,
}
impl From<EXTTRIGCF_AW> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub struct EXTTRIGCF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIGCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTTRIGCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EXTTRIGCF_AW::CLEAR)
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
#[doc = "Autoreload match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRMCF_AW {
    #[doc = "1: Autoreload match Clear Flag"]
    CLEAR = 1,
}
impl From<ARRMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARRMCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub struct ARRMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRMCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARRMCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARRMCF_AW::CLEAR)
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
#[doc = "compare match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMCF_AW {
    #[doc = "1: Compare match Clear Flag"]
    CLEAR = 1,
}
impl From<CMPMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPMCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub struct CMPMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPMCF_AW::CLEAR)
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
    #[doc = "Bit 8 - Repetition register update OK clear flag"]
    #[inline(always)]
    pub fn repokcf(&mut self) -> REPOKCF_W {
        REPOKCF_W { w: self }
    }
    #[doc = "Bit 7 - Update event clear flag"]
    #[inline(always)]
    pub fn uecf(&mut self) -> UECF_W {
        UECF_W { w: self }
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W {
        DOWNCF_W { w: self }
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W {
        UPCF_W { w: self }
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W {
        ARROKCF_W { w: self }
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W {
        CMPOKCF_W { w: self }
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W {
        EXTTRIGCF_W { w: self }
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W {
        ARRMCF_W { w: self }
    }
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W {
        CMPMCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
