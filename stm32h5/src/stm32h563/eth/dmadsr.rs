///Register `DMADSR` reader
pub type R = crate::R<DMADSRrs>;
///Field `AXWHSTS` reader - AHB Master Write Channel When high, this bit indicates that the write channel of the AHB master FMSs are in non-idle state.
pub type AXWHSTS_R = crate::BitReader;
///Field `RPS0` reader - DMA Channel Receive Process State This field indicates the Rx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt.
pub type RPS0_R = crate::FieldReader;
///Field `TPS0` reader - DMA Channel Transmit Process State This field indicates the Tx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt.
pub type TPS0_R = crate::FieldReader;
impl R {
    ///Bit 0 - AHB Master Write Channel When high, this bit indicates that the write channel of the AHB master FMSs are in non-idle state.
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - DMA Channel Receive Process State This field indicates the Rx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt.
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State This field indicates the Tx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt.
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMADSR")
            .field("axwhsts", &self.axwhsts())
            .field("rps0", &self.rps0())
            .field("tps0", &self.tps0())
            .finish()
    }
}
/**Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:DMADSR)*/
pub struct DMADSRrs;
impl crate::RegisterSpec for DMADSRrs {
    type Ux = u32;
}
///`read()` method returns [`dmadsr::R`](R) reader structure
impl crate::Readable for DMADSRrs {}
///`reset()` method sets DMADSR to value 0
impl crate::Resettable for DMADSRrs {}
