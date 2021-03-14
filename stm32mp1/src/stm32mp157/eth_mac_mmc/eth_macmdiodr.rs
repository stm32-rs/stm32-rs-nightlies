#[doc = "Reader of register ETH_MACMDIODR"]
pub type R = crate::R<u32, super::ETH_MACMDIODR>;
#[doc = "Writer for register ETH_MACMDIODR"]
pub type W = crate::W<u32, super::ETH_MACMDIODR>;
#[doc = "Register ETH_MACMDIODR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACMDIODR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GD`"]
pub type GD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GD`"]
pub struct GD_W<'a> {
    w: &'a mut W,
}
impl<'a> GD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    pub fn gd(&mut self) -> GD_W {
        GD_W { w: self }
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
}
