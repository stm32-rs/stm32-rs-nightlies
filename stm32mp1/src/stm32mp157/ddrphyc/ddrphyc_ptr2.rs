#[doc = "Reader of register DDRPHYC_PTR2"]
pub type R = crate::R<u32, super::DDRPHYC_PTR2>;
#[doc = "Writer for register DDRPHYC_PTR2"]
pub type W = crate::W<u32, super::DDRPHYC_PTR2>;
#[doc = "Register DDRPHYC_PTR2 `reset()`'s with value 0x042d_a072"]
impl crate::ResetValue for super::DDRPHYC_PTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x042d_a072
    }
}
#[doc = "Reader of field `TDINIT2`"]
pub type TDINIT2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDINIT2`"]
pub struct TDINIT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDINIT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `TDINIT3`"]
pub type TDINIT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDINIT3`"]
pub struct TDINIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> TDINIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 17)) | (((value as u32) & 0x03ff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    pub fn tdinit2(&self) -> TDINIT2_R {
        TDINIT2_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    pub fn tdinit3(&self) -> TDINIT3_R {
        TDINIT3_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    pub fn tdinit2(&mut self) -> TDINIT2_W {
        TDINIT2_W { w: self }
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    pub fn tdinit3(&mut self) -> TDINIT3_W {
        TDINIT3_W { w: self }
    }
}
