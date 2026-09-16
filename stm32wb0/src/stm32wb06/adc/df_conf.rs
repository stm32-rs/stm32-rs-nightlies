///Register `DF_CONF` reader
pub type R = crate::R<DF_CONFrs>;
///Register `DF_CONF` writer
pub type W = crate::W<DF_CONFrs>;
///Field `DF_CIC_DEC_FACTOR` reader -
pub type DF_CIC_DEC_FACTOR_R = crate::FieldReader;
///Field `DF_CIC_DEC_FACTOR` writer -
pub type DF_CIC_DEC_FACTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DF_CIC_DHF` reader - CIC filter decimator half factor
pub type DF_CIC_DHF_R = crate::BitReader;
///Field `DF_CIC_DHF` writer - CIC filter decimator half factor
pub type DF_CIC_DHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_ITP1P2` reader - 1.2 fractional interpolator enable
pub type DF_ITP1P2_R = crate::BitReader;
///Field `DF_ITP1P2` writer - 1.2 fractional interpolator enable
pub type DF_ITP1P2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_I_U2S` reader - select signed/unsigned format for input
pub type DF_I_U2S_R = crate::BitReader;
///Field `DF_I_U2S` writer - select signed/unsigned format for input
pub type DF_I_U2S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_O_S2U` reader - select signed/unsigned format for data output
pub type DF_O_S2U_R = crate::BitReader;
///Field `DF_O_S2U` writer - select signed/unsigned format for data output
pub type DF_O_S2U_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDM_RATE` reader - select the PDM clock rate.
pub type PDM_RATE_R = crate::FieldReader;
///Field `PDM_RATE` writer - select the PDM clock rate.
pub type PDM_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DF_MICROL_RN` reader - left/right channel selection on digital microphone
pub type DF_MICROL_RN_R = crate::BitReader;
///Field `DF_MICROL_RN` writer - left/right channel selection on digital microphone
pub type DF_MICROL_RN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_HPF_EN` reader - high pass filter enable.
pub type DF_HPF_EN_R = crate::BitReader;
///Field `DF_HPF_EN` writer - high pass filter enable.
pub type DF_HPF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_HALF_D_EN` reader - half dynamic enable.
pub type DF_HALF_D_EN_R = crate::BitReader;
///Field `DF_HALF_D_EN` writer - half dynamic enable.
pub type DF_HALF_D_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn df_cic_dec_factor(&self) -> DF_CIC_DEC_FACTOR_R {
        DF_CIC_DEC_FACTOR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - CIC filter decimator half factor
    #[inline(always)]
    pub fn df_cic_dhf(&self) -> DF_CIC_DHF_R {
        DF_CIC_DHF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - 1.2 fractional interpolator enable
    #[inline(always)]
    pub fn df_itp1p2(&self) -> DF_ITP1P2_R {
        DF_ITP1P2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - select signed/unsigned format for input
    #[inline(always)]
    pub fn df_i_u2s(&self) -> DF_I_U2S_R {
        DF_I_U2S_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - select signed/unsigned format for data output
    #[inline(always)]
    pub fn df_o_s2u(&self) -> DF_O_S2U_R {
        DF_O_S2U_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:14 - select the PDM clock rate.
    #[inline(always)]
    pub fn pdm_rate(&self) -> PDM_RATE_R {
        PDM_RATE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - left/right channel selection on digital microphone
    #[inline(always)]
    pub fn df_microl_rn(&self) -> DF_MICROL_RN_R {
        DF_MICROL_RN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - high pass filter enable.
    #[inline(always)]
    pub fn df_hpf_en(&self) -> DF_HPF_EN_R {
        DF_HPF_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - half dynamic enable.
    #[inline(always)]
    pub fn df_half_d_en(&self) -> DF_HALF_D_EN_R {
        DF_HALF_D_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DF_CONF")
            .field("df_half_d_en", &self.df_half_d_en())
            .field("df_hpf_en", &self.df_hpf_en())
            .field("df_microl_rn", &self.df_microl_rn())
            .field("pdm_rate", &self.pdm_rate())
            .field("df_o_s2u", &self.df_o_s2u())
            .field("df_i_u2s", &self.df_i_u2s())
            .field("df_itp1p2", &self.df_itp1p2())
            .field("df_cic_dhf", &self.df_cic_dhf())
            .field("df_cic_dec_factor", &self.df_cic_dec_factor())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    pub fn df_cic_dec_factor(&mut self) -> DF_CIC_DEC_FACTOR_W<'_, DF_CONFrs> {
        DF_CIC_DEC_FACTOR_W::new(self, 0)
    }
    ///Bit 7 - CIC filter decimator half factor
    #[inline(always)]
    pub fn df_cic_dhf(&mut self) -> DF_CIC_DHF_W<'_, DF_CONFrs> {
        DF_CIC_DHF_W::new(self, 7)
    }
    ///Bit 8 - 1.2 fractional interpolator enable
    #[inline(always)]
    pub fn df_itp1p2(&mut self) -> DF_ITP1P2_W<'_, DF_CONFrs> {
        DF_ITP1P2_W::new(self, 8)
    }
    ///Bit 9 - select signed/unsigned format for input
    #[inline(always)]
    pub fn df_i_u2s(&mut self) -> DF_I_U2S_W<'_, DF_CONFrs> {
        DF_I_U2S_W::new(self, 9)
    }
    ///Bit 10 - select signed/unsigned format for data output
    #[inline(always)]
    pub fn df_o_s2u(&mut self) -> DF_O_S2U_W<'_, DF_CONFrs> {
        DF_O_S2U_W::new(self, 10)
    }
    ///Bits 11:14 - select the PDM clock rate.
    #[inline(always)]
    pub fn pdm_rate(&mut self) -> PDM_RATE_W<'_, DF_CONFrs> {
        PDM_RATE_W::new(self, 11)
    }
    ///Bit 15 - left/right channel selection on digital microphone
    #[inline(always)]
    pub fn df_microl_rn(&mut self) -> DF_MICROL_RN_W<'_, DF_CONFrs> {
        DF_MICROL_RN_W::new(self, 15)
    }
    ///Bit 16 - high pass filter enable.
    #[inline(always)]
    pub fn df_hpf_en(&mut self) -> DF_HPF_EN_W<'_, DF_CONFrs> {
        DF_HPF_EN_W::new(self, 16)
    }
    ///Bit 17 - half dynamic enable.
    #[inline(always)]
    pub fn df_half_d_en(&mut self) -> DF_HALF_D_EN_W<'_, DF_CONFrs> {
        DF_HALF_D_EN_W::new(self, 17)
    }
}
/**Decimation filter configuration register

You can [`read`](crate::Reg::read) this register and get [`df_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`df_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:DF_CONF)*/
pub struct DF_CONFrs;
impl crate::RegisterSpec for DF_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`df_conf::R`](R) reader structure
impl crate::Readable for DF_CONFrs {}
///`write(|w| ..)` method takes [`df_conf::W`](W) writer structure
impl crate::Writable for DF_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DF_CONF to value 0x3015
impl crate::Resettable for DF_CONFrs {
    const RESET_VALUE: u32 = 0x3015;
}
