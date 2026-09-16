///Register `HWCFGR5` reader
pub type R = crate::R<HWCFGR5rs>;
///Field `PUPDR_RES` reader - Pull-up/pull-down register reset value
pub type PUPDR_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Pull-up/pull-down register reset value
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR5")
            .field("pupdr_res", &self.pupdr_res())
            .finish()
    }
}
/**GPIO port F hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOF:HWCFGR5)*/
pub struct HWCFGR5rs;
impl crate::RegisterSpec for HWCFGR5rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr5::R`](R) reader structure
impl crate::Readable for HWCFGR5rs {}
///`reset()` method sets HWCFGR5 to value 0
impl crate::Resettable for HWCFGR5rs {}
