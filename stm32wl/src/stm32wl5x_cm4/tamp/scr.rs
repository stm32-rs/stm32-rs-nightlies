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
#[doc = "CTAMP1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTAMP1F_AW {
    #[doc = "1: Clear tamper flag"]
    CLEAR = 1,
}
impl From<CTAMP1F_AW> for bool {
    #[inline(always)]
    fn from(variant: CTAMP1F_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub struct CTAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTAMP1F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTAMP1F_AW::CLEAR)
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
#[doc = "CTAMP2F"]
pub type CTAMP2F_AW = CTAMP1F_AW;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub struct CTAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTAMP2F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTAMP2F_AW::CLEAR)
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
#[doc = "CTAMP3F"]
pub type CTAMP3F_AW = CTAMP1F_AW;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub struct CTAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP3F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTAMP3F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTAMP3F_AW::CLEAR)
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
#[doc = "CITAMP3F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CITAMP3F_AW {
    #[doc = "1: Clear tamper flag"]
    CLEAR = 1,
}
impl From<CITAMP3F_AW> for bool {
    #[inline(always)]
    fn from(variant: CITAMP3F_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub struct CITAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP3F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CITAMP3F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITAMP3F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "CITAMP5F"]
pub type CITAMP5F_AW = CITAMP3F_AW;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub struct CITAMP5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP5F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CITAMP5F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITAMP5F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "CITAMP6F"]
pub type CITAMP6F_AW = CITAMP3F_AW;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub struct CITAMP6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP6F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CITAMP6F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITAMP6F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "CITAMP8F"]
pub type CITAMP8F_AW = CITAMP3F_AW;
#[doc = "Field `CITAMP8F` writer - CITAMP8F"]
pub struct CITAMP8F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP8F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CITAMP8F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear tamper flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITAMP8F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W {
        CTAMP1F_W { w: self }
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W {
        CTAMP2F_W { w: self }
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W {
        CTAMP3F_W { w: self }
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W {
        CITAMP3F_W { w: self }
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W {
        CITAMP5F_W { w: self }
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W {
        CITAMP6F_W { w: self }
    }
    #[doc = "Bit 23 - CITAMP8F"]
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W {
        CITAMP8F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
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
