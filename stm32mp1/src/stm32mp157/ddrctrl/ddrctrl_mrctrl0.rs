#[doc = "Reader of register DDRCTRL_MRCTRL0"]
pub type R = crate::R<u32, super::DDRCTRL_MRCTRL0>;
#[doc = "Writer for register DDRCTRL_MRCTRL0"]
pub type W = crate::W<u32, super::DDRCTRL_MRCTRL0>;
#[doc = "Register DDRCTRL_MRCTRL0 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::DDRCTRL_MRCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `MR_TYPE`"]
pub type MR_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR_TYPE`"]
pub struct MR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_TYPE_W<'a> {
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
#[doc = "Reader of field `MR_RANK`"]
pub type MR_RANK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR_RANK`"]
pub struct MR_RANK_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_RANK_W<'a> {
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
#[doc = "Reader of field `MR_ADDR`"]
pub type MR_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MR_ADDR`"]
pub struct MR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MR_WR`"]
pub type MR_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR_WR`"]
pub struct MR_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_WR_W<'a> {
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
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&self) -> MR_TYPE_R {
        MR_TYPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&self) -> MR_RANK_R {
        MR_RANK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&self) -> MR_ADDR_R {
        MR_ADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&self) -> MR_WR_R {
        MR_WR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&mut self) -> MR_TYPE_W {
        MR_TYPE_W { w: self }
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&mut self) -> MR_RANK_W {
        MR_RANK_W { w: self }
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&mut self) -> MR_ADDR_W {
        MR_ADDR_W { w: self }
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&mut self) -> MR_WR_W {
        MR_WR_W { w: self }
    }
}
