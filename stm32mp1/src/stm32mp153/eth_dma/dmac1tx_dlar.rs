///Register `DMAC1TxDLAR` reader
pub type R = crate::R<DMAC1TX_DLARrs>;
///Register `DMAC1TxDLAR` writer
pub type W = crate::W<DMAC1TX_DLARrs>;
///Field `TDESLA` reader - Start of Transmit List
pub type TDESLA_R = crate::FieldReader<u32>;
///Field `TDESLA` writer - Start of Transmit List
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1TxDLAR")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W<DMAC1TX_DLARrs> {
        TDESLA_W::new(self, 3)
    }
}
/**Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxDLAR)*/
pub struct DMAC1TX_DLARrs;
impl crate::RegisterSpec for DMAC1TX_DLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1tx_dlar::R`](R) reader structure
impl crate::Readable for DMAC1TX_DLARrs {}
///`write(|w| ..)` method takes [`dmac1tx_dlar::W`](W) writer structure
impl crate::Writable for DMAC1TX_DLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1TxDLAR to value 0
impl crate::Resettable for DMAC1TX_DLARrs {}
