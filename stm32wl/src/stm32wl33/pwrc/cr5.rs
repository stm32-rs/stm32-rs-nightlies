///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
///Field `SMPSLVL` reader - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
pub type SMPSLVL_R = crate::FieldReader;
///Field `SMPSLVL` writer - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
pub type SMPSLVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPSBOMSEL` reader - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a
pub type SMPSBOMSEL_R = crate::FieldReader;
///Field `SMPSBOMSEL` writer - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a
pub type SMPSBOMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SMPS_BOF_STATIC` reader - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)
pub type SMPS_BOF_STATIC_R = crate::BitReader;
///Field `SMPS_BOF_STATIC` writer - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)
pub type SMPS_BOF_STATIC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSMPS_BOF` reader - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;
pub type NOSMPS_BOF_R = crate::BitReader;
///Field `NOSMPS_BOF` writer - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;
pub type NOSMPS_BOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSLPOPEN` reader - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating
pub type SMPSLPOPEN_R = crate::BitReader;
///Field `SMPSLPOPEN` writer - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating
pub type SMPSLPOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSFBYP` reader - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)
pub type SMPSFBYP_R = crate::BitReader;
///Field `SMPSFBYP` writer - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)
pub type SMPSFBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSMPS` reader - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;
pub type NOSMPS_R = crate::BitReader;
///Field `NOSMPS` writer - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;
pub type NOSMPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_ENA_DCM` reader - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable
pub type SMPS_ENA_DCM_R = crate::BitReader;
///Field `SMPS_ENA_DCM` writer - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable
pub type SMPS_ENA_DCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKDETR_DISABLE` reader - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled
pub type CLKDETR_DISABLE_R = crate::BitReader;
///Field `CLKDETR_DISABLE` writer - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled
pub type CLKDETR_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_PRECH_CUR_SEL` reader - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)
pub type SMPS_PRECH_CUR_SEL_R = crate::FieldReader;
///Field `SMPS_PRECH_CUR_SEL` writer - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)
pub type SMPS_PRECH_CUR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SMPS_BOF_DYN` reader - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)
pub type SMPS_BOF_DYN_R = crate::BitReader;
///Field `SMPS_BOF_DYN` writer - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)
pub type SMPS_BOF_DYN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
    #[inline(always)]
    pub fn smpslvl(&self) -> SMPSLVL_R {
        SMPSLVL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a
    #[inline(always)]
    pub fn smpsbomsel(&self) -> SMPSBOMSEL_R {
        SMPSBOMSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)
    #[inline(always)]
    pub fn smps_bof_static(&self) -> SMPS_BOF_STATIC_R {
        SMPS_BOF_STATIC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;
    #[inline(always)]
    pub fn nosmps_bof(&self) -> NOSMPS_BOF_R {
        NOSMPS_BOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating
    #[inline(always)]
    pub fn smpslpopen(&self) -> SMPSLPOPEN_R {
        SMPSLPOPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)
    #[inline(always)]
    pub fn smpsfbyp(&self) -> SMPSFBYP_R {
        SMPSFBYP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;
    #[inline(always)]
    pub fn nosmps(&self) -> NOSMPS_R {
        NOSMPS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable
    #[inline(always)]
    pub fn smps_ena_dcm(&self) -> SMPS_ENA_DCM_R {
        SMPS_ENA_DCM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled
    #[inline(always)]
    pub fn clkdetr_disable(&self) -> CLKDETR_DISABLE_R {
        CLKDETR_DISABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)
    #[inline(always)]
    pub fn smps_prech_cur_sel(&self) -> SMPS_PRECH_CUR_SEL_R {
        SMPS_PRECH_CUR_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)
    #[inline(always)]
    pub fn smps_bof_dyn(&self) -> SMPS_BOF_DYN_R {
        SMPS_BOF_DYN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("smpslvl", &self.smpslvl())
            .field("smpsbomsel", &self.smpsbomsel())
            .field("smps_bof_static", &self.smps_bof_static())
            .field("nosmps_bof", &self.nosmps_bof())
            .field("smpslpopen", &self.smpslpopen())
            .field("smpsfbyp", &self.smpsfbyp())
            .field("nosmps", &self.nosmps())
            .field("smps_ena_dcm", &self.smps_ena_dcm())
            .field("clkdetr_disable", &self.clkdetr_disable())
            .field("smps_prech_cur_sel", &self.smps_prech_cur_sel())
            .field("smps_bof_dyn", &self.smps_bof_dyn())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SMPSLVL\[3:0\] SMPS Output Level Voltage Selection Select the SMPS output voltage with a granularity of 50mV. Default = '0100' (1.4V) Vout = 1.2 + 0.05*SMPSOUT (V)
    #[inline(always)]
    pub fn smpslvl(&mut self) -> SMPSLVL_W<'_, CR5rs> {
        SMPSLVL_W::new(self, 0)
    }
    ///Bits 4:5 - SMPSBOMSEL: SMPS BOM Selection: - 00: BOM1 - 01: BOM2 (default) - 10: BOM3 - 11: n/a
    #[inline(always)]
    pub fn smpsbomsel(&mut self) -> SMPSBOMSEL_W<'_, CR5rs> {
        SMPSBOMSEL_W::new(self, 4)
    }
    ///Bit 6 - SMPS_BOF_STATIC: SMPS Bypass on the Fly static - 0 : disabled (by default) - 1 : SMPS Bypass on the fly static is enabled (EN_SW=1)
    #[inline(always)]
    pub fn smps_bof_static(&mut self) -> SMPS_BOF_STATIC_W<'_, CR5rs> {
        SMPS_BOF_STATIC_W::new(self, 6)
    }
    ///Bit 7 - NOSMPS_BOF: No SMPS Mode to be used in accordance to SMPS_BOF_STATIC =1 When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only SMPS_BOF_STATIC=1. - 0 : No effect, SMPS is enabled. (default) - 1 : SMPS is disabled;
    #[inline(always)]
    pub fn nosmps_bof(&mut self) -> NOSMPS_BOF_W<'_, CR5rs> {
        NOSMPS_BOF_W::new(self, 7)
    }
    ///Bit 8 - SMPSLPOPEN: In Low Power mode SMPS is in OPEN mode (instead of PRECHARGE mode). When this bit is set, when the chip is in Low power mode the SMPS regulator will be disabled (HZ) Documentation needed. - 0 : in Low Power mode, SMPS is in PRECHARGE, output is connected to VDDIO. (default) - 1 : in Low Power mode, SMPS is disabled, output is floating
    #[inline(always)]
    pub fn smpslpopen(&mut self) -> SMPSLPOPEN_W<'_, CR5rs> {
        SMPSLPOPEN_W::new(self, 8)
    }
    ///Bit 9 - SMPSFB Force SMPS Regulator in bypass mode When this bit is set, the SMPS regulator will be forced to operate in precharge mode. the actual state of SMPS can be observed thanks to the replica SR2.SMPSBYPR. - 0 : no effect (by default) - 1 : SMPS is disabled and bypassed (ENABLE_3V3=0 and PRECHARGE_3V3=1)
    #[inline(always)]
    pub fn smpsfbyp(&mut self) -> SMPSFBYP_W<'_, CR5rs> {
        SMPSFBYP_W::new(self, 9)
    }
    ///Bit 10 - NOSMPS: No SMPS Mode When this bit is set, the SMPS regulator will be disabled. Note that this configuration should be used only when SMPS_FB pad is directly connected to VBATT or Vext, without L/C BOM. - 0 : No effect, SMPS is enabled. (Default) - 1 : SMPS is disabled;
    #[inline(always)]
    pub fn nosmps(&mut self) -> NOSMPS_W<'_, CR5rs> {
        NOSMPS_W::new(self, 10)
    }
    ///Bit 11 - SMPS_ENA_DCM: enable discontinuous conduction mode - 0 : disable (Default) - 1 : enable
    #[inline(always)]
    pub fn smps_ena_dcm(&mut self) -> SMPS_ENA_DCM_W<'_, CR5rs> {
        SMPS_ENA_DCM_W::new(self, 11)
    }
    ///Bit 12 - CLKDETR_DISABLE: disable SMPS clock detection The SMPS clock detection enables an automatic SMPS bypass switching in case of unwanted loss of SMPS clock. - 0 : SMPS clock detection enabled (default) - 1 : SMPS clock detection disabled
    #[inline(always)]
    pub fn clkdetr_disable(&mut self) -> CLKDETR_DISABLE_W<'_, CR5rs> {
        CLKDETR_DISABLE_W::new(self, 12)
    }
    ///Bits 13:14 - SMPS_PRECH_CUR_SEL\[1:0\] Selection for SMPS PRECHARGE limit current - 00: 2.5mA - 01: 5mA - 10: 10mA - 11: 20mA (default)
    #[inline(always)]
    pub fn smps_prech_cur_sel(&mut self) -> SMPS_PRECH_CUR_SEL_W<'_, CR5rs> {
        SMPS_PRECH_CUR_SEL_W::new(self, 13)
    }
    ///Bit 15 - SMPS_BOF_DYN: SMPS Bypass on the Fly dynamic - 0 : disabled (by default) - 1 : SMPS Bypass on the fly dynamic is enabled (EN_LDO=1)
    #[inline(always)]
    pub fn smps_bof_dyn(&mut self) -> SMPS_BOF_DYN_W<'_, CR5rs> {
        SMPS_BOF_DYN_W::new(self, 15)
    }
}
/**CR5 register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:CR5)*/
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
