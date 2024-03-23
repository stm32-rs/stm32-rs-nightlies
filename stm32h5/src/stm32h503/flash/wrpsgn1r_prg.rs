#[doc = "Register `WRPSGN1R_PRG` reader"]
pub type R = crate::R<WRPSGN1R_PRGrs>;
#[doc = "Register `WRPSGN1R_PRG` writer"]
pub type W = crate::W<WRPSGN1R_PRGrs>;
#[doc = "Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
pub type WRPSG1_R = crate::FieldReader;
#[doc = "Field `WRPSG1` writer - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
pub type WRPSG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<WRPSGN1R_PRGrs> {
        WRPSG1_W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for Bank1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpsgn1r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPSGN1R_PRGrs;
impl crate::RegisterSpec for WRPSGN1R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpsgn1r_prg::R`](R) reader structure"]
impl crate::Readable for WRPSGN1R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`wrpsgn1r_prg::W`](W) writer structure"]
impl crate::Writable for WRPSGN1R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRPSGN1R_PRG to value 0"]
impl crate::Resettable for WRPSGN1R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
