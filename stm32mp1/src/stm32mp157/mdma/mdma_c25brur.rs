#[doc = "Reader of register MDMA_C25BRUR"]
pub type R = crate::R<u32, super::MDMA_C25BRUR>;
#[doc = "Writer for register MDMA_C25BRUR"]
pub type W = crate::W<u32, super::MDMA_C25BRUR>;
#[doc = "Register MDMA_C25BRUR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C25BRUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUV`"]
pub type SUV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUV`"]
pub struct SUV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DUV`"]
pub type DUV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DUV`"]
pub struct DUV_W<'a> {
    w: &'a mut W,
}
impl<'a> DUV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SUV"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DUV"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SUV"]
    #[inline(always)]
    pub fn suv(&mut self) -> SUV_W {
        SUV_W { w: self }
    }
    #[doc = "Bits 16:31 - DUV"]
    #[inline(always)]
    pub fn duv(&mut self) -> DUV_W {
        DUV_W { w: self }
    }
}
