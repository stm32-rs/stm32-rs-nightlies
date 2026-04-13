///Register `WRPSGN2R_PRG` reader
pub type R = crate::R<WRPSGN2R_PRGrs>;
///Register `WRPSGN2R_PRG` writer
pub type W = crate::W<WRPSGN2R_PRGrs>;
///Field `WRPSG2` reader - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
pub type WRPSG2_R = crate::FieldReader;
///Field `WRPSG2` writer - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
pub type WRPSG2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSGN2R_PRG")
            .field("wrpsg2", &self.wrpsg2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg2(&mut self) -> WRPSG2_W<'_, WRPSGN2R_PRGrs> {
        WRPSG2_W::new(self, 0)
    }
}
/**FLASH write sector protection for Bank2

You can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN2R_PRG)*/
pub struct WRPSGN2R_PRGrs;
impl crate::RegisterSpec for WRPSGN2R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsgn2r_prg::R`](R) reader structure
impl crate::Readable for WRPSGN2R_PRGrs {}
///`write(|w| ..)` method takes [`wrpsgn2r_prg::W`](W) writer structure
impl crate::Writable for WRPSGN2R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPSGN2R_PRG to value 0
impl crate::Resettable for WRPSGN2R_PRGrs {}
