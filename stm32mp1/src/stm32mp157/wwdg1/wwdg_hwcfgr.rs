///Register `WWDG_HWCFGR` reader
pub type R = crate::R<WWDG_HWCFGRrs>;
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
        f.debug_struct("WWDG_HWCFGR")
            .field("prediv", &self.prediv())
            .finish()
    }
}
/**WWDG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#WWDG1:WWDG_HWCFGR)*/
pub struct WWDG_HWCFGRrs;
impl crate::RegisterSpec for WWDG_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_hwcfgr::R`](R) reader structure
impl crate::Readable for WWDG_HWCFGRrs {}
///`reset()` method sets WWDG_HWCFGR to value 0x0fff
impl crate::Resettable for WWDG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
