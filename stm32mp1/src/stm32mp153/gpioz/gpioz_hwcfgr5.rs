///Register `GPIOZ_HWCFGR5` reader
pub type R = crate::R<GPIOZ_HWCFGR5rs>;
///Field `PUPDR_RES` reader - PUPDR_RES
pub type PUPDR_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PUPDR_RES
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_HWCFGR5")
            .field("pupdr_res", &self.pupdr_res())
            .finish()
    }
}
/**GPIO hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`gpioz_hwcfgr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_HWCFGR5)*/
pub struct GPIOZ_HWCFGR5rs;
impl crate::RegisterSpec for GPIOZ_HWCFGR5rs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_hwcfgr5::R`](R) reader structure
impl crate::Readable for GPIOZ_HWCFGR5rs {}
///`reset()` method sets GPIOZ_HWCFGR5 to value 0
impl crate::Resettable for GPIOZ_HWCFGR5rs {}
