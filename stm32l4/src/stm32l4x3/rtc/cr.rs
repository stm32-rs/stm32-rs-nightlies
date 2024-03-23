#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection"]
pub type WUCKSEL_R = crate::FieldReader;
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection"]
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge"]
pub type TSEDGE_R = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge"]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_R = crate::BitReader;
#[doc = "Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)"]
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
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader;
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub type WUTE_R = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type ALRBIE_R = crate::BitReader;
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WUTIE_R = crate::BitReader;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` reader - Add 1 hour (summer time change)"]
pub type ADD1H_R = crate::BitReader;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` reader - Subtract 1 hour (winter time change)"]
pub type SUB1H_R = crate::BitReader;
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
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ITSE_R = crate::BitReader;
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
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
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
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
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<CRrs> {
        ALRBE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Time stamp enable"]
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
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<CRrs> {
        ALRBIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<CRrs> {
        WUTIE_W::new(self, 14)
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
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<CRrs> {
        ITSE_W::new(self, 24)
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
