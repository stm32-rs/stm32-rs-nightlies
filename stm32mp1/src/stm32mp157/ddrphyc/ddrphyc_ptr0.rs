#[doc = "Reader of register DDRPHYC_PTR0"]
pub type R = crate::R<u32, super::DDRPHYC_PTR0>;
#[doc = "Writer for register DDRPHYC_PTR0"]
pub type W = crate::W<u32, super::DDRPHYC_PTR0>;
#[doc = "Register DDRPHYC_PTR0 `reset()`'s with value 0x0022_af9b"]
impl crate::ResetValue for super::DDRPHYC_PTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0022_af9b
    }
}
#[doc = "Reader of field `TDLLSRST`"]
pub type TDLLSRST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDLLSRST`"]
pub struct TDLLSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLLSRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TDLLLOCK`"]
pub type TDLLLOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDLLLOCK`"]
pub struct TDLLLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLLLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 6)) | (((value as u32) & 0x0fff) << 6);
        self.w
    }
}
#[doc = "Reader of field `TITMSRST`"]
pub type TITMSRST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TITMSRST`"]
pub struct TITMSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TITMSRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    pub fn tdllsrst(&self) -> TDLLSRST_R {
        TDLLSRST_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    pub fn tdlllock(&self) -> TDLLLOCK_R {
        TDLLLOCK_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    pub fn titmsrst(&self) -> TITMSRST_R {
        TITMSRST_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    pub fn tdllsrst(&mut self) -> TDLLSRST_W {
        TDLLSRST_W { w: self }
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    pub fn tdlllock(&mut self) -> TDLLLOCK_W {
        TDLLLOCK_W { w: self }
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    pub fn titmsrst(&mut self) -> TITMSRST_W {
        TITMSRST_W { w: self }
    }
}
