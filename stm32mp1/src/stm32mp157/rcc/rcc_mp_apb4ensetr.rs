#[doc = "Register `RCC_MP_APB4ENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB4ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB4ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB4ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB4ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB4ENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB4ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB4ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB4ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB4ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCEN` reader - LTDCEN"]
pub struct LTDCEN_R(crate::FieldReader<bool, bool>);
impl LTDCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTDCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCEN` writer - LTDCEN"]
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
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
#[doc = "Field `DSIEN` reader - DSIEN"]
pub struct DSIEN_R(crate::FieldReader<bool, bool>);
impl DSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIEN` writer - DSIEN"]
pub struct DSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIEN_W<'a> {
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
#[doc = "Field `DDRPERFMEN` reader - DDRPERFMEN"]
pub struct DDRPERFMEN_R(crate::FieldReader<bool, bool>);
impl DDRPERFMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPERFMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPERFMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPERFMEN` writer - DDRPERFMEN"]
pub struct DDRPERFMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMEN_W<'a> {
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
#[doc = "Field `IWDG2APBEN` reader - IWDG2APBEN"]
pub struct IWDG2APBEN_R(crate::FieldReader<bool, bool>);
impl IWDG2APBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG2APBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG2APBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG2APBEN` writer - IWDG2APBEN"]
pub struct IWDG2APBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2APBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `USBPHYEN` reader - USBPHYEN"]
pub struct USBPHYEN_R(crate::FieldReader<bool, bool>);
impl USBPHYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBPHYEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBPHYEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBPHYEN` writer - USBPHYEN"]
pub struct USBPHYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYEN_W<'a> {
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
#[doc = "Field `STGENROEN` reader - STGENROEN"]
pub struct STGENROEN_R(crate::FieldReader<bool, bool>);
impl STGENROEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENROEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENROEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENROEN` writer - STGENROEN"]
pub struct STGENROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    pub fn ddrperfmen(&self) -> DDRPERFMEN_R {
        DDRPERFMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWDG2APBEN"]
    #[inline(always)]
    pub fn iwdg2apben(&self) -> IWDG2APBEN_R {
        IWDG2APBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    pub fn usbphyen(&self) -> USBPHYEN_R {
        USBPHYEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    pub fn stgenroen(&self) -> STGENROEN_R {
        STGENROEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W {
        DSIEN_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W {
        DDRPERFMEN_W { w: self }
    }
    #[doc = "Bit 15 - IWDG2APBEN"]
    #[inline(always)]
    pub fn iwdg2apben(&mut self) -> IWDG2APBEN_W {
        IWDG2APBEN_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    pub fn usbphyen(&mut self) -> USBPHYEN_W {
        USBPHYEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    pub fn stgenroen(&mut self) -> STGENROEN_W {
        STGENROEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb4ensetr](index.html) module"]
pub struct RCC_MP_APB4ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB4ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb4ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB4ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb4ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB4ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB4ENSETR to value 0"]
impl crate::Resettable for RCC_MP_APB4ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
