///Register `SYNTH2_ANA_ENG` reader
pub type R = crate::R<SYNTH2_ANA_ENGrs>;
///Register `SYNTH2_ANA_ENG` writer
pub type W = crate::W<SYNTH2_ANA_ENGrs>;
///Field `RFD_PLL_VCO_ALC_AMP` reader - Select the level of max VCO amplitude in amplitude level control loop.
pub type RFD_PLL_VCO_ALC_AMP_R = crate::FieldReader;
///Field `RFD_PLL_VCO_ALC_AMP` writer - Select the level of max VCO amplitude in amplitude level control loop.
pub type RFD_PLL_VCO_ALC_AMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RFD_PLL_LD_WIN_ACC` reader - Select the PLL lock detector window selection:
pub type RFD_PLL_LD_WIN_ACC_R = crate::BitReader;
///Field `RFD_PLL_LD_WIN_ACC` writer - Select the PLL lock detector window selection:
pub type RFD_PLL_LD_WIN_ACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Select the level of max VCO amplitude in amplitude level control loop.
    #[inline(always)]
    pub fn rfd_pll_vco_alc_amp(&self) -> RFD_PLL_VCO_ALC_AMP_R {
        RFD_PLL_VCO_ALC_AMP_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Select the PLL lock detector window selection:
    #[inline(always)]
    pub fn rfd_pll_ld_win_acc(&self) -> RFD_PLL_LD_WIN_ACC_R {
        RFD_PLL_LD_WIN_ACC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTH2_ANA_ENG")
            .field("rfd_pll_vco_alc_amp", &self.rfd_pll_vco_alc_amp())
            .field("rfd_pll_ld_win_acc", &self.rfd_pll_ld_win_acc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Select the level of max VCO amplitude in amplitude level control loop.
    #[inline(always)]
    pub fn rfd_pll_vco_alc_amp(&mut self) -> RFD_PLL_VCO_ALC_AMP_W<'_, SYNTH2_ANA_ENGrs> {
        RFD_PLL_VCO_ALC_AMP_W::new(self, 0)
    }
    ///Bit 3 - Select the PLL lock detector window selection:
    #[inline(always)]
    pub fn rfd_pll_ld_win_acc(&mut self) -> RFD_PLL_LD_WIN_ACC_W<'_, SYNTH2_ANA_ENGrs> {
        RFD_PLL_LD_WIN_ACC_W::new(self, 3)
    }
}
/**SYNTH2_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`synth2_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synth2_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SYNTH2_ANA_ENG)*/
pub struct SYNTH2_ANA_ENGrs;
impl crate::RegisterSpec for SYNTH2_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`synth2_ana_eng::R`](R) reader structure
impl crate::Readable for SYNTH2_ANA_ENGrs {}
///`write(|w| ..)` method takes [`synth2_ana_eng::W`](W) writer structure
impl crate::Writable for SYNTH2_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTH2_ANA_ENG to value 0x4c
impl crate::Resettable for SYNTH2_ANA_ENGrs {
    const RESET_VALUE: u32 = 0x4c;
}
