#[doc = "Register `CCIPR2` reader"]
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR2` writer"]
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C4 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: PCLK clock selected as I2C clock"]
    PCLK = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C clock"]
    SYSTEM = 1,
    #[doc = "2: HSI16 clock selected as I2C clock"]
    HSI16 = 2,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub struct I2C4SEL_R(crate::FieldReader<u8, I2C4SEL_A>);
impl I2C4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C4SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C4SEL_A> {
        match self.bits {
            0 => Some(I2C4SEL_A::PCLK),
            1 => Some(I2C4SEL_A::SYSTEM),
            2 => Some(I2C4SEL_A::HSI16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == I2C4SEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == I2C4SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == I2C4SEL_A::HSI16
    }
}
impl core::ops::Deref for I2C4SEL_R {
    type Target = crate::FieldReader<u8, I2C4SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PCLK clock selected as I2C clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as I2C clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C4SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Octospi clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSPISEL_A {
    #[doc = "0: System clock selected as QUADSPI kernel clock"]
    SYSTEM = 0,
    #[doc = "1: HSI16 clock selected as QUADSPI kernel clock"]
    HSI16 = 1,
    #[doc = "2: PLL 'Q' clock selected as QUADSPI kernel clock"]
    PLLQ = 2,
}
impl From<QSPISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QSPISEL` reader - Octospi clock source selection"]
pub struct QSPISEL_R(crate::FieldReader<u8, QSPISEL_A>);
impl QSPISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        QSPISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<QSPISEL_A> {
        match self.bits {
            0 => Some(QSPISEL_A::SYSTEM),
            1 => Some(QSPISEL_A::HSI16),
            2 => Some(QSPISEL_A::PLLQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == QSPISEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == QSPISEL_A::HSI16
    }
    #[doc = "Checks if the value of the field is `PLLQ`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == QSPISEL_A::PLLQ
    }
}
impl core::ops::Deref for QSPISEL_R {
    type Target = crate::FieldReader<u8, QSPISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPISEL` writer - Octospi clock source selection"]
pub struct QSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(QSPISEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(QSPISEL_A::HSI16)
    }
    #[doc = "PLL 'Q' clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(QSPISEL_A::PLLQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr2::R](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr2::W](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
