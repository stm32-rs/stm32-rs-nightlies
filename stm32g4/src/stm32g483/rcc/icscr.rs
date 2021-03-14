#[doc = "Reader of register ICSCR"]
pub type R = crate::R<u32, super::ICSCR>;
#[doc = "Writer for register ICSCR"]
pub type W = crate::W<u32, super::ICSCR>;
#[doc = "Register ICSCR `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::ICSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `HSICAL0`"]
pub type HSICAL0_R = crate::R<u8, u8>;
#[doc = "Reader of field `HSITRIM`"]
pub type HSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSITRIM`"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Internal High Speed clock Calibration"]
    #[inline(always)]
    pub fn hsical0(&self) -> HSICAL0_R {
        HSICAL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
}
