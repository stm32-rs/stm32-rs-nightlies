#[doc = "Register `FMC_PCSCNTR` reader"]
pub type R = crate::R<FMC_PCSCNTRrs>;
#[doc = "Register `FMC_PCSCNTR` writer"]
pub type W = crate::W<FMC_PCSCNTRrs>;
#[doc = "Field `CSCOUNT` reader - CSCOUNT"]
pub type CSCOUNT_R = crate::FieldReader<u16>;
#[doc = "Field `CSCOUNT` writer - CSCOUNT"]
pub type CSCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNTB1EN` reader - CNTB1EN"]
pub type CNTB1EN_R = crate::BitReader;
#[doc = "Field `CNTB1EN` writer - CNTB1EN"]
pub type CNTB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB2EN` reader - CNTB2EN"]
pub type CNTB2EN_R = crate::BitReader;
#[doc = "Field `CNTB2EN` writer - CNTB2EN"]
pub type CNTB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB3EN` reader - CNTB3EN"]
pub type CNTB3EN_R = crate::BitReader;
#[doc = "Field `CNTB3EN` writer - CNTB3EN"]
pub type CNTB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB4EN` reader - CNTB4EN"]
pub type CNTB4EN_R = crate::BitReader;
#[doc = "Field `CNTB4EN` writer - CNTB4EN"]
pub type CNTB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn cscount(&mut self) -> CSCOUNT_W<FMC_PCSCNTRrs> {
        CSCOUNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb1en(&mut self) -> CNTB1EN_W<FMC_PCSCNTRrs> {
        CNTB1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb2en(&mut self) -> CNTB2EN_W<FMC_PCSCNTRrs> {
        CNTB2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb3en(&mut self) -> CNTB3EN_W<FMC_PCSCNTRrs> {
        CNTB3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb4en(&mut self) -> CNTB4EN_W<FMC_PCSCNTRrs> {
        CNTB4EN_W::new(self, 19)
    }
}
#[doc = "This register contains the PSRAM chip select counter value for synchronous mode. The chip select counter is common to all banks and can be enabled separately on each bank. During PSRAM read or write accesses, this value is loaded into a timer which is decremented using the fmc_ker_ck while the NE signal is held low. When the timer reaches 0, the PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh and restarts a new access. The programmed counter value guarantees a maximum NE pulse width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts decrementing each time a new access is started by a transition of NE from high to low. h\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pcscntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pcscntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_PCSCNTRrs;
impl crate::RegisterSpec for FMC_PCSCNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_pcscntr::R`](R) reader structure"]
impl crate::Readable for FMC_PCSCNTRrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_pcscntr::W`](W) writer structure"]
impl crate::Writable for FMC_PCSCNTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_PCSCNTR to value 0"]
impl crate::Resettable for FMC_PCSCNTRrs {
    const RESET_VALUE: u32 = 0;
}
