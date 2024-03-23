#[doc = "Register `OPAMP2_CRS` reader"]
pub type R = crate::R<OPAMP2_CRSrs>;
#[doc = "Register `OPAMP2_CRS` writer"]
pub type W = crate::W<OPAMP2_CRSrs>;
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
#[doc = "Field `VM_SEL` reader - inverting input selection"]
pub type VM_SEL_R = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VP_SEL` reader - non inverted input selection"]
pub type VP_SEL_R = crate::BitReader;
#[doc = "Field `VP_SEL` writer - non inverted input selection"]
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - calibration mode enable"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - calibration mode enable"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - calibration selection"]
pub type CALSEL_R = crate::BitReader;
#[doc = "Field `CALSEL` writer - calibration selection"]
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USERTRIM` reader - User trimming enable"]
pub type USERTRIM_R = crate::BitReader;
#[doc = "Field `USERTRIM` writer - User trimming enable"]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 8:9 - inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP2_CRSrs> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OPALPM_W<OPAMP2_CRSrs> {
        OPALPM_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OPAMODE_W<OPAMP2_CRSrs> {
        OPAMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP2_CRSrs> {
        PGA_GAIN_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP2_CRSrs> {
        VM_SEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP2_CRSrs> {
        VP_SEL_W::new(self, 10)
    }
    #[doc = "Bit 12 - calibration mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<OPAMP2_CRSrs> {
        CALON_W::new(self, 12)
    }
    #[doc = "Bit 13 - calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP2_CRSrs> {
        CALSEL_W::new(self, 13)
    }
    #[doc = "Bit 14 - User trimming enable"]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP2_CRSrs> {
        USERTRIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<OPAMP2_CRSrs> {
        CALOUT_W::new(self, 15)
    }
}
#[doc = "OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_crs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_crs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_CRSrs;
impl crate::RegisterSpec for OPAMP2_CRSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_crs::R`](R) reader structure"]
impl crate::Readable for OPAMP2_CRSrs {}
#[doc = "`write(|w| ..)` method takes [`opamp2_crs::W`](W) writer structure"]
impl crate::Writable for OPAMP2_CRSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP2_CRS to value 0"]
impl crate::Resettable for OPAMP2_CRSrs {
    const RESET_VALUE: u32 = 0;
}
