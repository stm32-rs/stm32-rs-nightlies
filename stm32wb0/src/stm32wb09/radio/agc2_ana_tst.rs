///Register `AGC2_ANA_TST` reader
pub type R = crate::R<AGC2_ANA_TSTrs>;
///Register `AGC2_ANA_TST` writer
pub type W = crate::W<AGC2_ANA_TSTrs>;
///Field `AGC2_ANA_TST_SEL` reader - Selection:
pub type AGC2_ANA_TST_SEL_R = crate::BitReader;
///Field `AGC2_ANA_TST_SEL` writer - Selection:
pub type AGC2_ANA_TST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_ANTENNAE_USR_TRIM` reader - the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)
pub type AGC_ANTENNAE_USR_TRIM_R = crate::FieldReader;
///Field `AGC_ANTENNAE_USR_TRIM` writer - the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)
pub type AGC_ANTENNAE_USR_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Selection:
    #[inline(always)]
    pub fn agc2_ana_tst_sel(&self) -> AGC2_ANA_TST_SEL_R {
        AGC2_ANA_TST_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)
    #[inline(always)]
    pub fn agc_antennae_usr_trim(&self) -> AGC_ANTENNAE_USR_TRIM_R {
        AGC_ANTENNAE_USR_TRIM_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC2_ANA_TST")
            .field("agc2_ana_tst_sel", &self.agc2_ana_tst_sel())
            .field("agc_antennae_usr_trim", &self.agc_antennae_usr_trim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Selection:
    #[inline(always)]
    pub fn agc2_ana_tst_sel(&mut self) -> AGC2_ANA_TST_SEL_W<'_, AGC2_ANA_TSTrs> {
        AGC2_ANA_TST_SEL_W::new(self, 0)
    }
    ///Bits 1:3 - the AGC antenna trimming value ( when AGC2_ANA_TST_SEL = 1)
    #[inline(always)]
    pub fn agc_antennae_usr_trim(&mut self) -> AGC_ANTENNAE_USR_TRIM_W<'_, AGC2_ANA_TSTrs> {
        AGC_ANTENNAE_USR_TRIM_W::new(self, 1)
    }
}
/**AGC2_ANA_TST register

You can [`read`](crate::Reg::read) this register and get [`agc2_ana_tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc2_ana_tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:AGC2_ANA_TST)*/
pub struct AGC2_ANA_TSTrs;
impl crate::RegisterSpec for AGC2_ANA_TSTrs {
    type Ux = u32;
}
///`read()` method returns [`agc2_ana_tst::R`](R) reader structure
impl crate::Readable for AGC2_ANA_TSTrs {}
///`write(|w| ..)` method takes [`agc2_ana_tst::W`](W) writer structure
impl crate::Writable for AGC2_ANA_TSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC2_ANA_TST to value 0
impl crate::Resettable for AGC2_ANA_TSTrs {}
