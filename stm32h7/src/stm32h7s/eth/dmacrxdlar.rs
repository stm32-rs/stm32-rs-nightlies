///Register `DMACRXDLAR` reader
pub type R = crate::R<DMACRXDLARrs>;
///Register `DMACRXDLAR` writer
pub type W = crate::W<DMACRXDLARrs>;
///Field `RDESLA` reader - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type RDESLA_R = crate::FieldReader<u32>;
///Field `RDESLA` writer - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRXDLAR")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list. The DMA ignores the LSB bits (1:0) for 32-bit bus width and internally takes these bits as all-zero. Therefore, these LSB bits are read-only (RO).
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W<'_, DMACRXDLARrs> {
        RDESLA_W::new(self, 0)
    }
}
/**Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmacrxdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:DMACRXDLAR)*/
pub struct DMACRXDLARrs;
impl crate::RegisterSpec for DMACRXDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrxdlar::R`](R) reader structure
impl crate::Readable for DMACRXDLARrs {}
///`write(|w| ..)` method takes [`dmacrxdlar::W`](W) writer structure
impl crate::Writable for DMACRXDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRXDLAR to value 0
impl crate::Resettable for DMACRXDLARrs {}
