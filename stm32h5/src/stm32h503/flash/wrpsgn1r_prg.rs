///Register `WRPSGN1R_PRG` reader
pub type R = crate::R<WRPSGN1R_PRGrs>;
///Register `WRPSGN1R_PRG` writer
pub type W = crate::W<WRPSGN1R_PRGrs>;
///Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_R = crate::FieldReader;
///Field `WRPSG1` writer - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
pub type WRPSG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPSGN1R_PRG")
            .field("wrpsg1", &self.wrpsg1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)
    #[inline(always)]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<'_, WRPSGN1R_PRGrs> {
        WRPSG1_W::new(self, 0)
    }
}
/**FLASH write sector protection for Bank1

You can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:WRPSGN1R_PRG)*/
pub struct WRPSGN1R_PRGrs;
impl crate::RegisterSpec for WRPSGN1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`wrpsgn1r_prg::R`](R) reader structure
impl crate::Readable for WRPSGN1R_PRGrs {}
///`write(|w| ..)` method takes [`wrpsgn1r_prg::W`](W) writer structure
impl crate::Writable for WRPSGN1R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPSGN1R_PRG to value 0
impl crate::Resettable for WRPSGN1R_PRGrs {}
