#[doc = "Register `WRP1R_PRG` reader"]
pub type R = crate::R<WRP1R_PRGrs>;
#[doc = "Register `WRP1R_PRG` writer"]
pub type W = crate::W<WRP1R_PRGrs>;
#[doc = "Field `WRPSG1` reader - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG1_R = crate::FieldReader<u32>;
#[doc = "Field `WRPSG1` writer - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank1 sector group protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsg1(&mut self) -> WRPSG1_W<WRP1R_PRGrs> {
        WRPSG1_W::new(self, 0)
    }
}
#[doc = "FLASH write sector group protection for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp1r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1R_PRGrs;
impl crate::RegisterSpec for WRP1R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1r_prg::R`](R) reader structure"]
impl crate::Readable for WRP1R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`wrp1r_prg::W`](W) writer structure"]
impl crate::Writable for WRP1R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP1R_PRG to value 0"]
impl crate::Resettable for WRP1R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
