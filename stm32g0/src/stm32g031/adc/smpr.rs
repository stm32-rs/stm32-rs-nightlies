#[doc = "Reader of register SMPR"]
pub type R = crate::R<u32, super::SMPR>;
#[doc = "Writer for register SMPR"]
pub type W = crate::W<u32, super::SMPR>;
#[doc = "Register SMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMP1`"]
pub type SMP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMP1`"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SMP2`"]
pub type SMP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMP2`"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SMPSEL`"]
pub type SMPSEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SMPSEL`"]
pub struct SMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 8)) | (((value as u32) & 0x0007_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:26 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 8) & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bits 4:6 - Sampling time selection"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bits 8:26 - Channel sampling time selection"]
    #[inline(always)]
    pub fn smpsel(&mut self) -> SMPSEL_W {
        SMPSEL_W { w: self }
    }
}
