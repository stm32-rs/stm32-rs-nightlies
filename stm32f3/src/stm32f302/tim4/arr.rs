#[doc = "Reader of register ARR"]
pub type R = crate::R<u32, super::ARR>;
#[doc = "Writer for register ARR"]
pub type W = crate::W<u32, super::ARR>;
#[doc = "Register ARR `reset()`'s with value 0"]
impl crate::ResetValue for super::ARR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARR`"]
pub type ARR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR`"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ARRH`"]
pub type ARRH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARRH`"]
pub struct ARRH_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arrh(&self) -> ARRH_R {
        ARRH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
    #[doc = "Bits 16:31 - High Auto-reload value"]
    #[inline(always)]
    pub fn arrh(&mut self) -> ARRH_W {
        ARRH_W { w: self }
    }
}
