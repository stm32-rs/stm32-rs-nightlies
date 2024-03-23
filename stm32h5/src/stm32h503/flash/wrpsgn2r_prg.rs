#[doc = "Register `WRPSGN2R_PRG` reader"]
pub type R = crate::R<WRPSGN2R_PRGrs>;
#[doc = "Register `WRPSGN2R_PRG` writer"]
pub type W = crate::W<WRPSGN2R_PRGrs>;
#[doc = "Field `WRPSG2` reader - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
pub type WRPSG2_R = crate::FieldReader;
#[doc = "Field `WRPSG2` writer - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
pub type WRPSG2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsg2(&mut self) -> WRPSG2_W<WRPSGN2R_PRGrs> {
        WRPSG2_W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for Bank2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn2r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpsgn2r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPSGN2R_PRGrs;
impl crate::RegisterSpec for WRPSGN2R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpsgn2r_prg::R`](R) reader structure"]
impl crate::Readable for WRPSGN2R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`wrpsgn2r_prg::W`](W) writer structure"]
impl crate::Writable for WRPSGN2R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPSGN2R_PRG to value 0"]
impl crate::Resettable for WRPSGN2R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
