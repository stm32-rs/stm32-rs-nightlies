///Register `GPIOZ_HWCFGR4` reader
pub type R = crate::R<GPIOZ_HWCFGR4rs>;
///Field `OSPEED_RES` reader - OSPEED_RES
pub type OSPEED_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - OSPEED_RES
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_HWCFGR4")
            .field("ospeed_res", &self.ospeed_res())
            .finish()
    }
}
/**GPIO hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`gpioz_hwcfgr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_HWCFGR4)*/
pub struct GPIOZ_HWCFGR4rs;
impl crate::RegisterSpec for GPIOZ_HWCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_hwcfgr4::R`](R) reader structure
impl crate::Readable for GPIOZ_HWCFGR4rs {}
///`reset()` method sets GPIOZ_HWCFGR4 to value 0
impl crate::Resettable for GPIOZ_HWCFGR4rs {}
