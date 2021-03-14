#[doc = "Reader of register FDCAN_TTTS"]
pub type R = crate::R<u32, super::FDCAN_TTTS>;
#[doc = "Writer for register FDCAN_TTTS"]
pub type W = crate::W<u32, super::FDCAN_TTTS>;
#[doc = "Register FDCAN_TTTS `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWTDEL`"]
pub type SWTDEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWTDEL`"]
pub struct SWTDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `EVTSEL`"]
pub type EVTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVTSEL`"]
pub struct EVTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SWTDEL"]
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - EVTSEL"]
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SWTDEL"]
    #[inline(always)]
    pub fn swtdel(&mut self) -> SWTDEL_W {
        SWTDEL_W { w: self }
    }
    #[doc = "Bits 4:5 - EVTSEL"]
    #[inline(always)]
    pub fn evtsel(&mut self) -> EVTSEL_W {
        EVTSEL_W { w: self }
    }
}
