#[doc = "Reader of register CCIPR2"]
pub type R = crate::R<u32, super::CCIPR2>;
#[doc = "Writer for register CCIPR2"]
pub type W = crate::W<u32, super::CCIPR2>;
#[doc = "Register CCIPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCIPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C4SEL`"]
pub type I2C4SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C4SEL`"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `QSPISEL`"]
pub type QSPISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QSPISEL`"]
pub struct QSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn qspisel(&mut self) -> QSPISEL_W {
        QSPISEL_W { w: self }
    }
}
