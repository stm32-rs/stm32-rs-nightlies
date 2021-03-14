#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSER`"]
pub type TSER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSER`"]
pub struct TSER_W<'a> {
    w: &'a mut W,
}
impl<'a> TSER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSIZE`"]
pub type TSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSIZE`"]
pub struct TSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous"]
    #[inline(always)]
    pub fn tser(&self) -> TSER_R {
        TSER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous"]
    #[inline(always)]
    pub fn tser(&mut self) -> TSER_W {
        TSER_W { w: self }
    }
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W {
        TSIZE_W { w: self }
    }
}
