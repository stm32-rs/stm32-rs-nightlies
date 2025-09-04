///Register `DMAISR` reader
pub type R = crate::R<DMAISRrs>;
///Field `DC0IS` reader - DMA Channel Interrupt Status
pub type DC0IS_R = crate::BitReader;
///Field `DC1IS` reader - DC1IS
pub type DC1IS_R = crate::BitReader;
///Field `MTLIS` reader - MTL Interrupt Status
pub type MTLIS_R = crate::BitReader;
///Field `MACIS` reader - MAC Interrupt Status
pub type MACIS_R = crate::BitReader;
impl R {
    ///Bit 0 - DMA Channel Interrupt Status
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DC1IS
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - MTL Interrupt Status
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MAC Interrupt Status
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAISR")
            .field("dc0is", &self.dc0is())
            .field("dc1is", &self.dc1is())
            .field("mtlis", &self.mtlis())
            .field("macis", &self.macis())
            .finish()
    }
}
/**Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAISR)*/
pub struct DMAISRrs;
impl crate::RegisterSpec for DMAISRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaisr::R`](R) reader structure
impl crate::Readable for DMAISRrs {}
///`reset()` method sets DMAISR to value 0x8000
impl crate::Resettable for DMAISRrs {
    const RESET_VALUE: u32 = 0x8000;
}
