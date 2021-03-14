#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: Asynchronous clock mode"]
    ADCCLK = 0,
    #[doc = "1: Synchronous clock mode (PCLK/2)"]
    PCLK_DIV2 = 1,
    #[doc = "2: Sychronous clock mode (PCLK/4)"]
    PCLK_DIV4 = 2,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKMODE`"]
pub type CKMODE_R = crate::R<u8, CKMODE_A>;
impl CKMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKMODE_A::ADCCLK),
            1 => Val(CKMODE_A::PCLK_DIV2),
            2 => Val(CKMODE_A::PCLK_DIV4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCCLK`"]
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        *self == CKMODE_A::ADCCLK
    }
    #[doc = "Checks if the value of the field is `PCLK_DIV2`"]
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE_A::PCLK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCLK_DIV4`"]
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE_A::PCLK_DIV4
    }
}
#[doc = "Write proxy for field `CKMODE`"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Asynchronous clock mode"]
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut W {
        self.variant(CKMODE_A::ADCCLK)
    }
    #[doc = "Synchronous clock mode (PCLK/2)"]
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV2)
    }
    #[doc = "Sychronous clock mode (PCLK/4)"]
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
}
