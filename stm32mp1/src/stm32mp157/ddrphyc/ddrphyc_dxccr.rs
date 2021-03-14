#[doc = "Reader of register DDRPHYC_DXCCR"]
pub type R = crate::R<u32, super::DDRPHYC_DXCCR>;
#[doc = "Writer for register DDRPHYC_DXCCR"]
pub type W = crate::W<u32, super::DDRPHYC_DXCCR>;
#[doc = "Register DDRPHYC_DXCCR `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::DDRPHYC_DXCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Reader of field `DXODT`"]
pub type DXODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXODT`"]
pub struct DXODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DXODT_W<'a> {
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
#[doc = "Reader of field `DXIOM`"]
pub type DXIOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXIOM`"]
pub struct DXIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DXIOM_W<'a> {
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
#[doc = "Reader of field `DXPDD`"]
pub type DXPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXPDD`"]
pub struct DXPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDD_W<'a> {
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
#[doc = "Reader of field `DXPDR`"]
pub type DXPDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXPDR`"]
pub struct DXPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDR_W<'a> {
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
#[doc = "Reader of field `DQSRES`"]
pub type DQSRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSRES`"]
pub struct DQSRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DQSNRES`"]
pub type DQSNRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSNRES`"]
pub struct DQSNRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DQSNRST`"]
pub type DQSNRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSNRST`"]
pub struct DQSNRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RVSEL`"]
pub type RVSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RVSEL`"]
pub struct RVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RVSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AWDT`"]
pub type AWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AWDT`"]
pub struct AWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&self) -> DXODT_R {
        DXODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&self) -> DQSRES_R {
        DQSRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&self) -> DQSNRES_R {
        DQSNRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&self) -> DQSNRST_R {
        DQSNRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&self) -> RVSEL_R {
        RVSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&self) -> AWDT_R {
        AWDT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&mut self) -> DXODT_W {
        DXODT_W { w: self }
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W {
        DXIOM_W { w: self }
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W {
        DXPDD_W { w: self }
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W {
        DXPDR_W { w: self }
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&mut self) -> DQSRES_W {
        DQSRES_W { w: self }
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&mut self) -> DQSNRES_W {
        DQSNRES_W { w: self }
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&mut self) -> DQSNRST_W {
        DQSNRST_W { w: self }
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&mut self) -> RVSEL_W {
        RVSEL_W { w: self }
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&mut self) -> AWDT_W {
        AWDT_W { w: self }
    }
}
