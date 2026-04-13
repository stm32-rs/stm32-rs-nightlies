///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM uses:
pub type CKSEL_R = crate::BitReader;
///Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM uses:
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKPOL` reader - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes.
pub type CKPOL_R = crate::FieldReader;
///Field `CKPOL` writer - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes.
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type CKFLT_R = crate::FieldReader;
///Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type TRGFLT_R = crate::FieldReader;
///Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details.
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
pub type TRIGEN_R = crate::FieldReader;
///Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature
pub type TIMOUT_R = crate::BitReader;
///Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape
pub type WAVE_R = crate::BitReader;
///Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality
pub type PRELOAD_R = crate::BitReader;
///Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
pub type COUNTMODE_R = crate::BitReader;
///Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type ENC_R = crate::BitReader;
///Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("cksel", &self.cksel())
            .field("ckpol", &self.ckpol())
            .field("ckflt", &self.ckflt())
            .field("trgflt", &self.trgflt())
            .field("presc", &self.presc())
            .field("trigsel", &self.trigsel())
            .field("trigen", &self.trigen())
            .field("timout", &self.timout())
            .field("wave", &self.wave())
            .field("preload", &self.preload())
            .field("countmode", &self.countmode())
            .field("enc", &self.enc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<'_, CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    ///Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active. Refer to Section125.4.15: Encoder mode for more details about Encoder mode sub-modes.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W<'_, CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that are detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W<'_, CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, CFGRrs> {
        PRESC_W::new(self, 9)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See Section125.4.3: LPTIM input and trigger mapping for details.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<'_, CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<'_, CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W<'_, CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W<'_, CFGRrs> {
        WAVE_W::new(self, 20)
    }
    ///Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM2_ARR, LPTIM2_RCR and the LPTIM2_CCRx registers update modality
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<'_, CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    ///Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W<'_, CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    ///Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W<'_, CFGRrs> {
        ENC_W::new(self, 24)
    }
}
/**LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
