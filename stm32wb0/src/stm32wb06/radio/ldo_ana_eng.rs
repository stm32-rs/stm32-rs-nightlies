///Register `LDO_ANA_ENG` reader
pub type R = crate::R<LDO_ANA_ENGrs>;
///Register `LDO_ANA_ENG` writer
pub type W = crate::W<LDO_ANA_ENGrs>;
///Field `RFD_RF_REG_BYPASS` reader - RF_REG Bypass mode:
pub type RFD_RF_REG_BYPASS_R = crate::BitReader;
///Field `RFD_RF_REG_BYPASS` writer - RF_REG Bypass mode:
pub type RFD_RF_REG_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RF_REG Bypass mode:
    #[inline(always)]
    pub fn rfd_rf_reg_bypass(&self) -> RFD_RF_REG_BYPASS_R {
        RFD_RF_REG_BYPASS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_ANA_ENG")
            .field("rfd_rf_reg_bypass", &self.rfd_rf_reg_bypass())
            .finish()
    }
}
impl W {
    ///Bit 0 - RF_REG Bypass mode:
    #[inline(always)]
    pub fn rfd_rf_reg_bypass(&mut self) -> RFD_RF_REG_BYPASS_W<'_, LDO_ANA_ENGrs> {
        RFD_RF_REG_BYPASS_W::new(self, 0)
    }
}
/**LDO_ANA_ENG register

You can [`read`](crate::Reg::read) this register and get [`ldo_ana_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo_ana_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:LDO_ANA_ENG)*/
pub struct LDO_ANA_ENGrs;
impl crate::RegisterSpec for LDO_ANA_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`ldo_ana_eng::R`](R) reader structure
impl crate::Readable for LDO_ANA_ENGrs {}
///`write(|w| ..)` method takes [`ldo_ana_eng::W`](W) writer structure
impl crate::Writable for LDO_ANA_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LDO_ANA_ENG to value 0
impl crate::Resettable for LDO_ANA_ENGrs {}
