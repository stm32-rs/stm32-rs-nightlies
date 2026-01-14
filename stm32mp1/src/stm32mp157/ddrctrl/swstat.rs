///Register `SWSTAT` reader
pub type R = crate::R<SWSTATrs>;
///Field `SW_DONE_ACK` reader - SW_DONE_ACK
pub type SW_DONE_ACK_R = crate::BitReader;
impl R {
    ///Bit 0 - SW_DONE_ACK
    #[inline(always)]
    pub fn sw_done_ack(&self) -> SW_DONE_ACK_R {
        SW_DONE_ACK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWSTAT")
            .field("sw_done_ack", &self.sw_done_ack())
            .finish()
    }
}
/**DDRCTRL software register programming control status

You can [`read`](crate::Reg::read) this register and get [`swstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:SWSTAT)*/
pub struct SWSTATrs;
impl crate::RegisterSpec for SWSTATrs {
    type Ux = u32;
}
///`read()` method returns [`swstat::R`](R) reader structure
impl crate::Readable for SWSTATrs {}
///`reset()` method sets SWSTAT to value 0x01
impl crate::Resettable for SWSTATrs {
    const RESET_VALUE: u32 = 0x01;
}
