///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `SMPSBYPR` reader - SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state.
pub type SMPSBYPR_R = crate::BitReader;
///Field `SMPSENR` reader - SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state.
pub type SMPSENR_R = crate::BitReader;
///Field `SMPSRDY` reader - SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready.
pub type SMPSRDY_R = crate::BitReader;
///Field `IOBOOTVAL2` reader - Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR
pub type IOBOOTVAL2_R = crate::FieldReader;
///Field `REGLPS` reader - REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready.
pub type REGLPS_R = crate::BitReader;
///Field `REGMS` reader - REGMS: Regulator Main LDO Started This bit provides the information whether main regulator is ready.
pub type REGMS_R = crate::BitReader;
///Field `PVDO` reader - PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)
pub type PVDO_R = crate::BitReader;
///Field `IOBOOTVAL` reader - Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR
pub type IOBOOTVAL_R = crate::FieldReader;
impl R {
    ///Bit 0 - SMPSBYPR: SMPS Force Bypass Control Replica This bit mirrors the actual BYPASS_3V3 control signal driven to the SMPS regulator, dependant on the real working state.
    #[inline(always)]
    pub fn smpsbypr(&self) -> SMPSBYPR_R {
        SMPSBYPR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SMPSENR: SMPS Enable Control Replica This bit mirrors the actual ENABLE_3V3 control signal driven to the SMPS regulator, dependant on the real working state.
    #[inline(always)]
    pub fn smpsenr(&self) -> SMPSENR_R {
        SMPSENR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SMPSRDY: SMPS Ready Status This bit provides the information whether SMPS is ready.
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - Bit3: PB15 input value on VDD33 latched at POR Bit2: PB14 input value on VDD33 latched at POR Bit1: PB13 input value on VDD33 latched at POR Bit0: PB12 input value on VDD33 latched at POR
    #[inline(always)]
    pub fn iobootval2(&self) -> IOBOOTVAL2_R {
        IOBOOTVAL2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - REGLPS: Regulator Low Power Started This bit provides the information whether low power regulator is ready.
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - REGMS: Regulator Main LDO Started This bit provides the information whether main regulator is ready.
    #[inline(always)]
    pub fn regms(&self) -> REGMS_R {
        REGMS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - PVDO: Power Voltage Detector Output When the Power Voltage Detector is enabled (CR2.PVDE) this bit is set when the system supply (VDDIO) is lower than the selected PVD threshold (CR2.PVDLS)
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:15 - Bit3: PA11 input value on VDD33 latched at POR Bit2: PA10 input value on VDD33 latched at POR Bit1: PA9 input value on VDD33 latched at POR Bit0: PA8 input value on VDD33 latched at POR
    #[inline(always)]
    pub fn iobootval(&self) -> IOBOOTVAL_R {
        IOBOOTVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("smpsbypr", &self.smpsbypr())
            .field("smpsenr", &self.smpsenr())
            .field("smpsrdy", &self.smpsrdy())
            .field("iobootval2", &self.iobootval2())
            .field("reglps", &self.reglps())
            .field("regms", &self.regms())
            .field("pvdo", &self.pvdo())
            .field("iobootval", &self.iobootval())
            .finish()
    }
}
/**SR2 register

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0x0306
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0x0306;
}
