///Register `FDCAN_RWD` reader
pub type R = crate::R<FDCAN_RWDrs>;
///Register `FDCAN_RWD` writer
pub type W = crate::W<FDCAN_RWDrs>;
///Field `WDC` reader - Watchdog configuration
pub type WDC_R = crate::FieldReader;
///Field `WDC` writer - Watchdog configuration
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WDV` reader - Watchdog value
pub type WDV_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Watchdog value
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RWD")
            .field("wdv", &self.wdv())
            .field("wdc", &self.wdc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<FDCAN_RWDrs> {
        WDC_W::new(self, 0)
    }
}
/**FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FDCAN1_RAM:FDCAN_RWD)*/
pub struct FDCAN_RWDrs;
impl crate::RegisterSpec for FDCAN_RWDrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rwd::R`](R) reader structure
impl crate::Readable for FDCAN_RWDrs {}
///`write(|w| ..)` method takes [`fdcan_rwd::W`](W) writer structure
impl crate::Writable for FDCAN_RWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_RWD to value 0
impl crate::Resettable for FDCAN_RWDrs {
    const RESET_VALUE: u32 = 0;
}
