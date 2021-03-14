#[doc = "Reader of register CSSA"]
pub type R = crate::R<u32, super::CSSA>;
#[doc = "Writer for register CSSA"]
pub type W = crate::W<u32, super::CSSA>;
#[doc = "Register CSSA `reset()`'s with value 0"]
impl crate::ResetValue for super::CSSA {
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
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
}
