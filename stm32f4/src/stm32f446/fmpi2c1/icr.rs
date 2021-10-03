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
#[doc = "Address Matched flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRCF_AW {
    #[doc = "1: Clears the ADDR flag in ISR register"]
    CLEAR = 1,
}
impl From<ADDRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRCF` writer - Address Matched flag clear"]
pub struct ADDRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ADDR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRCF_AW::CLEAR)
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
#[doc = "Not Acknowledge flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKCF_AW {
    #[doc = "1: Clears the NACK flag in ISR register"]
    CLEAR = 1,
}
impl From<NACKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear"]
pub struct NACKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the NACK flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NACKCF_AW::CLEAR)
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
#[doc = "Stop detection flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPCF_AW {
    #[doc = "1: Clears the STOP flag in ISR register"]
    CLEAR = 1,
}
impl From<STOPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCF` writer - Stop detection flag clear"]
pub struct STOPCF_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the STOP flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPCF_AW::CLEAR)
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
#[doc = "Bus error flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERRCF_AW {
    #[doc = "1: Clears the BERR flag in ISR register"]
    CLEAR = 1,
}
impl From<BERRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: BERRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERRCF` writer - Bus error flag clear"]
pub struct BERRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> BERRCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BERRCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the BERR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRCF_AW::CLEAR)
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
#[doc = "Arbitration lost flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLOCF_AW {
    #[doc = "1: Clears the ARLO flag in ISR register"]
    CLEAR = 1,
}
impl From<ARLOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOCF` writer - Arbitration lost flag clear"]
pub struct ARLOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLOCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARLOCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ARLO flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOCF_AW::CLEAR)
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
#[doc = "Overrun/Underrun flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRCF_AW {
    #[doc = "1: Clears the OVR flag in ISR register"]
    CLEAR = 1,
}
impl From<OVRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear"]
pub struct OVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the OVR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRCF_AW::CLEAR)
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
#[doc = "PEC Error flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECCF_AW {
    #[doc = "1: Clears the PEC flag in ISR register"]
    CLEAR = 1,
}
impl From<PECCF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECCF` writer - PEC Error flag clear"]
pub struct PECCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PECCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the PEC flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECCF_AW::CLEAR)
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
#[doc = "Timeout detection flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTCF_AW {
    #[doc = "1: Clears the TIMOUT flag in ISR register"]
    CLEAR = 1,
}
impl From<TIMOUTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear"]
pub struct TIMOUTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUTCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOUTCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the TIMOUT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMOUTCF_AW::CLEAR)
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
#[doc = "Alert flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTCF_AW {
    #[doc = "1: Clears the ALERT flag in ISR register"]
    CLEAR = 1,
}
impl From<ALERTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTCF` writer - Alert flag clear"]
pub struct ALERTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERTCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ALERT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALERTCF_AW::CLEAR)
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
impl W {
    #[doc = "Bit 3 - Address Matched flag clear"]
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W {
        ADDRCF_W { w: self }
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear"]
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W {
        NACKCF_W { w: self }
    }
    #[doc = "Bit 5 - Stop detection flag clear"]
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W {
        STOPCF_W { w: self }
    }
    #[doc = "Bit 8 - Bus error flag clear"]
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W {
        BERRCF_W { w: self }
    }
    #[doc = "Bit 9 - Arbitration lost flag clear"]
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W {
        ARLOCF_W { w: self }
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W {
        OVRCF_W { w: self }
    }
    #[doc = "Bit 11 - PEC Error flag clear"]
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W {
        PECCF_W { w: self }
    }
    #[doc = "Bit 12 - Timeout detection flag clear"]
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W {
        TIMOUTCF_W { w: self }
    }
    #[doc = "Bit 13 - Alert flag clear"]
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W {
        ALERTCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
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
