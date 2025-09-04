///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
///Field `SMPSLVL` reader - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
pub type SMPSLVL_R = crate::FieldReader;
///Field `SMPSLVL` writer - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
pub type SMPSLVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPSBOMSEL` reader - SMPSBOMSEL: SMPS BOM Selection:
pub type SMPSBOMSEL_R = crate::FieldReader;
///Field `SMPSBOMSEL` writer - SMPSBOMSEL: SMPS BOM Selection:
pub type SMPSBOMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SMPSFRDY` reader - SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready .
pub type SMPSFRDY_R = crate::BitReader;
///Field `SMPSFRDY` writer - SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready .
pub type SMPSFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSLPOPEN` reader - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed.
pub type SMPSLPOPEN_R = crate::BitReader;
///Field `SMPSLPOPEN` writer - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed.
pub type SMPSLPOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSFBYP` reader - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR.
pub type SMPSFBYP_R = crate::BitReader;
///Field `SMPSFBYP` writer - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR.
pub type SMPSFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSMPS` reader - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM.
pub type NOSMPS_R = crate::BitReader;
///Field `NOSMPS` writer - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM.
pub type NOSMPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_ENA_DCM` reader - SMPS_ENA_DCM: enable discontinuous conduction mode
pub type SMPS_ENA_DCM_R = crate::BitReader;
///Field `SMPS_ENA_DCM` writer - SMPS_ENA_DCM: enable discontinuous conduction mode
pub type SMPS_ENA_DCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKDETR_DISABLE` reader - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock.
pub type CLKDETR_DISABLE_R = crate::BitReader;
///Field `CLKDETR_DISABLE` writer - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock.
pub type CLKDETR_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_PRECH_CUR_SEL` reader - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current
pub type SMPS_PRECH_CUR_SEL_R = crate::FieldReader;
///Field `SMPS_PRECH_CUR_SEL` writer - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current
pub type SMPS_PRECH_CUR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
    #[inline(always)]
    pub fn smpslvl(&self) -> SMPSLVL_R {
        SMPSLVL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection:
    #[inline(always)]
    pub fn smpsbomsel(&self) -> SMPSBOMSEL_R {
        SMPSBOMSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready .
    #[inline(always)]
    pub fn smpsfrdy(&self) -> SMPSFRDY_R {
        SMPSFRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed.
    #[inline(always)]
    pub fn smpslpopen(&self) -> SMPSLPOPEN_R {
        SMPSLPOPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR.
    #[inline(always)]
    pub fn smpsfbyp(&self) -> SMPSFBYP_R {
        SMPSFBYP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM.
    #[inline(always)]
    pub fn nosmps(&self) -> NOSMPS_R {
        NOSMPS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode
    #[inline(always)]
    pub fn smps_ena_dcm(&self) -> SMPS_ENA_DCM_R {
        SMPS_ENA_DCM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock.
    #[inline(always)]
    pub fn clkdetr_disable(&self) -> CLKDETR_DISABLE_R {
        CLKDETR_DISABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current
    #[inline(always)]
    pub fn smps_prech_cur_sel(&self) -> SMPS_PRECH_CUR_SEL_R {
        SMPS_PRECH_CUR_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("smpslvl", &self.smpslvl())
            .field("smpsbomsel", &self.smpsbomsel())
            .field("smpsfrdy", &self.smpsfrdy())
            .field("smpslpopen", &self.smpslpopen())
            .field("smpsfbyp", &self.smpsfbyp())
            .field("nosmps", &self.nosmps())
            .field("smps_ena_dcm", &self.smps_ena_dcm())
            .field("clkdetr_disable", &self.clkdetr_disable())
            .field("smps_prech_cur_sel", &self.smps_prech_cur_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
    #[inline(always)]
    pub fn smpslvl(&mut self) -> SMPSLVL_W<CR5rs> {
        SMPSLVL_W::new(self, 0)
    }
    ///Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection:
    #[inline(always)]
    pub fn smpsbomsel(&mut self) -> SMPSBOMSEL_W<CR5rs> {
        SMPSBOMSEL_W::new(self, 4)
    }
    ///Bit 7 - SMPSFB Force ready check When this bit is set, the SMPS FSM will consider the SMPS ready .
    #[inline(always)]
    pub fn smpsfrdy(&mut self) -> SMPSFRDY_W<CR5rs> {
        SMPSFRDY_W::new(self, 7)
    }
    ///Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed.
    #[inline(always)]
    pub fn smpslpopen(&mut self) -> SMPSLPOPEN_W<CR5rs> {
        SMPSLPOPEN_W::new(self, 8)
    }
    ///Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR.
    #[inline(always)]
    pub fn smpsfbyp(&mut self) -> SMPSFBYP_W<CR5rs> {
        SMPSFBYP_W::new(self, 9)
    }
    ///Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM.
    #[inline(always)]
    pub fn nosmps(&mut self) -> NOSMPS_W<CR5rs> {
        NOSMPS_W::new(self, 10)
    }
    ///Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode
    #[inline(always)]
    pub fn smps_ena_dcm(&mut self) -> SMPS_ENA_DCM_W<CR5rs> {
        SMPS_ENA_DCM_W::new(self, 11)
    }
    ///Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock.
    #[inline(always)]
    pub fn clkdetr_disable(&mut self) -> CLKDETR_DISABLE_W<CR5rs> {
        CLKDETR_DISABLE_W::new(self, 12)
    }
    ///Bits 13:14 - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current
    #[inline(always)]
    pub fn smps_prech_cur_sel(&mut self) -> SMPS_PRECH_CUR_SEL_W<CR5rs> {
        SMPS_PRECH_CUR_SEL_W::new(self, 13)
    }
}
/**CR5 register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0x6014
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x6014;
}
