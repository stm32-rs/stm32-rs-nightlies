#[doc = "Writer for register HSEM_CR"]
pub type W = crate::W<u32, super::HSEM_CR>;
#[doc = "Register HSEM_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MASTERID`"]
pub struct MASTERID_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTERID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 8:11 - MASTERID"]
    #[inline(always)]
    pub fn masterid(&mut self) -> MASTERID_W {
        MASTERID_W { w: self }
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
