///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `AFRL_RES` reader - AF register low reset value
pub type AFRL_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - AF register low reset value
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("afrl_res", &self.afrl_res())
            .finish()
    }
}
/**GPIO port N hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPION:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0
impl crate::Resettable for HWCFGR2rs {}
