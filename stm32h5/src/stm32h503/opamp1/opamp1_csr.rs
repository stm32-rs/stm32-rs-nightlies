#[doc = "Register `OPAMP1_CSR` reader"]
pub type R = crate::R<OPAMP1_CSRrs>;
#[doc = "Register `OPAMP1_CSR` writer"]
pub type W = crate::W<OPAMP1_CSRrs>;
#[doc = "Field `OPAEN` reader - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
pub type OPAEN_R = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_R = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VP_SEL_R = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPAHSM` reader - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
pub type OPAHSM_R = crate::BitReader;
#[doc = "Field `OPAHSM` writer - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
pub type CALSEL_R = crate::FieldReader;
#[doc = "Field `CALSEL` writer - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USERTRIM` reader - User trimming enable This bit allows to switch from ‘factory’ AOP offset trimmed values to ‘user’ AOP offset trimmed values This bit is active for both mode normal and high-power."]
pub type USERTRIM_R = crate::BitReader;
#[doc = "Field `USERTRIM` writer - User trimming enable This bit allows to switch from ‘factory’ AOP offset trimmed values to ‘user’ AOP offset trimmed values This bit is active for both mode normal and high-power."]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_R = crate::BitReader;
#[doc = "Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output OPAMP output status flag. During the calibration mode, OPAMP is used as comparator."]
pub type CALOUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable This bit allows to switch from ‘factory’ AOP offset trimmed values to ‘user’ AOP offset trimmed values This bit is active for both mode normal and high-power."]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output OPAMP output status flag. During the calibration mode, OPAMP is used as comparator."]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP1_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<OPAMP1_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP1_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP1_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<OPAMP1_CSRrs> {
        OPAHSM_W::new(self, 8)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP1_CSRrs> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP1_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP1_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable This bit allows to switch from ‘factory’ AOP offset trimmed values to ‘user’ AOP offset trimmed values This bit is active for both mode normal and high-power."]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP1_CSRrs> {
        USERTRIM_W::new(self, 18)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<OPAMP1_CSRrs> {
        TSTREF_W::new(self, 29)
    }
}
#[doc = "OPAMP1 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP1_CSRrs;
impl crate::RegisterSpec for OPAMP1_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP1_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP1_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP1_CSR to value 0"]
impl crate::Resettable for OPAMP1_CSRrs {
    const RESET_VALUE: u32 = 0;
}
