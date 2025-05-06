///Register `VMCCR` reader
pub type R = crate::R<VMCCRrs>;
/**Video mode type This field returns the current video mode transmission type: 1x: Burst mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VMT {
    ///0: Non-burst with sync pulses
    B0x0 = 0,
    ///1: Non-burst with sync events
    B0x1 = 1,
}
impl From<VMT> for u8 {
    #[inline(always)]
    fn from(variant: VMT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VMT {
    type Ux = u8;
}
impl crate::IsEnum for VMT {}
///Field `VMT` reader - Video mode type This field returns the current video mode transmission type: 1x: Burst mode
pub type VMT_R = crate::FieldReader<VMT>;
impl VMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VMT> {
        match self.bits {
            0 => Some(VMT::B0x0),
            1 => Some(VMT::B0x1),
            _ => None,
        }
    }
    ///Non-burst with sync pulses
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VMT::B0x0
    }
    ///Non-burst with sync events
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VMT::B0x1
    }
}
/**Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPVSAE {
    ///0: Return to low-power inside the VSA is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the VSA is enabled
    B0x1 = 1,
}
impl From<LPVSAE> for bool {
    #[inline(always)]
    fn from(variant: LPVSAE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPVSAE` reader - Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows.
pub type LPVSAE_R = crate::BitReader<LPVSAE>;
impl LPVSAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPVSAE {
        match self.bits {
            false => LPVSAE::B0x0,
            true => LPVSAE::B0x1,
        }
    }
    ///Return to low-power inside the VSA is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPVSAE::B0x0
    }
    ///Return to low-power inside the VSA is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPVSAE::B0x1
    }
}
/**Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPVBPE {
    ///0: Return to low-power inside the VBP is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the VBP is enabled.
    B0x1 = 1,
}
impl From<LPVBPE> for bool {
    #[inline(always)]
    fn from(variant: LPVBPE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPVBPE` reader - Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows.
pub type LPVBPE_R = crate::BitReader<LPVBPE>;
impl LPVBPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPVBPE {
        match self.bits {
            false => LPVBPE::B0x0,
            true => LPVBPE::B0x1,
        }
    }
    ///Return to low-power inside the VBP is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPVBPE::B0x0
    }
    ///Return to low-power inside the VBP is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPVBPE::B0x1
    }
}
/**Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPVFPE {
    ///0: Return to low-power inside the VFP is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the VFP is enabled.
    B0x1 = 1,
}
impl From<LPVFPE> for bool {
    #[inline(always)]
    fn from(variant: LPVFPE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPVFPE` reader - Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows.
pub type LPVFPE_R = crate::BitReader<LPVFPE>;
impl LPVFPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPVFPE {
        match self.bits {
            false => LPVFPE::B0x0,
            true => LPVFPE::B0x1,
        }
    }
    ///Return to low-power inside the VFP is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPVFPE::B0x0
    }
    ///Return to low-power inside the VFP is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPVFPE::B0x1
    }
}
/**Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPVAE {
    ///0: Return to low-power inside the VACT is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the VACT is enabled.
    B0x1 = 1,
}
impl From<LPVAE> for bool {
    #[inline(always)]
    fn from(variant: LPVAE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPVAE` reader - Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows.
pub type LPVAE_R = crate::BitReader<LPVAE>;
impl LPVAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPVAE {
        match self.bits {
            false => LPVAE::B0x0,
            true => LPVAE::B0x1,
        }
    }
    ///Return to low-power inside the VACT is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPVAE::B0x0
    }
    ///Return to low-power inside the VACT is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPVAE::B0x1
    }
}
/**Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPHBPE {
    ///0: Return to low-power inside the HBP period is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the HBP period is enabled.
    B0x1 = 1,
}
impl From<LPHBPE> for bool {
    #[inline(always)]
    fn from(variant: LPHBPE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPHBPE` reader - Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows.
pub type LPHBPE_R = crate::BitReader<LPHBPE>;
impl LPHBPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPHBPE {
        match self.bits {
            false => LPHBPE::B0x0,
            true => LPHBPE::B0x1,
        }
    }
    ///Return to low-power inside the HBP period is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPHBPE::B0x0
    }
    ///Return to low-power inside the HBP period is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPHBPE::B0x1
    }
}
/**Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPHFE {
    ///0: Return to low-power inside the HFP period is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the HFP period is enabled.
    B0x1 = 1,
}
impl From<LPHFE> for bool {
    #[inline(always)]
    fn from(variant: LPHFE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPHFE` reader - Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows.
pub type LPHFE_R = crate::BitReader<LPHFE>;
impl LPHFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPHFE {
        match self.bits {
            false => LPHFE::B0x0,
            true => LPHFE::B0x1,
        }
    }
    ///Return to low-power inside the HFP period is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPHFE::B0x0
    }
    ///Return to low-power inside the HFP period is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPHFE::B0x1
    }
}
/**Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBTAAE {
    ///0: Acknowledge response at the end of a frame is disabled.
    B0x0 = 0,
    ///1: Acknowledge response at the end of a frame is enabled.
    B0x1 = 1,
}
impl From<FBTAAE> for bool {
    #[inline(always)]
    fn from(variant: FBTAAE) -> Self {
        variant as u8 != 0
    }
}
///Field `FBTAAE` reader - Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame.
pub type FBTAAE_R = crate::BitReader<FBTAAE>;
impl FBTAAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FBTAAE {
        match self.bits {
            false => FBTAAE::B0x0,
            true => FBTAAE::B0x1,
        }
    }
    ///Acknowledge response at the end of a frame is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FBTAAE::B0x0
    }
    ///Acknowledge response at the end of a frame is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FBTAAE::B0x1
    }
}
/**Low-power command enable This bit returns the current command transmission state in low-power mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCE {
    ///0: Command transmission in low-power mode is disabled.
    B0x0 = 0,
    ///1: Command transmission in low-power mode is enabled.
    B0x1 = 1,
}
impl From<LPCE> for bool {
    #[inline(always)]
    fn from(variant: LPCE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPCE` reader - Low-power command enable This bit returns the current command transmission state in low-power mode.
pub type LPCE_R = crate::BitReader<LPCE>;
impl LPCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPCE {
        match self.bits {
            false => LPCE::B0x0,
            true => LPCE::B0x1,
        }
    }
    ///Command transmission in low-power mode is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPCE::B0x0
    }
    ///Command transmission in low-power mode is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPCE::B0x1
    }
}
impl R {
    ///Bits 0:1 - Video mode type This field returns the current video mode transmission type: 1x: Burst mode
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows.
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows.
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows.
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows.
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows.
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows.
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame.
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power command enable This bit returns the current command transmission state in low-power mode.
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCCR")
            .field("vmt", &self.vmt())
            .field("lpvsae", &self.lpvsae())
            .field("lpvbpe", &self.lpvbpe())
            .field("lpvfpe", &self.lpvfpe())
            .field("lpvae", &self.lpvae())
            .field("lphbpe", &self.lphbpe())
            .field("lphfe", &self.lphfe())
            .field("fbtaae", &self.fbtaae())
            .field("lpce", &self.lpce())
            .finish()
    }
}
/**DSI Host video mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`vmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VMCCR)*/
pub struct VMCCRrs;
impl crate::RegisterSpec for VMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmccr::R`](R) reader structure
impl crate::Readable for VMCCRrs {}
///`reset()` method sets VMCCR to value 0
impl crate::Resettable for VMCCRrs {}
