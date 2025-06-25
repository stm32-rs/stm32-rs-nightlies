///Register `GPIOZ_HWCFGR1` reader
pub type R = crate::R<GPIOZ_HWCFGR1rs>;
///Field `AFRH_RES` reader - AFRH_RES
pub type AFRH_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - AFRH_RES
    #[inline(always)]
    pub fn afrh_res(&self) -> AFRH_RES_R {
        AFRH_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_HWCFGR1")
            .field("afrh_res", &self.afrh_res())
            .finish()
    }
}
/**GPIO hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`gpioz_hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_HWCFGR1)*/
pub struct GPIOZ_HWCFGR1rs;
impl crate::RegisterSpec for GPIOZ_HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_hwcfgr1::R`](R) reader structure
impl crate::Readable for GPIOZ_HWCFGR1rs {}
///`reset()` method sets GPIOZ_HWCFGR1 to value 0
impl crate::Resettable for GPIOZ_HWCFGR1rs {}
