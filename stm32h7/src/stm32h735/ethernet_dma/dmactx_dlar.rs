///Register `DMACTxDLAR` reader
pub type R = crate::R<DMACTX_DLARrs>;
///Register `DMACTxDLAR` writer
pub type W = crate::W<DMACTX_DLARrs>;
///Field `TDESLA` reader - Start of Transmit List
pub type TDESLA_R = crate::FieldReader<u32>;
///Field `TDESLA` writer - Start of Transmit List
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTxDLAR")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Start of Transmit List
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W<'_, DMACTX_DLARrs> {
        TDESLA_W::new(self, 2)
    }
}
/**Channel Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmactx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_DMA:DMACTxDLAR)*/
pub struct DMACTX_DLARrs;
impl crate::RegisterSpec for DMACTX_DLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmactx_dlar::R`](R) reader structure
impl crate::Readable for DMACTX_DLARrs {}
///`write(|w| ..)` method takes [`dmactx_dlar::W`](W) writer structure
impl crate::Writable for DMACTX_DLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTxDLAR to value 0
impl crate::Resettable for DMACTX_DLARrs {}
