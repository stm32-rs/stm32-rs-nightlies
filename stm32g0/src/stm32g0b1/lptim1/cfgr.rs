#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
pub type CKSEL_R = crate::BitReader;
#[doc = "Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPOL` reader - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_R = crate::FieldReader;
#[doc = "Field `CKPOL` writer - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_R = crate::FieldReader;
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_R = crate::FieldReader;
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_R = crate::FieldReader;
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_R = crate::BitReader;
#[doc = "Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVPOL` reader - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
pub type WAVPOL_R = crate::BitReader;
#[doc = "Field `WAVPOL` writer - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
pub type PRELOAD_R = crate::BitReader;
#[doc = "Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_R = crate::BitReader;
#[doc = "Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_R = crate::BitReader;
#[doc = "Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CFGRrs> {
        PRESC_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<CFGRrs> {
        WAVE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<CFGRrs> {
        WAVPOL_W::new(self, 21)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<CFGRrs> {
        ENC_W::new(self, 24)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
