#[doc = "Reader of register HWCFGR"]
pub type R = crate::R<u32, super::HWCFGR>;
#[doc = "Writer for register HWCFGR"]
pub type W = crate::W<u32, super::HWCFGR>;
#[doc = "Register HWCFGR `reset()`'s with value 0x71"]
impl crate::ResetValue for super::HWCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x71
    }
}
#[doc = "Reader of field `WINDOW`"]
pub type WINDOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINDOW`"]
pub struct WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PR_DEFAULT`"]
pub type PR_DEFAULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PR_DEFAULT`"]
pub struct PR_DEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_DEFAULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Support of Window function"]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W {
        WINDOW_W { w: self }
    }
    #[doc = "Bits 4:7 - Prescaler default value"]
    #[inline(always)]
    pub fn pr_default(&mut self) -> PR_DEFAULT_W {
        PR_DEFAULT_W { w: self }
    }
}
