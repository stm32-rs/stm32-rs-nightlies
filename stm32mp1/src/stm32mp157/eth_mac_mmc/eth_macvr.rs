///Register `ETH_MACVR` reader
pub type R = crate::R<ETH_MACVRrs>;
///Field `SNPSVER` reader - SNPSVER
pub type SNPSVER_R = crate::FieldReader;
///Field `USERVER` reader - USERVER
pub type USERVER_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - SNPSVER
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - USERVER
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACVR")
            .field("snpsver", &self.snpsver())
            .field("userver", &self.userver())
            .finish()
    }
}
/**The version register identifies the version of the Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`eth_macvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACVR)*/
pub struct ETH_MACVRrs;
impl crate::RegisterSpec for ETH_MACVRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macvr::R`](R) reader structure
impl crate::Readable for ETH_MACVRrs {}
///`reset()` method sets ETH_MACVR to value 0x4042
impl crate::Resettable for ETH_MACVRrs {
    const RESET_VALUE: u32 = 0x4042;
}
