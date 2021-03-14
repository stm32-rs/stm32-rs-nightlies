#[doc = "Reader of register RCC_HSICFGR"]
pub type R = crate::R<u32, super::RCC_HSICFGR>;
#[doc = "Writer for register RCC_HSICFGR"]
pub type W = crate::W<u32, super::RCC_HSICFGR>;
#[doc = "Register RCC_HSICFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_HSICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSIDIV`"]
pub type HSIDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSIDIV`"]
pub struct HSIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
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
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSICAL`"]
pub type HSICAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W {
        HSIDIV_W { w: self }
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
}
