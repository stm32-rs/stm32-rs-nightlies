#[doc = "Register `DMADSR` reader"]
pub type R = crate::R<DMADSRrs>;
#[doc = "Field `AXWHSTS` reader - AHB Master Write Channel When high, this bit indicates that the write channel of the AHB master FMSs are in non-idle state."]
pub type AXWHSTS_R = crate::BitReader;
#[doc = "Field `RPS0` reader - DMA Channel Receive Process State This field indicates the Rx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt."]
pub type RPS0_R = crate::FieldReader;
#[doc = "Field `TPS0` reader - DMA Channel Transmit Process State This field indicates the Tx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt."]
pub type TPS0_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel When high, this bit indicates that the write channel of the AHB master FMSs are in non-idle state."]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State This field indicates the Rx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State This field indicates the Tx DMA FSM state for Channel: The MSB of this field always returns 0. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Debug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMADSRrs;
impl crate::RegisterSpec for DMADSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadsr::R`](R) reader structure"]
impl crate::Readable for DMADSRrs {}
#[doc = "`reset()` method sets DMADSR to value 0"]
impl crate::Resettable for DMADSRrs {
    const RESET_VALUE: u32 = 0;
}
