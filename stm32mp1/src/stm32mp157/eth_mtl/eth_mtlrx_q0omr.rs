#[doc = "Reader of register ETH_MTLRxQ0OMR"]
pub type R = crate::R<u32, super::ETH_MTLRXQ0OMR>;
#[doc = "Writer for register ETH_MTLRxQ0OMR"]
pub type W = crate::W<u32, super::ETH_MTLRXQ0OMR>;
#[doc = "Register ETH_MTLRxQ0OMR `reset()`'s with value 0x0070_0000"]
impl crate::ResetValue for super::ETH_MTLRXQ0OMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0070_0000
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FUP`"]
pub type FUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUP`"]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
#[doc = "Reader of field `FEP`"]
pub type FEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEP`"]
pub struct FEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FEP_W<'a> {
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
#[doc = "Reader of field `RSF`"]
pub type RSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSF`"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
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
#[doc = "Reader of field `DIS_TCP_EF`"]
pub type DIS_TCP_EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_TCP_EF`"]
pub struct DIS_TCP_EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCP_EF_W<'a> {
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
#[doc = "Reader of field `EHFC`"]
pub type EHFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EHFC`"]
pub struct EHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> EHFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RFA`"]
pub type RFA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFA`"]
pub struct RFA_W<'a> {
    w: &'a mut W,
}
impl<'a> RFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RFD`"]
pub type RFD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFD`"]
pub struct RFD_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `RQS`"]
pub type RQS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - RQS"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W {
        FEP_W { w: self }
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W {
        DIS_TCP_EF_W { w: self }
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&mut self) -> EHFC_W {
        EHFC_W { w: self }
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W {
        RFA_W { w: self }
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W {
        RFD_W { w: self }
    }
}
