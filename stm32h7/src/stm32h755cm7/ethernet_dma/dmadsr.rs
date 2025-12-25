///Register `DMADSR` reader
pub type R = crate::R<DMADSRrs>;
///Register `DMADSR` writer
pub type W = crate::W<DMADSRrs>;
///Field `AXWHSTS` reader - AHB Master Write Channel
pub type AXWHSTS_R = crate::BitReader;
///Field `AXWHSTS` writer - AHB Master Write Channel
pub type AXWHSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPS0` reader - DMA Channel Receive Process State
pub type RPS0_R = crate::FieldReader;
///Field `RPS0` writer - DMA Channel Receive Process State
pub type RPS0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TPS0` reader - DMA Channel Transmit Process State
pub type TPS0_R = crate::FieldReader;
///Field `TPS0` writer - DMA Channel Transmit Process State
pub type TPS0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - DMA Channel Receive Process State
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMADSR")
            .field("tps0", &self.tps0())
            .field("rps0", &self.rps0())
            .field("axwhsts", &self.axwhsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    pub fn axwhsts(&mut self) -> AXWHSTS_W<'_, DMADSRrs> {
        AXWHSTS_W::new(self, 0)
    }
    ///Bits 8:11 - DMA Channel Receive Process State
    #[inline(always)]
    pub fn rps0(&mut self) -> RPS0_W<'_, DMADSRrs> {
        RPS0_W::new(self, 8)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State
    #[inline(always)]
    pub fn tps0(&mut self) -> TPS0_W<'_, DMADSRrs> {
        TPS0_W::new(self, 12)
    }
}
/**Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#Ethernet_DMA:DMADSR)*/
pub struct DMADSRrs;
impl crate::RegisterSpec for DMADSRrs {
    type Ux = u32;
}
///`read()` method returns [`dmadsr::R`](R) reader structure
impl crate::Readable for DMADSRrs {}
///`write(|w| ..)` method takes [`dmadsr::W`](W) writer structure
impl crate::Writable for DMADSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMADSR to value 0
impl crate::Resettable for DMADSRrs {}
