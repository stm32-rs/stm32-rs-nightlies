///Register `VCO_CAL_CONFIG` reader
pub type R = crate::R<VCO_CAL_CONFIGrs>;
///Register `VCO_CAL_CONFIG` writer
pub type W = crate::W<VCO_CAL_CONFIGrs>;
///Field `VCO_CALAMP_EXT` reader - VCO magnitude calibration word in thermometric code
pub type VCO_CALAMP_EXT_R = crate::FieldReader<u16>;
///Field `VCO_CALAMP_EXT` writer - VCO magnitude calibration word in thermometric code
pub type VCO_CALAMP_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `VCO_CALAMP_EXT_SEL` reader - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field
pub type VCO_CALAMP_EXT_SEL_R = crate::BitReader;
///Field `VCO_CALAMP_EXT_SEL` writer - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field
pub type VCO_CALAMP_EXT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCO_CALFREQ_EXT` reader - VCO Cbank frequency calibration word.
pub type VCO_CALFREQ_EXT_R = crate::FieldReader;
///Field `VCO_CALFREQ_EXT` writer - VCO Cbank frequency calibration word.
pub type VCO_CALFREQ_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `VCO_CALFREQ_EXT_SEL` reader - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field
pub type VCO_CALFREQ_EXT_SEL_R = crate::BitReader;
///Field `VCO_CALFREQ_EXT_SEL` writer - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field
pub type VCO_CALFREQ_EXT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCO_CALIB_REQ` reader - Define if the Radio FSM must launch a VCO calibration request after VCO start-up
pub type VCO_CALIB_REQ_R = crate::BitReader;
///Field `VCO_CALIB_REQ` writer - Define if the Radio FSM must launch a VCO calibration request after VCO start-up
pub type VCO_CALIB_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - VCO magnitude calibration word in thermometric code
    #[inline(always)]
    pub fn vco_calamp_ext(&self) -> VCO_CALAMP_EXT_R {
        VCO_CALAMP_EXT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 15 - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field
    #[inline(always)]
    pub fn vco_calamp_ext_sel(&self) -> VCO_CALAMP_EXT_SEL_R {
        VCO_CALAMP_EXT_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - VCO Cbank frequency calibration word.
    #[inline(always)]
    pub fn vco_calfreq_ext(&self) -> VCO_CALFREQ_EXT_R {
        VCO_CALFREQ_EXT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field
    #[inline(always)]
    pub fn vco_calfreq_ext_sel(&self) -> VCO_CALFREQ_EXT_SEL_R {
        VCO_CALFREQ_EXT_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Define if the Radio FSM must launch a VCO calibration request after VCO start-up
    #[inline(always)]
    pub fn vco_calib_req(&self) -> VCO_CALIB_REQ_R {
        VCO_CALIB_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCO_CAL_CONFIG")
            .field("vco_calamp_ext", &self.vco_calamp_ext())
            .field("vco_calamp_ext_sel", &self.vco_calamp_ext_sel())
            .field("vco_calfreq_ext", &self.vco_calfreq_ext())
            .field("vco_calfreq_ext_sel", &self.vco_calfreq_ext_sel())
            .field("vco_calib_req", &self.vco_calib_req())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - VCO magnitude calibration word in thermometric code
    #[inline(always)]
    pub fn vco_calamp_ext(&mut self) -> VCO_CALAMP_EXT_W<'_, VCO_CAL_CONFIGrs> {
        VCO_CALAMP_EXT_W::new(self, 0)
    }
    ///Bit 15 - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field
    #[inline(always)]
    pub fn vco_calamp_ext_sel(&mut self) -> VCO_CALAMP_EXT_SEL_W<'_, VCO_CAL_CONFIGrs> {
        VCO_CALAMP_EXT_SEL_W::new(self, 15)
    }
    ///Bits 16:22 - VCO Cbank frequency calibration word.
    #[inline(always)]
    pub fn vco_calfreq_ext(&mut self) -> VCO_CALFREQ_EXT_W<'_, VCO_CAL_CONFIGrs> {
        VCO_CALFREQ_EXT_W::new(self, 16)
    }
    ///Bit 23 - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field
    #[inline(always)]
    pub fn vco_calfreq_ext_sel(&mut self) -> VCO_CALFREQ_EXT_SEL_W<'_, VCO_CAL_CONFIGrs> {
        VCO_CALFREQ_EXT_SEL_W::new(self, 23)
    }
    ///Bit 31 - Define if the Radio FSM must launch a VCO calibration request after VCO start-up
    #[inline(always)]
    pub fn vco_calib_req(&mut self) -> VCO_CALIB_REQ_W<'_, VCO_CAL_CONFIGrs> {
        VCO_CALIB_REQ_W::new(self, 31)
    }
}
/**VCO_CAL_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`vco_cal_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco_cal_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:VCO_CAL_CONFIG)*/
pub struct VCO_CAL_CONFIGrs;
impl crate::RegisterSpec for VCO_CAL_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`vco_cal_config::R`](R) reader structure
impl crate::Readable for VCO_CAL_CONFIGrs {}
///`write(|w| ..)` method takes [`vco_cal_config::W`](W) writer structure
impl crate::Writable for VCO_CAL_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VCO_CAL_CONFIG to value 0x0040_0088
impl crate::Resettable for VCO_CAL_CONFIGrs {
    const RESET_VALUE: u32 = 0x0040_0088;
}
