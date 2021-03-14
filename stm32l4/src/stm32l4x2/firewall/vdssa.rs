#[doc = "Reader of register VDSSA"]
pub type R = crate::R<u32, super::VDSSA>;
#[doc = "Writer for register VDSSA"]
pub type W = crate::W<u32, super::VDSSA>;
#[doc = "Register VDSSA `reset()`'s with value 0"]
impl crate::ResetValue for super::VDSSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
}
