///Register `OPAMP2_CSR` reader
pub type R = crate::R<OPAMP2_CSRrs>;
///Register `OPAMP2_CSR` writer
pub type W = crate::W<OPAMP2_CSRrs>;
///Operational amplifier Enable
pub use super::opamp1_csr::OPAEN;
///Field `OPAEN` reader - Operational amplifier Enable
pub use super::opamp1_csr::OPAEN_R;
///Field `OPAEN` writer - Operational amplifier Enable
pub use super::opamp1_csr::OPAEN_W;
///Operational amplifier Low Power Mode
pub use super::opamp1_csr::OPALPM;
///Field `OPALPM` reader - Operational amplifier Low Power Mode
pub use super::opamp1_csr::OPALPM_R;
///Field `OPALPM` writer - Operational amplifier Low Power Mode
pub use super::opamp1_csr::OPALPM_W;
///Operational amplifier PGA mode
pub use super::opamp1_csr::OPAMODE;
///Field `OPAMODE` reader - Operational amplifier PGA mode
pub use super::opamp1_csr::OPAMODE_R;
///Field `OPAMODE` writer - Operational amplifier PGA mode
pub use super::opamp1_csr::OPAMODE_W;
/**Operational amplifier Programmable amplifier gain value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN {
    ///0: Gain 2
    Gain2 = 0,
    ///1: Gain 4
    Gain4 = 1,
    ///2: Gain 8
    Gain8 = 2,
    ///3: Gain 16
    Gain16 = 3,
}
impl From<PGA_GAIN> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PGA_GAIN {
    type Ux = u8;
}
impl crate::IsEnum for PGA_GAIN {}
///Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN>;
impl PGA_GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGA_GAIN {
        match self.bits {
            0 => PGA_GAIN::Gain2,
            1 => PGA_GAIN::Gain4,
            2 => PGA_GAIN::Gain8,
            3 => PGA_GAIN::Gain16,
            _ => unreachable!(),
        }
    }
    ///Gain 2
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN::Gain2
    }
    ///Gain 4
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN::Gain4
    }
    ///Gain 8
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN::Gain8
    }
    ///Gain 16
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN::Gain16
    }
}
///Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PGA_GAIN, crate::Safe>;
impl<'a, REG> PGA_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Gain 2
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain2)
    }
    ///Gain 4
    #[inline(always)]
    pub fn gain4(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain4)
    }
    ///Gain 8
    #[inline(always)]
    pub fn gain8(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain8)
    }
    ///Gain 16
    #[inline(always)]
    pub fn gain16(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN::Gain16)
    }
}
///Calibration mode enabled
pub use super::opamp1_csr::CALON;
///Field `CALON` reader - Calibration mode enabled
pub use super::opamp1_csr::CALON_R;
///Field `CALON` writer - Calibration mode enabled
pub use super::opamp1_csr::CALON_W;
///Calibration selection
pub use super::opamp1_csr::CALSEL;
///Field `CALSEL` reader - Calibration selection
pub use super::opamp1_csr::CALSEL_R;
///Field `CALSEL` writer - Calibration selection
pub use super::opamp1_csr::CALSEL_W;
///allows to switch from AOP offset trimmed values to AOP offset
pub use super::opamp1_csr::USERTRIM;
///Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset
pub use super::opamp1_csr::USERTRIM_R;
///Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset
pub use super::opamp1_csr::USERTRIM_W;
///Inverting input selection
pub use super::opamp1_csr::VM_SEL;
///Field `VM_SEL` reader - Inverting input selection
pub use super::opamp1_csr::VM_SEL_R;
///Field `VM_SEL` writer - Inverting input selection
pub use super::opamp1_csr::VM_SEL_W;
///Non inverted input selection
pub use super::opamp1_csr::VP_SEL;
///Field `VP_SEL` reader - Non inverted input selection
pub use super::opamp1_csr::VP_SEL_R;
///Field `VP_SEL` writer - Non inverted input selection
pub use super::opamp1_csr::VP_SEL_W;
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
    pub fn opaen(&mut self) -> OPAEN_W<'_, OPAMP2_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<'_, OPAMP2_CSRrs> {
        OPALPM_W::new(self, 1)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<'_, OPAMP2_CSRrs> {
        OPAMODE_W::new(self, 2)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP2_CSRrs> {
        PGA_GAIN_W::new(self, 4)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP2_CSRrs> {
        VM_SEL_W::new(self, 8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP2_CSRrs> {
        VP_SEL_W::new(self, 10)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP2_CSRrs> {
        CALON_W::new(self, 12)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP2_CSRrs> {
        CALSEL_W::new(self, 13)
    }
    ///Bit 14 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<'_, OPAMP2_CSRrs> {
        USERTRIM_W::new(self, 14)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<'_, OPAMP2_CSRrs> {
        CALOUT_W::new(self, 15)
    }
}
/**OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#OPAMP:OPAMP2_CSR)*/
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
