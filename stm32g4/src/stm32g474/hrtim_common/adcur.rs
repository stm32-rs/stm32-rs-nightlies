#[doc = "Reader of register ADCUR"]
pub type R = crate::R<u32, super::ADCUR>;
#[doc = "Writer for register ADCUR"]
pub type W = crate::W<u32, super::ADCUR>;
#[doc = "Register ADCUR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AD10USRC`"]
pub type AD10USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD10USRC`"]
pub struct AD10USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD10USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `AD9USRC`"]
pub type AD9USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD9USRC`"]
pub struct AD9USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD9USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `AD8USRC`"]
pub type AD8USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD8USRC`"]
pub struct AD8USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD8USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `AD7USRC`"]
pub type AD7USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD7USRC`"]
pub struct AD7USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD7USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `AD6USRC`"]
pub type AD6USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD6USRC`"]
pub struct AD6USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD6USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `AD5USRC`"]
pub type AD5USRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD5USRC`"]
pub struct AD5USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD5USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&self) -> AD10USRC_R {
        AD10USRC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&self) -> AD9USRC_R {
        AD9USRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&self) -> AD8USRC_R {
        AD8USRC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&self) -> AD7USRC_R {
        AD7USRC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&self) -> AD6USRC_R {
        AD6USRC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&self) -> AD5USRC_R {
        AD5USRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&mut self) -> AD10USRC_W {
        AD10USRC_W { w: self }
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&mut self) -> AD9USRC_W {
        AD9USRC_W { w: self }
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&mut self) -> AD8USRC_W {
        AD8USRC_W { w: self }
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&mut self) -> AD7USRC_W {
        AD7USRC_W { w: self }
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&mut self) -> AD6USRC_W {
        AD6USRC_W { w: self }
    }
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&mut self) -> AD5USRC_W {
        AD5USRC_W { w: self }
    }
}
