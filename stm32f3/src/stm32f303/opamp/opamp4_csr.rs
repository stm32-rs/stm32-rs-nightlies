#[doc = "Register `OPAMP4_CSR` reader"]
pub type R = crate::R<OPAMP4_CSRrs>;
#[doc = "Register `OPAMP4_CSR` writer"]
pub type W = crate::W<OPAMP4_CSRrs>;
#[doc = "Field `OPAMP4EN` reader - OPAMP4 enable"]
pub type OPAMP4EN_R = crate::BitReader;
#[doc = "Field `OPAMP4EN` writer - OPAMP4 enable"]
pub type OPAMP4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type FORCE_VP_R = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VP_SEL` reader - OPAMP Non inverting input selection"]
pub type VP_SEL_R = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - OPAMP Non inverting input selection"]
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VM_SEL` reader - OPAMP inverting input selection"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - OPAMP inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub type TCM_EN_R = crate::BitReader;
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub type TCM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMS_SEL` reader - OPAMP inverting input secondary selection"]
pub type VMS_SEL_R = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - OPAMP inverting input secondary selection"]
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_R = crate::FieldReader;
#[doc = "Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CALON` reader - Calibration mode enable"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enable"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::FieldReader;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
pub type PGA_GAIN_R = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub type USER_TRIM_R = crate::BitReader;
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub type USER_TRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSTREF` reader - TSTREF"]
pub type TSTREF_R = crate::BitReader;
#[doc = "Field `TSTREF` writer - TSTREF"]
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTCAL` reader - OPAMP ouput status flag"]
pub type OUTCAL_R = crate::BitReader;
#[doc = "Field `LOCK` reader - OPAMP lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - OPAMP lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OPAMP4 enable"]
    #[inline(always)]
    pub fn opamp4en(&self) -> OPAMP4EN_R {
        OPAMP4EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn opamp4en(&mut self) -> OPAMP4EN_W<OPAMP4_CSRrs> {
        OPAMP4EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<OPAMP4_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP4_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP4_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcm_en(&mut self) -> TCM_EN_W<OPAMP4_CSRrs> {
        TCM_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<OPAMP4_CSRrs> {
        VMS_SEL_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<OPAMP4_CSRrs> {
        VPS_SEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP4_CSRrs> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP4_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP4_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn user_trim(&mut self) -> USER_TRIM_W<OPAMP4_CSRrs> {
        USER_TRIM_W::new(self, 18)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<OPAMP4_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<OPAMP4_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<OPAMP4_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<OPAMP4_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "OPAMP4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp4_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp4_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP4_CSRrs;
impl crate::RegisterSpec for OPAMP4_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp4_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP4_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp4_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP4_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP4_CSR to value 0"]
impl crate::Resettable for OPAMP4_CSRrs {
    const RESET_VALUE: u32 = 0;
}
