///Register `HWCFGR7` reader
pub type R = crate::R<HWCFGR7rs>;
///Field `CPUEVENT` reader - HW configuration CPU event generation
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR7")
            .field("cpuevent", &self.cpuevent())
            .finish()
    }
}
/**EXTI Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR7)*/
pub struct HWCFGR7rs;
impl crate::RegisterSpec for HWCFGR7rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr7::R`](R) reader structure
impl crate::Readable for HWCFGR7rs {}
///`reset()` method sets HWCFGR7 to value 0
impl crate::Resettable for HWCFGR7rs {}
