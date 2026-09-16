///Register `MACHWF2R` reader
pub type R = crate::R<MACHWF2Rrs>;
///Field `RXQCNT` reader - Number of MTL Receive Queues
pub type RXQCNT_R = crate::FieldReader;
///Field `TXQCNT` reader - Number of MTL Transmit Queues
pub type TXQCNT_R = crate::FieldReader;
///Field `RXCHCNT` reader - Number of DMA Receive Channels
pub type RXCHCNT_R = crate::FieldReader;
///Field `TXCHCNT` reader - Number of DMA Transmit Channels
pub type TXCHCNT_R = crate::FieldReader;
///Field `PPSOUTNUM` reader - Number of PPS Outputs
pub type PPSOUTNUM_R = crate::FieldReader;
///Field `AUXSNAPNUM` reader - Number of Auxiliary Snapshot Inputs
pub type AUXSNAPNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Number of MTL Receive Queues
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - Number of MTL Transmit Queues
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - Number of DMA Receive Channels
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 18:21 - Number of DMA Transmit Channels
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 24:26 - Number of PPS Outputs
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Number of Auxiliary Snapshot Inputs
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF2R")
            .field("rxqcnt", &self.rxqcnt())
            .field("txqcnt", &self.txqcnt())
            .field("rxchcnt", &self.rxchcnt())
            .field("txchcnt", &self.txchcnt())
            .field("ppsoutnum", &self.ppsoutnum())
            .field("auxsnapnum", &self.auxsnapnum())
            .finish()
    }
}
/**HW feature 2 register

You can [`read`](crate::Reg::read) this register and get [`machwf2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#Ethernet_MAC:MACHWF2R)*/
pub struct MACHWF2Rrs;
impl crate::RegisterSpec for MACHWF2Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf2r::R`](R) reader structure
impl crate::Readable for MACHWF2Rrs {}
///`reset()` method sets MACHWF2R to value 0x4100_0000
impl crate::Resettable for MACHWF2Rrs {
    const RESET_VALUE: u32 = 0x4100_0000;
}
