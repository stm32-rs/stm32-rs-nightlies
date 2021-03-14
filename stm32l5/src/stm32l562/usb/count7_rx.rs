#[doc = "Reader of register COUNT7_RX"]
pub type R = crate::R<u16, super::COUNT7_RX>;
#[doc = "Writer for register COUNT7_RX"]
pub type W = crate::W<u16, super::COUNT7_RX>;
#[doc = "Register COUNT7_RX `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNT7_RX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT7_RX`"]
pub type COUNT7_RX_R = crate::R<u16, u16>;
#[doc = "Reader of field `NUM_BLOCK`"]
pub type NUM_BLOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_BLOCK`"]
pub struct NUM_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u16) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `BL_SIZE`"]
pub type BL_SIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL_SIZE`"]
pub struct BL_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_SIZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Reception byte count"]
    #[inline(always)]
    pub fn count7_rx(&self) -> COUNT7_RX_R {
        COUNT7_RX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&mut self) -> NUM_BLOCK_W {
        NUM_BLOCK_W { w: self }
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&mut self) -> BL_SIZE_W {
        BL_SIZE_W { w: self }
    }
}
