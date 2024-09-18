///Register `IPCC_HWCFGR` reader
pub type R = crate::R<IPCC_HWCFGRrs>;
///Field `CHANNELS` reader - CHANNELS
pub type CHANNELS_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - CHANNELS
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC_HWCFGR")
            .field("channels", &self.channels())
            .finish()
    }
}
/**IPCC Hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`ipcc_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IPCC:IPCC_HWCFGR)*/
pub struct IPCC_HWCFGRrs;
impl crate::RegisterSpec for IPCC_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_hwcfgr::R`](R) reader structure
impl crate::Readable for IPCC_HWCFGRrs {}
///`reset()` method sets IPCC_HWCFGR to value 0x02
impl crate::Resettable for IPCC_HWCFGRrs {
    const RESET_VALUE: u32 = 0x02;
}
