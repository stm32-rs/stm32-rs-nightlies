///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `TS1_EN` reader - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.
pub type TS1_EN_R = crate::BitReader;
///Field `TS1_EN` writer - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.
pub type TS1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_START` reader - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.
pub type TS1_START_R = crate::BitReader;
///Field `TS1_START` writer - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.
pub type TS1_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_INTRIG_SEL` reader - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input.
pub type TS1_INTRIG_SEL_R = crate::FieldReader;
///Field `TS1_INTRIG_SEL` writer - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input.
pub type TS1_INTRIG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TS1_SMP_TIME` reader - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE.
pub type TS1_SMP_TIME_R = crate::FieldReader;
///Field `TS1_SMP_TIME` writer - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE.
pub type TS1_SMP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REFCLK_SEL` reader - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).
pub type REFCLK_SEL_R = crate::BitReader;
///Field `REFCLK_SEL` writer - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).
pub type REFCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Q_MEAS_OPT` reader - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).
pub type Q_MEAS_OPT_R = crate::BitReader;
///Field `Q_MEAS_OPT` writer - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).
pub type Q_MEAS_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSREF_CLK_DIV` reader - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...
pub type HSREF_CLK_DIV_R = crate::FieldReader;
///Field `HSREF_CLK_DIV` writer - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...
pub type HSREF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input.
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE.
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("ts1_en", &self.ts1_en())
            .field("ts1_start", &self.ts1_start())
            .field("ts1_intrig_sel", &self.ts1_intrig_sel())
            .field("ts1_smp_time", &self.ts1_smp_time())
            .field("refclk_sel", &self.refclk_sel())
            .field("q_meas_opt", &self.q_meas_opt())
            .field("hsref_clk_div", &self.hsref_clk_div())
            .finish()
    }
}
impl W {
    ///Bit 0 - Temperature sensor 1 enable bit This bit is set and cleared by software. Note: Once enabled, the temperature sensor is active after a specific delay time. The TS1_RDY flag will be set when the sensor is ready.
    #[inline(always)]
    pub fn ts1_en(&mut self) -> TS1_EN_W<'_, CFGR1rs> {
        TS1_EN_W::new(self, 0)
    }
    ///Bit 4 - Start frequency measurement on temperature sensor 1 This bit is set and cleared by software.
    #[inline(always)]
    pub fn ts1_start(&mut self) -> TS1_START_W<'_, CFGR1rs> {
        TS1_START_W::new(self, 4)
    }
    ///Bits 8:11 - Input trigger selection bit for temperature sensor 1 These bits are set and cleared by software. They select which input triggers a temperature measurement. Refer to Section 19.3.10: Trigger input.
    #[inline(always)]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W<'_, CFGR1rs> {
        TS1_INTRIG_SEL_W::new(self, 8)
    }
    ///Bits 16:19 - Sampling time for temperature sensor 1 These bits allow increasing the sampling time to improve measurement precision. When the PCLK clock is selected as reference clock (REFCLK_SEL = 0), the measurement will be performed at TS1_SMP_TIME period of CLK_PTAT. When the LSE is selected as reference clock (REFCLK_SEL =1), the measurement will be performed at TS1_SMP_TIME period of LSE.
    #[inline(always)]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W<'_, CFGR1rs> {
        TS1_SMP_TIME_W::new(self, 16)
    }
    ///Bit 20 - Reference clock selection bit This bit is set and cleared by software. It indicates whether the reference clock is the high speed clock (PCLK) or the low speed clock (LSE).
    #[inline(always)]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<'_, CFGR1rs> {
        REFCLK_SEL_W::new(self, 20)
    }
    ///Bit 21 - Quick measurement option bit This bit is set and cleared by software. It is used to increase the measurement speed by suppressing the calibration step. It is effective only when the LSE clock is used as reference clock (REFCLK_SEL=1).
    #[inline(always)]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W<'_, CFGR1rs> {
        Q_MEAS_OPT_W::new(self, 21)
    }
    ///Bits 24:30 - High speed clock division ratio These bits are set and cleared by software. They can be used to define the division ratio for the main clock in order to obtain the internal frequency lower than 1 MHz required for the calibration. They are applicable only for calibration when PCLK is selected as reference clock (REFCLK_SEL=0). ...
    #[inline(always)]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W<'_, CFGR1rs> {
        HSREF_CLK_DIV_W::new(self, 24)
    }
}
/**Temperature sensor configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#DTS:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
