#[doc = "Register `APB1H_FZ` reader"]
pub struct R(crate::R<APB1H_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1H_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1H_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1H_FZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1H_FZ` writer"]
pub struct W(crate::W<APB1H_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1H_FZ_SPEC>;
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
impl From<crate::W<APB1H_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1H_FZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_I2C4_STOP` reader - DBG_I2C4_STOP"]
pub struct DBG_I2C4_STOP_R(crate::FieldReader<bool, bool>);
impl DBG_I2C4_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C4_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C4_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C4_STOP` writer - DBG_I2C4_STOP"]
pub struct DBG_I2C4_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C4_STOP_W<'a> {
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
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W {
        DBG_I2C4_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Low Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1h_fz](index.html) module"]
pub struct APB1H_FZ_SPEC;
impl crate::RegisterSpec for APB1H_FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1h_fz::R](R) reader structure"]
impl crate::Readable for APB1H_FZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1h_fz::W](W) writer structure"]
impl crate::Writable for APB1H_FZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1H_FZ to value 0"]
impl crate::Resettable for APB1H_FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
