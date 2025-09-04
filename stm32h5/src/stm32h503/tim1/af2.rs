///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
/**TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE {
    ///0: BKIN input disabled
    Disabled = 0,
    ///1: BKIN input enabled
    Enabled = 1,
}
impl From<BK2INE> for bool {
    #[inline(always)]
    fn from(variant: BK2INE) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INE` reader - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_R = crate::BitReader<BK2INE>;
impl BK2INE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE {
        match self.bits {
            false => BK2INE::Disabled,
            true => BK2INE::Enabled,
        }
    }
    ///BKIN input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2INE::Disabled
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2INE::Enabled
    }
}
///Field `BK2INE` writer - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Disabled)
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Enabled)
    }
}
/**tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E {
    ///0: Input disabled
    Disabled = 0,
    ///1: Input enabled
    Enabled = 1,
}
impl From<BK2CMP1E> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP1E` reader - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E>;
impl BK2CMP1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1E {
        match self.bits {
            false => BK2CMP1E::Disabled,
            true => BK2CMP1E::Enabled,
        }
    }
    ///Input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP1E::Disabled
    }
    ///Input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP1E::Enabled
    }
}
///Field `BK2CMP1E` writer - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1E>;
impl<'a, REG> BK2CMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Disabled)
    }
    ///Input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Enabled)
    }
}
///Field `BK2CMP2E` reader - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP2E_R;
///Field `BK2CMP3E` reader - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP3E_R;
///Field `BK2CMP4E` reader - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP4E_R;
///Field `BK2CMP5E` reader - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP5E_R;
///Field `BK2CMP6E` reader - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP6E_R;
///Field `BK2CMP7E` reader - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP7E_R;
///Field `BK2CMP8E` reader - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_R as BK2CMP8E_R;
///Field `BK2CMP2E` writer - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP2E_W;
///Field `BK2CMP3E` writer - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP3E_W;
///Field `BK2CMP4E` writer - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP4E_W;
///Field `BK2CMP5E` writer - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP5E_W;
///Field `BK2CMP6E` writer - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP6E_W;
///Field `BK2CMP7E` writer - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP7E_W;
///Field `BK2CMP8E` writer - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1E_W as BK2CMP8E_W;
/**TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP {
    ///0: Input polarity not inverted
    NotInverted = 0,
    ///1: Input polarity inverted
    Inverted = 1,
}
impl From<BK2INP> for bool {
    #[inline(always)]
    fn from(variant: BK2INP) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INP` reader - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_R = crate::BitReader<BK2INP>;
impl BK2INP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP {
        match self.bits {
            false => BK2INP::NotInverted,
            true => BK2INP::Inverted,
        }
    }
    ///Input polarity not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2INP::NotInverted
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2INP::Inverted
    }
}
///Field `BK2INP` writer - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input polarity not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::NotInverted)
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::Inverted)
    }
}
/**tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P {
    ///0: Input polarity not inverted
    NotInverted = 0,
    ///1: Input polarity inverted
    Inverted = 1,
}
impl From<BK2CMP1P> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP1P` reader - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P>;
impl BK2CMP1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1P {
        match self.bits {
            false => BK2CMP1P::NotInverted,
            true => BK2CMP1P::Inverted,
        }
    }
    ///Input polarity not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP1P::NotInverted
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP1P::Inverted
    }
}
///Field `BK2CMP1P` writer - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1P>;
impl<'a, REG> BK2CMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input polarity not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::NotInverted)
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::Inverted)
    }
}
///Field `BK2CMP2P` reader - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_R as BK2CMP2P_R;
///Field `BK2CMP3P` reader - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_R as BK2CMP3P_R;
///Field `BK2CMP4P` reader - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_R as BK2CMP4P_R;
///Field `BK2CMP2P` writer - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_W as BK2CMP2P_W;
///Field `BK2CMP3P` writer - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_W as BK2CMP3P_W;
///Field `BK2CMP4P` writer - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub use BK2CMP1P_W as BK2CMP4P_W;
///Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_R = crate::FieldReader;
///Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    ///Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp8e(&self) -> BK2CMP8E_R {
        BK2CMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("bk2ine", &self.bk2ine())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2cmp3e", &self.bk2cmp3e())
            .field("bk2cmp4e", &self.bk2cmp4e())
            .field("bk2cmp5e", &self.bk2cmp5e())
            .field("bk2cmp6e", &self.bk2cmp6e())
            .field("bk2cmp7e", &self.bk2cmp7e())
            .field("bk2cmp8e", &self.bk2cmp8e())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2cmp2p", &self.bk2cmp2p())
            .field("bk2cmp3p", &self.bk2cmp3p())
            .field("bk2cmp4p", &self.bk2cmp4p())
            .field("ocrsel", &self.ocrsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W<AF2rs> {
        BK2INE_W::new(self, 0)
    }
    ///Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<AF2rs> {
        BK2CMP1E_W::new(self, 1)
    }
    ///Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<AF2rs> {
        BK2CMP2E_W::new(self, 2)
    }
    ///Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3e(&mut self) -> BK2CMP3E_W<AF2rs> {
        BK2CMP3E_W::new(self, 3)
    }
    ///Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4e(&mut self) -> BK2CMP4E_W<AF2rs> {
        BK2CMP4E_W::new(self, 4)
    }
    ///Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp5e(&mut self) -> BK2CMP5E_W<AF2rs> {
        BK2CMP5E_W::new(self, 5)
    }
    ///Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp6e(&mut self) -> BK2CMP6E_W<AF2rs> {
        BK2CMP6E_W::new(self, 6)
    }
    ///Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp7e(&mut self) -> BK2CMP7E_W<AF2rs> {
        BK2CMP7E_W::new(self, 7)
    }
    ///Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp8e(&mut self) -> BK2CMP8E_W<AF2rs> {
        BK2CMP8E_W::new(self, 8)
    }
    ///Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
    ///Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<AF2rs> {
        BK2CMP1P_W::new(self, 10)
    }
    ///Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<AF2rs> {
        BK2CMP2P_W::new(self, 11)
    }
    ///Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3p(&mut self) -> BK2CMP3P_W<AF2rs> {
        BK2CMP3P_W::new(self, 12)
    }
    ///Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4p(&mut self) -> BK2CMP4P_W<AF2rs> {
        BK2CMP4P_W::new(self, 13)
    }
    ///Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OCRSEL_W<AF2rs> {
        OCRSEL_W::new(self, 16)
    }
}
/**TIM1 alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TIM1:AF2)*/
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
///`read()` method returns [`af2::R`](R) reader structure
impl crate::Readable for AF2rs {}
///`write(|w| ..)` method takes [`af2::W`](W) writer structure
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF2 to value 0x01
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
