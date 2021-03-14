#[doc = "Reader of register DDRCTRL_INIT5"]
pub type R = crate::R<u32, super::DDRCTRL_INIT5>;
#[doc = "Writer for register DDRCTRL_INIT5"]
pub type W = crate::W<u32, super::DDRCTRL_INIT5>;
#[doc = "Register DDRCTRL_INIT5 `reset()`'s with value 0x0010_0004"]
impl crate::ResetValue for super::DDRCTRL_INIT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0004
    }
}
#[doc = "Reader of field `MAX_AUTO_INIT_X1024`"]
pub type MAX_AUTO_INIT_X1024_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_AUTO_INIT_X1024`"]
pub struct MAX_AUTO_INIT_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_AUTO_INIT_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DEV_ZQINIT_X32`"]
pub type DEV_ZQINIT_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEV_ZQINIT_X32`"]
pub struct DEV_ZQINIT_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ZQINIT_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    pub fn max_auto_init_x1024(&self) -> MAX_AUTO_INIT_X1024_R {
        MAX_AUTO_INIT_X1024_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    pub fn dev_zqinit_x32(&self) -> DEV_ZQINIT_X32_R {
        DEV_ZQINIT_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    pub fn max_auto_init_x1024(&mut self) -> MAX_AUTO_INIT_X1024_W {
        MAX_AUTO_INIT_X1024_W { w: self }
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    pub fn dev_zqinit_x32(&mut self) -> DEV_ZQINIT_X32_W {
        DEV_ZQINIT_X32_W { w: self }
    }
}
