///Register `ENGTRIM` reader
pub type R = crate::R<ENGTRIMrs>;
///Register `ENGTRIM` writer
pub type W = crate::W<ENGTRIMrs>;
///Field `TRIMRFDREGEN` reader - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type TRIMRFDREGEN_R = crate::BitReader;
///Field `TRIMRFDREGEN` writer - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type TRIMRFDREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIM_RFDREG` reader - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register.
pub type TRIM_RFDREG_R = crate::FieldReader;
///Field `TRIM_RFDREG` writer - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register.
pub type TRIM_RFDREG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPARE` reader -
pub type SPARE_R = crate::BitReader;
///Field `SPARE` writer -
pub type SPARE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIMMREN` reader - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type TRIMMREN_R = crate::BitReader;
///Field `TRIMMREN` writer - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type TRIMMREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIM_MR` reader - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register.
pub type TRIM_MR_R = crate::FieldReader;
///Field `TRIM_MR` writer - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register.
pub type TRIM_MR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPSTRIMEN` reader - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type SMPSTRIMEN_R = crate::BitReader;
///Field `SMPSTRIMEN` writer - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
pub type SMPSTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_TRIM` reader - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register.
pub type SMPS_TRIM_R = crate::FieldReader;
///Field `SMPS_TRIM` writer - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register.
pub type SMPS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn trimrfdregen(&self) -> TRIMRFDREGEN_R {
        TRIMRFDREGEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register.
    #[inline(always)]
    pub fn trim_rfdreg(&self) -> TRIM_RFDREG_R {
        TRIM_RFDREG_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn trimmren(&self) -> TRIMMREN_R {
        TRIMMREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:9 - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register.
    #[inline(always)]
    pub fn trim_mr(&self) -> TRIM_MR_R {
        TRIM_MR_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 10 - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn smpstrimen(&self) -> SMPSTRIMEN_R {
        SMPSTRIMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register.
    #[inline(always)]
    pub fn smps_trim(&self) -> SMPS_TRIM_R {
        SMPS_TRIM_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENGTRIM")
            .field("trimrfdregen", &self.trimrfdregen())
            .field("trim_rfdreg", &self.trim_rfdreg())
            .field("spare", &self.spare())
            .field("trimmren", &self.trimmren())
            .field("trim_mr", &self.trim_mr())
            .field("smpstrimen", &self.smpstrimen())
            .field("smps_trim", &self.smps_trim())
            .finish()
    }
}
impl W {
    ///Bit 0 - TRIMRFDREGEN: trimming RFREG enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn trimrfdregen(&mut self) -> TRIMRFDREGEN_W<'_, ENGTRIMrs> {
        TRIMRFDREGEN_W::new(self, 0)
    }
    ///Bits 1:3 - TRIM_RFDREG: RF Regulator Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMRFDREGEN=1, the startup current can be controlled by this register.
    #[inline(always)]
    pub fn trim_rfdreg(&mut self) -> TRIM_RFDREG_W<'_, ENGTRIMrs> {
        TRIM_RFDREG_W::new(self, 1)
    }
    ///Bit 4
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W<'_, ENGTRIMrs> {
        SPARE_W::new(self, 4)
    }
    ///Bit 5 - TRIMMREN: trimming MR enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn trimmren(&mut self) -> TRIMMREN_W<'_, ENGTRIMrs> {
        TRIMMREN_W::new(self, 5)
    }
    ///Bits 6:9 - TRIM_MR: Main Regulator Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.TRIMMREN=1, the startup current can be controlled by this register.
    #[inline(always)]
    pub fn trim_mr(&mut self) -> TRIM_MR_W<'_, ENGTRIMrs> {
        TRIM_MR_W::new(self, 6)
    }
    ///Bit 10 - SMPSTRIMEN: trimming SMPS enabled - 1: trimming bit applied from ENGTRIM register - 0: trimming bit applied from OBL (can be read on TRIMR register)
    #[inline(always)]
    pub fn smpstrimen(&mut self) -> SMPSTRIMEN_W<'_, ENGTRIMrs> {
        SMPSTRIMEN_W::new(self, 10)
    }
    ///Bits 11:13 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.SMPSTRIMEN=1, the SMPS output voltage can be controlled by this register.
    #[inline(always)]
    pub fn smps_trim(&mut self) -> SMPS_TRIM_W<'_, ENGTRIMrs> {
        SMPS_TRIM_W::new(self, 11)
    }
}
/**ENGTRIM register

You can [`read`](crate::Reg::read) this register and get [`engtrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:ENGTRIM)*/
pub struct ENGTRIMrs;
impl crate::RegisterSpec for ENGTRIMrs {
    type Ux = u32;
}
///`read()` method returns [`engtrim::R`](R) reader structure
impl crate::Readable for ENGTRIMrs {}
///`write(|w| ..)` method takes [`engtrim::W`](W) writer structure
impl crate::Writable for ENGTRIMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ENGTRIM to value 0
impl crate::Resettable for ENGTRIMrs {}
