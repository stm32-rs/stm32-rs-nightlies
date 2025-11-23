///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `WUCKSEL` reader - ck_wut wake-up clock selection
pub type WUCKSEL_R = crate::FieldReader;
///Field `WUCKSEL` writer - ck_wut wake-up clock selection
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TSEDGE` reader - Timestamp event active edge
pub type TSEDGE_R = crate::BitReader;
///Field `TSEDGE` writer - Timestamp event active edge
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub type REFCKON_R = crate::BitReader;
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPSHAD` reader - Bypass the shadow registers
pub type BYPSHAD_R = crate::BitReader;
///Field `BYPSHAD` writer - Bypass the shadow registers
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader;
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSRUIE` reader - SSR underflow interrupt enable
pub type SSRUIE_R = crate::BitReader;
///Field `SSRUIE` writer - SSR underflow interrupt enable
pub type SSRUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader;
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBE` reader - Alarm B enable
pub type ALRBE_R = crate::BitReader;
///Field `ALRBE` writer - Alarm B enable
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTE` reader - Wake-up timer enable
pub type WUTE_R = crate::BitReader;
///Field `WUTE` writer - Wake-up timer enable
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - timestamp enable
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - timestamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader;
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBIE` reader - Alarm B interrupt enable
pub type ALRBIE_R = crate::BitReader;
///Field `ALRBIE` writer - Alarm B interrupt enable
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTIE` reader - Wake-up timer interrupt enable
pub type WUTIE_R = crate::BitReader;
///Field `WUTIE` writer - Wake-up timer interrupt enable
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - Timestamp interrupt enable
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - Timestamp interrupt enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Backup
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Backup
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COSEL` reader - Calibration output selection
pub type COSEL_R = crate::BitReader;
///Field `COSEL` writer - Calibration output selection
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POL` reader - Output polarity
pub type POL_R = crate::BitReader;
///Field `POL` writer - Output polarity
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSEL` reader - Output selection
pub type OSEL_R = crate::FieldReader;
///Field `OSEL` writer - Output selection
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COE` reader - Calibration output enable
pub type COE_R = crate::BitReader;
///Field `COE` writer - Calibration output enable
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITSE` reader - timestamp on internal event enable
pub type ITSE_R = crate::BitReader;
///Field `ITSE` writer - timestamp on internal event enable
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader;
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM
pub type TAMPOE_R = crate::BitReader;
///Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAFCLR` reader - Alarm A flag automatic clear
pub type ALRAFCLR_R = crate::BitReader;
///Field `ALRAFCLR` writer - Alarm A flag automatic clear
pub type ALRAFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBFCLR` reader - Alarm B flag automatic clear
pub type ALRBFCLR_R = crate::BitReader;
///Field `ALRBFCLR` writer - Alarm B flag automatic clear
pub type ALRBFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable
pub type TAMPALRM_PU_R = crate::BitReader;
///Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPALRM_TYPE` reader - TAMPALRM output type
pub type TAMPALRM_TYPE_R = crate::BitReader;
///Field `TAMPALRM_TYPE` writer - TAMPALRM output type
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT2EN` reader - RTC_OUT2 output enable
pub type OUT2EN_R = crate::BitReader;
///Field `OUT2EN` writer - RTC_OUT2 output enable
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - ck_wut wake-up clock selection
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Timestamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SSR underflow interrupt enable
    #[inline(always)]
    pub fn ssruie(&self) -> SSRUIE_R {
        SSRUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wake-up timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wake-up timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Tamper detection output enable on TAMPALRM
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Alarm A flag automatic clear
    #[inline(always)]
    pub fn alrafclr(&self) -> ALRAFCLR_R {
        ALRAFCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Alarm B flag automatic clear
    #[inline(always)]
    pub fn alrbfclr(&self) -> ALRBFCLR_R {
        ALRBFCLR_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RTC_OUT2 output enable
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("wucksel", &self.wucksel())
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("ssruie", &self.ssruie())
            .field("alrae", &self.alrae())
            .field("alrbe", &self.alrbe())
            .field("wute", &self.wute())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("alrbie", &self.alrbie())
            .field("wutie", &self.wutie())
            .field("tsie", &self.tsie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .field("itse", &self.itse())
            .field("tampts", &self.tampts())
            .field("tampoe", &self.tampoe())
            .field("alrafclr", &self.alrafclr())
            .field("alrbfclr", &self.alrbfclr())
            .field("tampalrm_pu", &self.tampalrm_pu())
            .field("tampalrm_type", &self.tampalrm_type())
            .field("out2en", &self.out2en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - ck_wut wake-up clock selection
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W<'_, CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    ///Bit 3 - Timestamp event active edge
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<'_, CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<'_, CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<'_, CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 7 - SSR underflow interrupt enable
    #[inline(always)]
    pub fn ssruie(&mut self) -> SSRUIE_W<'_, CRrs> {
        SSRUIE_W::new(self, 7)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<'_, CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W<'_, CRrs> {
        ALRBE_W::new(self, 9)
    }
    ///Bit 10 - Wake-up timer enable
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<'_, CRrs> {
        WUTE_W::new(self, 10)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, CRrs> {
        TSE_W::new(self, 11)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<'_, CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W<'_, CRrs> {
        ALRBIE_W::new(self, 13)
    }
    ///Bit 14 - Wake-up timer interrupt enable
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<'_, CRrs> {
        WUTIE_W::new(self, 14)
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<'_, CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, CRrs> {
        COE_W::new(self, 23)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W<'_, CRrs> {
        ITSE_W::new(self, 24)
    }
    ///Bit 25 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<'_, CRrs> {
        TAMPTS_W::new(self, 25)
    }
    ///Bit 26 - Tamper detection output enable on TAMPALRM
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W<'_, CRrs> {
        TAMPOE_W::new(self, 26)
    }
    ///Bit 27 - Alarm A flag automatic clear
    #[inline(always)]
    pub fn alrafclr(&mut self) -> ALRAFCLR_W<'_, CRrs> {
        ALRAFCLR_W::new(self, 27)
    }
    ///Bit 28 - Alarm B flag automatic clear
    #[inline(always)]
    pub fn alrbfclr(&mut self) -> ALRBFCLR_W<'_, CRrs> {
        ALRBFCLR_W::new(self, 28)
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<'_, CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<'_, CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    ///Bit 31 - RTC_OUT2 output enable
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W<'_, CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
/**RTC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RTC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
