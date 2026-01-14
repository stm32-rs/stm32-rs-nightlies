///Register `OPAMP2_CSR` reader
pub type R = crate::R<OPAMP2_CSRrs>;
///Register `OPAMP2_CSR` writer
pub type W = crate::W<OPAMP2_CSRrs>;
///Field `OPAEN` reader - Operational amplifier Enable
pub type OPAEN_R = crate::BitReader;
///Field `OPAEN` writer - Operational amplifier Enable
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)
pub type FORCE_VP_R = crate::BitReader;
///Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VM_SEL` reader - Inverting input selection
pub type VM_SEL_R = crate::FieldReader;
///Field `VM_SEL` writer - Inverting input selection
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OPAHSM` reader - Operational amplifier high-speed mode
pub type OPAHSM_R = crate::BitReader;
///Field `OPAHSM` writer - Operational amplifier high-speed mode
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALON` reader - Calibration mode enabled
pub type CALON_R = crate::BitReader;
///Field `CALON` writer - Calibration mode enabled
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::FieldReader;
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader;
///Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `USERTRIM` reader - User trimming enable
pub type USERTRIM_R = crate::BitReader;
///Field `USERTRIM` writer - User trimming enable
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)
pub type TSTREF_R = crate::BitReader;
///Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 1 - Force internal reference on VP (reserved for test)
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:6 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 8 - Operational amplifier high-speed mode
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 29 - OPAMP calibration reference voltage output control (reserved for test)
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP2_CSR")
            .field("opaen", &self.opaen())
            .field("force_vp", &self.force_vp())
            .field("vm_sel", &self.vm_sel())
            .field("opahsm", &self.opahsm())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("pga_gain", &self.pga_gain())
            .field("usertrim", &self.usertrim())
            .field("tstref", &self.tstref())
            .field("calout", &self.calout())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<'_, OPAMP2_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - Force internal reference on VP (reserved for test)
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W<'_, OPAMP2_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    ///Bits 5:6 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    ///Bit 8 - Operational amplifier high-speed mode
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W<'_, OPAMP2_CSRrs> {
        OPAHSM_W::new(self, 8)
    }
    ///Bit 11 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP2_CSRrs> {
        CALON_W::new(self, 11)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP2_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    ///Bits 14:17 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<'_, OPAMP2_CSRrs> {
        USERTRIM_W::new(self, 18)
    }
    ///Bit 29 - OPAMP calibration reference voltage output control (reserved for test)
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W<'_, OPAMP2_CSRrs> {
        TSTREF_W::new(self, 29)
    }
    ///Bit 30 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<'_, OPAMP2_CSRrs> {
        CALOUT_W::new(self, 30)
    }
}
/**OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#OPAMP:OPAMP2_CSR)*/
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
