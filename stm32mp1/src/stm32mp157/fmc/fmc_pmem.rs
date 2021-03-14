#[doc = "Reader of register FMC_PMEM"]
pub type R = crate::R<u32, super::FMC_PMEM>;
#[doc = "Writer for register FMC_PMEM"]
pub type W = crate::W<u32, super::FMC_PMEM>;
#[doc = "Register FMC_PMEM `reset()`'s with value 0x0a0a_0a0a"]
impl crate::ResetValue for super::FMC_PMEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a0a_0a0a
    }
}
#[doc = "Reader of field `MEMSET`"]
pub type MEMSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMSET`"]
pub struct MEMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MEMWAIT`"]
pub type MEMWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMWAIT`"]
pub struct MEMWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEMHOLD`"]
pub type MEMHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMHOLD`"]
pub struct MEMHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEMHIZ`"]
pub type MEMHIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMHIZ`"]
pub struct MEMHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MEMSET"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAIT"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLD"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZ"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSET"]
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W {
        MEMSET_W { w: self }
    }
    #[doc = "Bits 8:15 - MEMWAIT"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W {
        MEMWAIT_W { w: self }
    }
    #[doc = "Bits 16:23 - MEMHOLD"]
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W {
        MEMHOLD_W { w: self }
    }
    #[doc = "Bits 24:31 - MEMHIZ"]
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W {
        MEMHIZ_W { w: self }
    }
}
