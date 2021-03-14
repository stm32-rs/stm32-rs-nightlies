#[doc = "Writer for register GICD_SGIR"]
pub type W = crate::W<u32, super::GICD_SGIR>;
#[doc = "Register GICD_SGIR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_SGIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SGIINTID`"]
pub struct SGIINTID_W<'a> {
    w: &'a mut W,
}
impl<'a> SGIINTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Write proxy for field `NSATT`"]
pub struct NSATT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSATT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `CPUTARGETLIST`"]
pub struct CPUTARGETLIST_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUTARGETLIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `TARGETLISTFILTER`"]
pub struct TARGETLISTFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGETLISTFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - SGIINTID"]
    #[inline(always)]
    pub fn sgiintid(&mut self) -> SGIINTID_W {
        SGIINTID_W { w: self }
    }
    #[doc = "Bit 15 - NSATT"]
    #[inline(always)]
    pub fn nsatt(&mut self) -> NSATT_W {
        NSATT_W { w: self }
    }
    #[doc = "Bits 16:17 - CPUTARGETLIST"]
    #[inline(always)]
    pub fn cputargetlist(&mut self) -> CPUTARGETLIST_W {
        CPUTARGETLIST_W { w: self }
    }
    #[doc = "Bits 24:25 - TARGETLISTFILTER"]
    #[inline(always)]
    pub fn targetlistfilter(&mut self) -> TARGETLISTFILTER_W {
        TARGETLISTFILTER_W { w: self }
    }
}
