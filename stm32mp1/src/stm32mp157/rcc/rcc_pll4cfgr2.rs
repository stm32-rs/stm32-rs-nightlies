#[doc = "Reader of register RCC_PLL4CFGR2"]
pub type R = crate::R<u32, super::RCC_PLL4CFGR2>;
#[doc = "Writer for register RCC_PLL4CFGR2"]
pub type W = crate::W<u32, super::RCC_PLL4CFGR2>;
#[doc = "Register RCC_PLL4CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PLL4CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVP`"]
pub type DIVP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVP`"]
pub struct DIVP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `DIVQ`"]
pub type DIVQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVQ`"]
pub struct DIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIVR`"]
pub type DIVR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVR`"]
pub struct DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W {
        DIVP_W { w: self }
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W {
        DIVQ_W { w: self }
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&mut self) -> DIVR_W {
        DIVR_W { w: self }
    }
}
