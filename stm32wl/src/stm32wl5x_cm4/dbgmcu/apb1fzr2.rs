#[doc = "Register `APB1FZR2` reader"]
pub struct R(crate::R<APB1FZR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR2` writer"]
pub struct W(crate::W<APB1FZR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR2_SPEC>;
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
impl From<crate::W<APB1FZR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DBG_LPTIM2_STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_LPTIM2_STOP_A {
    #[doc = "0: LPTIM1 counter clock is fed even if the core is halted"]
    CONTINUE = 0,
    #[doc = "1: LPTIM1 counter clock is stopped when the core is halted"]
    STOP = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - DBG_LPTIM2_STOP"]
pub struct DBG_LPTIM2_STOP_R(crate::FieldReader<bool, DBG_LPTIM2_STOP_A>);
impl DBG_LPTIM2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM2_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::CONTINUE,
            true => DBG_LPTIM2_STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_LPTIM2_STOP_A::CONTINUE
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_LPTIM2_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_LPTIM2_STOP_R {
    type Target = crate::FieldReader<bool, DBG_LPTIM2_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - DBG_LPTIM2_STOP"]
pub struct DBG_LPTIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM2_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_LPTIM2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTIM1 counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::CONTINUE)
    }
    #[doc = "LPTIM1 counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "DBG_LPTIM3_STOP"]
pub type DBG_LPTIM3_STOP_A = DBG_LPTIM2_STOP_A;
#[doc = "Field `DBG_LPTIM3_STOP` reader - DBG_LPTIM3_STOP"]
pub type DBG_LPTIM3_STOP_R = DBG_LPTIM2_STOP_R;
#[doc = "Field `DBG_LPTIM3_STOP` writer - DBG_LPTIM3_STOP"]
pub struct DBG_LPTIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM3_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_LPTIM3_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTIM1 counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIM3_STOP_A::CONTINUE)
    }
    #[doc = "LPTIM1 counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIM3_STOP_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W {
        DBG_LPTIM2_STOP_W { w: self }
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W {
        DBG_LPTIM3_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr2](index.html) module"]
pub struct APB1FZR2_SPEC;
impl crate::RegisterSpec for APB1FZR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr2::R](R) reader structure"]
impl crate::Readable for APB1FZR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr2::W](W) writer structure"]
impl crate::Writable for APB1FZR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1FZR2 to value 0"]
impl crate::Resettable for APB1FZR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
