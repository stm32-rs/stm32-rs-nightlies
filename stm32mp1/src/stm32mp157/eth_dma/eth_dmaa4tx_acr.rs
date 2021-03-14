#[doc = "Reader of register ETH_DMAA4TxACR"]
pub type R = crate::R<u32, super::ETH_DMAA4TXACR>;
#[doc = "Writer for register ETH_DMAA4TxACR"]
pub type W = crate::W<u32, super::ETH_DMAA4TXACR>;
#[doc = "Register ETH_DMAA4TxACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAA4TXACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDRC`"]
pub type TDRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDRC`"]
pub struct TDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEC`"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `THC`"]
pub type THC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THC`"]
pub struct THC_W<'a> {
    w: &'a mut W,
}
impl<'a> THC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    pub fn tdrc(&mut self) -> TDRC_W {
        TDRC_W { w: self }
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    pub fn thc(&mut self) -> THC_W {
        THC_W { w: self }
    }
}
