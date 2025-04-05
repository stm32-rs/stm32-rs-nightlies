///Register `VMCR` reader
pub type R = crate::R<VMCRrs>;
///Register `VMCR` writer
pub type W = crate::W<VMCRrs>;
/**Video mode type This field configures the video mode transmission type : 1x: Burst mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VMT {
    ///0: Non-burst with sync pulses.
    B0x0 = 0,
    ///1: Non-burst with sync events.
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
///Field `VMT` reader - Video mode type This field configures the video mode transmission type : 1x: Burst mode
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
    ///Non-burst with sync pulses.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VMT::B0x0
    }
    ///Non-burst with sync events.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VMT::B0x1
    }
}
///Field `VMT` writer - Video mode type This field configures the video mode transmission type : 1x: Burst mode
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VMT>;
impl<'a, REG> VMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Non-burst with sync pulses.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VMT::B0x0)
    }
    ///Non-burst with sync events.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VMT::B0x1)
    }
}
/**Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.

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
///Field `LPVSAE` reader - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
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
///Field `LPVSAE` writer - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG, LPVSAE>;
impl<'a, REG> LPVSAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the VSA is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPVSAE::B0x0)
    }
    ///Return to low-power inside the VSA is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPVSAE::B0x1)
    }
}
/**Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.

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
///Field `LPVBPE` reader - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
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
///Field `LPVBPE` writer - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG, LPVBPE>;
impl<'a, REG> LPVBPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the VBP is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPVBPE::B0x0)
    }
    ///Return to low-power inside the VBP is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPVBPE::B0x1)
    }
}
/**Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.

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
///Field `LPVFPE` reader - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
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
///Field `LPVFPE` writer - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG, LPVFPE>;
impl<'a, REG> LPVFPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the VFP is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPVFPE::B0x0)
    }
    ///Return to low-power inside the VFP is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPVFPE::B0x1)
    }
}
/**Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.

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
///Field `LPVAE` reader - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
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
///Field `LPVAE` writer - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
pub type LPVAE_W<'a, REG> = crate::BitWriter<'a, REG, LPVAE>;
impl<'a, REG> LPVAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the VACT is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPVAE::B0x0)
    }
    ///Return to low-power inside the VACT is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPVAE::B0x1)
    }
}
/**Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.

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
///Field `LPHBPE` reader - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
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
///Field `LPHBPE` writer - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG, LPHBPE>;
impl<'a, REG> LPHBPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the HBP period is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPHBPE::B0x0)
    }
    ///Return to low-power inside the HBP period is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPHBPE::B0x1)
    }
}
/**Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPHFPE {
    ///0: Return to low-power inside the HFP period is disabled.
    B0x0 = 0,
    ///1: Return to low-power inside the HFP period is enabled.
    B0x1 = 1,
}
impl From<LPHFPE> for bool {
    #[inline(always)]
    fn from(variant: LPHFPE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPHFPE` reader - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
pub type LPHFPE_R = crate::BitReader<LPHFPE>;
impl LPHFPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPHFPE {
        match self.bits {
            false => LPHFPE::B0x0,
            true => LPHFPE::B0x1,
        }
    }
    ///Return to low-power inside the HFP period is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPHFPE::B0x0
    }
    ///Return to low-power inside the HFP period is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPHFPE::B0x1
    }
}
///Field `LPHFPE` writer - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
pub type LPHFPE_W<'a, REG> = crate::BitWriter<'a, REG, LPHFPE>;
impl<'a, REG> LPHFPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Return to low-power inside the HFP period is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPHFPE::B0x0)
    }
    ///Return to low-power inside the HFP period is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPHFPE::B0x1)
    }
}
/**Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.

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
///Field `FBTAAE` reader - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
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
///Field `FBTAAE` writer - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG, FBTAAE>;
impl<'a, REG> FBTAAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Acknowledge response at the end of a frame is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FBTAAE::B0x0)
    }
    ///Acknowledge response at the end of a frame is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FBTAAE::B0x1)
    }
}
/**Low-power command enable This bit enables the command transmission only in low-power mode.

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
///Field `LPCE` reader - Low-power command enable This bit enables the command transmission only in low-power mode.
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
///Field `LPCE` writer - Low-power command enable This bit enables the command transmission only in low-power mode.
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG, LPCE>;
impl<'a, REG> LPCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command transmission in low-power mode is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPCE::B0x0)
    }
    ///Command transmission in low-power mode is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPCE::B0x1)
    }
}
/**Pattern generator enable This bit enables the video mode pattern generator.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGE {
    ///0: Pattern generator is disabled.
    B0x0 = 0,
    ///1: Pattern generator is enabled.
    B0x1 = 1,
}
impl From<PGE> for bool {
    #[inline(always)]
    fn from(variant: PGE) -> Self {
        variant as u8 != 0
    }
}
///Field `PGE` reader - Pattern generator enable This bit enables the video mode pattern generator.
pub type PGE_R = crate::BitReader<PGE>;
impl PGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGE {
        match self.bits {
            false => PGE::B0x0,
            true => PGE::B0x1,
        }
    }
    ///Pattern generator is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PGE::B0x0
    }
    ///Pattern generator is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PGE::B0x1
    }
}
///Field `PGE` writer - Pattern generator enable This bit enables the video mode pattern generator.
pub type PGE_W<'a, REG> = crate::BitWriter<'a, REG, PGE>;
impl<'a, REG> PGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pattern generator is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PGE::B0x0)
    }
    ///Pattern generator is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PGE::B0x1)
    }
}
/**Pattern generator mode This bit configures the pattern generator mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGM {
    ///0: Color bars (horizontal or vertical).
    B0x0 = 0,
    ///1: BER pattern (vertical only).
    B0x1 = 1,
}
impl From<PGM> for bool {
    #[inline(always)]
    fn from(variant: PGM) -> Self {
        variant as u8 != 0
    }
}
///Field `PGM` reader - Pattern generator mode This bit configures the pattern generator mode.
pub type PGM_R = crate::BitReader<PGM>;
impl PGM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGM {
        match self.bits {
            false => PGM::B0x0,
            true => PGM::B0x1,
        }
    }
    ///Color bars (horizontal or vertical).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PGM::B0x0
    }
    ///BER pattern (vertical only).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PGM::B0x1
    }
}
///Field `PGM` writer - Pattern generator mode This bit configures the pattern generator mode.
pub type PGM_W<'a, REG> = crate::BitWriter<'a, REG, PGM>;
impl<'a, REG> PGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Color bars (horizontal or vertical).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PGM::B0x0)
    }
    ///BER pattern (vertical only).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PGM::B0x1)
    }
}
/**Pattern generator orientation This bit configures the color bar orientation.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGO {
    ///0: Vertical color bars.
    B0x0 = 0,
    ///1: Horizontal color bars.
    B0x1 = 1,
}
impl From<PGO> for bool {
    #[inline(always)]
    fn from(variant: PGO) -> Self {
        variant as u8 != 0
    }
}
///Field `PGO` reader - Pattern generator orientation This bit configures the color bar orientation.
pub type PGO_R = crate::BitReader<PGO>;
impl PGO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGO {
        match self.bits {
            false => PGO::B0x0,
            true => PGO::B0x1,
        }
    }
    ///Vertical color bars.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PGO::B0x0
    }
    ///Horizontal color bars.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PGO::B0x1
    }
}
///Field `PGO` writer - Pattern generator orientation This bit configures the color bar orientation.
pub type PGO_W<'a, REG> = crate::BitWriter<'a, REG, PGO>;
impl<'a, REG> PGO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Vertical color bars.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PGO::B0x0)
    }
    ///Horizontal color bars.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PGO::B0x1)
    }
}
impl R {
    ///Bits 0:1 - Video mode type This field configures the video mode transmission type : 1x: Burst mode
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Low-power command enable This bit enables the command transmission only in low-power mode.
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pattern generator enable This bit enables the video mode pattern generator.
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Pattern generator mode This bit configures the pattern generator mode.
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Pattern generator orientation This bit configures the color bar orientation.
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCR")
            .field("vmt", &self.vmt())
            .field("lpvsae", &self.lpvsae())
            .field("lpvbpe", &self.lpvbpe())
            .field("lpvfpe", &self.lpvfpe())
            .field("lpvae", &self.lpvae())
            .field("lphbpe", &self.lphbpe())
            .field("lphfpe", &self.lphfpe())
            .field("fbtaae", &self.fbtaae())
            .field("lpce", &self.lpce())
            .field("pge", &self.pge())
            .field("pgm", &self.pgm())
            .field("pgo", &self.pgo())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Video mode type This field configures the video mode transmission type : 1x: Burst mode
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W<VMCRrs> {
        VMT_W::new(self, 0)
    }
    ///Bit 8 - Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows.
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W<VMCRrs> {
        LPVSAE_W::new(self, 8)
    }
    ///Bit 9 - Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows.
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<VMCRrs> {
        LPVBPE_W::new(self, 9)
    }
    ///Bit 10 - Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows.
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<VMCRrs> {
        LPVFPE_W::new(self, 10)
    }
    ///Bit 11 - Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows.
    #[inline(always)]
    pub fn lpvae(&mut self) -> LPVAE_W<VMCRrs> {
        LPVAE_W::new(self, 11)
    }
    ///Bit 12 - Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows.
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W<VMCRrs> {
        LPHBPE_W::new(self, 12)
    }
    ///Bit 13 - Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows.
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LPHFPE_W<VMCRrs> {
        LPHFPE_W::new(self, 13)
    }
    ///Bit 14 - Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame.
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W<VMCRrs> {
        FBTAAE_W::new(self, 14)
    }
    ///Bit 15 - Low-power command enable This bit enables the command transmission only in low-power mode.
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W<VMCRrs> {
        LPCE_W::new(self, 15)
    }
    ///Bit 16 - Pattern generator enable This bit enables the video mode pattern generator.
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W<VMCRrs> {
        PGE_W::new(self, 16)
    }
    ///Bit 20 - Pattern generator mode This bit configures the pattern generator mode.
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W<VMCRrs> {
        PGM_W::new(self, 20)
    }
    ///Bit 24 - Pattern generator orientation This bit configures the color bar orientation.
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W<VMCRrs> {
        PGO_W::new(self, 24)
    }
}
/**DSI Host video mode configuration register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VMCR)*/
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmcr::R`](R) reader structure
impl crate::Readable for VMCRrs {}
///`write(|w| ..)` method takes [`vmcr::W`](W) writer structure
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VMCR to value 0
impl crate::Resettable for VMCRrs {}
