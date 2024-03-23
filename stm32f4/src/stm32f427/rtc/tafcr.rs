#[doc = "Register `TAFCR` reader"]
pub type R = crate::R<TAFCRrs>;
#[doc = "Register `TAFCR` writer"]
pub type W = crate::W<TAFCRrs>;
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type TAMP1E_R = crate::BitReader;
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type TAMP1TRG_R = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - Tamper 2 detection enable"]
pub type TAMP2E_R = crate::BitReader;
#[doc = "Field `TAMP2E` writer - Tamper 2 detection enable"]
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2"]
pub type TAMP2TRG_R = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2"]
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - Tamper filter count"]
pub type TAMPFLT_R = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - Tamper filter count"]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - Tamper precharge duration"]
pub type TAMPPRCH_R = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - Tamper precharge duration"]
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - TAMPER pull-up disable"]
pub type TAMPPUDIS_R = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - TAMPER pull-up disable"]
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1INSEL` reader - TAMPER1 mapping"]
pub type TAMP1INSEL_R = crate::BitReader;
#[doc = "Field `TAMP1INSEL` writer - TAMPER1 mapping"]
pub type TAMP1INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINSEL` reader - TIMESTAMP mapping"]
pub type TSINSEL_R = crate::BitReader;
#[doc = "Field `TSINSEL` writer - TIMESTAMP mapping"]
pub type TSINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMOUTTYPE` reader - AFO_ALARM output type"]
pub type ALARMOUTTYPE_R = crate::BitReader;
#[doc = "Field `ALARMOUTTYPE` writer - AFO_ALARM output type"]
pub type ALARMOUTTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<TAFCRrs> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<TAFCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<TAFCRrs> {
        TAMPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<TAFCRrs> {
        TAMP2E_W::new(self, 3)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<TAFCRrs> {
        TAMP2TRG_W::new(self, 4)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<TAFCRrs> {
        TAMPTS_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<TAFCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<TAFCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<TAFCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<TAFCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1insel(&mut self) -> TAMP1INSEL_W<TAFCRrs> {
        TAMP1INSEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    #[must_use]
    pub fn tsinsel(&mut self) -> TSINSEL_W<TAFCRrs> {
        TSINSEL_W::new(self, 17)
    }
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    #[must_use]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<TAFCRrs> {
        ALARMOUTTYPE_W::new(self, 18)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tafcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tafcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAFCRrs;
impl crate::RegisterSpec for TAFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tafcr::R`](R) reader structure"]
impl crate::Readable for TAFCRrs {}
#[doc = "`write(|w| ..)` method takes [`tafcr::W`](W) writer structure"]
impl crate::Writable for TAFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCRrs {
    const RESET_VALUE: u32 = 0;
}
