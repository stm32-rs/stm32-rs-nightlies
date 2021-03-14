#[doc = "Reader of register ETH_DMAC0CR"]
pub type R = crate::R<u32, super::ETH_DMAC0CR>;
#[doc = "Writer for register ETH_DMAC0CR"]
pub type W = crate::W<u32, super::ETH_DMAC0CR>;
#[doc = "Register ETH_DMAC0CR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC0CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSS`"]
pub type MSS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MSS`"]
pub struct MSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `PBLX8`"]
pub type PBLX8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBLX8`"]
pub struct PBLX8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLX8_W<'a> {
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
#[doc = "Reader of field `DSL`"]
pub type DSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSL`"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W {
        MSS_W { w: self }
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W {
        PBLX8_W { w: self }
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
}
