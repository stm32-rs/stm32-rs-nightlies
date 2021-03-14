#[doc = "Reader of register IPCCBR"]
pub type R = crate::R<u32, super::IPCCBR>;
#[doc = "Writer for register IPCCBR"]
pub type W = crate::W<u32, super::IPCCBR>;
#[doc = "Register IPCCBR `reset()`'s with value 0xffff_c000"]
impl crate::ResetValue for super::IPCCBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_c000
    }
}
#[doc = "Reader of field `IPCCDBA`"]
pub type IPCCDBA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IPCCDBA`"]
pub struct IPCCDBA_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCDBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - PCC mailbox data buffer base address"]
    #[inline(always)]
    pub fn ipccdba(&self) -> IPCCDBA_R {
        IPCCDBA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PCC mailbox data buffer base address"]
    #[inline(always)]
    pub fn ipccdba(&mut self) -> IPCCDBA_W {
        IPCCDBA_W { w: self }
    }
}
