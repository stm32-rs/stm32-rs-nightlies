#[doc = "Reader of register DDRPHYC_PTR1"]
pub type R = crate::R<u32, super::DDRPHYC_PTR1>;
#[doc = "Writer for register DDRPHYC_PTR1"]
pub type W = crate::W<u32, super::DDRPHYC_PTR1>;
#[doc = "Register DDRPHYC_PTR1 `reset()`'s with value 0x0604_111d"]
impl crate::ResetValue for super::DDRPHYC_PTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0604_111d
    }
}
#[doc = "Reader of field `TDINIT0`"]
pub type TDINIT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDINIT0`"]
pub struct TDINIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TDINIT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
#[doc = "Reader of field `TDINIT1`"]
pub type TDINIT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDINIT1`"]
pub struct TDINIT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDINIT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | (((value as u32) & 0xff) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    pub fn tdinit0(&self) -> TDINIT0_R {
        TDINIT0_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    pub fn tdinit1(&self) -> TDINIT1_R {
        TDINIT1_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    pub fn tdinit0(&mut self) -> TDINIT0_W {
        TDINIT0_W { w: self }
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    pub fn tdinit1(&mut self) -> TDINIT1_W {
        TDINIT1_W { w: self }
    }
}
