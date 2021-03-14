#[doc = "Reader of register DFSDM_CHDATIN4R"]
pub type R = crate::R<u32, super::DFSDM_CHDATIN4R>;
#[doc = "Writer for register DFSDM_CHDATIN4R"]
pub type W = crate::W<u32, super::DFSDM_CHDATIN4R>;
#[doc = "Register DFSDM_CHDATIN4R `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_CHDATIN4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INDAT0`"]
pub type INDAT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INDAT0`"]
pub struct INDAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `INDAT1`"]
pub type INDAT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INDAT1`"]
pub struct INDAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data for channel 4"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Input data for channel 5"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input data for channel 4"]
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W {
        INDAT0_W { w: self }
    }
    #[doc = "Bits 16:31 - Input data for channel 5"]
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W {
        INDAT1_W { w: self }
    }
}
