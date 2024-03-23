#[doc = "Register `OPAMP1_CSR` reader"]
pub type R = crate::R<OPAMP1_CSRrs>;
#[doc = "Register `OPAMP1_CSR` writer"]
pub type W = crate::W<OPAMP1_CSRrs>;
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPALPM` reader - Operational amplifier Low Power Mode"]
pub type OPALPM_R = crate::BitReader;
#[doc = "Field `OPALPM` writer - Operational amplifier Low Power Mode"]
pub type OPALPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMODE` reader - Operational amplifier PGA mode"]
pub type OPAMODE_R = crate::FieldReader;
#[doc = "Field `OPAMODE` writer - Operational amplifier PGA mode"]
pub type OPAMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VP_SEL_R = crate::BitReader;
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::BitReader;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_R = crate::BitReader;
#[doc = "Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub type CALOUT_R = crate::BitReader;
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA_RANGE` reader - Operational amplifier power supply range for stability"]
pub type OPA_RANGE_R = crate::BitReader;
#[doc = "Field `OPA_RANGE` writer - Operational amplifier power supply range for stability"]
pub type OPA_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability"]
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP1_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OPALPM_W<OPAMP1_CSRrs> {
        OPALPM_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OPAMODE_W<OPAMP1_CSRrs> {
        OPAMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP1_CSRrs> {
        PGA_GAIN_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP1_CSRrs> {
        VM_SEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP1_CSRrs> {
        VP_SEL_W::new(self, 10)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP1_CSRrs> {
        CALON_W::new(self, 12)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP1_CSRrs> {
        CALSEL_W::new(self, 13)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP1_CSRrs> {
        USERTRIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<OPAMP1_CSRrs> {
        CALOUT_W::new(self, 15)
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability"]
    #[inline(always)]
    #[must_use]
    pub fn opa_range(&mut self) -> OPA_RANGE_W<OPAMP1_CSRrs> {
        OPA_RANGE_W::new(self, 31)
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
