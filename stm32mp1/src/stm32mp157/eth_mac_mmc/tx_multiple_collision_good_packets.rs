#[doc = "Register `TX_MULTIPLE_COLLISION_GOOD_PACKETS` reader"]
pub struct R(crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXMULTCOLG` reader - TXMULTCOLG"]
pub struct TXMULTCOLG_R(crate::FieldReader<u32, u32>);
impl TXMULTCOLG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXMULTCOLG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMULTCOLG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TXMULTCOLG"]
    #[inline(always)]
    pub fn txmultcolg(&self) -> TXMULTCOLG_R {
        TXMULTCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after multiple collisions in the half-duplex mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_multiple_collision_good_packets](index.html) module"]
pub struct TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC;
impl crate::RegisterSpec for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_multiple_collision_good_packets::R](R) reader structure"]
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_MULTIPLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
