///Register `DMACTXDLAR` reader
pub type R = crate::R<DMACTXDLARrs>;
///Register `DMACTXDLAR` writer
pub type W = crate::W<DMACTXDLARrs>;
///Field `TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type TDESLA_R = crate::FieldReader<u32>;
///Field `TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTXDLAR")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W<'_, DMACTXDLARrs> {
        TDESLA_W::new(self, 0)
    }
}
/**Channel Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmactxdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#ETH:DMACTXDLAR)*/
pub struct DMACTXDLARrs;
impl crate::RegisterSpec for DMACTXDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmactxdlar::R`](R) reader structure
impl crate::Readable for DMACTXDLARrs {}
///`write(|w| ..)` method takes [`dmactxdlar::W`](W) writer structure
impl crate::Writable for DMACTXDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTXDLAR to value 0
impl crate::Resettable for DMACTXDLARrs {}
