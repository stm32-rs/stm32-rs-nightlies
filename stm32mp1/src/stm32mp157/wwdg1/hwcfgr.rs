///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `PREDIV` reader - PREDIV
pub type PREDIV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - PREDIV
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("prediv", &self.prediv())
            .finish()
    }
}
/**WWDG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#WWDG1:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x0fff
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
