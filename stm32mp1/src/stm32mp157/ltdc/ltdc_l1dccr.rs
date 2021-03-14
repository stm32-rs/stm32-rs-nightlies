#[doc = "Reader of register LTDC_L1DCCR"]
pub type R = crate::R<u32, super::LTDC_L1DCCR>;
#[doc = "Writer for register LTDC_L1DCCR"]
pub type W = crate::W<u32, super::LTDC_L1DCCR>;
#[doc = "Register LTDC_L1DCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_L1DCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCBLUE`"]
pub type DCBLUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCBLUE`"]
pub struct DCBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DCGREEN`"]
pub type DCGREEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCGREEN`"]
pub struct DCGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DCRED`"]
pub type DCRED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCRED`"]
pub struct DCRED_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DCALPHA`"]
pub type DCALPHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCALPHA`"]
pub struct DCALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCBLUE"]
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DCGREEN"]
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DCRED"]
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DCALPHA"]
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCBLUE"]
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W {
        DCBLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - DCGREEN"]
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W {
        DCGREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - DCRED"]
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W {
        DCRED_W { w: self }
    }
    #[doc = "Bits 24:31 - DCALPHA"]
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W {
        DCALPHA_W { w: self }
    }
}
