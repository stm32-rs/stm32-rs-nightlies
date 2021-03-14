#[doc = "Reader of register DDRCTRL_ADDRMAP1"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP1>;
#[doc = "Writer for register DDRCTRL_ADDRMAP1"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP1>;
#[doc = "Register DDRCTRL_ADDRMAP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_BANK_B0`"]
pub type ADDRMAP_BANK_B0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_BANK_B0`"]
pub struct ADDRMAP_BANK_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_BANK_B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_BANK_B1`"]
pub type ADDRMAP_BANK_B1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_BANK_B1`"]
pub struct ADDRMAP_BANK_B1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_BANK_B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_BANK_B2`"]
pub type ADDRMAP_BANK_B2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_BANK_B2`"]
pub struct ADDRMAP_BANK_B2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_BANK_B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ADDRMAP_BANK_B0"]
    #[inline(always)]
    pub fn addrmap_bank_b0(&self) -> ADDRMAP_BANK_B0_R {
        ADDRMAP_BANK_B0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - ADDRMAP_BANK_B1"]
    #[inline(always)]
    pub fn addrmap_bank_b1(&self) -> ADDRMAP_BANK_B1_R {
        ADDRMAP_BANK_B1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - ADDRMAP_BANK_B2"]
    #[inline(always)]
    pub fn addrmap_bank_b2(&self) -> ADDRMAP_BANK_B2_R {
        ADDRMAP_BANK_B2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ADDRMAP_BANK_B0"]
    #[inline(always)]
    pub fn addrmap_bank_b0(&mut self) -> ADDRMAP_BANK_B0_W {
        ADDRMAP_BANK_B0_W { w: self }
    }
    #[doc = "Bits 8:13 - ADDRMAP_BANK_B1"]
    #[inline(always)]
    pub fn addrmap_bank_b1(&mut self) -> ADDRMAP_BANK_B1_W {
        ADDRMAP_BANK_B1_W { w: self }
    }
    #[doc = "Bits 16:21 - ADDRMAP_BANK_B2"]
    #[inline(always)]
    pub fn addrmap_bank_b2(&mut self) -> ADDRMAP_BANK_B2_W {
        ADDRMAP_BANK_B2_W { w: self }
    }
}
