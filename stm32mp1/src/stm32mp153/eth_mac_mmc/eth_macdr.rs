///Register `ETH_MACDR` reader
pub type R = crate::R<ETH_MACDRrs>;
///Field `RPESTS` reader - RPESTS
pub type RPESTS_R = crate::BitReader;
///Field `RFCFCSTS` reader - RFCFCSTS
pub type RFCFCSTS_R = crate::FieldReader;
///Field `TPESTS` reader - TPESTS
pub type TPESTS_R = crate::BitReader;
///Field `TFCSTS` reader - TFCSTS
pub type TFCSTS_R = crate::FieldReader;
impl R {
    ///Bit 0 - RPESTS
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - RFCFCSTS
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 16 - TPESTS
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - TFCSTS
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACDR")
            .field("rpests", &self.rpests())
            .field("rfcfcsts", &self.rfcfcsts())
            .field("tpests", &self.tpests())
            .field("tfcsts", &self.tfcsts())
            .finish()
    }
}
/**The Debug register provides the debug status of various MAC blocks.

You can [`read`](crate::Reg::read) this register and get [`eth_macdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACDR)*/
pub struct ETH_MACDRrs;
impl crate::RegisterSpec for ETH_MACDRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macdr::R`](R) reader structure
impl crate::Readable for ETH_MACDRrs {}
///`reset()` method sets ETH_MACDR to value 0
impl crate::Resettable for ETH_MACDRrs {
    const RESET_VALUE: u32 = 0;
}
