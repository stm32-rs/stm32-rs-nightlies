#[doc = "Reader of register RCC_CSICFGR"]
pub type R = crate::R<u32, super::RCC_CSICFGR>;
#[doc = "Writer for register RCC_CSICFGR"]
pub type W = crate::W<u32, super::RCC_CSICFGR>;
#[doc = "Register RCC_CSICFGR `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::RCC_CSICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `CSITRIM`"]
pub type CSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSITRIM`"]
pub struct CSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CSICAL`"]
pub type CSICAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - CSICAL"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W {
        CSITRIM_W { w: self }
    }
}
