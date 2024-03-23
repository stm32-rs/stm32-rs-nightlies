#[doc = "Register `ETH_MACDR` reader"]
pub type R = crate::R<ETH_MACDRrs>;
#[doc = "Field `RPESTS` reader - RPESTS"]
pub type RPESTS_R = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - RFCFCSTS"]
pub type RFCFCSTS_R = crate::FieldReader;
#[doc = "Field `TPESTS` reader - TPESTS"]
pub type TPESTS_R = crate::BitReader;
#[doc = "Field `TFCSTS` reader - TFCSTS"]
pub type TFCSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RPESTS"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RFCFCSTS"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - TPESTS"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - TFCSTS"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[doc = "The Debug register provides the debug status of various MAC blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACDRrs;
impl crate::RegisterSpec for ETH_MACDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macdr::R`](R) reader structure"]
impl crate::Readable for ETH_MACDRrs {}
#[doc = "`reset()` method sets ETH_MACDR to value 0"]
impl crate::Resettable for ETH_MACDRrs {
    const RESET_VALUE: u32 = 0;
}
