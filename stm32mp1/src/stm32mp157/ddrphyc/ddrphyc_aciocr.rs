#[doc = "Reader of register DDRPHYC_ACIOCR"]
pub type R = crate::R<u32, super::DDRPHYC_ACIOCR>;
#[doc = "Writer for register DDRPHYC_ACIOCR"]
pub type W = crate::W<u32, super::DDRPHYC_ACIOCR>;
#[doc = "Register DDRPHYC_ACIOCR `reset()`'s with value 0x33c0_3812"]
impl crate::ResetValue for super::DDRPHYC_ACIOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33c0_3812
    }
}
#[doc = "Reader of field `ACIOM`"]
pub type ACIOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACIOM`"]
pub struct ACIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIOM_W<'a> {
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
#[doc = "Reader of field `ACOE`"]
pub type ACOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACOE`"]
pub struct ACOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOE_W<'a> {
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
#[doc = "Reader of field `ACODT`"]
pub type ACODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACODT`"]
pub struct ACODT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACODT_W<'a> {
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
#[doc = "Reader of field `ACPDD`"]
pub type ACPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACPDD`"]
pub struct ACPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPDD_W<'a> {
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
#[doc = "Reader of field `ACPDR`"]
pub type ACPDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACPDR`"]
pub struct ACPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPDR_W<'a> {
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
#[doc = "Reader of field `CKODT`"]
pub type CKODT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKODT`"]
pub struct CKODT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKODT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `CKPDD`"]
pub type CKPDD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPDD`"]
pub struct CKPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CKPDR`"]
pub type CKPDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPDR`"]
pub struct CKPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `RANKODT`"]
pub type RANKODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANKODT`"]
pub struct RANKODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKODT_W<'a> {
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
#[doc = "Reader of field `CSPDD`"]
pub type CSPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPDD`"]
pub struct CSPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPDD_W<'a> {
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
#[doc = "Reader of field `RANKPDR`"]
pub type RANKPDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANKPDR`"]
pub struct RANKPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKPDR_W<'a> {
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
#[doc = "Reader of field `RSTODT`"]
pub type RSTODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTODT`"]
pub struct RSTODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTODT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RSTPDD`"]
pub type RSTPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPDD`"]
pub struct RSTPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RSTPDR`"]
pub type RSTPDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPDR`"]
pub struct RSTPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPDR_W<'a> {
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
#[doc = "Reader of field `RSTIOM`"]
pub type RSTIOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIOM`"]
pub struct RSTIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ACSR`"]
pub type ACSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACSR`"]
pub struct ACSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    pub fn aciom(&self) -> ACIOM_R {
        ACIOM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    pub fn acodt(&self) -> ACODT_R {
        ACODT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    pub fn acpdd(&self) -> ACPDD_R {
        ACPDD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    pub fn acpdr(&self) -> ACPDR_R {
        ACPDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    pub fn ckodt(&self) -> CKODT_R {
        CKODT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    pub fn ckpdd(&self) -> CKPDD_R {
        CKPDD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    pub fn ckpdr(&self) -> CKPDR_R {
        CKPDR_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    pub fn rankodt(&self) -> RANKODT_R {
        RANKODT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    pub fn cspdd(&self) -> CSPDD_R {
        CSPDD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    pub fn rankpdr(&self) -> RANKPDR_R {
        RANKPDR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    pub fn rstodt(&self) -> RSTODT_R {
        RSTODT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    pub fn rstpdd(&self) -> RSTPDD_R {
        RSTPDD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    pub fn rstpdr(&self) -> RSTPDR_R {
        RSTPDR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    pub fn rstiom(&self) -> RSTIOM_R {
        RSTIOM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    pub fn acsr(&self) -> ACSR_R {
        ACSR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    pub fn aciom(&mut self) -> ACIOM_W {
        ACIOM_W { w: self }
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    pub fn acoe(&mut self) -> ACOE_W {
        ACOE_W { w: self }
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    pub fn acodt(&mut self) -> ACODT_W {
        ACODT_W { w: self }
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    pub fn acpdd(&mut self) -> ACPDD_W {
        ACPDD_W { w: self }
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    pub fn acpdr(&mut self) -> ACPDR_W {
        ACPDR_W { w: self }
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    pub fn ckodt(&mut self) -> CKODT_W {
        CKODT_W { w: self }
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    pub fn ckpdd(&mut self) -> CKPDD_W {
        CKPDD_W { w: self }
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    pub fn ckpdr(&mut self) -> CKPDR_W {
        CKPDR_W { w: self }
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    pub fn rankodt(&mut self) -> RANKODT_W {
        RANKODT_W { w: self }
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    pub fn cspdd(&mut self) -> CSPDD_W {
        CSPDD_W { w: self }
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    pub fn rankpdr(&mut self) -> RANKPDR_W {
        RANKPDR_W { w: self }
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    pub fn rstodt(&mut self) -> RSTODT_W {
        RSTODT_W { w: self }
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    pub fn rstpdd(&mut self) -> RSTPDD_W {
        RSTPDD_W { w: self }
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    pub fn rstpdr(&mut self) -> RSTPDR_W {
        RSTPDR_W { w: self }
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    pub fn rstiom(&mut self) -> RSTIOM_W {
        RSTIOM_W { w: self }
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    pub fn acsr(&mut self) -> ACSR_W {
        ACSR_W { w: self }
    }
}
