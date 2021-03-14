#[doc = "Reader of register BCCR"]
pub type R = crate::R<u32, super::BCCR>;
#[doc = "Writer for register BCCR"]
pub type W = crate::W<u32, super::BCCR>;
#[doc = "Register BCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCBLUE`"]
pub type BCBLUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCBLUE`"]
pub struct BCBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BCGREEN`"]
pub type BCGREEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCGREEN`"]
pub struct BCGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BCRED`"]
pub type BCRED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCRED`"]
pub struct BCRED_W<'a> {
    w: &'a mut W,
}
impl<'a> BCRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Background Color Blue value"]
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background Color Green value"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background Color Blue value"]
    #[inline(always)]
    pub fn bcblue(&mut self) -> BCBLUE_W {
        BCBLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - Background Color Green value"]
    #[inline(always)]
    pub fn bcgreen(&mut self) -> BCGREEN_W {
        BCGREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bcred(&mut self) -> BCRED_W {
        BCRED_W { w: self }
    }
}
