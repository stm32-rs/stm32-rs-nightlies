#[doc = "Register `ETH_MACVHTR` reader"]
pub struct R(crate::R<ETH_MACVHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVHTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACVHTR` writer"]
pub struct W(crate::W<ETH_MACVHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACVHTR_SPEC>;
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
impl From<crate::W<ETH_MACVHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACVHTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLHT` reader - VLHT"]
pub struct VLHT_R(crate::FieldReader<u16, u16>);
impl VLHT_R {
    pub(crate) fn new(bits: u16) -> Self {
        VLHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLHT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLHT` writer - VLHT"]
pub struct VLHT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W {
        VLHT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvhtr](index.html) module"]
pub struct ETH_MACVHTR_SPEC;
impl crate::RegisterSpec for ETH_MACVHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macvhtr::R](R) reader structure"]
impl crate::Readable for ETH_MACVHTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macvhtr::W](W) writer structure"]
impl crate::Writable for ETH_MACVHTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACVHTR to value 0"]
impl crate::Resettable for ETH_MACVHTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
