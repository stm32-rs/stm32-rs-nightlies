#[doc = "Reader of register DTS_ITR1"]
pub type R = crate::R<u32, super::DTS_ITR1>;
#[doc = "Writer for register DTS_ITR1"]
pub type W = crate::W<u32, super::DTS_ITR1>;
#[doc = "Register DTS_ITR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_ITR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS1_LITTHD`"]
pub type TS1_LITTHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TS1_LITTHD`"]
pub struct TS1_LITTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_LITTHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TS1_HITTHD`"]
pub type TS1_HITTHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TS1_HITTHD`"]
pub struct TS1_HITTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_HITTHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TS1_LITTHD"]
    #[inline(always)]
    pub fn ts1_litthd(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS1_HITTHD"]
    #[inline(always)]
    pub fn ts1_hitthd(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TS1_LITTHD"]
    #[inline(always)]
    pub fn ts1_litthd(&mut self) -> TS1_LITTHD_W {
        TS1_LITTHD_W { w: self }
    }
    #[doc = "Bits 16:31 - TS1_HITTHD"]
    #[inline(always)]
    pub fn ts1_hitthd(&mut self) -> TS1_HITTHD_W {
        TS1_HITTHD_W { w: self }
    }
}
