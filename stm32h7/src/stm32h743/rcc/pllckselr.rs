#[doc = "Reader of register PLLCKSELR"]
pub type R = crate::R<u32, super::PLLCKSELR>;
#[doc = "Writer for register PLLCKSELR"]
pub type W = crate::W<u32, super::PLLCKSELR>;
#[doc = "Register PLLCKSELR `reset()`'s with value 0x0202_0200"]
impl crate::ResetValue for super::PLLCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0202_0200
    }
}
#[doc = "DIVMx and PLLs clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: HSI selected as PLL clock"]
    HSI = 0,
    #[doc = "1: CSI selected as PLL clock"]
    CSI = 1,
    #[doc = "2: HSE selected as PLL clock"]
    HSE = 2,
    #[doc = "3: No clock sent to DIVMx dividers and PLLs"]
    NONE = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<u8, PLLSRC_A>;
impl PLLSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::HSI,
            1 => PLLSRC_A::CSI,
            2 => PLLSRC_A::HSE,
            3 => PLLSRC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRC_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == PLLSRC_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::HSE
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLSRC_A::NONE
    }
}
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI)
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(PLLSRC_A::CSI)
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
    }
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLSRC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DIVM1`"]
pub type DIVM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVM1`"]
pub struct DIVM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIVM2`"]
pub type DIVM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVM2`"]
pub struct DIVM2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIVM3`"]
pub type DIVM3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVM3`"]
pub struct DIVM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&mut self) -> DIVM1_W {
        DIVM1_W { w: self }
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&mut self) -> DIVM2_W {
        DIVM2_W { w: self }
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&mut self) -> DIVM3_W {
        DIVM3_W { w: self }
    }
}
