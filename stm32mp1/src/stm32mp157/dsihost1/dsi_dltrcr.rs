#[doc = "Reader of register DSI_DLTRCR"]
pub type R = crate::R<u32, super::DSI_DLTRCR>;
#[doc = "Writer for register DSI_DLTRCR"]
pub type W = crate::W<u32, super::DSI_DLTRCR>;
#[doc = "Register DSI_DLTRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_DLTRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MRD_TIME`"]
pub type MRD_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MRD_TIME`"]
pub struct MRD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W {
        MRD_TIME_W { w: self }
    }
}
