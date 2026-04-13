///Register `WRP2R_PRG` reader
pub type R = crate::R<WRP2R_PRGrs>;
///Register `WRP2R_PRG` writer
pub type W = crate::W<WRP2R_PRGrs>;
///Field `WRPSG2` reader - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
pub type WRPSG2_R = crate::FieldReader<u32>;
///Field `WRPSG2` writer - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
pub type WRPSG2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2R_PRG")
            .field("wrpsg2", &self.wrpsg2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bank 2 sector group protection option status byte Setting WRPSGn2 bits to 0 write protects the corresponding group of four consecutive sectors in Bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
    #[inline(always)]
    pub fn wrpsg2(&mut self) -> WRPSG2_W<'_, WRP2R_PRGrs> {
        WRPSG2_W::new(self, 0)
    }
}
/**FLASH write sector group protection for Bank 2

You can [`read`](crate::Reg::read) this register and get [`wrp2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:WRP2R_PRG)*/
pub struct WRP2R_PRGrs;
impl crate::RegisterSpec for WRP2R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2r_prg::R`](R) reader structure
impl crate::Readable for WRP2R_PRGrs {}
///`write(|w| ..)` method takes [`wrp2r_prg::W`](W) writer structure
impl crate::Writable for WRP2R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP2R_PRG to value 0
impl crate::Resettable for WRP2R_PRGrs {}
