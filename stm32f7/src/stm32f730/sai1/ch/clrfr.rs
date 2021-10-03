#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear late frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLFSDET_AW {
    #[doc = "1: Clears the LFSDET flag"]
    CLEAR = 1,
}
impl From<CLFSDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag"]
pub struct CLFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLFSDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLFSDET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDET_AW::CLEAR)
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
#[doc = "Clear anticipated frame synchronization detection flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAFSDET_AW {
    #[doc = "1: Clears the AFSDET flag"]
    CLEAR = 1,
}
impl From<CAFSDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag."]
pub struct CAFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAFSDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAFSDET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDET_AW::CLEAR)
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
#[doc = "Clear codec not ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCNRDY_AW {
    #[doc = "1: Clears the CNRDY flag"]
    CLEAR = 1,
}
impl From<CCNRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCNRDY` writer - Clear codec not ready flag"]
pub struct CCNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCNRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCNRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDY_AW::CLEAR)
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
#[doc = "Clear wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWCKCFG_AW {
    #[doc = "1: Clears the WCKCFG flag"]
    CLEAR = 1,
}
impl From<CWCKCFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag"]
pub struct CWCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CWCKCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWCKCFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFG_AW::CLEAR)
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
#[doc = "Mute detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMUTEDET_AW {
    #[doc = "1: Clears the MUTEDET flag"]
    CLEAR = 1,
}
impl From<CMUTEDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUTEDET` writer - Mute detection flag"]
pub struct CMUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUTEDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMUTEDET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDET_AW::CLEAR)
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
#[doc = "Clear overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVRUDR_AW {
    #[doc = "1: Clears the OVRUDR flag"]
    CLEAR = 1,
}
impl From<COVRUDR_AW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun"]
pub struct COVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> COVRUDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COVRUDR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDR_AW::CLEAR)
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
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W {
        CLFSDET_W { w: self }
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag."]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W {
        CAFSDET_W { w: self }
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W {
        CCNRDY_W { w: self }
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W {
        CWCKCFG_W { w: self }
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W {
        CMUTEDET_W { w: self }
    }
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W {
        COVRUDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AClear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
