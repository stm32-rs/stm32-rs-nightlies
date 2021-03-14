#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Write proxy for field `BMPERC`"]
pub struct BMPERC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPERC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPERC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
#[doc = "Write proxy for field `DLLRDYC`"]
pub struct DLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLLRDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
#[doc = "Write proxy for field `SYSFLTC`"]
pub struct SYSFLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFLTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Fault 5 Interrupt Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5C_AW {
    #[doc = "1: Clears FLTx flag"]
    CLEAR = 1,
}
impl From<FLT5C_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT5C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FLT5C`"]
pub struct FLT5C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Fault 4 Interrupt Flag Clear"]
pub type FLT4C_AW = FLT5C_AW;
#[doc = "Write proxy for field `FLT4C`"]
pub struct FLT4C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Fault 3 Interrupt Flag Clear"]
pub type FLT3C_AW = FLT5C_AW;
#[doc = "Write proxy for field `FLT3C`"]
pub struct FLT3C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Fault 2 Interrupt Flag Clear"]
pub type FLT2C_AW = FLT5C_AW;
#[doc = "Write proxy for field `FLT2C`"]
pub struct FLT2C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Fault 1 Interrupt Flag Clear"]
pub type FLT1C_AW = FLT5C_AW;
#[doc = "Write proxy for field `FLT1C`"]
pub struct FLT1C_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
