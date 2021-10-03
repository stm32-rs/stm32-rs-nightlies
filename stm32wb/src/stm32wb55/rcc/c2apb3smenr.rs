#[doc = "Register `C2APB3SMENR` reader"]
pub struct R(crate::R<C2APB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB3SMENR` writer"]
pub struct W(crate::W<C2APB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB3SMENR_SPEC>;
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
impl From<crate::W<C2APB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMEN802` reader - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
pub struct SMEN802_R(crate::FieldReader<bool, bool>);
impl SMEN802_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMEN802_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMEN802_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMEN802` writer - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
pub struct SMEN802_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN802_W<'a> {
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
#[doc = "Field `BLESMEN` reader - BLE interface clocks enable during CPU2 Sleep mode"]
pub struct BLESMEN_R(crate::FieldReader<bool, bool>);
impl BLESMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLESMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLESMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLESMEN` writer - BLE interface clocks enable during CPU2 Sleep mode"]
pub struct BLESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLESMEN_W<'a> {
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
    #[doc = "Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
    #[inline(always)]
    pub fn smen802(&self) -> SMEN802_R {
        SMEN802_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BLE interface clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn blesmen(&self) -> BLESMEN_R {
        BLESMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes"]
    #[inline(always)]
    pub fn smen802(&mut self) -> SMEN802_W {
        SMEN802_W { w: self }
    }
    #[doc = "Bit 0 - BLE interface clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn blesmen(&mut self) -> BLESMEN_W {
        BLESMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB3SMENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3smenr](index.html) module"]
pub struct C2APB3SMENR_SPEC;
impl crate::RegisterSpec for C2APB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb3smenr::R](R) reader structure"]
impl crate::Readable for C2APB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb3smenr::W](W) writer structure"]
impl crate::Writable for C2APB3SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB3SMENR to value 0x03"]
impl crate::Resettable for C2APB3SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
