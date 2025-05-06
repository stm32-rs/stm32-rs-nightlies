///Register `OPAMP2_CSR` reader
pub type R = crate::R<OPAMP2_CSRrs>;
///Register `OPAMP2_CSR` writer
pub type W = crate::W<OPAMP2_CSRrs>;
///Field `OPAEN` reader - Operational amplifier Enable
pub type OPAEN_R = crate::BitReader;
///Field `OPAEN` writer - Operational amplifier Enable
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPALPM` reader - Operational amplifier Low Power Mode
pub type OPALPM_R = crate::BitReader;
///Field `OPALPM` writer - Operational amplifier Low Power Mode
pub type OPALPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMODE` reader - Operational amplifier PGA mode
pub type OPAMODE_R = crate::FieldReader;
///Field `OPAMODE` writer - Operational amplifier PGA mode
pub type OPAMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader;
///Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VM_SEL` reader - Inverting input selection
pub type VM_SEL_R = crate::FieldReader;
///Field `VM_SEL` writer - Inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VP_SEL` reader - Non inverted input selection
pub type VP_SEL_R = crate::BitReader;
///Field `VP_SEL` writer - Non inverted input selection
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALON` reader - Calibration mode enabled
pub type CALON_R = crate::BitReader;
///Field `CALON` writer - Calibration mode enabled
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::BitReader;
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset
pub type USERTRIM_R = crate::BitReader;
///Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALOUT` reader - Operational amplifier calibration output
pub type CALOUT_R = crate::BitReader;
///Field `CALOUT` writer - Operational amplifier calibration output
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP2_CSR")
            .field("opaen", &self.opaen())
            .field("opalpm", &self.opalpm())
            .field("opamode", &self.opamode())
            .field("pga_gain", &self.pga_gain())
            .field("vm_sel", &self.vm_sel())
            .field("vp_sel", &self.vp_sel())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("usertrim", &self.usertrim())
            .field("calout", &self.calout())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<OPAMP2_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<OPAMP2_CSRrs> {
        OPALPM_W::new(self, 1)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<OPAMP2_CSRrs> {
        OPAMODE_W::new(self, 2)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 4)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<OPAMP2_CSRrs> {
        VP_SEL_W::new(self, 10)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<OPAMP2_CSRrs> {
        CALON_W::new(self, 12)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<OPAMP2_CSRrs> {
        CALSEL_W::new(self, 13)
    }
    ///Bit 14 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<OPAMP2_CSRrs> {
        USERTRIM_W::new(self, 14)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<OPAMP2_CSRrs> {
        CALOUT_W::new(self, 15)
    }
}
/**OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#OPAMP:OPAMP2_CSR)*/
pub struct OPAMP2_CSRrs;
impl crate::RegisterSpec for OPAMP2_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp2_csr::R`](R) reader structure
impl crate::Readable for OPAMP2_CSRrs {}
///`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure
impl crate::Writable for OPAMP2_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP2_CSR to value 0
impl crate::Resettable for OPAMP2_CSRrs {}
