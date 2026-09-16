///Register `OPAMP2_CRS` reader
pub type R = crate::R<OPAMP2_CRSrs>;
///Register `OPAMP2_CRS` writer
pub type W = crate::W<OPAMP2_CRSrs>;
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
///Field `VM_SEL` reader - inverting input selection
pub type VM_SEL_R = crate::FieldReader;
///Field `VM_SEL` writer - inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VP_SEL` reader - non inverted input selection
pub type VP_SEL_R = crate::BitReader;
///Field `VP_SEL` writer - non inverted input selection
pub type VP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALON` reader - calibration mode enable
pub type CALON_R = crate::BitReader;
///Field `CALON` writer - calibration mode enable
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEL` reader - calibration selection
pub type CALSEL_R = crate::BitReader;
///Field `CALSEL` writer - calibration selection
pub type CALSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USERTRIM` reader - User trimming enable
pub type USERTRIM_R = crate::BitReader;
///Field `USERTRIM` writer - User trimming enable
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
    ///Bits 8:9 - inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - calibration mode enable
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - User trimming enable
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
        f.debug_struct("OPAMP2_CRS")
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
    pub fn opaen(&mut self) -> OPAEN_W<'_, OPAMP2_CRSrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<'_, OPAMP2_CRSrs> {
        OPALPM_W::new(self, 1)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<'_, OPAMP2_CRSrs> {
        OPAMODE_W::new(self, 2)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP2_CRSrs> {
        PGA_GAIN_W::new(self, 4)
    }
    ///Bits 8:9 - inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP2_CRSrs> {
        VM_SEL_W::new(self, 8)
    }
    ///Bit 10 - non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP2_CRSrs> {
        VP_SEL_W::new(self, 10)
    }
    ///Bit 12 - calibration mode enable
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP2_CRSrs> {
        CALON_W::new(self, 12)
    }
    ///Bit 13 - calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP2_CRSrs> {
        CALSEL_W::new(self, 13)
    }
    ///Bit 14 - User trimming enable
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<'_, OPAMP2_CRSrs> {
        USERTRIM_W::new(self, 14)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<'_, OPAMP2_CRSrs> {
        CALOUT_W::new(self, 15)
    }
}
/**OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_crs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_crs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#OPAMP:OPAMP2_CRS)*/
pub struct OPAMP2_CRSrs;
impl crate::RegisterSpec for OPAMP2_CRSrs {
    type Ux = u32;
}
///`read()` method returns [`opamp2_crs::R`](R) reader structure
impl crate::Readable for OPAMP2_CRSrs {}
///`write(|w| ..)` method takes [`opamp2_crs::W`](W) writer structure
impl crate::Writable for OPAMP2_CRSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP2_CRS to value 0
impl crate::Resettable for OPAMP2_CRSrs {}
