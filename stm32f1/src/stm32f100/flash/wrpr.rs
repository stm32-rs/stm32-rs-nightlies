///Register `WRPR` reader
pub type R = crate::R<WRPRrs>;
///Field `WRP` reader - Write protect
pub type WRP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Write protect
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPR").field("wrp", &self.wrp()).finish()
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FLASH:WRPR)*/
pub struct WRPRrs;
impl crate::RegisterSpec for WRPRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpr::R`](R) reader structure
impl crate::Readable for WRPRrs {}
///`reset()` method sets WRPR to value 0xffff_ffff
impl crate::Resettable for WRPRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
