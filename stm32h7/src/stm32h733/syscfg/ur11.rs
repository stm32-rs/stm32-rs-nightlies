///Register `UR11` reader
pub type R = crate::R<UR11rs>;
///Field `IWDG1M` reader - Independent Watchdog 1 mode
pub type IWDG1M_R = crate::BitReader;
impl R {
    ///Bit 16 - Independent Watchdog 1 mode
    #[inline(always)]
    pub fn iwdg1m(&self) -> IWDG1M_R {
        IWDG1M_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR11")
            .field("iwdg1m", &self.iwdg1m())
            .finish()
    }
}
/**SYSCFG user register 11

You can [`read`](crate::Reg::read) this register and get [`ur11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SYSCFG:UR11)*/
pub struct UR11rs;
impl crate::RegisterSpec for UR11rs {
    type Ux = u32;
}
///`read()` method returns [`ur11::R`](R) reader structure
impl crate::Readable for UR11rs {}
///`reset()` method sets UR11 to value 0
impl crate::Resettable for UR11rs {}
