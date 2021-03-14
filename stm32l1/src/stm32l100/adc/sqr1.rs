#[doc = "Reader of register SQR1"]
pub type R = crate::R<u32, super::SQR1>;
#[doc = "Writer for register SQR1"]
pub type W = crate::W<u32, super::SQR1>;
#[doc = "Register SQR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SQR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L`"]
pub type L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L`"]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SQ28`"]
pub type SQ28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ28`"]
pub struct SQ28_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SQ27`"]
pub type SQ27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ27`"]
pub struct SQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SQ26`"]
pub type SQ26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ26`"]
pub struct SQ26_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SQ25`"]
pub type SQ25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ25`"]
pub struct SQ25_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq28(&self) -> SQ28_R {
        SQ28_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq27(&self) -> SQ27_R {
        SQ27_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq26(&self) -> SQ26_R {
        SQ26_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq25(&self) -> SQ25_R {
        SQ25_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq28(&mut self) -> SQ28_W {
        SQ28_W { w: self }
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq27(&mut self) -> SQ27_W {
        SQ27_W { w: self }
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq26(&mut self) -> SQ26_W {
        SQ26_W { w: self }
    }
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq25(&mut self) -> SQ25_W {
        SQ25_W { w: self }
    }
}
