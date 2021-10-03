#[doc = "Register `C2APB3ENR` reader"]
pub struct R(crate::R<C2APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB3ENR` writer"]
pub struct W(crate::W<C2APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB3ENR_SPEC>;
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
impl From<crate::W<C2APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN802` reader - CPU2 802.15.4 interface clock enable"]
pub struct EN802_R(crate::FieldReader<bool, bool>);
impl EN802_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN802_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN802_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN802` writer - CPU2 802.15.4 interface clock enable"]
pub struct EN802_W<'a> {
    w: &'a mut W,
}
impl<'a> EN802_W<'a> {
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
#[doc = "Field `BLEEN` reader - CPU2 BLE interface clock enable"]
pub struct BLEEN_R(crate::FieldReader<bool, bool>);
impl BLEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEEN` writer - CPU2 BLE interface clock enable"]
pub struct BLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEEN_W<'a> {
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
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&self) -> EN802_R {
        EN802_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&self) -> BLEEN_R {
        BLEEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CPU2 802.15.4 interface clock enable"]
    #[inline(always)]
    pub fn en802(&mut self) -> EN802_W {
        EN802_W { w: self }
    }
    #[doc = "Bit 0 - CPU2 BLE interface clock enable"]
    #[inline(always)]
    pub fn bleen(&mut self) -> BLEEN_W {
        BLEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB3ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3enr](index.html) module"]
pub struct C2APB3ENR_SPEC;
impl crate::RegisterSpec for C2APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb3enr::R](R) reader structure"]
impl crate::Readable for C2APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb3enr::W](W) writer structure"]
impl crate::Writable for C2APB3ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB3ENR to value 0"]
impl crate::Resettable for C2APB3ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
