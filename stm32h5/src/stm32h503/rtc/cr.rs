#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `WUCKSEL` reader - ck_wut wakeup clock selection 10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. 11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. Furthermore, 2&lt;sup>16&lt;/sup> is added to the WUT counter value."]
pub type WUCKSEL_R = crate::FieldReader;
#[doc = "Field `WUCKSEL` writer - ck_wut wakeup clock selection 10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. 11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. Furthermore, 2&lt;sup>16&lt;/sup> is added to the WUT counter value."]
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_R = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: BIN must be 0x00 and PREDIV_S must be 0x00FF."]
pub type REFCKON_R = crate::BitReader;
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: BIN must be 0x00 and PREDIV_S must be 0x00FF."]
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_R = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader;
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSRUIE` reader - SSR underflow interrupt enable"]
pub type SSRUIE_R = crate::BitReader;
#[doc = "Field `SSRUIE` writer - SSR underflow interrupt enable"]
pub type SSRUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader;
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader;
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF = 1 before enabling it again."]
pub type WUTE_R = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF = 1 before enabling it again."]
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - timestamp enable"]
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
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 31.3.17: Calibration clock output."]
pub type COSEL_R = crate::BitReader;
#[doc = "Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 31.3.17: Calibration clock output."]
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_R = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - Calibration output enable This bit enables the CALIB output"]
pub type COE_R = crate::BitReader;
#[doc = "Field `COE` writer - Calibration output enable This bit enables the CALIB output"]
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ITSE_R = crate::BitReader;
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set up to 3 ck_apre cycles after the tamper flags."]
pub type TAMPTS_R = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set up to 3 ck_apre cycles after the tamper flags."]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_R = crate::BitReader;
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAFCLR` reader - Alarm A flag automatic clear"]
pub type ALRAFCLR_R = crate::BitReader;
#[doc = "Field `ALRAFCLR` writer - Alarm A flag automatic clear"]
pub type ALRAFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBFCLR` reader - Alarm B flag automatic clear"]
pub type ALRBFCLR_R = crate::BitReader;
#[doc = "Field `ALRBFCLR` writer - Alarm B flag automatic clear"]
pub type ALRBFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_R = crate::BitReader;
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub type TAMPALRM_TYPE_R = crate::BitReader;
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable With this bit set, the RTC outputs can be remapped on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL ≠ 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL ≠ 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL ≠ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_R = crate::BitReader;
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable With this bit set, the RTC outputs can be remapped on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL ≠ 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL ≠ 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL ≠ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. 11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. Furthermore, 2&lt;sup>16&lt;/sup> is added to the WUT counter value."]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: BIN must be 0x00 and PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    pub fn ssruie(&self) -> SSRUIE_R {
        SSRUIE_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF = 1 before enabling it again."]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 31.3.17: Calibration clock output."]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set up to 3 ck_apre cycles after the tamper flags."]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Alarm A flag automatic clear"]
    #[inline(always)]
    pub fn alrafclr(&self) -> ALRAFCLR_R {
        ALRAFCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Alarm B flag automatic clear"]
    #[inline(always)]
    pub fn alrbfclr(&self) -> ALRBFCLR_R {
        ALRBFCLR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable With this bit set, the RTC outputs can be remapped on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL ≠ 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL ≠ 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL ≠ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. 11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is the clock selected by BCDU. Furthermore, 2&lt;sup>16&lt;/sup> is added to the WUT counter value."]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: BIN must be 0x00 and PREDIV_S must be 0x00FF."]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
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
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssruie(&mut self) -> SSRUIE_W<CRrs> {
        SSRUIE_W::new(self, 7)
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
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF = 1 before enabling it again."]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
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
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 31.3.17: Calibration clock output."]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
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
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set up to 3 ck_apre cycles after the tamper flags."]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<CRrs> {
        TAMPTS_W::new(self, 25)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<CRrs> {
        TAMPOE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Alarm A flag automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn alrafclr(&mut self) -> ALRAFCLR_W<CRrs> {
        ALRAFCLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Alarm B flag automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn alrbfclr(&mut self) -> ALRBFCLR_W<CRrs> {
        ALRBFCLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable With this bit set, the RTC outputs can be remapped on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL ≠ 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL ≠ 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL ≠ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
#[doc = "RTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
