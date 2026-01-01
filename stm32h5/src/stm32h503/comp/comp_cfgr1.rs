///Register `COMP_CFGR1` reader
pub type R = crate::R<COMP_CFGR1rs>;
///Register `COMP_CFGR1` writer
pub type W = crate::W<COMP_CFGR1rs>;
///Field `EN` reader - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1.
pub type EN_R = crate::BitReader;
///Field `EN` writer - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRGEN` reader - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively.
pub type BRGEN_R = crate::BitReader;
///Field `BRGEN` writer - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively.
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALEN` reader - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels.
pub type SCALEN_R = crate::BitReader;
///Field `SCALEN` writer - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels.
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLARITY` reader - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.
pub type POLARITY_R = crate::BitReader;
///Field `POLARITY` writer - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITEN` reader - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.
pub type ITEN_R = crate::BitReader;
///Field `ITEN` writer - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.
pub type ITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PWRMODE` reader - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INMSEL` reader - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details.
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details.
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INPSEL1` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL1_R = crate::BitReader;
///Field `INPSEL1` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INPSEL2` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL2_R = crate::BitReader;
///Field `INPSEL2` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLANKING` reader - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved
pub type BLANKING_R = crate::FieldReader;
///Field `BLANKING` writer - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\[31:0\]
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\[31:0\]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively.
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels.
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details.
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel1(&self) -> INPSEL1_R {
        INPSEL1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel2(&self) -> INPSEL2_R {
        INPSEL2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\[31:0\]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_CFGR1")
            .field("en", &self.en())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("polarity", &self.polarity())
            .field("iten", &self.iten())
            .field("hyst", &self.hyst())
            .field("pwrmode", &self.pwrmode())
            .field("inmsel", &self.inmsel())
            .field("inpsel1", &self.inpsel1())
            .field("inpsel2", &self.inpsel2())
            .field("blanking", &self.blanking())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, COMP_CFGR1rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively.
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, COMP_CFGR1rs> {
        BRGEN_W::new(self, 1)
    }
    ///Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels.
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, COMP_CFGR1rs> {
        SCALEN_W::new(self, 2)
    }
    ///Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<'_, COMP_CFGR1rs> {
        POLARITY_W::new(self, 3)
    }
    ///Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.
    #[inline(always)]
    pub fn iten(&mut self) -> ITEN_W<'_, COMP_CFGR1rs> {
        ITEN_W::new(self, 6)
    }
    ///Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, COMP_CFGR1rs> {
        HYST_W::new(self, 8)
    }
    ///Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<'_, COMP_CFGR1rs> {
        PWRMODE_W::new(self, 12)
    }
    ///Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details.
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, COMP_CFGR1rs> {
        INMSEL_W::new(self, 16)
    }
    ///Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel1(&mut self) -> INPSEL1_W<'_, COMP_CFGR1rs> {
        INPSEL1_W::new(self, 20)
    }
    ///Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel2(&mut self) -> INPSEL2_W<'_, COMP_CFGR1rs> {
        INPSEL2_W::new(self, 22)
    }
    ///Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<'_, COMP_CFGR1rs> {
        BLANKING_W::new(self, 24)
    }
    ///Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\[31:0\]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, COMP_CFGR1rs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator configuration register 1

You can [`read`](crate::Reg::read) this register and get [`comp_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_CFGR1)*/
pub struct COMP_CFGR1rs;
impl crate::RegisterSpec for COMP_CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`comp_cfgr1::R`](R) reader structure
impl crate::Readable for COMP_CFGR1rs {}
///`write(|w| ..)` method takes [`comp_cfgr1::W`](W) writer structure
impl crate::Writable for COMP_CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_CFGR1 to value 0
impl crate::Resettable for COMP_CFGR1rs {}
