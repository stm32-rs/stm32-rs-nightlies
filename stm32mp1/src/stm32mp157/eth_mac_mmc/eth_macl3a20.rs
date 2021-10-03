#[doc = "Register `ETH_MACL3A20` reader"]
pub struct R(crate::R<ETH_MACL3A20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL3A20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL3A20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL3A20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL3A20` writer"]
pub struct W(crate::W<ETH_MACL3A20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL3A20_SPEC>;
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
impl From<crate::W<ETH_MACL3A20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL3A20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A20` reader - L3A20"]
pub struct L3A20_R(crate::FieldReader<u32, u32>);
impl L3A20_R {
    pub(crate) fn new(bits: u32) -> Self {
        L3A20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3A20_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3A20` writer - L3A20"]
pub struct L3A20_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&mut self) -> L3A20_W {
        L3A20_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a20](index.html) module"]
pub struct ETH_MACL3A20_SPEC;
impl crate::RegisterSpec for ETH_MACL3A20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl3a20::R](R) reader structure"]
impl crate::Readable for ETH_MACL3A20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl3a20::W](W) writer structure"]
impl crate::Writable for ETH_MACL3A20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACL3A20 to value 0"]
impl crate::Resettable for ETH_MACL3A20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
