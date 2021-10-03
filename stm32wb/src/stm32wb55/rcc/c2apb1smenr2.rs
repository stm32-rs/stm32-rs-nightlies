#[doc = "Register `C2APB1SMENR2` reader"]
pub struct R(crate::R<C2APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB1SMENR2` writer"]
pub struct W(crate::W<C2APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1SMENR2_SPEC>;
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
impl From<crate::W<C2APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2SMEN` reader - Low power timer 2 clocks enable during CPU2 Sleep mode"]
pub struct LPTIM2SMEN_R(crate::FieldReader<bool, bool>);
impl LPTIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2SMEN` writer - Low power timer 2 clocks enable during CPU2 Sleep mode"]
pub struct LPTIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SMEN_W<'a> {
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
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during CPU2 Sleep mode"]
pub struct LPUART1SMEN_R(crate::FieldReader<bool, bool>);
impl LPUART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during CPU2 Sleep mode"]
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
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
impl R {
    #[doc = "Bit 5 - Low power timer 2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Low power UART 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Low power timer 2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    #[doc = "Bit 0 - Low power UART 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1smenr2](index.html) module"]
pub struct C2APB1SMENR2_SPEC;
impl crate::RegisterSpec for C2APB1SMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb1smenr2::R](R) reader structure"]
impl crate::Readable for C2APB1SMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb1smenr2::W](W) writer structure"]
impl crate::Writable for C2APB1SMENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB1SMENR2 to value 0x21"]
impl crate::Resettable for C2APB1SMENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
