#[doc = "Register `SDMMC_IDMABASE1R` reader"]
pub type R = crate::R<SDMMC_IDMABASE1Rrs>;
#[doc = "Register `SDMMC_IDMABASE1R` writer"]
pub type W = crate::W<SDMMC_IDMABASE1Rrs>;
#[doc = "Field `IDMABASE1` reader - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type IDMABASE1_R = crate::FieldReader<u32>;
#[doc = "Field `IDMABASE1` writer - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
pub type IDMABASE1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    pub fn idmabase1(&self) -> IDMABASE1_R {
        IDMABASE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only). This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1) and memory buffer 1 is inactive (IDMABACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabase1(&mut self) -> IDMABASE1_W<SDMMC_IDMABASE1Rrs> {
        IDMABASE1_W::new(self, 0)
    }
}
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabase1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabase1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDMABASE1Rrs;
impl crate::RegisterSpec for SDMMC_IDMABASE1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idmabase1r::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDMABASE1Rrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idmabase1r::W`](W) writer structure"]
impl crate::Writable for SDMMC_IDMABASE1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMABASE1R to value 0"]
impl crate::Resettable for SDMMC_IDMABASE1Rrs {
    const RESET_VALUE: u32 = 0;
}
