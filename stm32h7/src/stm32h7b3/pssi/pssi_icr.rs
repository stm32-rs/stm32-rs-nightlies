///Register `PSSI_ICR` writer
pub type W = crate::W<PSSI_ICRrs>;
///Field `OVR_ISC` writer - Data buffer overrun/underrun interrupt status clear Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS.
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PSSI_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Data buffer overrun/underrun interrupt status clear Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS.
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<PSSI_ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
}
/**PSSI interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_ICR)*/
pub struct PSSI_ICRrs;
impl crate::RegisterSpec for PSSI_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pssi_icr::W`](W) writer structure
impl crate::Writable for PSSI_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSSI_ICR to value 0
impl crate::Resettable for PSSI_ICRrs {
    const RESET_VALUE: u32 = 0;
}
