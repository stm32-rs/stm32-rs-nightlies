///Register `CMCR` reader
pub type R = crate::R<CMCRrs>;
///Register `CMCR` writer
pub type W = crate::W<CMCRrs>;
/**Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEARE {
    ///0: Tearing effect acknowledge request is disabled.
    B0x0 = 0,
    ///1: Tearing effect acknowledge request is enabled.
    B0x1 = 1,
}
impl From<TEARE> for bool {
    #[inline(always)]
    fn from(variant: TEARE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEARE` reader - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
pub type TEARE_R = crate::BitReader<TEARE>;
impl TEARE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEARE {
        match self.bits {
            false => TEARE::B0x0,
            true => TEARE::B0x1,
        }
    }
    ///Tearing effect acknowledge request is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEARE::B0x0
    }
    ///Tearing effect acknowledge request is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEARE::B0x1
    }
}
///Field `TEARE` writer - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
pub type TEARE_W<'a, REG> = crate::BitWriter<'a, REG, TEARE>;
impl<'a, REG> TEARE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tearing effect acknowledge request is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEARE::B0x0)
    }
    ///Tearing effect acknowledge request is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEARE::B0x1)
    }
}
/**Acknowledge request enable This bit enables the acknowledge request after each packet transmission:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARE {
    ///0: Acknowledge request is disabled.
    B0x0 = 0,
    ///1: Acknowledge request is enabled.
    B0x1 = 1,
}
impl From<ARE> for bool {
    #[inline(always)]
    fn from(variant: ARE) -> Self {
        variant as u8 != 0
    }
}
///Field `ARE` reader - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
pub type ARE_R = crate::BitReader<ARE>;
impl ARE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARE {
        match self.bits {
            false => ARE::B0x0,
            true => ARE::B0x1,
        }
    }
    ///Acknowledge request is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ARE::B0x0
    }
    ///Acknowledge request is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ARE::B0x1
    }
}
///Field `ARE` writer - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
pub type ARE_W<'a, REG> = crate::BitWriter<'a, REG, ARE>;
impl<'a, REG> ARE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Acknowledge request is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARE::B0x0)
    }
    ///Acknowledge request is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARE::B0x1)
    }
}
/**Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSW0TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSW0TX> for bool {
    #[inline(always)]
    fn from(variant: GSW0TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSW0TX` reader - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
pub type GSW0TX_R = crate::BitReader<GSW0TX>;
impl GSW0TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSW0TX {
        match self.bits {
            false => GSW0TX::B0x0,
            true => GSW0TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSW0TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSW0TX::B0x1
    }
}
///Field `GSW0TX` writer - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
pub type GSW0TX_W<'a, REG> = crate::BitWriter<'a, REG, GSW0TX>;
impl<'a, REG> GSW0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSW0TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSW0TX::B0x1)
    }
}
/**Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSW1TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSW1TX> for bool {
    #[inline(always)]
    fn from(variant: GSW1TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSW1TX` reader - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
pub type GSW1TX_R = crate::BitReader<GSW1TX>;
impl GSW1TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSW1TX {
        match self.bits {
            false => GSW1TX::B0x0,
            true => GSW1TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSW1TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSW1TX::B0x1
    }
}
///Field `GSW1TX` writer - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
pub type GSW1TX_W<'a, REG> = crate::BitWriter<'a, REG, GSW1TX>;
impl<'a, REG> GSW1TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSW1TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSW1TX::B0x1)
    }
}
/**Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSW2TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSW2TX> for bool {
    #[inline(always)]
    fn from(variant: GSW2TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSW2TX` reader - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
pub type GSW2TX_R = crate::BitReader<GSW2TX>;
impl GSW2TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSW2TX {
        match self.bits {
            false => GSW2TX::B0x0,
            true => GSW2TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSW2TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSW2TX::B0x1
    }
}
///Field `GSW2TX` writer - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
pub type GSW2TX_W<'a, REG> = crate::BitWriter<'a, REG, GSW2TX>;
impl<'a, REG> GSW2TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSW2TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSW2TX::B0x1)
    }
}
/**Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSR0TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSR0TX> for bool {
    #[inline(always)]
    fn from(variant: GSR0TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSR0TX` reader - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
pub type GSR0TX_R = crate::BitReader<GSR0TX>;
impl GSR0TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSR0TX {
        match self.bits {
            false => GSR0TX::B0x0,
            true => GSR0TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSR0TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSR0TX::B0x1
    }
}
///Field `GSR0TX` writer - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
pub type GSR0TX_W<'a, REG> = crate::BitWriter<'a, REG, GSR0TX>;
impl<'a, REG> GSR0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSR0TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSR0TX::B0x1)
    }
}
/**Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSR1TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSR1TX> for bool {
    #[inline(always)]
    fn from(variant: GSR1TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSR1TX` reader - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
pub type GSR1TX_R = crate::BitReader<GSR1TX>;
impl GSR1TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSR1TX {
        match self.bits {
            false => GSR1TX::B0x0,
            true => GSR1TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSR1TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSR1TX::B0x1
    }
}
///Field `GSR1TX` writer - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
pub type GSR1TX_W<'a, REG> = crate::BitWriter<'a, REG, GSR1TX>;
impl<'a, REG> GSR1TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSR1TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSR1TX::B0x1)
    }
}
/**Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSR2TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GSR2TX> for bool {
    #[inline(always)]
    fn from(variant: GSR2TX) -> Self {
        variant as u8 != 0
    }
}
///Field `GSR2TX` reader - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
pub type GSR2TX_R = crate::BitReader<GSR2TX>;
impl GSR2TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GSR2TX {
        match self.bits {
            false => GSR2TX::B0x0,
            true => GSR2TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSR2TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSR2TX::B0x1
    }
}
///Field `GSR2TX` writer - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
pub type GSR2TX_W<'a, REG> = crate::BitWriter<'a, REG, GSR2TX>;
impl<'a, REG> GSR2TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GSR2TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GSR2TX::B0x1)
    }
}
/**Generic long write transmission This bit configures the generic long write packet command transmission type :

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GLWTX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<GLWTX> for bool {
    #[inline(always)]
    fn from(variant: GLWTX) -> Self {
        variant as u8 != 0
    }
}
///Field `GLWTX` reader - Generic long write transmission This bit configures the generic long write packet command transmission type :
pub type GLWTX_R = crate::BitReader<GLWTX>;
impl GLWTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GLWTX {
        match self.bits {
            false => GLWTX::B0x0,
            true => GLWTX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GLWTX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GLWTX::B0x1
    }
}
///Field `GLWTX` writer - Generic long write transmission This bit configures the generic long write packet command transmission type :
pub type GLWTX_W<'a, REG> = crate::BitWriter<'a, REG, GLWTX>;
impl<'a, REG> GLWTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GLWTX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GLWTX::B0x1)
    }
}
/**DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSW0TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<DSW0TX> for bool {
    #[inline(always)]
    fn from(variant: DSW0TX) -> Self {
        variant as u8 != 0
    }
}
///Field `DSW0TX` reader - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
pub type DSW0TX_R = crate::BitReader<DSW0TX>;
impl DSW0TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSW0TX {
        match self.bits {
            false => DSW0TX::B0x0,
            true => DSW0TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSW0TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSW0TX::B0x1
    }
}
///Field `DSW0TX` writer - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
pub type DSW0TX_W<'a, REG> = crate::BitWriter<'a, REG, DSW0TX>;
impl<'a, REG> DSW0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSW0TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSW0TX::B0x1)
    }
}
/**DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSW1TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<DSW1TX> for bool {
    #[inline(always)]
    fn from(variant: DSW1TX) -> Self {
        variant as u8 != 0
    }
}
///Field `DSW1TX` reader - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
pub type DSW1TX_R = crate::BitReader<DSW1TX>;
impl DSW1TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSW1TX {
        match self.bits {
            false => DSW1TX::B0x0,
            true => DSW1TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSW1TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSW1TX::B0x1
    }
}
///Field `DSW1TX` writer - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
pub type DSW1TX_W<'a, REG> = crate::BitWriter<'a, REG, DSW1TX>;
impl<'a, REG> DSW1TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSW1TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSW1TX::B0x1)
    }
}
/**DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSR0TX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<DSR0TX> for bool {
    #[inline(always)]
    fn from(variant: DSR0TX) -> Self {
        variant as u8 != 0
    }
}
///Field `DSR0TX` reader - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
pub type DSR0TX_R = crate::BitReader<DSR0TX>;
impl DSR0TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSR0TX {
        match self.bits {
            false => DSR0TX::B0x0,
            true => DSR0TX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSR0TX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSR0TX::B0x1
    }
}
///Field `DSR0TX` writer - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
pub type DSR0TX_W<'a, REG> = crate::BitWriter<'a, REG, DSR0TX>;
impl<'a, REG> DSR0TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSR0TX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSR0TX::B0x1)
    }
}
/**DCS long write transmission This bit configures the DCS long write packet command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLWTX {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<DLWTX> for bool {
    #[inline(always)]
    fn from(variant: DLWTX) -> Self {
        variant as u8 != 0
    }
}
///Field `DLWTX` reader - DCS long write transmission This bit configures the DCS long write packet command transmission type:
pub type DLWTX_R = crate::BitReader<DLWTX>;
impl DLWTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLWTX {
        match self.bits {
            false => DLWTX::B0x0,
            true => DLWTX::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DLWTX::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DLWTX::B0x1
    }
}
///Field `DLWTX` writer - DCS long write transmission This bit configures the DCS long write packet command transmission type:
pub type DLWTX_W<'a, REG> = crate::BitWriter<'a, REG, DLWTX>;
impl<'a, REG> DLWTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DLWTX::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DLWTX::B0x1)
    }
}
/**Maximum read packet size This bit configures the maximum read packet size command transmission type:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRDPS {
    ///0: High-speed
    B0x0 = 0,
    ///1: Low-power
    B0x1 = 1,
}
impl From<MRDPS> for bool {
    #[inline(always)]
    fn from(variant: MRDPS) -> Self {
        variant as u8 != 0
    }
}
///Field `MRDPS` reader - Maximum read packet size This bit configures the maximum read packet size command transmission type:
pub type MRDPS_R = crate::BitReader<MRDPS>;
impl MRDPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MRDPS {
        match self.bits {
            false => MRDPS::B0x0,
            true => MRDPS::B0x1,
        }
    }
    ///High-speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRDPS::B0x0
    }
    ///Low-power
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRDPS::B0x1
    }
}
///Field `MRDPS` writer - Maximum read packet size This bit configures the maximum read packet size command transmission type:
pub type MRDPS_W<'a, REG> = crate::BitWriter<'a, REG, MRDPS>;
impl<'a, REG> MRDPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///High-speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MRDPS::B0x0)
    }
    ///Low-power
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MRDPS::B0x1)
    }
}
impl R {
    ///Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMCR")
            .field("teare", &self.teare())
            .field("are", &self.are())
            .field("gsw0tx", &self.gsw0tx())
            .field("gsw1tx", &self.gsw1tx())
            .field("gsw2tx", &self.gsw2tx())
            .field("gsr0tx", &self.gsr0tx())
            .field("gsr1tx", &self.gsr1tx())
            .field("gsr2tx", &self.gsr2tx())
            .field("glwtx", &self.glwtx())
            .field("dsw0tx", &self.dsw0tx())
            .field("dsw1tx", &self.dsw1tx())
            .field("dsr0tx", &self.dsr0tx())
            .field("dlwtx", &self.dlwtx())
            .field("mrdps", &self.mrdps())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
    #[inline(always)]
    pub fn teare(&mut self) -> TEARE_W<CMCRrs> {
        TEARE_W::new(self, 0)
    }
    ///Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
    #[inline(always)]
    pub fn are(&mut self) -> ARE_W<CMCRrs> {
        ARE_W::new(self, 1)
    }
    ///Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<CMCRrs> {
        GSW0TX_W::new(self, 8)
    }
    ///Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<CMCRrs> {
        GSW1TX_W::new(self, 9)
    }
    ///Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<CMCRrs> {
        GSW2TX_W::new(self, 10)
    }
    ///Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<CMCRrs> {
        GSR0TX_W::new(self, 11)
    }
    ///Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<CMCRrs> {
        GSR1TX_W::new(self, 12)
    }
    ///Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<CMCRrs> {
        GSR2TX_W::new(self, 13)
    }
    ///Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :
    #[inline(always)]
    pub fn glwtx(&mut self) -> GLWTX_W<CMCRrs> {
        GLWTX_W::new(self, 14)
    }
    ///Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<CMCRrs> {
        DSW0TX_W::new(self, 16)
    }
    ///Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<CMCRrs> {
        DSW1TX_W::new(self, 17)
    }
    ///Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<CMCRrs> {
        DSR0TX_W::new(self, 18)
    }
    ///Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DLWTX_W<CMCRrs> {
        DLWTX_W::new(self, 19)
    }
    ///Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:
    #[inline(always)]
    pub fn mrdps(&mut self) -> MRDPS_W<CMCRrs> {
        MRDPS_W::new(self, 24)
    }
}
/**DSI Host command mode configuration register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:CMCR)*/
pub struct CMCRrs;
impl crate::RegisterSpec for CMCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmcr::R`](R) reader structure
impl crate::Readable for CMCRrs {}
///`write(|w| ..)` method takes [`cmcr::W`](W) writer structure
impl crate::Writable for CMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMCR to value 0
impl crate::Resettable for CMCRrs {}
