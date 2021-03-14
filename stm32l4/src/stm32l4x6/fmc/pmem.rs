#[doc = "Reader of register PMEM"]
pub type R = crate::R<u32, super::PMEM>;
#[doc = "Writer for register PMEM"]
pub type W = crate::W<u32, super::PMEM>;
#[doc = "Register PMEM `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::PMEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `MEMHIZx`"]
pub type MEMHIZX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMHIZx`"]
pub struct MEMHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEMHOLDx`"]
pub type MEMHOLDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMHOLDx`"]
pub struct MEMHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEMWAITx`"]
pub type MEMWAITX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMWAITx`"]
pub struct MEMWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEMSETx`"]
pub type MEMSETX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMSETx`"]
pub struct MEMSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&mut self) -> MEMHIZX_W {
        MEMHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&mut self) -> MEMHOLDX_W {
        MEMHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MEMWAITX_W {
        MEMWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&mut self) -> MEMSETX_W {
        MEMSETX_W { w: self }
    }
}
