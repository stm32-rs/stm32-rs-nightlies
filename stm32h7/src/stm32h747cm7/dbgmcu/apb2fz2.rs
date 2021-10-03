#[doc = "Register `APB2FZ2` reader"]
pub struct R(crate::R<APB2FZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2FZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2FZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2FZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2FZ2` writer"]
pub struct W(crate::W<APB2FZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2FZ2_SPEC>;
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
impl From<crate::W<APB2FZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2FZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM1` reader - TIM1 stop in debug"]
pub struct DBG_TIM1_R(crate::FieldReader<bool, bool>);
impl DBG_TIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM1` writer - TIM1 stop in debug"]
pub struct DBG_TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM1_W<'a> {
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
#[doc = "Field `DBG_TIM8` reader - TIM8 stop in debug"]
pub struct DBG_TIM8_R(crate::FieldReader<bool, bool>);
impl DBG_TIM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM8` writer - TIM8 stop in debug"]
pub struct DBG_TIM8_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM8_W<'a> {
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
#[doc = "Field `DBG_TIM15` reader - TIM15 stop in debug"]
pub struct DBG_TIM15_R(crate::FieldReader<bool, bool>);
impl DBG_TIM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM15` writer - TIM15 stop in debug"]
pub struct DBG_TIM15_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM15_W<'a> {
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
#[doc = "Field `DBG_TIM16` reader - TIM16 stop in debug"]
pub struct DBG_TIM16_R(crate::FieldReader<bool, bool>);
impl DBG_TIM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM16` writer - TIM16 stop in debug"]
pub struct DBG_TIM16_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DBG_TIM17` reader - TIM17 stop in debug"]
pub struct DBG_TIM17_R(crate::FieldReader<bool, bool>);
impl DBG_TIM17_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM17` writer - TIM17 stop in debug"]
pub struct DBG_TIM17_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `DBG_HRTIM` reader - HRTIM stop in debug"]
pub struct DBG_HRTIM_R(crate::FieldReader<bool, bool>);
impl DBG_HRTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_HRTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_HRTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_HRTIM` writer - HRTIM stop in debug"]
pub struct DBG_HRTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_HRTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&self) -> DBG_TIM1_R {
        DBG_TIM1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&self) -> DBG_TIM8_R {
        DBG_TIM8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&self) -> DBG_TIM15_R {
        DBG_TIM15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&self) -> DBG_TIM16_R {
        DBG_TIM16_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&self) -> DBG_TIM17_R {
        DBG_TIM17_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&self) -> DBG_HRTIM_R {
        DBG_HRTIM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&mut self) -> DBG_TIM1_W {
        DBG_TIM1_W { w: self }
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&mut self) -> DBG_TIM8_W {
        DBG_TIM8_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&mut self) -> DBG_TIM15_W {
        DBG_TIM15_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&mut self) -> DBG_TIM16_W {
        DBG_TIM16_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&mut self) -> DBG_TIM17_W {
        DBG_TIM17_W { w: self }
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&mut self) -> DBG_HRTIM_W {
        DBG_HRTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz2](index.html) module"]
pub struct APB2FZ2_SPEC;
impl crate::RegisterSpec for APB2FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2fz2::R](R) reader structure"]
impl crate::Readable for APB2FZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2fz2::W](W) writer structure"]
impl crate::Writable for APB2FZ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2FZ2 to value 0"]
impl crate::Resettable for APB2FZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
