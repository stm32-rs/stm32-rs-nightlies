#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge"]
pub type TSEDGE_R = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge"]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_R = crate::BitReader;
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers"]
pub type BYPSHAD_R = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers"]
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader;
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader;
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change)"]
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup"]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - Backup"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection"]
pub type COSEL_R = crate::BitReader;
#[doc = "Field `COSEL` writer - Calibration output selection"]
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Output polarity"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - Output polarity"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection"]
pub type OSEL_R = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection"]
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - Calibration output enable"]
pub type COE_R = crate::BitReader;
#[doc = "Field `COE` writer - Calibration output enable"]
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<CRrs> {
        FMT_W::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<CRrs> {
        ALRAE_W::new(self, 8)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<CRrs> {
        TSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<CRrs> {
        ALRAIE_W::new(self, 12)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<CRrs> {
        COE_W::new(self, 23)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
