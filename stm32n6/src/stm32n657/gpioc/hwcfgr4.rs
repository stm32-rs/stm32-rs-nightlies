///Register `HWCFGR4` reader
pub type R = crate::R<HWCFGR4rs>;
///Field `OSPEED_RES` reader - OSPEED register reset value
pub type OSPEED_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - OSPEED register reset value
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR4")
            .field("ospeed_res", &self.ospeed_res())
            .finish()
    }
}
/**GPIO port C hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOC:HWCFGR4)*/
pub struct HWCFGR4rs;
impl crate::RegisterSpec for HWCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr4::R`](R) reader structure
impl crate::Readable for HWCFGR4rs {}
///`reset()` method sets HWCFGR4 to value 0
impl crate::Resettable for HWCFGR4rs {}
