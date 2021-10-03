#[doc = "Register `APB2FZ` reader"]
pub struct R(crate::R<APB2FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2FZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2FZ` writer"]
pub struct W(crate::W<APB2FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2FZ_SPEC>;
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
impl From<crate::W<APB2FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2FZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM15_STOP` reader - Debug Timer 15 stopped when Core is halted"]
pub struct DBG_TIM15_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM15_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM15_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM15_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM15_STOP` writer - Debug Timer 15 stopped when Core is halted"]
pub struct DBG_TIM15_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM15_STOP_W<'a> {
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
#[doc = "Field `DBG_TIM16_STOP` reader - Debug Timer 16 stopped when Core is halted"]
pub struct DBG_TIM16_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM16_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM16_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM16_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM16_STOP` writer - Debug Timer 16 stopped when Core is halted"]
pub struct DBG_TIM16_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM16_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DBG_TIM17_STO` reader - Debug Timer 17 stopped when Core is halted"]
pub struct DBG_TIM17_STO_R(crate::FieldReader<bool, bool>);
impl DBG_TIM17_STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM17_STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM17_STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM17_STO` writer - Debug Timer 17 stopped when Core is halted"]
pub struct DBG_TIM17_STO_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_STO_W<'a> {
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
#[doc = "Field `DBG_TIM19_STOP` reader - Debug Timer 19 stopped when Core is halted"]
pub struct DBG_TIM19_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_TIM19_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM19_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM19_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM19_STOP` writer - Debug Timer 19 stopped when Core is halted"]
pub struct DBG_TIM19_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM19_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&self) -> DBG_TIM17_STO_R {
        DBG_TIM17_STO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DBG_TIM19_STOP_R {
        DBG_TIM19_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W {
        DBG_TIM15_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W {
        DBG_TIM16_STOP_W { w: self }
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&mut self) -> DBG_TIM17_STO_W {
        DBG_TIM17_STO_W { w: self }
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&mut self) -> DBG_TIM19_STOP_W {
        DBG_TIM19_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB High Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz](index.html) module"]
pub struct APB2FZ_SPEC;
impl crate::RegisterSpec for APB2FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2fz::R](R) reader structure"]
impl crate::Readable for APB2FZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2fz::W](W) writer structure"]
impl crate::Writable for APB2FZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2FZ to value 0"]
impl crate::Resettable for APB2FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
