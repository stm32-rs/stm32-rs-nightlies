///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**Memory mapping selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///0: Main Flash memory mapped at 0x0000_0000
    MainFlash = 0,
    ///1: System Flash memory mapped at 0x0000_0000
    SystemFlash = 1,
    ///2: Main Flash memory mapped at 0x0000_0000
    MainFlash2 = 2,
    ///3: Embedded SRAM mapped at 0x0000_0000
    Sram = 3,
}
impl From<MEM_MODE> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MODE {
    type Ux = u8;
}
impl crate::IsEnum for MEM_MODE {}
///Field `MEM_MODE` reader - Memory mapping selection bits
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEM_MODE {
        match self.bits {
            0 => MEM_MODE::MainFlash,
            1 => MEM_MODE::SystemFlash,
            2 => MEM_MODE::MainFlash2,
            3 => MEM_MODE::Sram,
            _ => unreachable!(),
        }
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_main_flash2(&self) -> bool {
        *self == MEM_MODE::MainFlash2
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE, crate::Safe>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash2(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash2)
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
}
/**PA11 and PA12 remapping bit for small packages (28 and 20 pins)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA11_PA12_RMP {
    ///0: Pin pair PA9/PA10 mapped on the pins
    NotRemapped = 0,
    ///1: Pin pair PA11/PA12 mapped instead of PA9/PA10
    Remapped = 1,
}
impl From<PA11_PA12_RMP> for bool {
    #[inline(always)]
    fn from(variant: PA11_PA12_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit for small packages (28 and 20 pins)
pub type PA11_PA12_RMP_R = crate::BitReader<PA11_PA12_RMP>;
impl PA11_PA12_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PA11_PA12_RMP {
        match self.bits {
            false => PA11_PA12_RMP::NotRemapped,
            true => PA11_PA12_RMP::Remapped,
        }
    }
    ///Pin pair PA9/PA10 mapped on the pins
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == PA11_PA12_RMP::NotRemapped
    }
    ///Pin pair PA11/PA12 mapped instead of PA9/PA10
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == PA11_PA12_RMP::Remapped
    }
}
///Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit for small packages (28 and 20 pins)
pub type PA11_PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG, PA11_PA12_RMP>;
impl<'a, REG> PA11_PA12_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pin pair PA9/PA10 mapped on the pins
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_PA12_RMP::NotRemapped)
    }
    ///Pin pair PA11/PA12 mapped instead of PA9/PA10
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_PA12_RMP::Remapped)
    }
}
/**IR Modulation Envelope signal selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IR_MOD {
    ///0: TIM16 selected
    Tim16 = 0,
    ///1: USART1 selected
    Usart1 = 1,
    ///2: USART4 selected
    Usart4 = 2,
}
impl From<IR_MOD> for u8 {
    #[inline(always)]
    fn from(variant: IR_MOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IR_MOD {
    type Ux = u8;
}
impl crate::IsEnum for IR_MOD {}
///Field `IR_MOD` reader - IR Modulation Envelope signal selection
pub type IR_MOD_R = crate::FieldReader<IR_MOD>;
impl IR_MOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IR_MOD> {
        match self.bits {
            0 => Some(IR_MOD::Tim16),
            1 => Some(IR_MOD::Usart1),
            2 => Some(IR_MOD::Usart4),
            _ => None,
        }
    }
    ///TIM16 selected
    #[inline(always)]
    pub fn is_tim16(&self) -> bool {
        *self == IR_MOD::Tim16
    }
    ///USART1 selected
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == IR_MOD::Usart1
    }
    ///USART4 selected
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == IR_MOD::Usart4
    }
}
///Field `IR_MOD` writer - IR Modulation Envelope signal selection
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IR_MOD>;
impl<'a, REG> IR_MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM16 selected
    #[inline(always)]
    pub fn tim16(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Tim16)
    }
    ///USART1 selected
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Usart1)
    }
    ///USART4 selected
    #[inline(always)]
    pub fn usart4(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::Usart4)
    }
}
/**ADC DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DMA_RMP {
    ///0: ADC DMA request mapped on DMA channel 1
    NotRemapped = 0,
    ///1: ADC DMA request mapped on DMA channel 2
    Remapped = 1,
}
impl From<ADC_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC_DMA_RMP` reader - ADC DMA remapping bit
pub type ADC_DMA_RMP_R = crate::BitReader<ADC_DMA_RMP>;
impl ADC_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DMA_RMP {
        match self.bits {
            false => ADC_DMA_RMP::NotRemapped,
            true => ADC_DMA_RMP::Remapped,
        }
    }
    ///ADC DMA request mapped on DMA channel 1
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == ADC_DMA_RMP::NotRemapped
    }
    ///ADC DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == ADC_DMA_RMP::Remapped
    }
}
///Field `ADC_DMA_RMP` writer - ADC DMA remapping bit
pub type ADC_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC_DMA_RMP>;
impl<'a, REG> ADC_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC DMA request mapped on DMA channel 1
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DMA_RMP::NotRemapped)
    }
    ///ADC DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DMA_RMP::Remapped)
    }
}
/**USART1_TX DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1_TX_DMA_RMP {
    ///0: USART1_TX DMA request mapped on DMA channel 2
    NotRemapped = 0,
    ///1: USART1_TX DMA request mapped on DMA channel 4
    Remapped = 1,
}
impl From<USART1_TX_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: USART1_TX_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1_TX_DMA_RMP` reader - USART1_TX DMA remapping bit
pub type USART1_TX_DMA_RMP_R = crate::BitReader<USART1_TX_DMA_RMP>;
impl USART1_TX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1_TX_DMA_RMP {
        match self.bits {
            false => USART1_TX_DMA_RMP::NotRemapped,
            true => USART1_TX_DMA_RMP::Remapped,
        }
    }
    ///USART1_TX DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP::NotRemapped
    }
    ///USART1_TX DMA request mapped on DMA channel 4
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP::Remapped
    }
}
///Field `USART1_TX_DMA_RMP` writer - USART1_TX DMA remapping bit
pub type USART1_TX_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, USART1_TX_DMA_RMP>;
impl<'a, REG> USART1_TX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART1_TX DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_TX_DMA_RMP::NotRemapped)
    }
    ///USART1_TX DMA request mapped on DMA channel 4
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_TX_DMA_RMP::Remapped)
    }
}
/**USART1_RX DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1_RX_DMA_RMP {
    ///0: USART1_RX DMA request mapped on DMA channel 3
    NotRemapped = 0,
    ///1: USART1_RX DMA request mapped on DMA channel 5
    Remapped = 1,
}
impl From<USART1_RX_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: USART1_RX_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1_RX_DMA_RMP` reader - USART1_RX DMA request remapping bit
pub type USART1_RX_DMA_RMP_R = crate::BitReader<USART1_RX_DMA_RMP>;
impl USART1_RX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1_RX_DMA_RMP {
        match self.bits {
            false => USART1_RX_DMA_RMP::NotRemapped,
            true => USART1_RX_DMA_RMP::Remapped,
        }
    }
    ///USART1_RX DMA request mapped on DMA channel 3
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP::NotRemapped
    }
    ///USART1_RX DMA request mapped on DMA channel 5
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP::Remapped
    }
}
///Field `USART1_RX_DMA_RMP` writer - USART1_RX DMA request remapping bit
pub type USART1_RX_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, USART1_RX_DMA_RMP>;
impl<'a, REG> USART1_RX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART1_RX DMA request mapped on DMA channel 3
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_RX_DMA_RMP::NotRemapped)
    }
    ///USART1_RX DMA request mapped on DMA channel 5
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_RX_DMA_RMP::Remapped)
    }
}
/**TIM16 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP {
    ///0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3
    NotRemapped = 0,
    ///1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4
    Remapped = 1,
}
impl From<TIM16_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit
pub type TIM16_DMA_RMP_R = crate::BitReader<TIM16_DMA_RMP>;
impl TIM16_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16_DMA_RMP {
        match self.bits {
            false => TIM16_DMA_RMP::NotRemapped,
            true => TIM16_DMA_RMP::Remapped,
        }
    }
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP::NotRemapped
    }
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP::Remapped
    }
}
///Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit
pub type TIM16_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM16_DMA_RMP>;
impl<'a, REG> TIM16_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP::NotRemapped)
    }
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP::Remapped)
    }
}
/**TIM17 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP {
    ///0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1
    NotRemapped = 0,
    ///1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2
    Remapped = 1,
}
impl From<TIM17_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit
pub type TIM17_DMA_RMP_R = crate::BitReader<TIM17_DMA_RMP>;
impl TIM17_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17_DMA_RMP {
        match self.bits {
            false => TIM17_DMA_RMP::NotRemapped,
            true => TIM17_DMA_RMP::Remapped,
        }
    }
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP::NotRemapped
    }
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP::Remapped
    }
}
///Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit
pub type TIM17_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM17_DMA_RMP>;
impl<'a, REG> TIM17_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP::NotRemapped)
    }
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP::Remapped)
    }
}
/**TIM16 alternate DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP2 {
    ///0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit
    NotAlternateRemapped = 0,
    ///1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6
    AlternateRemapped = 1,
}
impl From<TIM16_DMA_RMP2> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP2) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16_DMA_RMP2` reader - TIM16 alternate DMA request remapping bit
pub type TIM16_DMA_RMP2_R = crate::BitReader<TIM16_DMA_RMP2>;
impl TIM16_DMA_RMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16_DMA_RMP2 {
        match self.bits {
            false => TIM16_DMA_RMP2::NotAlternateRemapped,
            true => TIM16_DMA_RMP2::AlternateRemapped,
        }
    }
    ///TIM16 DMA request mapped according to TIM16_DMA_RMP bit
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2::NotAlternateRemapped
    }
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2::AlternateRemapped
    }
}
///Field `TIM16_DMA_RMP2` writer - TIM16 alternate DMA request remapping bit
pub type TIM16_DMA_RMP2_W<'a, REG> = crate::BitWriter<'a, REG, TIM16_DMA_RMP2>;
impl<'a, REG> TIM16_DMA_RMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM16 DMA request mapped according to TIM16_DMA_RMP bit
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP2::NotAlternateRemapped)
    }
    ///TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16_DMA_RMP2::AlternateRemapped)
    }
}
/**TIM17 alternate DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP2 {
    ///0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit
    NotAlternateRemapped = 0,
    ///1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7
    AlternateRemapped = 1,
}
impl From<TIM17_DMA_RMP2> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP2) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17_DMA_RMP2` reader - TIM17 alternate DMA request remapping bit
pub type TIM17_DMA_RMP2_R = crate::BitReader<TIM17_DMA_RMP2>;
impl TIM17_DMA_RMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17_DMA_RMP2 {
        match self.bits {
            false => TIM17_DMA_RMP2::NotAlternateRemapped,
            true => TIM17_DMA_RMP2::AlternateRemapped,
        }
    }
    ///TIM17 DMA request mapped according to TIM16_DMA_RMP bit
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2::NotAlternateRemapped
    }
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2::AlternateRemapped
    }
}
///Field `TIM17_DMA_RMP2` writer - TIM17 alternate DMA request remapping bit
pub type TIM17_DMA_RMP2_W<'a, REG> = crate::BitWriter<'a, REG, TIM17_DMA_RMP2>;
impl<'a, REG> TIM17_DMA_RMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM17 DMA request mapped according to TIM16_DMA_RMP bit
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP2::NotAlternateRemapped)
    }
    ///TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17_DMA_RMP2::AlternateRemapped)
    }
}
/**Fast Mode Plus (FM plus) driving capability activation bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    ///0: PB6 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM plus) driving capability activation bits.
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::Standard,
            true => I2C_PB6_FMP::Fmp,
        }
    }
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP::Fmp
    }
}
///Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM plus) driving capability activation bits.
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::Fmp)
    }
}
/**Fast Mode Plus (FM+) driving capability activation bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP {
    ///0: PB7 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP>;
impl I2C_PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB7_FMP {
        match self.bits {
            false => I2C_PB7_FMP::Standard,
            true => I2C_PB7_FMP::Fmp,
        }
    }
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP::Fmp
    }
}
///Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB7_FMP>;
impl<'a, REG> I2C_PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::Fmp)
    }
}
/**Fast Mode Plus (FM+) driving capability activation bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP {
    ///0: PB8 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP>;
impl I2C_PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB8_FMP {
        match self.bits {
            false => I2C_PB8_FMP::Standard,
            true => I2C_PB8_FMP::Fmp,
        }
    }
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP::Fmp
    }
}
///Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB8_FMP>;
impl<'a, REG> I2C_PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::Fmp)
    }
}
/**Fast Mode Plus (FM+) driving capability activation bits.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP {
    ///0: PB9 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP>;
impl I2C_PB9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB9_FMP {
        match self.bits {
            false => I2C_PB9_FMP::Standard,
            true => I2C_PB9_FMP::Fmp,
        }
    }
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP::Standard
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP::Fmp
    }
}
///Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB9_FMP>;
impl<'a, REG> I2C_PB9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::Fmp)
    }
}
/**FM+ driving capability activation for I2C1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::Standard,
            true => I2C1_FMP::Fmp,
        }
    }
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP::Standard
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP::Fmp
    }
}
///Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Standard)
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::Fmp)
    }
}
/**FM+ driving capability activation for I2C2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
    Fmp = 1,
}
impl From<I2C2_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP>;
impl I2C2_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2_FMP {
        match self.bits {
            false => I2C2_FMP::Standard,
            true => I2C2_FMP::Fmp,
        }
    }
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP::Standard
    }
    ///FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP::Fmp
    }
}
///Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C2_FMP>;
impl<'a, REG> I2C2_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Standard)
    }
    ///FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::Fmp)
    }
}
/**Fast Mode Plus (FM+) driving capability activation bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA9_FMP {
    ///0: PA9 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PA9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA9_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PA9_FMP_R = crate::BitReader<I2C_PA9_FMP>;
impl I2C_PA9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PA9_FMP {
        match self.bits {
            false => I2C_PA9_FMP::Standard,
            true => I2C_PA9_FMP::Fmp,
        }
    }
    ///PA9 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA9_FMP::Standard
    }
    ///I2C FM+ mode enabled on PA9 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA9_FMP::Fmp
    }
}
///Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PA9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PA9_FMP>;
impl<'a, REG> I2C_PA9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PA9 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA9_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PA9 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA9_FMP::Fmp)
    }
}
/**Fast Mode Plus (FM+) driving capability activation bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA10_FMP {
    ///0: PA10 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PA10_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA10_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PA10_FMP_R = crate::BitReader<I2C_PA10_FMP>;
impl I2C_PA10_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PA10_FMP {
        match self.bits {
            false => I2C_PA10_FMP::Standard,
            true => I2C_PA10_FMP::Fmp,
        }
    }
    ///PA10 pin operate in standard mode
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA10_FMP::Standard
    }
    ///I2C FM+ mode enabled on PA10 and the Speed control is bypassed
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA10_FMP::Fmp
    }
}
///Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits
pub type I2C_PA10_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PA10_FMP>;
impl<'a, REG> I2C_PA10_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PA10 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA10_FMP::Standard)
    }
    ///I2C FM+ mode enabled on PA10 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA10_FMP::Fmp)
    }
}
/**SPI2 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2_DMA_RMP {
    ///0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively
    NotRemapped = 0,
    ///1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    Remapped = 1,
}
impl From<SPI2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: SPI2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2_DMA_RMP` reader - SPI2 DMA request remapping bit
pub type SPI2_DMA_RMP_R = crate::BitReader<SPI2_DMA_RMP>;
impl SPI2_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2_DMA_RMP {
        match self.bits {
            false => SPI2_DMA_RMP::NotRemapped,
            true => SPI2_DMA_RMP::Remapped,
        }
    }
    ///SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP::NotRemapped
    }
    ///SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP::Remapped
    }
}
///Field `SPI2_DMA_RMP` writer - SPI2 DMA request remapping bit
pub type SPI2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, SPI2_DMA_RMP>;
impl<'a, REG> SPI2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2_DMA_RMP::NotRemapped)
    }
    ///SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2_DMA_RMP::Remapped)
    }
}
/**USART2 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2_DMA_RMP {
    ///0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively
    NotRemapped = 0,
    ///1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    Remapped = 1,
}
impl From<USART2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: USART2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2_DMA_RMP` reader - USART2 DMA request remapping bit
pub type USART2_DMA_RMP_R = crate::BitReader<USART2_DMA_RMP>;
impl USART2_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2_DMA_RMP {
        match self.bits {
            false => USART2_DMA_RMP::NotRemapped,
            true => USART2_DMA_RMP::Remapped,
        }
    }
    ///USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART2_DMA_RMP::NotRemapped
    }
    ///USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART2_DMA_RMP::Remapped
    }
}
///Field `USART2_DMA_RMP` writer - USART2 DMA request remapping bit
pub type USART2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, USART2_DMA_RMP>;
impl<'a, REG> USART2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART2_DMA_RMP::NotRemapped)
    }
    ///USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART2_DMA_RMP::Remapped)
    }
}
/**USART3 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3_DMA_RMP {
    ///0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)
    NotRemapped = 0,
    ///1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively
    Remapped = 1,
}
impl From<USART3_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: USART3_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `USART3_DMA_RMP` reader - USART3 DMA request remapping bit
pub type USART3_DMA_RMP_R = crate::BitReader<USART3_DMA_RMP>;
impl USART3_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART3_DMA_RMP {
        match self.bits {
            false => USART3_DMA_RMP::NotRemapped,
            true => USART3_DMA_RMP::Remapped,
        }
    }
    ///USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART3_DMA_RMP::NotRemapped
    }
    ///USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART3_DMA_RMP::Remapped
    }
}
///Field `USART3_DMA_RMP` writer - USART3 DMA request remapping bit
pub type USART3_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, USART3_DMA_RMP>;
impl<'a, REG> USART3_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_DMA_RMP::NotRemapped)
    }
    ///USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_DMA_RMP::Remapped)
    }
}
/**I2C1 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_DMA_RMP {
    ///0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively
    NotRemapped = 0,
    ///1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively
    Remapped = 1,
}
impl From<I2C1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1_DMA_RMP` reader - I2C1 DMA request remapping bit
pub type I2C1_DMA_RMP_R = crate::BitReader<I2C1_DMA_RMP>;
impl I2C1_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_DMA_RMP {
        match self.bits {
            false => I2C1_DMA_RMP::NotRemapped,
            true => I2C1_DMA_RMP::Remapped,
        }
    }
    ///I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP::NotRemapped
    }
    ///I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP::Remapped
    }
}
///Field `I2C1_DMA_RMP` writer - I2C1 DMA request remapping bit
pub type I2C1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_DMA_RMP>;
impl<'a, REG> I2C1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_DMA_RMP::NotRemapped)
    }
    ///I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_DMA_RMP::Remapped)
    }
}
/**TIM1 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_DMA_RMP {
    ///0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively
    NotRemapped = 0,
    ///1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6
    Remapped = 1,
}
impl From<TIM1_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM1_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1_DMA_RMP` reader - TIM1 DMA request remapping bit
pub type TIM1_DMA_RMP_R = crate::BitReader<TIM1_DMA_RMP>;
impl TIM1_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1_DMA_RMP {
        match self.bits {
            false => TIM1_DMA_RMP::NotRemapped,
            true => TIM1_DMA_RMP::Remapped,
        }
    }
    ///TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP::NotRemapped
    }
    ///TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP::Remapped
    }
}
///Field `TIM1_DMA_RMP` writer - TIM1 DMA request remapping bit
pub type TIM1_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM1_DMA_RMP>;
impl<'a, REG> TIM1_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_DMA_RMP::NotRemapped)
    }
    ///TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_DMA_RMP::Remapped)
    }
}
/**TIM2 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2_DMA_RMP {
    ///0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively
    NotRemapped = 0,
    ///1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7
    Remapped = 1,
}
impl From<TIM2_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM2_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2_DMA_RMP` reader - TIM2 DMA request remapping bit
pub type TIM2_DMA_RMP_R = crate::BitReader<TIM2_DMA_RMP>;
impl TIM2_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2_DMA_RMP {
        match self.bits {
            false => TIM2_DMA_RMP::NotRemapped,
            true => TIM2_DMA_RMP::Remapped,
        }
    }
    ///TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP::NotRemapped
    }
    ///TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP::Remapped
    }
}
///Field `TIM2_DMA_RMP` writer - TIM2 DMA request remapping bit
pub type TIM2_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM2_DMA_RMP>;
impl<'a, REG> TIM2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2_DMA_RMP::NotRemapped)
    }
    ///TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2_DMA_RMP::Remapped)
    }
}
/**TIM3 DMA request remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3_DMA_RMP {
    ///0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4
    NotRemapped = 0,
    ///1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6
    Remapped = 1,
}
impl From<TIM3_DMA_RMP> for bool {
    #[inline(always)]
    fn from(variant: TIM3_DMA_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3_DMA_RMP` reader - TIM3 DMA request remapping bit
pub type TIM3_DMA_RMP_R = crate::BitReader<TIM3_DMA_RMP>;
impl TIM3_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3_DMA_RMP {
        match self.bits {
            false => TIM3_DMA_RMP::NotRemapped,
            true => TIM3_DMA_RMP::Remapped,
        }
    }
    ///TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP::NotRemapped
    }
    ///TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP::Remapped
    }
}
///Field `TIM3_DMA_RMP` writer - TIM3 DMA request remapping bit
pub type TIM3_DMA_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TIM3_DMA_RMP>;
impl<'a, REG> TIM3_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3_DMA_RMP::NotRemapped)
    }
    ///TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6
    #[inline(always)]
    pub fn remapped(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3_DMA_RMP::Remapped)
    }
}
impl R {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - ADC DMA remapping bit
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USART1_TX DMA remapping bit
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&self) -> USART1_TX_DMA_RMP_R {
        USART1_TX_DMA_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USART1_RX DMA request remapping bit
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&self) -> USART1_RX_DMA_RMP_R {
        USART1_RX_DMA_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM16 DMA request remapping bit
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIM17 DMA request remapping bit
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM16 alternate DMA request remapping bit
    #[inline(always)]
    pub fn tim16_dma_rmp2(&self) -> TIM16_DMA_RMP2_R {
        TIM16_DMA_RMP2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TIM17 alternate DMA request remapping bit
    #[inline(always)]
    pub fn tim17_dma_rmp2(&self) -> TIM17_DMA_RMP2_R {
        TIM17_DMA_RMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SPI2 DMA request remapping bit
    #[inline(always)]
    pub fn spi2_dma_rmp(&self) -> SPI2_DMA_RMP_R {
        SPI2_DMA_RMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USART2 DMA request remapping bit
    #[inline(always)]
    pub fn usart2_dma_rmp(&self) -> USART2_DMA_RMP_R {
        USART2_DMA_RMP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USART3 DMA request remapping bit
    #[inline(always)]
    pub fn usart3_dma_rmp(&self) -> USART3_DMA_RMP_R {
        USART3_DMA_RMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - I2C1 DMA request remapping bit
    #[inline(always)]
    pub fn i2c1_dma_rmp(&self) -> I2C1_DMA_RMP_R {
        I2C1_DMA_RMP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - TIM1 DMA request remapping bit
    #[inline(always)]
    pub fn tim1_dma_rmp(&self) -> TIM1_DMA_RMP_R {
        TIM1_DMA_RMP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TIM2 DMA request remapping bit
    #[inline(always)]
    pub fn tim2_dma_rmp(&self) -> TIM2_DMA_RMP_R {
        TIM2_DMA_RMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TIM3 DMA request remapping bit
    #[inline(always)]
    pub fn tim3_dma_rmp(&self) -> TIM3_DMA_RMP_R {
        TIM3_DMA_RMP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("mem_mode", &self.mem_mode())
            .field("adc_dma_rmp", &self.adc_dma_rmp())
            .field("usart1_tx_dma_rmp", &self.usart1_tx_dma_rmp())
            .field("usart1_rx_dma_rmp", &self.usart1_rx_dma_rmp())
            .field("tim16_dma_rmp", &self.tim16_dma_rmp())
            .field("tim17_dma_rmp", &self.tim17_dma_rmp())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("spi2_dma_rmp", &self.spi2_dma_rmp())
            .field("usart2_dma_rmp", &self.usart2_dma_rmp())
            .field("usart3_dma_rmp", &self.usart3_dma_rmp())
            .field("i2c1_dma_rmp", &self.i2c1_dma_rmp())
            .field("tim1_dma_rmp", &self.tim1_dma_rmp())
            .field("tim2_dma_rmp", &self.tim2_dma_rmp())
            .field("tim3_dma_rmp", &self.tim3_dma_rmp())
            .field("ir_mod", &self.ir_mod())
            .field("tim16_dma_rmp2", &self.tim16_dma_rmp2())
            .field("tim17_dma_rmp2", &self.tim17_dma_rmp2())
            .field("pa11_pa12_rmp", &self.pa11_pa12_rmp())
            .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
            .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W<'_, CFGR1rs> {
        PA11_PA12_RMP_W::new(self, 4)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W<'_, CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    ///Bit 8 - ADC DMA remapping bit
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W<'_, CFGR1rs> {
        ADC_DMA_RMP_W::new(self, 8)
    }
    ///Bit 9 - USART1_TX DMA remapping bit
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&mut self) -> USART1_TX_DMA_RMP_W<'_, CFGR1rs> {
        USART1_TX_DMA_RMP_W::new(self, 9)
    }
    ///Bit 10 - USART1_RX DMA request remapping bit
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&mut self) -> USART1_RX_DMA_RMP_W<'_, CFGR1rs> {
        USART1_RX_DMA_RMP_W::new(self, 10)
    }
    ///Bit 11 - TIM16 DMA request remapping bit
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<'_, CFGR1rs> {
        TIM16_DMA_RMP_W::new(self, 11)
    }
    ///Bit 12 - TIM17 DMA request remapping bit
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<'_, CFGR1rs> {
        TIM17_DMA_RMP_W::new(self, 12)
    }
    ///Bit 13 - TIM16 alternate DMA request remapping bit
    #[inline(always)]
    pub fn tim16_dma_rmp2(&mut self) -> TIM16_DMA_RMP2_W<'_, CFGR1rs> {
        TIM16_DMA_RMP2_W::new(self, 13)
    }
    ///Bit 14 - TIM17 alternate DMA request remapping bit
    #[inline(always)]
    pub fn tim17_dma_rmp2(&mut self) -> TIM17_DMA_RMP2_W<'_, CFGR1rs> {
        TIM17_DMA_RMP2_W::new(self, 14)
    }
    ///Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - FM+ driving capability activation for I2C1
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - FM+ driving capability activation for I2C2
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<'_, CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    ///Bit 23 - Fast Mode Plus (FM+) driving capability activation bits
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<'_, CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    ///Bit 24 - SPI2 DMA request remapping bit
    #[inline(always)]
    pub fn spi2_dma_rmp(&mut self) -> SPI2_DMA_RMP_W<'_, CFGR1rs> {
        SPI2_DMA_RMP_W::new(self, 24)
    }
    ///Bit 25 - USART2 DMA request remapping bit
    #[inline(always)]
    pub fn usart2_dma_rmp(&mut self) -> USART2_DMA_RMP_W<'_, CFGR1rs> {
        USART2_DMA_RMP_W::new(self, 25)
    }
    ///Bit 26 - USART3 DMA request remapping bit
    #[inline(always)]
    pub fn usart3_dma_rmp(&mut self) -> USART3_DMA_RMP_W<'_, CFGR1rs> {
        USART3_DMA_RMP_W::new(self, 26)
    }
    ///Bit 27 - I2C1 DMA request remapping bit
    #[inline(always)]
    pub fn i2c1_dma_rmp(&mut self) -> I2C1_DMA_RMP_W<'_, CFGR1rs> {
        I2C1_DMA_RMP_W::new(self, 27)
    }
    ///Bit 28 - TIM1 DMA request remapping bit
    #[inline(always)]
    pub fn tim1_dma_rmp(&mut self) -> TIM1_DMA_RMP_W<'_, CFGR1rs> {
        TIM1_DMA_RMP_W::new(self, 28)
    }
    ///Bit 29 - TIM2 DMA request remapping bit
    #[inline(always)]
    pub fn tim2_dma_rmp(&mut self) -> TIM2_DMA_RMP_W<'_, CFGR1rs> {
        TIM2_DMA_RMP_W::new(self, 29)
    }
    ///Bit 30 - TIM3 DMA request remapping bit
    #[inline(always)]
    pub fn tim3_dma_rmp(&mut self) -> TIM3_DMA_RMP_W<'_, CFGR1rs> {
        TIM3_DMA_RMP_W::new(self, 30)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
