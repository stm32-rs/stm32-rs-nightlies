///Register `OPAMP5_CSR` reader
pub type R = crate::R<OPAMP5_CSRrs>;
///Register `OPAMP5_CSR` writer
pub type W = crate::W<OPAMP5_CSRrs>;
///FORCE_VP
pub use super::opamp1_csr::FORCE_VP;
///Field `FORCE_VP` reader - FORCE_VP
pub use super::opamp1_csr::FORCE_VP_R;
///Field `FORCE_VP` writer - FORCE_VP
pub use super::opamp1_csr::FORCE_VP_W;
///Operational amplifier Enable
pub use super::opamp1_csr::OPAEN;
///Field `OPAEN` reader - Operational amplifier Enable
pub use super::opamp1_csr::OPAEN_R;
///Field `OPAEN` writer - Operational amplifier Enable
pub use super::opamp1_csr::OPAEN_W;
/**VP_SEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL {
    ///0: VINP0 connected to VINP input
    Vinp0 = 0,
    ///1: VINP1 connected to VINP input
    Vinp1 = 1,
    ///2: VINP2 connected to VINP input
    Vinp2 = 2,
    ///3: DAC4_CH2 connected to VINP input
    Dac4Ch2 = 3,
}
impl From<VP_SEL> for u8 {
    #[inline(always)]
    fn from(variant: VP_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VP_SEL {
    type Ux = u8;
}
impl crate::IsEnum for VP_SEL {}
///Field `VP_SEL` reader - VP_SEL
pub type VP_SEL_R = crate::FieldReader<VP_SEL>;
impl VP_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VP_SEL {
        match self.bits {
            0 => VP_SEL::Vinp0,
            1 => VP_SEL::Vinp1,
            2 => VP_SEL::Vinp2,
            3 => VP_SEL::Dac4Ch2,
            _ => unreachable!(),
        }
    }
    ///VINP0 connected to VINP input
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VP_SEL::Vinp0
    }
    ///VINP1 connected to VINP input
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VP_SEL::Vinp1
    }
    ///VINP2 connected to VINP input
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VP_SEL::Vinp2
    }
    ///DAC4_CH2 connected to VINP input
    #[inline(always)]
    pub fn is_dac4_ch2(&self) -> bool {
        *self == VP_SEL::Dac4Ch2
    }
}
///Field `VP_SEL` writer - VP_SEL
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VP_SEL, crate::Safe>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VINP0 connected to VINP input
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp0)
    }
    ///VINP1 connected to VINP input
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp1)
    }
    ///VINP2 connected to VINP input
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Vinp2)
    }
    ///DAC4_CH2 connected to VINP input
    #[inline(always)]
    pub fn dac4_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL::Dac4Ch2)
    }
}
///CALON
pub use super::opamp1_csr::CALON;
///Field `CALON` reader - CALON
pub use super::opamp1_csr::CALON_R;
///Field `CALON` writer - CALON
pub use super::opamp1_csr::CALON_W;
///CALSEL
pub use super::opamp1_csr::CALSEL;
///Field `CALSEL` reader - CALSEL
pub use super::opamp1_csr::CALSEL_R;
///Field `CALSEL` writer - CALSEL
pub use super::opamp1_csr::CALSEL_W;
///OPAHSM
pub use super::opamp1_csr::OPAHSM;
///Field `OPAHSM` reader - OPAHSM
pub use super::opamp1_csr::OPAHSM_R;
///Field `OPAHSM` writer - OPAHSM
pub use super::opamp1_csr::OPAHSM_W;
///OPAINTOEN
pub use super::opamp1_csr::OPAINTOEN;
///Field `OPAINTOEN` reader - OPAINTOEN
pub use super::opamp1_csr::OPAINTOEN_R;
///Field `OPAINTOEN` writer - OPAINTOEN
pub use super::opamp1_csr::OPAINTOEN_W;
///PGA_GAIN
pub use super::opamp1_csr::PGA_GAIN;
///Field `PGA_GAIN` reader - PGA_GAIN
pub use super::opamp1_csr::PGA_GAIN_R;
///Field `PGA_GAIN` writer - PGA_GAIN
pub use super::opamp1_csr::PGA_GAIN_W;
///USERTRIM
pub use super::opamp1_csr::USERTRIM;
///Field `USERTRIM` reader - USERTRIM
pub use super::opamp1_csr::USERTRIM_R;
///Field `USERTRIM` writer - USERTRIM
pub use super::opamp1_csr::USERTRIM_W;
///VM_SEL
pub use super::opamp1_csr::VM_SEL;
///Field `VM_SEL` reader - VM_SEL
pub use super::opamp1_csr::VM_SEL_R;
///Field `VM_SEL` writer - VM_SEL
pub use super::opamp1_csr::VM_SEL_W;
///Field `TRIMOFFSETP` reader - TRIMOFFSETP
pub type TRIMOFFSETP_R = crate::FieldReader;
///Field `TRIMOFFSETP` writer - TRIMOFFSETP
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `TRIMOFFSETN` reader - TRIMOFFSETN
pub type TRIMOFFSETN_R = crate::FieldReader;
///Field `TRIMOFFSETN` writer - TRIMOFFSETN
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `CALOUT` reader - CALOUT
pub type CALOUT_R = crate::BitReader;
///Field `CALOUT` writer - CALOUT
pub type CALOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///LOCK
pub use super::opamp1_csr::LOCK;
///Field `LOCK` reader - LOCK
pub use super::opamp1_csr::LOCK_R;
///Field `LOCK` writer - LOCK
pub use super::opamp1_csr::LOCK_W;
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - VP_SEL
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - USERTRIM
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - VM_SEL
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - OPAHSM
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAINTOEN
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - CALON
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - CALSEL
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:18 - PGA_GAIN
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    ///Bits 19:23 - TRIMOFFSETP
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - TRIMOFFSETN
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 30 - CALOUT
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP5_CSR")
            .field("opaen", &self.opaen())
            .field("force_vp", &self.force_vp())
            .field("vp_sel", &self.vp_sel())
            .field("usertrim", &self.usertrim())
            .field("vm_sel", &self.vm_sel())
            .field("opahsm", &self.opahsm())
            .field("opaintoen", &self.opaintoen())
            .field("calon", &self.calon())
            .field("calsel", &self.calsel())
            .field("pga_gain", &self.pga_gain())
            .field("trimoffsetp", &self.trimoffsetp())
            .field("trimoffsetn", &self.trimoffsetn())
            .field("calout", &self.calout())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<'_, OPAMP5_CSRrs> {
        OPAEN_W::new(self, 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W<'_, OPAMP5_CSRrs> {
        FORCE_VP_W::new(self, 1)
    }
    ///Bits 2:3 - VP_SEL
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<'_, OPAMP5_CSRrs> {
        VP_SEL_W::new(self, 2)
    }
    ///Bit 4 - USERTRIM
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<'_, OPAMP5_CSRrs> {
        USERTRIM_W::new(self, 4)
    }
    ///Bits 5:6 - VM_SEL
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<'_, OPAMP5_CSRrs> {
        VM_SEL_W::new(self, 5)
    }
    ///Bit 7 - OPAHSM
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W<'_, OPAMP5_CSRrs> {
        OPAHSM_W::new(self, 7)
    }
    ///Bit 8 - OPAINTOEN
    #[inline(always)]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W<'_, OPAMP5_CSRrs> {
        OPAINTOEN_W::new(self, 8)
    }
    ///Bit 11 - CALON
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<'_, OPAMP5_CSRrs> {
        CALON_W::new(self, 11)
    }
    ///Bits 12:13 - CALSEL
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<'_, OPAMP5_CSRrs> {
        CALSEL_W::new(self, 12)
    }
    ///Bits 14:18 - PGA_GAIN
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, OPAMP5_CSRrs> {
        PGA_GAIN_W::new(self, 14)
    }
    ///Bits 19:23 - TRIMOFFSETP
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<'_, OPAMP5_CSRrs> {
        TRIMOFFSETP_W::new(self, 19)
    }
    ///Bits 24:28 - TRIMOFFSETN
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<'_, OPAMP5_CSRrs> {
        TRIMOFFSETN_W::new(self, 24)
    }
    ///Bit 30 - CALOUT
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<'_, OPAMP5_CSRrs> {
        CALOUT_W::new(self, 30)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, OPAMP5_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**OPAMP5 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp5_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp5_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#OPAMP:OPAMP5_CSR)*/
pub struct OPAMP5_CSRrs;
impl crate::RegisterSpec for OPAMP5_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp5_csr::R`](R) reader structure
impl crate::Readable for OPAMP5_CSRrs {}
///`write(|w| ..)` method takes [`opamp5_csr::W`](W) writer structure
impl crate::Writable for OPAMP5_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP5_CSR to value 0
impl crate::Resettable for OPAMP5_CSRrs {}
