#[doc = "Reader of register RXESC"]
pub type R = crate::R<u32, super::RXESC>;
#[doc = "Writer for register RXESC"]
pub type W = crate::W<u32, super::RXESC>;
#[doc = "Register RXESC `reset()`'s with value 0"]
impl crate::ResetValue for super::RXESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0DS`"]
pub type F0DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0DS`"]
pub struct F0DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F0DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `F1DS`"]
pub type F1DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F1DS`"]
pub struct F1DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F1DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RBDS`"]
pub type RBDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RBDS`"]
pub struct RBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0DS_W {
        F0DS_W { w: self }
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1DS_W {
        F1DS_W { w: self }
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RBDS_W {
        RBDS_W { w: self }
    }
}
