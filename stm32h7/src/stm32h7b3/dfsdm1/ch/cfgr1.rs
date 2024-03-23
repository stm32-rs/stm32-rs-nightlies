#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Field `SITP` reader - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type SITP_R = crate::FieldReader;
#[doc = "Field `SITP` writer - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPICKSEL` reader - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type SPICKSEL_R = crate::FieldReader;
#[doc = "Field `SPICKSEL` writer - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCDEN` reader - Short-circuit detector enable on channel y"]
pub type SCDEN_R = crate::BitReader;
#[doc = "Field `SCDEN` writer - Short-circuit detector enable on channel y"]
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABEN` reader - Clock absence detector enable on channel y"]
pub type CKABEN_R = crate::BitReader;
#[doc = "Field `CKABEN` writer - Clock absence detector enable on channel y"]
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN` reader - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHINSEL` reader - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type CHINSEL_R = crate::BitReader;
#[doc = "Field `CHINSEL` writer - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATMPX` reader - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type DATMPX_R = crate::FieldReader;
#[doc = "Field `DATMPX` writer - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATPACK` reader - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type DATPACK_R = crate::FieldReader;
#[doc = "Field `DATPACK` writer - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKOUTDIV` reader - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
pub type CKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CKOUTDIV` writer - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKOUTSRC` reader - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub type CKOUTSRC_R = crate::BitReader;
#[doc = "Field `CKOUTSRC` writer - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMEN` reader - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub type DFSDMEN_R = crate::BitReader;
#[doc = "Field `DFSDMEN` writer - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel y"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel y"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<CFGR1rs> {
        SITP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel y"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel y"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<CFGR1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    #[doc = "Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<CFGR1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<CFGR1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
#[doc = "DFSDM channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
