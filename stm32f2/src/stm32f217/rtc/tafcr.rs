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
