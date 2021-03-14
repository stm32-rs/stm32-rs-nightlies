#[doc = "Reader of register DSI_WPCR1"]
pub type R = crate::R<u32, super::DSI_WPCR1>;
#[doc = "Writer for register DSI_WPCR1"]
pub type W = crate::W<u32, super::DSI_WPCR1>;
#[doc = "Register DSI_WPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SKEWCL`"]
pub type SKEWCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SKEWCL`"]
pub struct SKEWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SKEWDL`"]
pub type SKEWDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SKEWDL`"]
pub struct SKEWDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `LPTXSRCL`"]
pub type LPTXSRCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPTXSRCL`"]
pub struct LPTXSRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTXSRCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPTXSRDL`"]
pub type LPTXSRDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPTXSRDL`"]
pub struct LPTXSRDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTXSRDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SDDCCL`"]
pub type SDDCCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDDCCL`"]
pub struct SDDCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDCCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SDDCDL`"]
pub type SDDCDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDDCDL`"]
pub struct SDDCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDCDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRUCL`"]
pub type HSTXSRUCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTXSRUCL`"]
pub struct HSTXSRUCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRUCL_W<'a> {
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
#[doc = "Reader of field `HSTXSRDCL`"]
pub type HSTXSRDCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTXSRDCL`"]
pub struct HSTXSRDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRDCL_W<'a> {
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
#[doc = "Reader of field `HSTXSRUDL`"]
pub type HSTXSRUDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTXSRUDL`"]
pub struct HSTXSRUDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRUDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRDDL`"]
pub type HSTXSRDDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTXSRDDL`"]
pub struct HSTXSRDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRDDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&self) -> SKEWCL_R {
        SKEWCL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&self) -> SKEWDL_R {
        SKEWDL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&self) -> LPTXSRCL_R {
        LPTXSRCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&self) -> LPTXSRDL_R {
        LPTXSRDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&self) -> SDDCCL_R {
        SDDCCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&self) -> SDDCDL_R {
        SDDCDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&self) -> HSTXSRUCL_R {
        HSTXSRUCL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&self) -> HSTXSRDCL_R {
        HSTXSRDCL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&self) -> HSTXSRUDL_R {
        HSTXSRUDL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&self) -> HSTXSRDDL_R {
        HSTXSRDDL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&mut self) -> SKEWCL_W {
        SKEWCL_W { w: self }
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&mut self) -> SKEWDL_W {
        SKEWDL_W { w: self }
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&mut self) -> LPTXSRCL_W {
        LPTXSRCL_W { w: self }
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&mut self) -> LPTXSRDL_W {
        LPTXSRDL_W { w: self }
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&mut self) -> SDDCCL_W {
        SDDCCL_W { w: self }
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&mut self) -> SDDCDL_W {
        SDDCDL_W { w: self }
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&mut self) -> HSTXSRUCL_W {
        HSTXSRUCL_W { w: self }
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&mut self) -> HSTXSRDCL_W {
        HSTXSRDCL_W { w: self }
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&mut self) -> HSTXSRUDL_W {
        HSTXSRUDL_W { w: self }
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&mut self) -> HSTXSRDDL_W {
        HSTXSRDDL_W { w: self }
    }
}
