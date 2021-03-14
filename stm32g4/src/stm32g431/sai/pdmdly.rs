#[doc = "Reader of register PDMDLY"]
pub type R = crate::R<u32, super::PDMDLY>;
#[doc = "Writer for register PDMDLY"]
pub type W = crate::W<u32, super::PDMDLY>;
#[doc = "Register PDMDLY `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMDLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLYM1L`"]
pub type DLYM1L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM1L`"]
pub struct DLYM1L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DLYM1R`"]
pub type DLYM1R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM1R`"]
pub struct DLYM1R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DLYM2L`"]
pub type DLYM2L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM2L`"]
pub struct DLYM2L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM2L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DLYM2R`"]
pub type DLYM2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM2R`"]
pub struct DLYM2R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DLYM3L`"]
pub type DLYM3L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM3L`"]
pub struct DLYM3L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM3L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DLYM3R`"]
pub type DLYM3R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM3R`"]
pub struct DLYM3R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM3R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `DLYM4L`"]
pub type DLYM4L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM4L`"]
pub struct DLYM4L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM4L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `DLYM4R`"]
pub type DLYM4R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYM4R`"]
pub struct DLYM4R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    pub fn dlym1l(&mut self) -> DLYM1L_W {
        DLYM1L_W { w: self }
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    pub fn dlym1r(&mut self) -> DLYM1R_W {
        DLYM1R_W { w: self }
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    pub fn dlym2l(&mut self) -> DLYM2L_W {
        DLYM2L_W { w: self }
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    pub fn dlym2r(&mut self) -> DLYM2R_W {
        DLYM2R_W { w: self }
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    pub fn dlym3l(&mut self) -> DLYM3L_W {
        DLYM3L_W { w: self }
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    pub fn dlym3r(&mut self) -> DLYM3R_W {
        DLYM3R_W { w: self }
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    pub fn dlym4l(&mut self) -> DLYM4L_W {
        DLYM4L_W { w: self }
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    pub fn dlym4r(&mut self) -> DLYM4R_W {
        DLYM4R_W { w: self }
    }
}
