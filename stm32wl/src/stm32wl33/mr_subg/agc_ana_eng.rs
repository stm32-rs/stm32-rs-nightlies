///Register `AGC_ANA_ENG` reader
pub type R = crate::R<AGC_ANA_ENGrs>;
///Register `AGC_ANA_ENG` writer
pub type W = crate::W<AGC_ANA_ENGrs>;
///Field `FORCE_AGC_GAINS` reader - Select the mode for AGC analog part:
pub type FORCE_AGC_GAINS_R = crate::BitReader;
///Field `FORCE_AGC_GAINS` writer - Select the mode for AGC analog part:
pub type FORCE_AGC_GAINS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFD_RX_ATTEN_AGCGAIN` reader - Attenuation at LNA level by step of 6dB with thermometric code:
pub type RFD_RX_ATTEN_AGCGAIN_R = crate::FieldReader;
///Field `RFD_RX_ATTEN_AGCGAIN` writer - Attenuation at LNA level by step of 6dB with thermometric code:
pub type RFD_RX_ATTEN_AGCGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RFD_RX_PGA_AGCGAIN` reader - Attenuation at PGA level by step of 6dB with binary code:
pub type RFD_RX_PGA_AGCGAIN_R = crate::FieldReader;
///Field `RFD_RX_PGA_AGCGAIN` writer - Attenuation at PGA level by step of 6dB with binary code:
pub type RFD_RX_PGA_AGCGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Select the mode for AGC analog part:
    #[inline(always)]
    pub fn force_agc_gains(&self) -> FORCE_AGC_GAINS_R {
        FORCE_AGC_GAINS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - Attenuation at LNA level by step of 6dB with thermometric code:
    #[inline(always)]
    pub fn rfd_rx_atten_agcgain(&self) -> RFD_RX_ATTEN_AGCGAIN_R {
        RFD_RX_ATTEN_AGCGAIN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 5:7 - Attenuation at PGA level by step of 6dB with binary code:
    #[inline(always)]
    pub fn rfd_rx_pga_agcgain(&self) -> RFD_RX_PGA_AGCGAIN_R {
        RFD_RX_PGA_AGCGAIN_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC_ANA_ENG")
            .field("force_agc_gains", &self.force_agc_gains())
            .field("rfd_rx_atten_agcgain", &self.rfd_rx_atten_agcgain())
            .field("rfd_rx_pga_agcgain", &self.rfd_rx_pga_agcgain())
            .finish()
    }
}
impl W {
    ///Bit 0 - Select the mode for AGC analog part:
    #[inline(always)]
    pub fn force_agc_gains(&mut self) -> FORCE_AGC_GAINS_W<'_, AGC_ANA_ENGrs> {
        FORCE_AGC_GAINS_W::new(self, 0)
    }
    ///Bits 1:4 - Attenuation at LNA level by step of 6dB with thermometric code:
    #[inline(always)]
    pub fn rfd_rx_atten_agcgain(&mut self) -> RFD_RX_ATTEN_AGCGAIN_W<'_, AGC_ANA_ENGrs> {
        RFD_RX_ATTEN_AGCGAIN_W::new(self, 1)
    }
    ///Bits 5:7 - Attenuation at PGA level by step of 6dB with binary code:
    #[inline(always)]
    pub fn rfd_rx_pga_agcgain(&mut self) -> RFD_RX_PGA_AGCGAIN_W<'_, AGC_ANA_ENGrs> {
        RFD_RX_PGA_AGCGAIN_W::new(self, 5)
    }
}
/**AGC_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC_ANA_ENG)*/
pub struct AGC_ANA_ENGrs;
impl crate::RegisterSpec for AGC_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc_ana_eng::R`](R) reader structure
impl crate::Readable for AGC_ANA_ENGrs {}
///`write(|w| ..)` method takes [`agc_ana_eng::W`](W) writer structure
impl crate::Writable for AGC_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC_ANA_ENG to value 0
impl crate::Resettable for AGC_ANA_ENGrs {}
