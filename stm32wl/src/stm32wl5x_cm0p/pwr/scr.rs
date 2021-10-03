#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC2HF` writer - lear CPU2 Hold interrupt flag"]
pub struct CC2HF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2HF_W<'a> {
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
#[doc = "Clear wakeup Radio BUSY flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWRFBUSYF_AW {
    #[doc = "1: Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    CLEAR = 1,
}
impl From<CWRFBUSYF_AW> for bool {
    #[inline(always)]
    fn from(variant: CWRFBUSYF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWRFBUSYF` writer - Clear wakeup Radio BUSY flag"]
pub struct CWRFBUSYF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWRFBUSYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWRFBUSYF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWRFBUSYF_AW::CLEAR)
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
#[doc = "Clear wakeup PVD interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWPVDF_AW {
    #[doc = "1: Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    CLEAR = 1,
}
impl From<CWPVDF_AW> for bool {
    #[inline(always)]
    fn from(variant: CWPVDF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWPVDF` writer - Clear wakeup PVD interrupt flag"]
pub struct CWPVDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWPVDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWPVDF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWPVDF_AW::CLEAR)
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
#[doc = "Clear wakeup flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWUF3_AW {
    #[doc = "1: Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    CLEAR = 1,
}
impl From<CWUF3_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub struct CWUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWUF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF3_AW::CLEAR)
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
#[doc = "Clear wakeup flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWUF2_AW {
    #[doc = "1: Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    CLEAR = 1,
}
impl From<CWUF2_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub struct CWUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWUF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF2_AW::CLEAR)
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
#[doc = "Clear wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWUF1_AW {
    #[doc = "1: Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    CLEAR = 1,
}
impl From<CWUF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub struct CWUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWUF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF1_AW::CLEAR)
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
    #[doc = "Bit 14 - lear CPU2 Hold interrupt flag"]
    #[inline(always)]
    pub fn cc2hf(&mut self) -> CC2HF_W {
        CC2HF_W { w: self }
    }
    #[doc = "Bit 11 - Clear wakeup Radio BUSY flag"]
    #[inline(always)]
    pub fn cwrfbusyf(&mut self) -> CWRFBUSYF_W {
        CWRFBUSYF_W { w: self }
    }
    #[doc = "Bit 8 - Clear wakeup PVD interrupt flag"]
    #[inline(always)]
    pub fn cwpvdf(&mut self) -> CWPVDF_W {
        CWPVDF_W { w: self }
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W {
        CWUF3_W { w: self }
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W {
        CWUF2_W { w: self }
    }
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W {
        CWUF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
