#[doc = "Reader of register HWCFGR6"]
pub type R = crate::R<u32, super::HWCFGR6>;
#[doc = "Writer for register HWCFGR6"]
pub type W = crate::W<u32, super::HWCFGR6>;
#[doc = "Register HWCFGR6 `reset()`'s with value 0x03"]
impl crate::ResetValue for super::HWCFGR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `CPUEVENT`"]
pub type CPUEVENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CPUEVENT`"]
pub struct CPUEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&mut self) -> CPUEVENT_W {
        CPUEVENT_W { w: self }
    }
}
