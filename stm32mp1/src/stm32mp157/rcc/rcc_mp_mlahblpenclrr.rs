#[doc = "Reader of register RCC_MP_MLAHBLPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_MLAHBLPENCLRR>;
#[doc = "Writer for register RCC_MP_MLAHBLPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_MLAHBLPENCLRR>;
#[doc = "Register RCC_MP_MLAHBLPENCLRR `reset()`'s with value 0x17"]
impl crate::ResetValue for super::RCC_MP_MLAHBLPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17
    }
}
#[doc = "Reader of field `SRAM1LPEN`"]
pub type SRAM1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1LPEN`"]
pub struct SRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1LPEN_W<'a> {
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
#[doc = "Reader of field `SRAM2LPEN`"]
pub type SRAM2LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2LPEN`"]
pub struct SRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2LPEN_W<'a> {
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
#[doc = "Reader of field `SRAM34LPEN`"]
pub type SRAM34LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM34LPEN`"]
pub struct SRAM34LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM34LPEN_W<'a> {
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
#[doc = "Reader of field `RETRAMLPEN`"]
pub type RETRAMLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRAMLPEN`"]
pub struct RETRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAMLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&self) -> SRAM34LPEN_R {
        SRAM34LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W {
        SRAM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W {
        SRAM2LPEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&mut self) -> SRAM34LPEN_W {
        SRAM34LPEN_W { w: self }
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W {
        RETRAMLPEN_W { w: self }
    }
}
