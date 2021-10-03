#[doc = "Register `ETH_MACHT0R` reader"]
pub struct R(crate::R<ETH_MACHT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHT0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACHT0R` writer"]
pub struct W(crate::W<ETH_MACHT0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACHT0R_SPEC>;
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
impl From<crate::W<ETH_MACHT0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACHT0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT31T0` reader - HT31T0"]
pub struct HT31T0_R(crate::FieldReader<u32, u32>);
impl HT31T0_R {
    pub(crate) fn new(bits: u32) -> Self {
        HT31T0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT31T0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT31T0` writer - HT31T0"]
pub struct HT31T0_W<'a> {
    w: &'a mut W,
}
impl<'a> HT31T0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W {
        HT31T0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macht0r](index.html) module"]
pub struct ETH_MACHT0R_SPEC;
impl crate::RegisterSpec for ETH_MACHT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macht0r::R](R) reader structure"]
impl crate::Readable for ETH_MACHT0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macht0r::W](W) writer structure"]
impl crate::Writable for ETH_MACHT0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACHT0R to value 0"]
impl crate::Resettable for ETH_MACHT0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
