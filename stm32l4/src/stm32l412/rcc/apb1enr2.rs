#[doc = "Register `APB1ENR2` reader"]
pub struct R(crate::R<APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR2` writer"]
pub struct W(crate::W<APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR2_SPEC>;
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
impl From<crate::W<APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub struct LPTIM2EN_R(crate::FieldReader<bool, bool>);
impl LPTIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SWPMI1EN` reader - Single wire protocol clock enable"]
pub struct SWPMI1EN_R(crate::FieldReader<bool, bool>);
impl SWPMI1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPMI1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPMI1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPMI1EN` writer - Single wire protocol clock enable"]
pub struct SWPMI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMI1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable"]
pub struct LPUART1EN_R(crate::FieldReader<bool, bool>);
impl LPUART1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable"]
pub struct LPUART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DFSDMEN` reader - DFSDMEN enable"]
pub struct DFSDMEN_R(crate::FieldReader<bool, bool>);
impl DFSDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMEN` writer - DFSDMEN enable"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub struct I2C4EN_R(crate::FieldReader<bool, bool>);
impl I2C4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    pub fn swpmi1en(&self) -> SWPMI1EN_R {
        SWPMI1EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 24 - DFSDMEN enable"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    pub fn swpmi1en(&mut self) -> SWPMI1EN_W {
        SWPMI1EN_W { w: self }
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W {
        LPUART1EN_W { w: self }
    }
    #[doc = "Bit 24 - DFSDMEN enable"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](index.html) module"]
pub struct APB1ENR2_SPEC;
impl crate::RegisterSpec for APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr2::R](R) reader structure"]
impl crate::Readable for APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](W) writer structure"]
impl crate::Writable for APB1ENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
