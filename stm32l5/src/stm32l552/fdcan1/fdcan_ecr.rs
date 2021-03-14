#[doc = "Reader of register FDCAN_ECR"]
pub type R = crate::R<u32, super::FDCAN_ECR>;
#[doc = "Writer for register FDCAN_ECR"]
pub type W = crate::W<u32, super::FDCAN_ECR>;
#[doc = "Register FDCAN_ECR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEL`"]
pub type CEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEL`"]
pub struct CEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RP`"]
pub type RP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RP`"]
pub struct RP_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_W<'a> {
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
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&mut self) -> CEL_W {
        CEL_W { w: self }
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&mut self) -> RP_W {
        RP_W { w: self }
    }
}
