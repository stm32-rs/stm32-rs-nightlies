#[doc = "Writer for register MICR"]
pub type W = crate::W<u32, super::MICR>;
#[doc = "Register MICR `reset()`'s with value 0"]
impl crate::ResetValue for super::MICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master update Interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUPDC_AW {
    #[doc = "1: Clears flag in MISR register"]
    CLEAR = 1,
}
impl From<MUPDC_AW> for bool {
    #[inline(always)]
    fn from(variant: MUPDC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MUPDC`"]
pub struct MUPDC_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUPDC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Sync Input Interrupt flag clear"]
pub type SYNCC_AW = MUPDC_AW;
#[doc = "Write proxy for field `SYNCC`"]
pub struct SYNCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
#[doc = "Repetition Interrupt flag clear"]
pub type MREPC_AW = MUPDC_AW;
#[doc = "Write proxy for field `MREPC`"]
pub struct MREPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
#[doc = "Master Compare 4 Interrupt flag clear"]
pub type MCMP4C_AW = MUPDC_AW;
#[doc = "Write proxy for field `MCMP4C`"]
pub struct MCMP4C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
#[doc = "Master Compare 3 Interrupt flag clear"]
pub type MCMP3C_AW = MUPDC_AW;
#[doc = "Write proxy for field `MCMP3C`"]
pub struct MCMP3C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
#[doc = "Master Compare 2 Interrupt flag clear"]
pub type MCMP2C_AW = MUPDC_AW;
#[doc = "Write proxy for field `MCMP2C`"]
pub struct MCMP2C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
#[doc = "Master Compare 1 Interrupt flag clear"]
pub type MCMP1C_AW = MUPDC_AW;
#[doc = "Write proxy for field `MCMP1C`"]
pub struct MCMP1C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
    #[doc = "Bit 6 - Master update Interrupt flag clear"]
    #[inline(always)]
    pub fn mupdc(&mut self) -> MUPDC_W {
        MUPDC_W { w: self }
    }
    #[doc = "Bit 5 - Sync Input Interrupt flag clear"]
    #[inline(always)]
    pub fn syncc(&mut self) -> SYNCC_W {
        SYNCC_W { w: self }
    }
    #[doc = "Bit 4 - Repetition Interrupt flag clear"]
    #[inline(always)]
    pub fn mrepc(&mut self) -> MREPC_W {
        MREPC_W { w: self }
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp4c(&mut self) -> MCMP4C_W {
        MCMP4C_W { w: self }
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp3c(&mut self) -> MCMP3C_W {
        MCMP3C_W { w: self }
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp2c(&mut self) -> MCMP2C_W {
        MCMP2C_W { w: self }
    }
    #[doc = "Bit 0 - Master Compare 1 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp1c(&mut self) -> MCMP1C_W {
        MCMP1C_W { w: self }
    }
}
