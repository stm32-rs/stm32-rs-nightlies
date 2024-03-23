#[doc = "Register `TAFCR` reader"]
pub type R = crate::R<TAFCRrs>;
#[doc = "Register `TAFCR` writer"]
pub type W = crate::W<TAFCRrs>;
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub type TAMP1E_R = crate::BitReader;
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_R = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub type TAMP2E_R = crate::BitReader;
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2_TRG` reader - Active level for RTC_TAMP2 input"]
pub type TAMP2_TRG_R = crate::BitReader;
#[doc = "Field `TAMP2_TRG` writer - Active level for RTC_TAMP2 input"]
pub type TAMP2_TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub type TAMP3E_R = crate::BitReader;
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_R = crate::BitReader;
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count"]
pub type TAMPFLT_R = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count"]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMP_PRCH` reader - RTC_TAMPx precharge duration"]
pub type TAMP_PRCH_R = crate::FieldReader;
#[doc = "Field `TAMP_PRCH` writer - RTC_TAMPx precharge duration"]
pub type TAMP_PRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMP_PUDIS` reader - RTC_TAMPx pull-up disable"]
pub type TAMP_PUDIS_R = crate::BitReader;
#[doc = "Field `TAMP_PUDIS` writer - RTC_TAMPx pull-up disable"]
pub type TAMP_PUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13VALUE` reader - RTC_ALARM output type/PC13 value"]
pub type PC13VALUE_R = crate::BitReader;
#[doc = "Field `PC13VALUE` writer - RTC_ALARM output type/PC13 value"]
pub type PC13VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC13MODE` reader - PC13 mode"]
pub type PC13MODE_R = crate::BitReader;
#[doc = "Field `PC13MODE` writer - PC13 mode"]
pub type PC13MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14VALUE` reader - PC14 value"]
pub type PC14VALUE_R = crate::BitReader;
#[doc = "Field `PC14VALUE` writer - PC14 value"]
pub type PC14VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14MODE` reader - PC14 mode"]
pub type PC14MODE_R = crate::BitReader;
#[doc = "Field `PC14MODE` writer - PC14 mode"]
pub type PC14MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15VALUE` reader - PC15 value"]
pub type PC15VALUE_R = crate::BitReader;
#[doc = "Field `PC15VALUE` writer - PC15 value"]
pub type PC15VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15MODE` reader - PC15 mode"]
pub type PC15MODE_R = crate::BitReader;
#[doc = "Field `PC15MODE` writer - PC15 mode"]
pub type PC15MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2_trg(&self) -> TAMP2_TRG_R {
        TAMP2_TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tamp_prch(&self) -> TAMP_PRCH_R {
        TAMP_PRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamp_pudis(&self) -> TAMP_PUDIS_R {
        TAMP_PUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13value(&self) -> PC13VALUE_R {
        PC13VALUE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&self) -> PC13MODE_R {
        PC13MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&self) -> PC14VALUE_R {
        PC14VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mode(&self) -> PC14MODE_R {
        PC14MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&self) -> PC15VALUE_R {
        PC15VALUE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&self) -> PC15MODE_R {
        PC15MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<TAFCRrs> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
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
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<TAFCRrs> {
        TAMP2E_W::new(self, 3)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2_trg(&mut self) -> TAMP2_TRG_W<TAFCRrs> {
        TAMP2_TRG_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3e(&mut self) -> TAMP3E_W<TAFCRrs> {
        TAMP3E_W::new(self, 5)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<TAFCRrs> {
        TAMP3TRG_W::new(self, 6)
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
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<TAFCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    #[must_use]
    pub fn tamp_prch(&mut self) -> TAMP_PRCH_W<TAFCRrs> {
        TAMP_PRCH_W::new(self, 13)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp_pudis(&mut self) -> TAMP_PUDIS_W<TAFCRrs> {
        TAMP_PUDIS_W::new(self, 15)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc13value(&mut self) -> PC13VALUE_W<TAFCRrs> {
        PC13VALUE_W::new(self, 18)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc13mode(&mut self) -> PC13MODE_W<TAFCRrs> {
        PC13MODE_W::new(self, 19)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc14value(&mut self) -> PC14VALUE_W<TAFCRrs> {
        PC14VALUE_W::new(self, 20)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc14mode(&mut self) -> PC14MODE_W<TAFCRrs> {
        PC14MODE_W::new(self, 21)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    #[must_use]
    pub fn pc15value(&mut self) -> PC15VALUE_W<TAFCRrs> {
        PC15VALUE_W::new(self, 22)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pc15mode(&mut self) -> PC15MODE_W<TAFCRrs> {
        PC15MODE_W::new(self, 23)
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
