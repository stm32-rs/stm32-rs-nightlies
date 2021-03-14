#[doc = "Reader of register OTG_HS_HFIR"]
pub type R = crate::R<u32, super::OTG_HS_HFIR>;
#[doc = "Writer for register OTG_HS_HFIR"]
pub type W = crate::W<u32, super::OTG_HS_HFIR>;
#[doc = "Register OTG_HS_HFIR `reset()`'s with value 0xea60"]
impl crate::ResetValue for super::OTG_HS_HFIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xea60
    }
}
#[doc = "Reader of field `FRIVL`"]
pub type FRIVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRIVL`"]
pub struct FRIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn frivl(&mut self) -> FRIVL_W {
        FRIVL_W { w: self }
    }
}
