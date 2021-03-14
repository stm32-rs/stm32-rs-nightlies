#[doc = "Reader of register C1EMR1"]
pub type R = crate::R<u32, super::C1EMR1>;
#[doc = "Writer for register C1EMR1"]
pub type W = crate::W<u32, super::C1EMR1>;
#[doc = "Register C1EMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1EMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM0_15`"]
pub type EM0_15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EM0_15`"]
pub struct EM0_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EM17_21`"]
pub type EM17_21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EM17_21`"]
pub struct EM17_21_W<'a> {
    w: &'a mut W,
}
impl<'a> EM17_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0_15(&mut self) -> EM0_15_W {
        EM0_15_W { w: self }
    }
    #[doc = "Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17_21(&mut self) -> EM17_21_W {
        EM17_21_W { w: self }
    }
}
