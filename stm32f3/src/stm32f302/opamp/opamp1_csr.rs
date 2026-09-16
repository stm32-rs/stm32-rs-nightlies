///Register `OPAMP1_CSR` reader
pub type R = crate::R<OPAMP1_CSRrs>;
///Register `OPAMP1_CSR` writer
pub type W = crate::W<OPAMP1_CSRrs>;
///Field `OPAMP1EN` reader - OPAMP1 enable
pub type OPAMP1EN_R = crate::BitReader;
///Field `OPAMP1EN` writer - OPAMP1 enable
pub type OPAMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_VP` reader - FORCE_VP
pub type FORCE_VP_R = crate::BitReader;
///Field `FORCE_VP` writer - FORCE_VP
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VP_SEL` reader - OPAMP Non inverting input selection
pub type VP_SEL_R = crate::FieldReader;
///Field `VP_SEL` writer - OPAMP Non inverting input selection
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VM_SEL` reader - OPAMP inverting input selection
pub type VM_SEL_R = crate::FieldReader;
///Field `VM_SEL` writer - OPAMP inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCM_EN` reader - Timer controlled Mux mode enable
pub type TCM_EN_R = crate::BitReader;
///Field `TCM_EN` writer - Timer controlled Mux mode enable
pub type TCM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMS_SEL` reader - OPAMP inverting input secondary selection
pub type VMS_SEL_R = crate::BitReader;
///Field `VMS_SEL` writer - OPAMP inverting input secondary selection
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection
pub type VPS_SEL_R = crate::FieldReader;
///Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection
pub type VPS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CALON` reader - Calibration mode enable
pub type CALON_R = crate::BitReader;
///Field `CALON` writer - Calibration mode enable
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::FieldReader;
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PGA_GAIN` reader - Gain in PGA mode
pub type PGA_GAIN_R = crate::FieldReader;
///Field `PGA_GAIN` writer - Gain in PGA mode
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `USER_TRIM` reader - User trimming enable
pub type USER_TRIM_R = crate::BitReader;
///Field `USER_TRIM` writer - User trimming enable
pub type USER_TRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)
pub type TRIMOFFSETP_R = crate::FieldReader;
///Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)
pub type TRIMOFFSETN_R = crate::FieldReader;
///Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TSTREF` reader - TSTREF
pub type TSTREF_R = crate::BitReader;
///Field `TSTREF` writer - TSTREF
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTCAL` reader - OPAMP ouput status flag
pub type OUTCAL_R = crate::BitReader;
///Field `LOCK` reader - OPAMP lock
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - OPAMP lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OPAMP1 enable
    #[inline(always)]
    pub fn opamp1en(&self) -> OPAMP1EN_R {
        OPAMP1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP ouput status flag
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OPAMP lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP1_CSR")
            .field("opamp1en", &self.opamp1en())
            .field("force_vp", &self.force_vp())
            .field("vp_sel", &self.vp_sel())
            .field("vm_sel", &self.vm_sel())
            .field("tcm_en", &self.tcm_en())
            .field("vms_sel", &self.vms_sel())
            .field("vps_sel", &self.vps_sel())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("pga_gain", &self.pga_gain())
            .field("user_trim", &self.user_trim())
            .field("trimoffsetp", &self.trimoffsetp())
            .field("trimoffsetn", &self.trimoffsetn())
            .field("tstref", &self.tstref())
            .field("outcal", &self.outcal())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - OPAMP1 enable
    #[inline(always)]
    pub fn opamp1en(&mut self) -> OPAMP1EN_W<'_, OPAMP1_CSRrs> {
        OPAMP1EN_W::new(self, 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W<'_, OPAMP1_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP1_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP1_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TCM_EN_W<'_, OPAMP1_CSRrs> {
        TCM_EN_W::new(self, 7)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<'_, OPAMP1_CSRrs> {
        VMS_SEL_W::new(self, 8)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<'_, OPAMP1_CSRrs> {
        VPS_SEL_W::new(self, 9)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP1_CSRrs> {
        CALON_W::new(self, 11)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP1_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP1_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn user_trim(&mut self) -> USER_TRIM_W<'_, OPAMP1_CSRrs> {
        USER_TRIM_W::new(self, 18)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<'_, OPAMP1_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<'_, OPAMP1_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W<'_, OPAMP1_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    ///Bit 31 - OPAMP lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OPAMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**OPAMP1 control register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#OPAMP:OPAMP1_CSR)*/
pub struct OPAMP1_CSRrs;
impl crate::RegisterSpec for OPAMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_csr::R`](R) reader structure
impl crate::Readable for OPAMP1_CSRrs {}
///`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure
impl crate::Writable for OPAMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_CSR to value 0
impl crate::Resettable for OPAMP1_CSRrs {}
