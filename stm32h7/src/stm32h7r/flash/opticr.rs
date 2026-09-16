///Register `OPTICR` writer
pub type W = crate::W<OPTICRrs>;
///Field `KVEF` writer - key valid error flag Set this bit to clear KVEF flag in FLASH_OPTISR register.
pub type KVEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KTEF` writer - key transfer error flag Set this bit to clear KTEF flag in FLASH_OPTISR register.
pub type KTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTERRF` writer - Option byte change error flag Set this bit to clear OPTERRF flag in FLASH_OPTISR register.
pub type OPTERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OPTICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 27 - key valid error flag Set this bit to clear KVEF flag in FLASH_OPTISR register.
    #[inline(always)]
    pub fn kvef(&mut self) -> KVEF_W<'_, OPTICRrs> {
        KVEF_W::new(self, 27)
    }
    ///Bit 28 - key transfer error flag Set this bit to clear KTEF flag in FLASH_OPTISR register.
    #[inline(always)]
    pub fn ktef(&mut self) -> KTEF_W<'_, OPTICRrs> {
        KTEF_W::new(self, 28)
    }
    ///Bit 30 - Option byte change error flag Set this bit to clear OPTERRF flag in FLASH_OPTISR register.
    #[inline(always)]
    pub fn opterrf(&mut self) -> OPTERRF_W<'_, OPTICRrs> {
        OPTERRF_W::new(self, 30)
    }
}
/**FLASH options interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opticr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTICR)*/
pub struct OPTICRrs;
impl crate::RegisterSpec for OPTICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`opticr::W`](W) writer structure
impl crate::Writable for OPTICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTICR to value 0
impl crate::Resettable for OPTICRrs {}
