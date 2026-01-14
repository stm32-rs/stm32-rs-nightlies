///Register `ENGTRIM` reader
pub type R = crate::R<ENGTRIMrs>;
///Register `ENGTRIM` writer
pub type W = crate::W<ENGTRIMrs>;
///Field `TRIMRFDREGEN` reader - RFD_REG_TRIM software overwrite enable.
pub type TRIMRFDREGEN_R = crate::BitReader;
///Field `TRIMRFDREGEN` writer - RFD_REG_TRIM software overwrite enable.
pub type TRIMRFDREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFD_REG_TRIM` reader - RF LDO trimming chosen by the software.
pub type RFD_REG_TRIM_R = crate::FieldReader;
///Field `RFD_REG_TRIM` writer - RF LDO trimming chosen by the software.
pub type RFD_REG_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIMMREN` reader - TRIM_MR software overwrite enable.
pub type TRIMMREN_R = crate::BitReader;
///Field `TRIMMREN` writer - TRIM_MR software overwrite enable.
pub type TRIMMREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIM_MR` reader - Main regulator voltage trimming chosen by the software.
pub type TRIM_MR_R = crate::FieldReader;
///Field `TRIM_MR` writer - Main regulator voltage trimming chosen by the software.
pub type TRIM_MR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SMPSTRIMEN` reader - SMPS_TRIM software overwrite enable.
pub type SMPSTRIMEN_R = crate::BitReader;
///Field `SMPSTRIMEN` writer - SMPS_TRIM software overwrite enable.
pub type SMPSTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPS_TRIM` reader - SMPS output voltage trimming chosen by the software.
pub type SMPS_TRIM_R = crate::FieldReader;
///Field `SMPS_TRIM` writer - SMPS output voltage trimming chosen by the software.
pub type SMPS_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - RFD_REG_TRIM software overwrite enable.
    #[inline(always)]
    pub fn trimrfdregen(&self) -> TRIMRFDREGEN_R {
        TRIMRFDREGEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - RF LDO trimming chosen by the software.
    #[inline(always)]
    pub fn rfd_reg_trim(&self) -> RFD_REG_TRIM_R {
        RFD_REG_TRIM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - TRIM_MR software overwrite enable.
    #[inline(always)]
    pub fn trimmren(&self) -> TRIMMREN_R {
        TRIMMREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:9 - Main regulator voltage trimming chosen by the software.
    #[inline(always)]
    pub fn trim_mr(&self) -> TRIM_MR_R {
        TRIM_MR_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 10 - SMPS_TRIM software overwrite enable.
    #[inline(always)]
    pub fn smpstrimen(&self) -> SMPSTRIMEN_R {
        SMPSTRIMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - SMPS output voltage trimming chosen by the software.
    #[inline(always)]
    pub fn smps_trim(&self) -> SMPS_TRIM_R {
        SMPS_TRIM_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENGTRIM")
            .field("trimrfdregen", &self.trimrfdregen())
            .field("rfd_reg_trim", &self.rfd_reg_trim())
            .field("trimmren", &self.trimmren())
            .field("trim_mr", &self.trim_mr())
            .field("smpstrimen", &self.smpstrimen())
            .field("smps_trim", &self.smps_trim())
            .finish()
    }
}
impl W {
    ///Bit 0 - RFD_REG_TRIM software overwrite enable.
    #[inline(always)]
    pub fn trimrfdregen(&mut self) -> TRIMRFDREGEN_W<'_, ENGTRIMrs> {
        TRIMRFDREGEN_W::new(self, 0)
    }
    ///Bits 1:4 - RF LDO trimming chosen by the software.
    #[inline(always)]
    pub fn rfd_reg_trim(&mut self) -> RFD_REG_TRIM_W<'_, ENGTRIMrs> {
        RFD_REG_TRIM_W::new(self, 1)
    }
    ///Bit 5 - TRIM_MR software overwrite enable.
    #[inline(always)]
    pub fn trimmren(&mut self) -> TRIMMREN_W<'_, ENGTRIMrs> {
        TRIMMREN_W::new(self, 5)
    }
    ///Bits 6:9 - Main regulator voltage trimming chosen by the software.
    #[inline(always)]
    pub fn trim_mr(&mut self) -> TRIM_MR_W<'_, ENGTRIMrs> {
        TRIM_MR_W::new(self, 6)
    }
    ///Bit 10 - SMPS_TRIM software overwrite enable.
    #[inline(always)]
    pub fn smpstrimen(&mut self) -> SMPSTRIMEN_W<'_, ENGTRIMrs> {
        SMPSTRIMEN_W::new(self, 10)
    }
    ///Bits 11:13 - SMPS output voltage trimming chosen by the software.
    #[inline(always)]
    pub fn smps_trim(&mut self) -> SMPS_TRIM_W<'_, ENGTRIMrs> {
        SMPS_TRIM_W::new(self, 11)
    }
}
/**This register allows the software to overwrite the hardware trimming values.

You can [`read`](crate::Reg::read) this register and get [`engtrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:ENGTRIM)*/
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
