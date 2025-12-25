///Register `CRCSADDR` reader
pub type R = crate::R<CRCSADDRrs>;
///Register `CRCSADDR` writer
pub type W = crate::W<CRCSADDRrs>;
///Field `CRC_START_ADDR` reader - CRC start address on bank 1 CRC_START_ADDR is used when CRC_BY_SECT is cleared to 0. It must be programmed to the start address of the bank 1 memory area on which the CRC calculation is performed.
pub type CRC_START_ADDR_R = crate::FieldReader<u32>;
///Field `CRC_START_ADDR` writer - CRC start address on bank 1 CRC_START_ADDR is used when CRC_BY_SECT is cleared to 0. It must be programmed to the start address of the bank 1 memory area on which the CRC calculation is performed.
pub type CRC_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 2:19 - CRC start address on bank 1 CRC_START_ADDR is used when CRC_BY_SECT is cleared to 0. It must be programmed to the start address of the bank 1 memory area on which the CRC calculation is performed.
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new((self.bits >> 2) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCSADDR")
            .field("crc_start_addr", &self.crc_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 2:19 - CRC start address on bank 1 CRC_START_ADDR is used when CRC_BY_SECT is cleared to 0. It must be programmed to the start address of the bank 1 memory area on which the CRC calculation is performed.
    #[inline(always)]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W<'_, CRCSADDRrs> {
        CRC_START_ADDR_W::new(self, 2)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`crcsaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCSADDRrs;
impl crate::RegisterSpec for CRCSADDRrs {
    type Ux = u32;
}
///`read()` method returns [`crcsaddr::R`](R) reader structure
impl crate::Readable for CRCSADDRrs {}
///`write(|w| ..)` method takes [`crcsaddr::W`](W) writer structure
impl crate::Writable for CRCSADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCSADDR to value 0
impl crate::Resettable for CRCSADDRrs {}
