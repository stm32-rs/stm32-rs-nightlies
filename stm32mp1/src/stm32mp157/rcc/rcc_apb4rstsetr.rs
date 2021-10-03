#[doc = "Register `RCC_APB4RSTSETR` reader"]
pub struct R(crate::R<RCC_APB4RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB4RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB4RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB4RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB4RSTSETR` writer"]
pub struct W(crate::W<RCC_APB4RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB4RSTSETR_SPEC>;
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
impl From<crate::W<RCC_APB4RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB4RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCRST` reader - LTDCRST"]
pub struct LTDCRST_R(crate::FieldReader<bool, bool>);
impl LTDCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTDCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCRST` writer - LTDCRST"]
pub struct LTDCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCRST_W<'a> {
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
#[doc = "Field `DSIRST` reader - DSIRST"]
pub struct DSIRST_R(crate::FieldReader<bool, bool>);
impl DSIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIRST` writer - DSIRST"]
pub struct DSIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DDRPERFMRST` reader - DDRPERFMRST"]
pub struct DDRPERFMRST_R(crate::FieldReader<bool, bool>);
impl DDRPERFMRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPERFMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPERFMRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPERFMRST` writer - DDRPERFMRST"]
pub struct DDRPERFMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `USBPHYRST` reader - USBPHYRST"]
pub struct USBPHYRST_R(crate::FieldReader<bool, bool>);
impl USBPHYRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBPHYRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBPHYRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBPHYRST` writer - USBPHYRST"]
pub struct USBPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W {
        LTDCRST_W { w: self }
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W {
        DSIRST_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W {
        DDRPERFMRST_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W {
        USBPHYRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb4rstsetr](index.html) module"]
pub struct RCC_APB4RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_APB4RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb4rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_APB4RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb4rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_APB4RSTSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB4RSTSETR to value 0"]
impl crate::Resettable for RCC_APB4RSTSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
