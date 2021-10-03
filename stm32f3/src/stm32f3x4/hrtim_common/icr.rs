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
#[doc = "Burst mode period flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPERC_AW {
    #[doc = "1: Clears BMPER flag"]
    CLEAR = 1,
}
impl From<BMPERC_AW> for bool {
    #[inline(always)]
    fn from(variant: BMPERC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPERC` writer - Burst mode period flag Clear"]
pub struct BMPERC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPERC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPERC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears BMPER flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BMPERC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "DLL Ready Interrupt flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLRDYC_AW {
    #[doc = "1: Clears DLL ready flag"]
    CLEAR = 1,
}
impl From<DLLRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDYC` writer - DLL Ready Interrupt flag Clear"]
pub struct DLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLLRDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears DLL ready flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLLRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "System Fault Interrupt Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFLTC_AW {
    #[doc = "1: Clears SYSFLT flag"]
    CLEAR = 1,
}
impl From<SYSFLTC_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSFLTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSFLTC` writer - System Fault Interrupt Flag Clear"]
pub struct SYSFLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFLTC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears SYSFLT flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SYSFLTC_AW::CLEAR)
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
#[doc = "Fault 5 Interrupt Flag Clear"]
pub type FLT5C_AW = FLT1C_AW;
#[doc = "Field `FLT5C` writer - Fault 5 Interrupt Flag Clear"]
pub struct FLT5C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears FLTx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT5C_AW::CLEAR)
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
#[doc = "Fault 4 Interrupt Flag Clear"]
pub type FLT4C_AW = FLT1C_AW;
#[doc = "Field `FLT4C` writer - Fault 4 Interrupt Flag Clear"]
pub struct FLT4C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears FLTx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT4C_AW::CLEAR)
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
#[doc = "Fault 3 Interrupt Flag Clear"]
pub type FLT3C_AW = FLT1C_AW;
#[doc = "Field `FLT3C` writer - Fault 3 Interrupt Flag Clear"]
pub struct FLT3C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears FLTx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT3C_AW::CLEAR)
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
#[doc = "Fault 2 Interrupt Flag Clear"]
pub type FLT2C_AW = FLT1C_AW;
#[doc = "Field `FLT2C` writer - Fault 2 Interrupt Flag Clear"]
pub struct FLT2C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears FLTx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT2C_AW::CLEAR)
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
#[doc = "Fault 1 Interrupt Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1C_AW {
    #[doc = "1: Clears FLTx flag"]
    CLEAR = 1,
}
impl From<FLT1C_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT1C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1C` writer - Fault 1 Interrupt Flag Clear"]
pub struct FLT1C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears FLTx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT1C_AW::CLEAR)
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
    #[doc = "Bit 17 - Burst mode period flag Clear"]
    #[inline(always)]
    pub fn bmperc(&mut self) -> BMPERC_W {
        BMPERC_W { w: self }
    }
    #[doc = "Bit 16 - DLL Ready Interrupt flag Clear"]
    #[inline(always)]
    pub fn dllrdyc(&mut self) -> DLLRDYC_W {
        DLLRDYC_W { w: self }
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag Clear"]
    #[inline(always)]
    pub fn sysfltc(&mut self) -> SYSFLTC_W {
        SYSFLTC_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt5c(&mut self) -> FLT5C_W {
        FLT5C_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt4c(&mut self) -> FLT4C_W {
        FLT4C_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt3c(&mut self) -> FLT3C_W {
        FLT3C_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt2c(&mut self) -> FLT2C_W {
        FLT2C_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt1c(&mut self) -> FLT1C_W {
        FLT1C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
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
