#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRGOEN`"]
pub type TRGOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGOEN`"]
pub struct TRGOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `D3DBGCKEN`"]
pub type D3DBGCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D3DBGCKEN`"]
pub struct D3DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D3DBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `D1DBGCKEN`"]
pub type D1DBGCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D1DBGCKEN`"]
pub struct D1DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D1DBGCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TRACECLKEN`"]
pub type TRACECLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACECLKEN`"]
pub struct TRACECLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DBGSTBY_D1`"]
pub type DBGSTBY_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGSTBY_D1`"]
pub struct DBGSTBY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTBY_D1_W<'a> {
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
#[doc = "Reader of field `DBGSTOP_D1`"]
pub type DBGSTOP_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGSTOP_D1`"]
pub struct DBGSTOP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSTOP_D1_W<'a> {
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
#[doc = "Reader of field `DBGSLEEP_D1`"]
pub type DBGSLEEP_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGSLEEP_D1`"]
pub struct DBGSLEEP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSLEEP_D1_W<'a> {
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
impl R {
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Trace clock enable enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow debug in D1 Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&self) -> DBGSTBY_D1_R {
        DBGSTBY_D1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allow debug in D1 Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&self) -> DBGSTOP_D1_R {
        DBGSTOP_D1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Allow debug in D1 Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&self) -> DBGSLEEP_D1_R {
        DBGSLEEP_D1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W {
        TRGOEN_W { w: self }
    }
    #[doc = "Bit 22 - D3 debug clock enable enable"]
    #[inline(always)]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W {
        D3DBGCKEN_W { w: self }
    }
    #[doc = "Bit 21 - D1 debug clock enable enable"]
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W {
        D1DBGCKEN_W { w: self }
    }
    #[doc = "Bit 20 - Trace clock enable enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W {
        TRACECLKEN_W { w: self }
    }
    #[doc = "Bit 2 - Allow debug in D1 Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&mut self) -> DBGSTBY_D1_W {
        DBGSTBY_D1_W { w: self }
    }
    #[doc = "Bit 1 - Allow debug in D1 Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&mut self) -> DBGSTOP_D1_W {
        DBGSTOP_D1_W { w: self }
    }
    #[doc = "Bit 0 - Allow debug in D1 Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&mut self) -> DBGSLEEP_D1_W {
        DBGSLEEP_D1_W { w: self }
    }
}
