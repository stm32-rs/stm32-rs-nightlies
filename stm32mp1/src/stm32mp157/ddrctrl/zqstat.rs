///Register `ZQSTAT` reader
pub type R = crate::R<ZQSTATrs>;
///Field `ZQ_RESET_BUSY` reader - ZQ_RESET_BUSY
pub type ZQ_RESET_BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - ZQ_RESET_BUSY
    #[inline(always)]
    pub fn zq_reset_busy(&self) -> ZQ_RESET_BUSY_R {
        ZQ_RESET_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQSTAT")
            .field("zq_reset_busy", &self.zq_reset_busy())
            .finish()
    }
}
/**DDRCTRL ZQ status register

You can [`read`](crate::Reg::read) this register and get [`zqstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQSTAT)*/
pub struct ZQSTATrs;
impl crate::RegisterSpec for ZQSTATrs {
    type Ux = u32;
}
///`read()` method returns [`zqstat::R`](R) reader structure
impl crate::Readable for ZQSTATrs {}
///`reset()` method sets ZQSTAT to value 0
impl crate::Resettable for ZQSTATrs {}
