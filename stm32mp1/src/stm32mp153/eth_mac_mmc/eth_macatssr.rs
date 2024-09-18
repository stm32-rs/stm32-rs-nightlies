///Register `ETH_MACATSSR` reader
pub type R = crate::R<ETH_MACATSSRrs>;
///Field `AUXTSHI` reader - AUXTSHI
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - AUXTSHI
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACATSSR")
            .field("auxtshi", &self.auxtshi())
            .finish()
    }
}
/**The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.

You can [`read`](crate::Reg::read) this register and get [`eth_macatssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACATSSR)*/
pub struct ETH_MACATSSRrs;
impl crate::RegisterSpec for ETH_MACATSSRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macatssr::R`](R) reader structure
impl crate::Readable for ETH_MACATSSRrs {}
///`reset()` method sets ETH_MACATSSR to value 0
impl crate::Resettable for ETH_MACATSSRrs {
    const RESET_VALUE: u32 = 0;
}
