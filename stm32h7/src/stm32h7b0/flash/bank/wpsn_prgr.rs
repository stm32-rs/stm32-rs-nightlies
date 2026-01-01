///Register `WPSN_PRGR` reader
pub type R = crate::R<WPSN_PRGRrs>;
///Register `WPSN_PRGR` writer
pub type W = crate::W<WPSN_PRGRrs>;
///Field `WRPSGn` reader - Bank 1 sector group protection option status byte Setting WRPSGn1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
pub type WRPSGN_R = crate::FieldReader<u32>;
///Field `WRPSGn` writer - Bank 1 sector group protection option status byte Setting WRPSGn1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
pub type WRPSGN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bank 1 sector group protection option status byte Setting WRPSGn1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
    #[inline(always)]
    pub fn wrpsgn(&self) -> WRPSGN_R {
        WRPSGN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_PRGR")
            .field("wrpsgn", &self.wrpsgn())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bank 1 sector group protection option status byte Setting WRPSGn1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
    #[inline(always)]
    pub fn wrpsgn(&mut self) -> WRPSGN_W<'_, WPSN_PRGRrs> {
        WRPSGN_W::new(self, 0)
    }
}
/**FLASH write sector group protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_prgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WPSN_PRGRrs;
impl crate::RegisterSpec for WPSN_PRGRrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_prgr::R`](R) reader structure
impl crate::Readable for WPSN_PRGRrs {}
///`write(|w| ..)` method takes [`wpsn_prgr::W`](W) writer structure
impl crate::Writable for WPSN_PRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPSN_PRGR to value 0
impl crate::Resettable for WPSN_PRGRrs {}
