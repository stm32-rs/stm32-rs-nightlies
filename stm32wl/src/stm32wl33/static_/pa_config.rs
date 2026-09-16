///Register `PA_CONFIG` reader
pub type R = crate::R<PA_CONFIGrs>;
///Register `PA_CONFIG` writer
pub type W = crate::W<PA_CONFIGrs>;
///Field `PA_RAMP_STEP_WIDTH` reader - Step width (unit: 1/8 of bit period).
pub type PA_RAMP_STEP_WIDTH_R = crate::FieldReader;
///Field `PA_RAMP_STEP_WIDTH` writer - Step width (unit: 1/8 of bit period).
pub type PA_RAMP_STEP_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA_LEVEL_MAX_INDEX` reader - Final level for power ramping (i.
pub type PA_LEVEL_MAX_INDEX_R = crate::FieldReader;
///Field `PA_LEVEL_MAX_INDEX` writer - Final level for power ramping (i.
pub type PA_LEVEL_MAX_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PA_INTERP_EN` reader - Enable power level interpolator.
pub type PA_INTERP_EN_R = crate::BitReader;
///Field `PA_INTERP_EN` writer - Enable power level interpolator.
pub type PA_INTERP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASK_OOK_EN` reader - Enable the generation of the internal TXDATA signal provided to the FIR.
pub type ASK_OOK_EN_R = crate::BitReader;
///Field `ASK_OOK_EN` writer - Enable the generation of the internal TXDATA signal provided to the FIR.
pub type ASK_OOK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA_DRV_MODE` reader - Select the PA topology
pub type PA_DRV_MODE_R = crate::FieldReader;
///Field `PA_DRV_MODE` writer - Select the PA topology
pub type PA_DRV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA_MODE` reader - Configure the Power Amplifier (PA) mode
pub type PA_MODE_R = crate::FieldReader;
///Field `PA_MODE` writer - Configure the Power Amplifier (PA) mode
pub type PA_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LIN_NLOG` reader - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator
pub type LIN_NLOG_R = crate::BitReader;
///Field `LIN_NLOG` writer - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator
pub type LIN_NLOG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA_RAMP_ENABLE` reader - Enable the power ramping
pub type PA_RAMP_ENABLE_R = crate::BitReader;
///Field `PA_RAMP_ENABLE` writer - Enable the power ramping
pub type PA_RAMP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Step width (unit: 1/8 of bit period).
    #[inline(always)]
    pub fn pa_ramp_step_width(&self) -> PA_RAMP_STEP_WIDTH_R {
        PA_RAMP_STEP_WIDTH_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - Final level for power ramping (i.
    #[inline(always)]
    pub fn pa_level_max_index(&self) -> PA_LEVEL_MAX_INDEX_R {
        PA_LEVEL_MAX_INDEX_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 6 - Enable power level interpolator.
    #[inline(always)]
    pub fn pa_interp_en(&self) -> PA_INTERP_EN_R {
        PA_INTERP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable the generation of the internal TXDATA signal provided to the FIR.
    #[inline(always)]
    pub fn ask_ook_en(&self) -> ASK_OOK_EN_R {
        ASK_OOK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Select the PA topology
    #[inline(always)]
    pub fn pa_drv_mode(&self) -> PA_DRV_MODE_R {
        PA_DRV_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Configure the Power Amplifier (PA) mode
    #[inline(always)]
    pub fn pa_mode(&self) -> PA_MODE_R {
        PA_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 13 - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator
    #[inline(always)]
    pub fn lin_nlog(&self) -> LIN_NLOG_R {
        LIN_NLOG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable the power ramping
    #[inline(always)]
    pub fn pa_ramp_enable(&self) -> PA_RAMP_ENABLE_R {
        PA_RAMP_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_CONFIG")
            .field("pa_ramp_step_width", &self.pa_ramp_step_width())
            .field("pa_level_max_index", &self.pa_level_max_index())
            .field("pa_interp_en", &self.pa_interp_en())
            .field("ask_ook_en", &self.ask_ook_en())
            .field("pa_drv_mode", &self.pa_drv_mode())
            .field("pa_mode", &self.pa_mode())
            .field("lin_nlog", &self.lin_nlog())
            .field("pa_ramp_enable", &self.pa_ramp_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Step width (unit: 1/8 of bit period).
    #[inline(always)]
    pub fn pa_ramp_step_width(&mut self) -> PA_RAMP_STEP_WIDTH_W<'_, PA_CONFIGrs> {
        PA_RAMP_STEP_WIDTH_W::new(self, 0)
    }
    ///Bits 2:4 - Final level for power ramping (i.
    #[inline(always)]
    pub fn pa_level_max_index(&mut self) -> PA_LEVEL_MAX_INDEX_W<'_, PA_CONFIGrs> {
        PA_LEVEL_MAX_INDEX_W::new(self, 2)
    }
    ///Bit 6 - Enable power level interpolator.
    #[inline(always)]
    pub fn pa_interp_en(&mut self) -> PA_INTERP_EN_W<'_, PA_CONFIGrs> {
        PA_INTERP_EN_W::new(self, 6)
    }
    ///Bit 7 - Enable the generation of the internal TXDATA signal provided to the FIR.
    #[inline(always)]
    pub fn ask_ook_en(&mut self) -> ASK_OOK_EN_W<'_, PA_CONFIGrs> {
        ASK_OOK_EN_W::new(self, 7)
    }
    ///Bits 8:9 - Select the PA topology
    #[inline(always)]
    pub fn pa_drv_mode(&mut self) -> PA_DRV_MODE_W<'_, PA_CONFIGrs> {
        PA_DRV_MODE_W::new(self, 8)
    }
    ///Bits 10:11 - Configure the Power Amplifier (PA) mode
    #[inline(always)]
    pub fn pa_mode(&mut self) -> PA_MODE_W<'_, PA_CONFIGrs> {
        PA_MODE_W::new(self, 10)
    }
    ///Bit 13 - Enable/disable the linear-to- log conversion of the PA code output from Safe-ASK calibrator
    #[inline(always)]
    pub fn lin_nlog(&mut self) -> LIN_NLOG_W<'_, PA_CONFIGrs> {
        LIN_NLOG_W::new(self, 13)
    }
    ///Bit 14 - Enable the power ramping
    #[inline(always)]
    pub fn pa_ramp_enable(&mut self) -> PA_RAMP_ENABLE_W<'_, PA_CONFIGrs> {
        PA_RAMP_ENABLE_W::new(self, 14)
    }
}
/**PA_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pa_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_CONFIG)*/
pub struct PA_CONFIGrs;
impl crate::RegisterSpec for PA_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`pa_config::R`](R) reader structure
impl crate::Readable for PA_CONFIGrs {}
///`write(|w| ..)` method takes [`pa_config::W`](W) writer structure
impl crate::Writable for PA_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PA_CONFIG to value 0x015c
impl crate::Resettable for PA_CONFIGrs {
    const RESET_VALUE: u32 = 0x015c;
}
