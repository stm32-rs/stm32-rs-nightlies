///Register `RF_CONFIG` reader
pub type R = crate::R<RF_CONFIGrs>;
///Register `RF_CONFIG` writer
pub type W = crate::W<RF_CONFIGrs>;
///Field `ED_SWITCH` reader - - 0 : Normal operation (default)
pub type ED_SWITCH_R = crate::BitReader;
///Field `ED_SWITCH` writer - - 0 : Normal operation (default)
pub type ED_SWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKDIV` reader - Calibrate 4kHz clock (programmable divider)
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Calibrate 4kHz clock (programmable divider)
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AGC_LOW_LVL` reader - AGC level (Low) (default value: 0x2)
pub type AGC_LOW_LVL_R = crate::FieldReader;
///Field `AGC_LOW_LVL` writer - AGC level (Low) (default value: 0x2)
pub type AGC_LOW_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ED_DC_CTRL` reader - DC current subtraction enabling signal (default value: 0x1)
pub type ED_DC_CTRL_R = crate::BitReader;
///Field `ED_DC_CTRL` writer - DC current subtraction enabling signal (default value: 0x1)
pub type ED_DC_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_HIGH_LVL` reader - AGC level (High) (default value: 0x4)
pub type AGC_HIGH_LVL_R = crate::FieldReader;
///Field `AGC_HIGH_LVL` writer - AGC level (High) (default value: 0x4)
pub type AGC_HIGH_LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ED_ICAL` reader - Current versus VBAT calibration for ED
pub type ED_ICAL_R = crate::FieldReader;
///Field `ED_ICAL` writer - Current versus VBAT calibration for ED
pub type ED_ICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPF3_CAL` reader -
pub type LPF3_CAL_R = crate::BitReader;
///Field `LPF3_CAL` writer -
pub type LPF3_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - - 0 : Normal operation (default)
    #[inline(always)]
    pub fn ed_switch(&self) -> ED_SWITCH_R {
        ED_SWITCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - Calibrate 4kHz clock (programmable divider)
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 11:12 - AGC level (Low) (default value: 0x2)
    #[inline(always)]
    pub fn agc_low_lvl(&self) -> AGC_LOW_LVL_R {
        AGC_LOW_LVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - DC current subtraction enabling signal (default value: 0x1)
    #[inline(always)]
    pub fn ed_dc_ctrl(&self) -> ED_DC_CTRL_R {
        ED_DC_CTRL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:17 - AGC level (High) (default value: 0x4)
    #[inline(always)]
    pub fn agc_high_lvl(&self) -> AGC_HIGH_LVL_R {
        AGC_HIGH_LVL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bits 18:20 - Current versus VBAT calibration for ED
    #[inline(always)]
    pub fn ed_ical(&self) -> ED_ICAL_R {
        ED_ICAL_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21
    #[inline(always)]
    pub fn lpf3_cal(&self) -> LPF3_CAL_R {
        LPF3_CAL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_CONFIG")
            .field("ed_switch", &self.ed_switch())
            .field("clkdiv", &self.clkdiv())
            .field("agc_low_lvl", &self.agc_low_lvl())
            .field("ed_dc_ctrl", &self.ed_dc_ctrl())
            .field("agc_high_lvl", &self.agc_high_lvl())
            .field("ed_ical", &self.ed_ical())
            .field("lpf3_cal", &self.lpf3_cal())
            .finish()
    }
}
impl W {
    ///Bit 0 - - 0 : Normal operation (default)
    #[inline(always)]
    pub fn ed_switch(&mut self) -> ED_SWITCH_W<'_, RF_CONFIGrs> {
        ED_SWITCH_W::new(self, 0)
    }
    ///Bits 1:4 - Calibrate 4kHz clock (programmable divider)
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, RF_CONFIGrs> {
        CLKDIV_W::new(self, 1)
    }
    ///Bits 11:12 - AGC level (Low) (default value: 0x2)
    #[inline(always)]
    pub fn agc_low_lvl(&mut self) -> AGC_LOW_LVL_W<'_, RF_CONFIGrs> {
        AGC_LOW_LVL_W::new(self, 11)
    }
    ///Bit 13 - DC current subtraction enabling signal (default value: 0x1)
    #[inline(always)]
    pub fn ed_dc_ctrl(&mut self) -> ED_DC_CTRL_W<'_, RF_CONFIGrs> {
        ED_DC_CTRL_W::new(self, 13)
    }
    ///Bits 14:17 - AGC level (High) (default value: 0x4)
    #[inline(always)]
    pub fn agc_high_lvl(&mut self) -> AGC_HIGH_LVL_W<'_, RF_CONFIGrs> {
        AGC_HIGH_LVL_W::new(self, 14)
    }
    ///Bits 18:20 - Current versus VBAT calibration for ED
    #[inline(always)]
    pub fn ed_ical(&mut self) -> ED_ICAL_W<'_, RF_CONFIGrs> {
        ED_ICAL_W::new(self, 18)
    }
    ///Bit 21
    #[inline(always)]
    pub fn lpf3_cal(&mut self) -> LPF3_CAL_W<'_, RF_CONFIGrs> {
        LPF3_CAL_W::new(self, 21)
    }
}
/**RF_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rf_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:RF_CONFIG)*/
pub struct RF_CONFIGrs;
impl crate::RegisterSpec for RF_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`rf_config::R`](R) reader structure
impl crate::Readable for RF_CONFIGrs {}
///`write(|w| ..)` method takes [`rf_config::W`](W) writer structure
impl crate::Writable for RF_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_CONFIG to value 0x0001_33ee
impl crate::Resettable for RF_CONFIGrs {
    const RESET_VALUE: u32 = 0x0001_33ee;
}
