#[doc = "Register `TAMP_FLTCR` reader"]
pub type R = crate::R<TAMP_FLTCRrs>;
#[doc = "Register `TAMP_FLTCR` writer"]
pub type W = crate::W<TAMP_FLTCRrs>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TAMPFREQ_R = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TAMPFLT_R = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TAMPPRCH_R = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TAMPPUDIS_R = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<TAMP_FLTCRrs> {
        TAMPFREQ_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<TAMP_FLTCRrs> {
        TAMPFLT_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<TAMP_FLTCRrs> {
        TAMPPRCH_W::new(self, 5)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<TAMP_FLTCRrs> {
        TAMPPUDIS_W::new(self, 7)
    }
}
#[doc = "TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_fltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_fltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_FLTCRrs;
impl crate::RegisterSpec for TAMP_FLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_fltcr::R`](R) reader structure"]
impl crate::Readable for TAMP_FLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`tamp_fltcr::W`](W) writer structure"]
impl crate::Writable for TAMP_FLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_FLTCR to value 0"]
impl crate::Resettable for TAMP_FLTCRrs {
    const RESET_VALUE: u32 = 0;
}
