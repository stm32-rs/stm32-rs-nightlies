#[doc = "Reader of register EXTCFGR"]
pub type R = crate::R<u32, super::EXTCFGR>;
#[doc = "Writer for register EXTCFGR"]
pub type W = crate::W<u32, super::EXTCFGR>;
#[doc = "Register EXTCFGR `reset()`'s with value 0x0003_0000"]
impl crate::ResetValue for super::EXTCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0000
    }
}
#[doc = "Reader of field `RFCSS`"]
pub type RFCSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2HPREF`"]
pub type C2HPREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHDHPREF`"]
pub type SHDHPREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2HPRE`"]
pub type C2HPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C2HPRE`"]
pub struct C2HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> C2HPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SHDHPRE`"]
pub type SHDHPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHDHPRE`"]
pub struct SHDHPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHDHPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - RF clock source selected"]
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU2 AHB prescaler flag"]
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Shared AHB prescaler flag"]
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&mut self) -> C2HPRE_W {
        C2HPRE_W { w: self }
    }
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&mut self) -> SHDHPRE_W {
        SHDHPRE_W { w: self }
    }
}
