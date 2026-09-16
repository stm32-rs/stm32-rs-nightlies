///Register `DMACCR` reader
pub type R = crate::R<DMACCRrs>;
///Register `DMACCR` writer
pub type W = crate::W<DMACCRrs>;
///Field `MSS` reader - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
pub type MSS_R = crate::FieldReader<u16>;
///Field `MSS` writer - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
pub type MSS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `PBLX8` reader - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\] in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
pub type PBLX8_R = crate::BitReader;
///Field `PBLX8` writer - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\] in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSL` reader - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:13 - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\] in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:20 - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCR")
            .field("mss", &self.mss())
            .field("pblx8", &self.pblx8())
            .field("dsl", &self.dsl())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Maximum Segment Size This field specifies the maximum segment size that should be used while segmenting the packet. This field is valid only if the TSE bit of Channel transmit control register (ETH_DMACTXCR) is set. The value programmed in this field must be more than the configured Data width in bytes. It is recommended to use a MSS value of 64 bytes or more.
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W<'_, DMACCRrs> {
        MSS_W::new(self, 0)
    }
    ///Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\] in Channel transmit control register (ETH_DMACTXCR) is multiplied eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value.
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<'_, DMACCRrs> {
        PBLX8_W::new(self, 16)
    }
    ///Bits 18:20 - Descriptor Skip Length This bit specifies the 32-bit word number to skip between two unchained descriptors. The address skipping starts from the end of the current descriptor to the start of the next descriptor. When the DSL value is equal to zero, the DMA takes the descriptor table as contiguous.
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, DMACCRrs> {
        DSL_W::new(self, 18)
    }
}
/**Channel control register

You can [`read`](crate::Reg::read) this register and get [`dmaccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:DMACCR)*/
pub struct DMACCRrs;
impl crate::RegisterSpec for DMACCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccr::R`](R) reader structure
impl crate::Readable for DMACCRrs {}
///`write(|w| ..)` method takes [`dmaccr::W`](W) writer structure
impl crate::Writable for DMACCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACCR to value 0
impl crate::Resettable for DMACCRrs {}
