#[doc = "Reader of register DDRPHYC_DX0GCR"]
pub type R = crate::R<u32, super::DDRPHYC_DX0GCR>;
#[doc = "Writer for register DDRPHYC_DX0GCR"]
pub type W = crate::W<u32, super::DDRPHYC_DX0GCR>;
#[doc = "Register DDRPHYC_DX0GCR `reset()`'s with value 0xee81"]
impl crate::ResetValue for super::DDRPHYC_DX0GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xee81
    }
}
#[doc = "Reader of field `DXEN`"]
pub type DXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXEN`"]
pub struct DXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DXEN_W<'a> {
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
#[doc = "Reader of field `DQSODT`"]
pub type DQSODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSODT`"]
pub struct DQSODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSODT_W<'a> {
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
#[doc = "Reader of field `DQODT`"]
pub type DQODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQODT`"]
pub struct DQODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQODT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DQSRPD`"]
pub type DQSRPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSRPD`"]
pub struct DQSRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRPD_W<'a> {
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
#[doc = "Reader of field `DSEN`"]
pub type DSEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSEN`"]
pub struct DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `DQSRTT`"]
pub type DQSRTT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSRTT`"]
pub struct DQSRTT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRTT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DQRTT`"]
pub type DQRTT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQRTT`"]
pub struct DQRTT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQRTT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTTOH`"]
pub type RTTOH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTTOH`"]
pub struct RTTOH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTOH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTTOAL`"]
pub type RTTOAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTTOAL`"]
pub struct RTTOAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTOAL_W<'a> {
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
#[doc = "Reader of field `R0RVSL`"]
pub type R0RVSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R0RVSL`"]
pub struct R0RVSL_W<'a> {
    w: &'a mut W,
}
impl<'a> R0RVSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    pub fn dxen(&self) -> DXEN_R {
        DXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    pub fn dqsodt(&self) -> DQSODT_R {
        DQSODT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    pub fn dqodt(&self) -> DQODT_R {
        DQODT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    pub fn dqsrpd(&self) -> DQSRPD_R {
        DQSRPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    pub fn dqsrtt(&self) -> DQSRTT_R {
        DQSRTT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    pub fn dqrtt(&self) -> DQRTT_R {
        DQRTT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    pub fn rttoh(&self) -> RTTOH_R {
        RTTOH_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    pub fn rttoal(&self) -> RTTOAL_R {
        RTTOAL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    pub fn r0rvsl(&self) -> R0RVSL_R {
        R0RVSL_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    pub fn dxen(&mut self) -> DXEN_W {
        DXEN_W { w: self }
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    pub fn dqsodt(&mut self) -> DQSODT_W {
        DQSODT_W { w: self }
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    pub fn dqodt(&mut self) -> DQODT_W {
        DQODT_W { w: self }
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W {
        DXIOM_W { w: self }
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W {
        DXPDD_W { w: self }
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W {
        DXPDR_W { w: self }
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    pub fn dqsrpd(&mut self) -> DQSRPD_W {
        DQSRPD_W { w: self }
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W {
        DSEN_W { w: self }
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    pub fn dqsrtt(&mut self) -> DQSRTT_W {
        DQSRTT_W { w: self }
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    pub fn dqrtt(&mut self) -> DQRTT_W {
        DQRTT_W { w: self }
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    pub fn rttoh(&mut self) -> RTTOH_W {
        RTTOH_W { w: self }
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    pub fn rttoal(&mut self) -> RTTOAL_W {
        RTTOAL_W { w: self }
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    pub fn r0rvsl(&mut self) -> R0RVSL_W {
        R0RVSL_W { w: self }
    }
}
