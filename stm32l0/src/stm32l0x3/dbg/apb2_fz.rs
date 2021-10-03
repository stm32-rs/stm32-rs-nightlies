#[doc = "Register `APB2_FZ` reader"]
pub struct R(crate::R<APB2_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2_FZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2_FZ` writer"]
pub struct W(crate::W<APB2_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2_FZ_SPEC>;
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
impl From<crate::W<APB2_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2_FZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug Timer 21 stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIMER21_STOP_A {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    CONTINUE = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    STOP = 1,
}
impl From<DBG_TIMER21_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER21_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIMER21_STOP` reader - Debug Timer 21 stopped when Core is halted"]
pub struct DBG_TIMER21_STOP_R(crate::FieldReader<bool, DBG_TIMER21_STOP_A>);
impl DBG_TIMER21_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIMER21_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIMER21_STOP_A {
        match self.bits {
            false => DBG_TIMER21_STOP_A::CONTINUE,
            true => DBG_TIMER21_STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_TIMER21_STOP_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_TIMER21_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_TIMER21_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIMER21_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIMER21_STOP` writer - Debug Timer 21 stopped when Core is halted"]
pub struct DBG_TIMER21_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER21_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIMER21_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::CONTINUE)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::STOP)
    }
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
#[doc = "Field `DBG_TIMER22_STO` reader - Debug Timer 22 stopped when Core is halted"]
pub struct DBG_TIMER22_STO_R(crate::FieldReader<bool, bool>);
impl DBG_TIMER22_STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIMER22_STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIMER22_STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIMER22_STO` writer - Debug Timer 22 stopped when Core is halted"]
pub struct DBG_TIMER22_STO_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER22_STO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer21_stop(&self) -> DBG_TIMER21_STOP_R {
        DBG_TIMER21_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer22_sto(&self) -> DBG_TIMER22_STO_R {
        DBG_TIMER22_STO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 21 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer21_stop(&mut self) -> DBG_TIMER21_STOP_W {
        DBG_TIMER21_STOP_W { w: self }
    }
    #[doc = "Bit 6 - Debug Timer 22 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer22_sto(&mut self) -> DBG_TIMER22_STO_W {
        DBG_TIMER22_STO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB High Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2_fz](index.html) module"]
pub struct APB2_FZ_SPEC;
impl crate::RegisterSpec for APB2_FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2_fz::R](R) reader structure"]
impl crate::Readable for APB2_FZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2_fz::W](W) writer structure"]
impl crate::Writable for APB2_FZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2_FZ to value 0"]
impl crate::Resettable for APB2_FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
