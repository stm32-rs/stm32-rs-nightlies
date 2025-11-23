///Register `WRP1R_PRG` reader
pub type R = crate::R<WRP1R_PRGrs>;
///Register `WRP1R_PRG` writer
pub type W = crate::W<WRP1R_PRGrs>;
///Field `WRPSG1` reader - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
pub type WRPSG1_R = crate::FieldReader<u32>;
///Field `WRPSG1` writer - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
pub type WRPSG1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1R_PRG")
            .field("wrpsg1", &self.wrpsg1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127
    #[inline(always)]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<'_, WRP1R_PRGrs> {
        WRPSG1_W::new(self, 0)
    }
}
/**FLASH write sector group protection for Bank 1

You can [`read`](crate::Reg::read) this register and get [`wrp1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:WRP1R_PRG)*/
pub struct WRP1R_PRGrs;
impl crate::RegisterSpec for WRP1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1r_prg::R`](R) reader structure
impl crate::Readable for WRP1R_PRGrs {}
///`write(|w| ..)` method takes [`wrp1r_prg::W`](W) writer structure
impl crate::Writable for WRP1R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP1R_PRG to value 0
impl crate::Resettable for WRP1R_PRGrs {}
