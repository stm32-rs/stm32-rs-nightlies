#[doc = "Register `OPAMP2_CSR` reader"]
pub type R = crate::R<OPAMP2_CSRrs>;
#[doc = "Register `OPAMP2_CSR` writer"]
pub type W = crate::W<OPAMP2_CSRrs>;
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_R = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPAHSM` reader - Operational amplifier high-speed mode"]
pub type OPAHSM_R = crate::BitReader;
#[doc = "Field `OPAHSM` writer - Operational amplifier high-speed mode"]
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::FieldReader;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USERTRIM` reader - User trimming enable"]
pub type USERTRIM_R = crate::BitReader;
#[doc = "Field `USERTRIM` writer - User trimming enable"]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_R = crate::BitReader;
#[doc = "Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub type CALOUT_R = crate::BitReader;
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP2_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<OPAMP2_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<OPAMP2_CSRrs> {
        OPAHSM_W::new(self, 8)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP2_CSRrs> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP2_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP2_CSRrs> {
        USERTRIM_W::new(self, 18)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<OPAMP2_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<OPAMP2_CSRrs> {
        CALOUT_W::new(self, 30)
    }
}
#[doc = "OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_CSRrs;
impl crate::RegisterSpec for OPAMP2_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP2_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP2_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSRrs {
    const RESET_VALUE: u32 = 0;
}
