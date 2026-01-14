///Register `ADC_CFGR1` reader
pub type R = crate::R<ADC_CFGR1rs>;
///Register `ADC_CFGR1` writer
pub type W = crate::W<ADC_CFGR1rs>;
/**Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA disabled
    B0x0 = 0,
    ///1: DMA enabled
    B0x1 = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::B0x0,
            true => DMAEN::B0x1,
        }
    }
    ///DMA disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAEN::B0x0
    }
    ///DMA enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAEN::B0x1
    }
}
///Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::B0x0)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::B0x1)
    }
}
/**Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    ///0: DMA one shot mode selected
    B0x0 = 0,
    ///1: DMA circular mode selected
    B0x1 = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
pub type DMACFG_R = crate::BitReader<DMACFG>;
impl DMACFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG {
        match self.bits {
            false => DMACFG::B0x0,
            true => DMACFG::B0x1,
        }
    }
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMACFG::B0x0
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMACFG::B0x1
    }
}
///Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::B0x0)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::B0x1)
    }
}
/**Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR {
    ///0: Upward scan (from CHSEL0 to CHSEL22)
    B0x0 = 0,
    ///1: Backward scan (from CHSEL22 to CHSEL0)
    B0x1 = 1,
}
impl From<SCANDIR> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type SCANDIR_R = crate::BitReader<SCANDIR>;
impl SCANDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCANDIR {
        match self.bits {
            false => SCANDIR::B0x0,
            true => SCANDIR::B0x1,
        }
    }
    ///Upward scan (from CHSEL0 to CHSEL22)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCANDIR::B0x0
    }
    ///Backward scan (from CHSEL22 to CHSEL0)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SCANDIR::B0x1
    }
}
///Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG, SCANDIR>;
impl<'a, REG> SCANDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Upward scan (from CHSEL0 to CHSEL22)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::B0x0)
    }
    ///Backward scan (from CHSEL22 to CHSEL0)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::B0x1)
    }
}
/**Data resolution These bits are written by software to select the resolution of the conversion.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12 bits
    B0x0 = 0,
    ///1: 10 bits
    B0x1 = 1,
    ///2: 8 bits
    B0x2 = 2,
    ///3: 6 bits
    B0x3 = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
///Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion.
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::B0x0,
            1 => RES::B0x1,
            2 => RES::B0x2,
            3 => RES::B0x3,
            _ => unreachable!(),
        }
    }
    ///12 bits
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RES::B0x0
    }
    ///10 bits
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RES::B0x1
    }
    ///8 bits
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RES::B0x2
    }
    ///6 bits
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RES::B0x3
    }
}
///Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion.
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12 bits
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RES::B0x0)
    }
    ///10 bits
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RES::B0x1)
    }
    ///8 bits
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RES::B0x2)
    }
    ///6 bits
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RES::B0x3)
    }
}
/**Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 43: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 323

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    ///0: Right alignment
    B0x0 = 0,
    ///1: Left alignment
    B0x1 = 1,
}
impl From<ALIGN> for bool {
    #[inline(always)]
    fn from(variant: ALIGN) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 43: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 323
pub type ALIGN_R = crate::BitReader<ALIGN>;
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN {
        match self.bits {
            false => ALIGN::B0x0,
            true => ALIGN::B0x1,
        }
    }
    ///Right alignment
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALIGN::B0x0
    }
    ///Left alignment
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALIGN::B0x1
    }
}
///Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 43: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 323
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Right alignment
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::B0x0)
    }
    ///Left alignment
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::B0x1)
    }
}
/**External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table 67: External triggers for details):

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: TRG0
    B0x0 = 0,
    ///1: TRG1
    B0x1 = 1,
    ///2: TRG2
    B0x2 = 2,
    ///3: TRG3
    B0x3 = 3,
    ///4: TRG4
    B0x4 = 4,
    ///5: TRG5
    B0x5 = 5,
    ///6: TRG6
    B0x6 = 6,
    ///7: TRG7
    B0x7 = 7,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table 67: External triggers for details):
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::B0x0,
            1 => EXTSEL::B0x1,
            2 => EXTSEL::B0x2,
            3 => EXTSEL::B0x3,
            4 => EXTSEL::B0x4,
            5 => EXTSEL::B0x5,
            6 => EXTSEL::B0x6,
            7 => EXTSEL::B0x7,
            _ => unreachable!(),
        }
    }
    ///TRG0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EXTSEL::B0x0
    }
    ///TRG1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EXTSEL::B0x1
    }
    ///TRG2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == EXTSEL::B0x2
    }
    ///TRG3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == EXTSEL::B0x3
    }
    ///TRG4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == EXTSEL::B0x4
    }
    ///TRG5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == EXTSEL::B0x5
    }
    ///TRG6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == EXTSEL::B0x6
    }
    ///TRG7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == EXTSEL::B0x7
    }
}
///Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table 67: External triggers for details):
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL, crate::Safe>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TRG0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x0)
    }
    ///TRG1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x1)
    }
    ///TRG2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x2)
    }
    ///TRG3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x3)
    }
    ///TRG4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x4)
    }
    ///TRG5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x5)
    }
    ///TRG6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x6)
    }
    ///TRG7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::B0x7)
    }
}
/**External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Hardware trigger detection disabled (conversions can be started by software)
    B0x0 = 0,
    ///1: Hardware trigger detection on the rising edge
    B0x1 = 1,
    ///2: Hardware trigger detection on the falling edge
    B0x2 = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    B0x3 = 3,
}
impl From<EXTEN> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN {}
///Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN {
        match self.bits {
            0 => EXTEN::B0x0,
            1 => EXTEN::B0x1,
            2 => EXTEN::B0x2,
            3 => EXTEN::B0x3,
            _ => unreachable!(),
        }
    }
    ///Hardware trigger detection disabled (conversions can be started by software)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EXTEN::B0x0
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EXTEN::B0x1
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == EXTEN::B0x2
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == EXTEN::B0x3
    }
}
///Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Hardware trigger detection disabled (conversions can be started by software)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::B0x0)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::B0x1)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::B0x2)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::B0x3)
    }
}
/**Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected.
    B0x0 = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected.
    B0x1 = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::B0x0,
            true => OVRMOD::B0x1,
        }
    }
    ///ADC_DR register is preserved with the old data when an overrun is detected.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVRMOD::B0x0
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVRMOD::B0x1
    }
}
///Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC_DR register is preserved with the old data when an overrun is detected.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::B0x0)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::B0x1)
    }
}
/**Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: Single conversion mode
    B0x0 = 0,
    ///1: Continuous conversion mode
    B0x1 = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::B0x0,
            true => CONT::B0x1,
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CONT::B0x0
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CONT::B0x1
    }
}
///Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single conversion mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::B0x0)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::B0x1)
    }
}
/**Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT {
    ///0: Wait conversion mode off
    B0x0 = 0,
    ///1: Wait conversion mode on
    B0x1 = 1,
}
impl From<WAIT> for bool {
    #[inline(always)]
    fn from(variant: WAIT) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
pub type WAIT_R = crate::BitReader<WAIT>;
impl WAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAIT {
        match self.bits {
            false => WAIT::B0x0,
            true => WAIT::B0x1,
        }
    }
    ///Wait conversion mode off
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAIT::B0x0
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAIT::B0x1
    }
}
///Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG, WAIT>;
impl<'a, REG> WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wait conversion mode off
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::B0x0)
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::B0x1)
    }
}
/**Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF {
    ///0: Auto-off mode disabled
    B0x0 = 0,
    ///1: Auto-off mode enabled
    B0x1 = 1,
}
impl From<AUTOFF> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
pub type AUTOFF_R = crate::BitReader<AUTOFF>;
impl AUTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOFF {
        match self.bits {
            false => AUTOFF::B0x0,
            true => AUTOFF::B0x1,
        }
    }
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AUTOFF::B0x0
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AUTOFF::B0x1
    }
}
///Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG, AUTOFF>;
impl<'a, REG> AUTOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::B0x0)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::B0x1)
    }
}
/**Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode disabled
    B0x0 = 0,
    ///1: Discontinuous mode enabled
    B0x1 = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::B0x0,
            true => DISCEN::B0x1,
        }
    }
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DISCEN::B0x0
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DISCEN::B0x1
    }
}
///Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::B0x0)
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::B0x1)
    }
}
/**Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELRMOD {
    ///0: Each bit of the ADC_CHSELR register enables an input
    B0x0 = 0,
    ///1: ADC_CHSELR register is able to sequence up to 8 channels
    B0x1 = 1,
}
impl From<CHSELRMOD> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSELRMOD_R = crate::BitReader<CHSELRMOD>;
impl CHSELRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSELRMOD {
        match self.bits {
            false => CHSELRMOD::B0x0,
            true => CHSELRMOD::B0x1,
        }
    }
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSELRMOD::B0x0
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSELRMOD::B0x1
    }
}
///Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG, CHSELRMOD>;
impl<'a, REG> CHSELRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD::B0x0)
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD::B0x1)
    }
}
/**Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    ///0: Analog watchdog 1 enabled on all channels
    B0x0 = 0,
    ///1: Analog watchdog 1 enabled on a single channel
    B0x1 = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::B0x0,
            true => AWD1SGL::B0x1,
        }
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1SGL::B0x0
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1SGL::B0x1
    }
}
///Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::B0x0)
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::B0x1)
    }
}
/**Analog watchdog enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    ///0: Analog watchdog 1 disabled
    B0x0 = 0,
    ///1: Analog watchdog 1 enabled
    B0x1 = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software.
pub type AWD1EN_R = crate::BitReader<AWD1EN>;
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN {
        match self.bits {
            false => AWD1EN::B0x0,
            true => AWD1EN::B0x1,
        }
    }
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1EN::B0x0
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1EN::B0x1
    }
}
///Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software.
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::B0x0)
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWD1CH {
    ///0: ADC analog input Channel 0 monitored by AWD
    B0x0 = 0,
    ///1: ADC analog input Channel 1 monitored by AWD
    B0x1 = 1,
    ///22: ADC analog input Channel 22 monitored by AWD
    B0x16 = 22,
}
impl From<AWD1CH> for u8 {
    #[inline(always)]
    fn from(variant: AWD1CH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWD1CH {
    type Ux = u8;
}
impl crate::IsEnum for AWD1CH {}
///Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
pub type AWD1CH_R = crate::FieldReader<AWD1CH>;
impl AWD1CH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWD1CH> {
        match self.bits {
            0 => Some(AWD1CH::B0x0),
            1 => Some(AWD1CH::B0x1),
            22 => Some(AWD1CH::B0x16),
            _ => None,
        }
    }
    ///ADC analog input Channel 0 monitored by AWD
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1CH::B0x0
    }
    ///ADC analog input Channel 1 monitored by AWD
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1CH::B0x1
    }
    ///ADC analog input Channel 22 monitored by AWD
    #[inline(always)]
    pub fn is_b_0x16(&self) -> bool {
        *self == AWD1CH::B0x16
    }
}
///Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, AWD1CH>;
impl<'a, REG> AWD1CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADC analog input Channel 0 monitored by AWD
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH::B0x0)
    }
    ///ADC analog input Channel 1 monitored by AWD
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH::B0x1)
    }
    ///ADC analog input Channel 22 monitored by AWD
    #[inline(always)]
    pub fn b_0x16(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH::B0x16)
    }
}
impl R {
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion.
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 43: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 323
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table 67: External triggers for details):
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR1")
            .field("dmaen", &self.dmaen())
            .field("dmacfg", &self.dmacfg())
            .field("scandir", &self.scandir())
            .field("res", &self.res())
            .field("align", &self.align())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("wait", &self.wait())
            .field("autoff", &self.autoff())
            .field("discen", &self.discen())
            .field("chselrmod", &self.chselrmod())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("awd1ch", &self.awd1ch())
            .finish()
    }
}
impl W {
    ///Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, ADC_CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN = 1. For more details, refer to Section 16.6.5: Managing converted data using the DMA on page 325.
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, ADC_CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    ///Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared. Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W<'_, ADC_CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    ///Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion.
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, ADC_CFGR1rs> {
        RES_W::new(self, 3)
    }
    ///Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Figure 43: Data alignment and resolution (oversampling disabled: OVSE = 0) on page 323
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, ADC_CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    ///Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to Table 67: External triggers for details):
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, ADC_CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    ///Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger.
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, ADC_CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed.
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, ADC_CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, ADC_CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.<sup>.</sup>
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<'_, ADC_CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    ///Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.<sup>.</sup>
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<'_, ADC_CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    ///Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN = 1 and CONT = 1.
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, ADC_CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<'_, ADC_CFGR1rs> {
        CHSELRMOD_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\[4:0\] bits or on all the channels
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, ADC_CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, ADC_CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\[4:0\] bits must be also set into the CHSELR register.
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, ADC_CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#ADC:ADC_CFGR1)*/
pub struct ADC_CFGR1rs;
impl crate::RegisterSpec for ADC_CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr1::R`](R) reader structure
impl crate::Readable for ADC_CFGR1rs {}
///`write(|w| ..)` method takes [`adc_cfgr1::W`](W) writer structure
impl crate::Writable for ADC_CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CFGR1 to value 0
impl crate::Resettable for ADC_CFGR1rs {}
