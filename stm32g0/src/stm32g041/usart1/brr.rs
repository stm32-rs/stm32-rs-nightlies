#[doc = "Reader of register BRR"]
pub type R = crate::R<u32, super::BRR>;
#[doc = "Writer for register BRR"]
pub type W = crate::W<u32, super::BRR>;
#[doc = "Register BRR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRR_4_15`"]
pub type BRR_4_15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRR_4_15`"]
pub struct BRR_4_15_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_4_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `BRR_0_3`"]
pub type BRR_0_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRR_0_3`"]
pub struct BRR_0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_0_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> BRR_4_15_R {
        BRR_4_15_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> BRR_0_3_R {
        BRR_0_3_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - BRR_4_15"]
    #[inline(always)]
    pub fn brr_4_15(&mut self) -> BRR_4_15_W {
        BRR_4_15_W { w: self }
    }
    #[doc = "Bits 0:3 - BRR_0_3"]
    #[inline(always)]
    pub fn brr_0_3(&mut self) -> BRR_0_3_W {
        BRR_0_3_W { w: self }
    }
}
