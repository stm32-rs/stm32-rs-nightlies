#[doc = "Register `FMC_CSQISR` reader"]
pub type R = crate::R<FMC_CSQISRrs>;
#[doc = "Register `FMC_CSQISR` writer"]
pub type W = crate::W<FMC_CSQISRrs>;
#[doc = "Field `TCF` reader - TCF"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TCF` writer - TCF"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCF` reader - SCF"]
pub type SCF_R = crate::BitReader;
#[doc = "Field `SCF` writer - SCF"]
pub type SCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEF` reader - SEF"]
pub type SEF_R = crate::BitReader;
#[doc = "Field `SEF` writer - SEF"]
pub type SEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUEF` reader - SUEF"]
pub type SUEF_R = crate::BitReader;
#[doc = "Field `SUEF` writer - SUEF"]
pub type SUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTCF` reader - CMDTCF"]
pub type CMDTCF_R = crate::BitReader;
#[doc = "Field `CMDTCF` writer - CMDTCF"]
pub type CMDTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    pub fn suef(&self) -> SUEF_R {
        SUEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    pub fn cmdtcf(&self) -> CMDTCF_R {
        CMDTCF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<FMC_CSQISRrs> {
        TCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    #[must_use]
    pub fn scf(&mut self) -> SCF_W<FMC_CSQISRrs> {
        SCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SEF_W<FMC_CSQISRrs> {
        SEF_W::new(self, 2)
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    #[must_use]
    pub fn suef(&mut self) -> SUEF_W<FMC_CSQISRrs> {
        SUEF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtcf(&mut self) -> CMDTCF_W<FMC_CSQISRrs> {
        CMDTCF_W::new(self, 4)
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQISRrs;
impl crate::RegisterSpec for FMC_CSQISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqisr::R`](R) reader structure"]
impl crate::Readable for FMC_CSQISRrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqisr::W`](W) writer structure"]
impl crate::Writable for FMC_CSQISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQISR to value 0"]
impl crate::Resettable for FMC_CSQISRrs {
    const RESET_VALUE: u32 = 0;
}
