///Register `DDRCTRL_SWSTAT` reader
pub type R = crate::R<DDRCTRL_SWSTATrs>;
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
        f.debug_struct("DDRCTRL_SWSTAT")
            .field("sw_done_ack", &self.sw_done_ack())
            .finish()
    }
}
/**DDRCTRL software register programming control status

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_swstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_SWSTAT)*/
pub struct DDRCTRL_SWSTATrs;
impl crate::RegisterSpec for DDRCTRL_SWSTATrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_swstat::R`](R) reader structure
impl crate::Readable for DDRCTRL_SWSTATrs {}
///`reset()` method sets DDRCTRL_SWSTAT to value 0x01
impl crate::Resettable for DDRCTRL_SWSTATrs {
    const RESET_VALUE: u32 = 0x01;
}
