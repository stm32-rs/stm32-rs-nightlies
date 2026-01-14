///Register `AGC2_CTRL` reader
pub type R = crate::R<AGC2_CTRLrs>;
///Register `AGC2_CTRL` writer
pub type W = crate::W<AGC2_CTRLrs>;
///Field `AGC_MEAS_TIME` reader - Measure time.
pub type AGC_MEAS_TIME_R = crate::FieldReader;
///Field `AGC_MEAS_TIME` writer - Measure time.
pub type AGC_MEAS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AGC_START_MAX_ATTEN` reader - Start the AGC with maximum attenuation.
pub type AGC_START_MAX_ATTEN_R = crate::BitReader;
///Field `AGC_START_MAX_ATTEN` writer - Start the AGC with maximum attenuation.
pub type AGC_START_MAX_ATTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_FREEZE_ON_SYNC` reader - Enable the freeze on SYNC detection feature
pub type AGC_FREEZE_ON_SYNC_R = crate::BitReader;
///Field `AGC_FREEZE_ON_SYNC` writer - Enable the freeze on SYNC detection feature
pub type AGC_FREEZE_ON_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_FREEZE_ON_STEADY` reader - Enable the autofreeze feature
pub type AGC_FREEZE_ON_STEADY_R = crate::BitReader;
///Field `AGC_FREEZE_ON_STEADY` writer - Enable the autofreeze feature
pub type AGC_FREEZE_ON_STEADY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_HIGH_ATTEN_MODE` reader - Enable the high attenuation mode.
pub type AGC_HIGH_ATTEN_MODE_R = crate::BitReader;
///Field `AGC_HIGH_ATTEN_MODE` writer - Enable the high attenuation mode.
pub type AGC_HIGH_ATTEN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Measure time.
    #[inline(always)]
    pub fn agc_meas_time(&self) -> AGC_MEAS_TIME_R {
        AGC_MEAS_TIME_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Start the AGC with maximum attenuation.
    #[inline(always)]
    pub fn agc_start_max_atten(&self) -> AGC_START_MAX_ATTEN_R {
        AGC_START_MAX_ATTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable the freeze on SYNC detection feature
    #[inline(always)]
    pub fn agc_freeze_on_sync(&self) -> AGC_FREEZE_ON_SYNC_R {
        AGC_FREEZE_ON_SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enable the autofreeze feature
    #[inline(always)]
    pub fn agc_freeze_on_steady(&self) -> AGC_FREEZE_ON_STEADY_R {
        AGC_FREEZE_ON_STEADY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable the high attenuation mode.
    #[inline(always)]
    pub fn agc_high_atten_mode(&self) -> AGC_HIGH_ATTEN_MODE_R {
        AGC_HIGH_ATTEN_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC2_CTRL")
            .field("agc_meas_time", &self.agc_meas_time())
            .field("agc_start_max_atten", &self.agc_start_max_atten())
            .field("agc_freeze_on_sync", &self.agc_freeze_on_sync())
            .field("agc_freeze_on_steady", &self.agc_freeze_on_steady())
            .field("agc_high_atten_mode", &self.agc_high_atten_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Measure time.
    #[inline(always)]
    pub fn agc_meas_time(&mut self) -> AGC_MEAS_TIME_W<'_, AGC2_CTRLrs> {
        AGC_MEAS_TIME_W::new(self, 0)
    }
    ///Bit 4 - Start the AGC with maximum attenuation.
    #[inline(always)]
    pub fn agc_start_max_atten(&mut self) -> AGC_START_MAX_ATTEN_W<'_, AGC2_CTRLrs> {
        AGC_START_MAX_ATTEN_W::new(self, 4)
    }
    ///Bit 5 - Enable the freeze on SYNC detection feature
    #[inline(always)]
    pub fn agc_freeze_on_sync(&mut self) -> AGC_FREEZE_ON_SYNC_W<'_, AGC2_CTRLrs> {
        AGC_FREEZE_ON_SYNC_W::new(self, 5)
    }
    ///Bit 6 - Enable the autofreeze feature
    #[inline(always)]
    pub fn agc_freeze_on_steady(&mut self) -> AGC_FREEZE_ON_STEADY_W<'_, AGC2_CTRLrs> {
        AGC_FREEZE_ON_STEADY_W::new(self, 6)
    }
    ///Bit 7 - Enable the high attenuation mode.
    #[inline(always)]
    pub fn agc_high_atten_mode(&mut self) -> AGC_HIGH_ATTEN_MODE_W<'_, AGC2_CTRLrs> {
        AGC_HIGH_ATTEN_MODE_W::new(self, 7)
    }
}
/**AGC2_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC2_CTRL)*/
pub struct AGC2_CTRLrs;
impl crate::RegisterSpec for AGC2_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`agc2_ctrl::R`](R) reader structure
impl crate::Readable for AGC2_CTRLrs {}
///`write(|w| ..)` method takes [`agc2_ctrl::W`](W) writer structure
impl crate::Writable for AGC2_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC2_CTRL to value 0xaf
impl crate::Resettable for AGC2_CTRLrs {
    const RESET_VALUE: u32 = 0xaf;
}
