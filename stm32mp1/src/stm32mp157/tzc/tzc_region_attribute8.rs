#[doc = "Reader of register TZC_REGION_ATTRIBUTE8"]
pub type R = crate::R<u32, super::TZC_REGION_ATTRIBUTE8>;
#[doc = "Writer for register TZC_REGION_ATTRIBUTE8"]
pub type W = crate::W<u32, super::TZC_REGION_ATTRIBUTE8>;
#[doc = "Register TZC_REGION_ATTRIBUTE8 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_REGION_ATTRIBUTE8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILTER_EN`"]
pub type FILTER_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER_EN`"]
pub struct FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `S_RD_EN`"]
pub type S_RD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_RD_EN`"]
pub struct S_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `S_WR_EN`"]
pub type S_WR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_WR_EN`"]
pub struct S_WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> S_WR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&self) -> S_RD_EN_R {
        S_RD_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&self) -> S_WR_EN_R {
        S_WR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FILTER_EN"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W {
        FILTER_EN_W { w: self }
    }
    #[doc = "Bit 30 - S_RD_EN"]
    #[inline(always)]
    pub fn s_rd_en(&mut self) -> S_RD_EN_W {
        S_RD_EN_W { w: self }
    }
    #[doc = "Bit 31 - S_WR_EN"]
    #[inline(always)]
    pub fn s_wr_en(&mut self) -> S_WR_EN_W {
        S_WR_EN_W { w: self }
    }
}
