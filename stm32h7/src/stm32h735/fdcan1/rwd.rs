///Register `RWD` reader
pub type R = crate::R<RWDrs>;
///Field `WDC` reader - Watchdog configuration
pub type WDC_R = crate::FieldReader;
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
        f.debug_struct("RWD")
            .field("wdv", &self.wdv())
            .field("wdc", &self.wdc())
            .finish()
    }
}
/**FDCAN RAM Watchdog Register

You can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:RWD)*/
pub struct RWDrs;
impl crate::RegisterSpec for RWDrs {
    type Ux = u32;
}
///`read()` method returns [`rwd::R`](R) reader structure
impl crate::Readable for RWDrs {}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWDrs {}
