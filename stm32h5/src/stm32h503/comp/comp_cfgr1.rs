#[doc = "Register `COMP_CFGR1` reader"]
pub type R = crate::R<COMP_CFGR1rs>;
#[doc = "Register `COMP_CFGR1` writer"]
pub type W = crate::W<COMP_CFGR1rs>;
#[doc = "Field `EN` reader - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRGEN` reader - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V&lt;sub>REF_COMP&lt;/sub> (similar to V&lt;sub>REFINT&lt;/sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V&lt;sub>REF_COMP&lt;/sub>, 3/4�V&lt;sub>REF_COMP&lt;/sub>, 1/2�V&lt;sub>REF_COMP&lt;/sub> and 1/4�V&lt;sub>REF_COMP&lt;/sub> levels, respectively."]
pub type BRGEN_R = crate::BitReader;
#[doc = "Field `BRGEN` writer - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V&lt;sub>REF_COMP&lt;/sub> (similar to V&lt;sub>REFINT&lt;/sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V&lt;sub>REF_COMP&lt;/sub>, 3/4�V&lt;sub>REF_COMP&lt;/sub>, 1/2�V&lt;sub>REF_COMP&lt;/sub> and 1/4�V&lt;sub>REF_COMP&lt;/sub> levels, respectively."]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V&lt;sub>REFINT&lt;/sub> scaler for the COMP channels."]
pub type SCALEN_R = crate::BitReader;
#[doc = "Field `SCALEN` writer - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V&lt;sub>REFINT&lt;/sub> scaler for the COMP channels."]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_R = crate::BitReader;
#[doc = "Field `POLARITY` writer - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN` reader - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_R = crate::BitReader;
#[doc = "Field `ITEN` writer - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRMODE` reader - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_R = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INMSEL` reader - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL1` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_R = crate::BitReader;
#[doc = "Field `INPSEL1` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPSEL2` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_R = crate::BitReader;
#[doc = "Field `INPSEL2` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKING` reader - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_R = crate::FieldReader;
#[doc = "Field `BLANKING` writer - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V&lt;sub>REF_COMP&lt;/sub> (similar to V&lt;sub>REFINT&lt;/sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V&lt;sub>REF_COMP&lt;/sub>, 3/4�V&lt;sub>REF_COMP&lt;/sub>, 1/2�V&lt;sub>REF_COMP&lt;/sub> and 1/4�V&lt;sub>REF_COMP&lt;/sub> levels, respectively."]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V&lt;sub>REFINT&lt;/sub> scaler for the COMP channels."]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel1(&self) -> INPSEL1_R {
        INPSEL1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel2(&self) -> INPSEL2_R {
        INPSEL2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP_CFGR1rs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V&lt;sub>REF_COMP&lt;/sub> (similar to V&lt;sub>REFINT&lt;/sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V&lt;sub>REF_COMP&lt;/sub>, 3/4�V&lt;sub>REF_COMP&lt;/sub>, 1/2�V&lt;sub>REF_COMP&lt;/sub> and 1/4�V&lt;sub>REF_COMP&lt;/sub> levels, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<COMP_CFGR1rs> {
        BRGEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V&lt;sub>REFINT&lt;/sub> scaler for the COMP channels."]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<COMP_CFGR1rs> {
        SCALEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<COMP_CFGR1rs> {
        POLARITY_W::new(self, 3)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<COMP_CFGR1rs> {
        ITEN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP_CFGR1rs> {
        HYST_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP_CFGR1rs> {
        PWRMODE_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP_CFGR1rs> {
        INMSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel1(&mut self) -> INPSEL1_W<COMP_CFGR1rs> {
        INPSEL1_W::new(self, 20)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel2(&mut self) -> INPSEL2_W<COMP_CFGR1rs> {
        INPSEL2_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<COMP_CFGR1rs> {
        BLANKING_W::new(self, 24)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP_CFGR1rs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_CFGR1rs;
impl crate::RegisterSpec for COMP_CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_cfgr1::R`](R) reader structure"]
impl crate::Readable for COMP_CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`comp_cfgr1::W`](W) writer structure"]
impl crate::Writable for COMP_CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP_CFGR1 to value 0"]
impl crate::Resettable for COMP_CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
