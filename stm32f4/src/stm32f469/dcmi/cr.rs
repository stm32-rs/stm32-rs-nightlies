///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Capture enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTURE {
    ///0: Capture disabled
    Disabled = 0,
    ///1: Capture enabled
    Enabled = 1,
}
impl From<CAPTURE> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE) -> Self {
        variant as u8 != 0
    }
}
///Field `CAPTURE` reader - Capture enable
pub type CAPTURE_R = crate::BitReader<CAPTURE>;
impl CAPTURE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CAPTURE {
        match self.bits {
            false => CAPTURE::Disabled,
            true => CAPTURE::Enabled,
        }
    }
    ///Capture disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPTURE::Disabled
    }
    ///Capture enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPTURE::Enabled
    }
}
///Field `CAPTURE` writer - Capture enable
pub type CAPTURE_W<'a, REG> = crate::BitWriter<'a, REG, CAPTURE>;
impl<'a, REG> CAPTURE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURE::Disabled)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURE::Enabled)
    }
}
/**Capture mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CM {
    ///0: Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA
    Continuous = 0,
    ///1: Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset
    Snapshot = 1,
}
impl From<CM> for bool {
    #[inline(always)]
    fn from(variant: CM) -> Self {
        variant as u8 != 0
    }
}
///Field `CM` reader - Capture mode
pub type CM_R = crate::BitReader<CM>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CM {
        match self.bits {
            false => CM::Continuous,
            true => CM::Snapshot,
        }
    }
    ///Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CM::Continuous
    }
    ///Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset
    #[inline(always)]
    pub fn is_snapshot(&self) -> bool {
        *self == CM::Snapshot
    }
}
///Field `CM` writer - Capture mode
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Continuous)
    }
    ///Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset
    #[inline(always)]
    pub fn snapshot(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Snapshot)
    }
}
/**Crop feature

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CROP {
    ///0: The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four
    Full = 0,
    ///1: Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured
    Cropped = 1,
}
impl From<CROP> for bool {
    #[inline(always)]
    fn from(variant: CROP) -> Self {
        variant as u8 != 0
    }
}
///Field `CROP` reader - Crop feature
pub type CROP_R = crate::BitReader<CROP>;
impl CROP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CROP {
        match self.bits {
            false => CROP::Full,
            true => CROP::Cropped,
        }
    }
    ///The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CROP::Full
    }
    ///Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured
    #[inline(always)]
    pub fn is_cropped(&self) -> bool {
        *self == CROP::Cropped
    }
}
///Field `CROP` writer - Crop feature
pub type CROP_W<'a, REG> = crate::BitWriter<'a, REG, CROP>;
impl<'a, REG> CROP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(CROP::Full)
    }
    ///Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured
    #[inline(always)]
    pub fn cropped(self) -> &'a mut crate::W<REG> {
        self.variant(CROP::Cropped)
    }
}
/**JPEG format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JPEG {
    ///0: Uncompressed video format
    Uncompressed = 0,
    ///1: This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode
    Jpeg = 1,
}
impl From<JPEG> for bool {
    #[inline(always)]
    fn from(variant: JPEG) -> Self {
        variant as u8 != 0
    }
}
///Field `JPEG` reader - JPEG format
pub type JPEG_R = crate::BitReader<JPEG>;
impl JPEG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JPEG {
        match self.bits {
            false => JPEG::Uncompressed,
            true => JPEG::Jpeg,
        }
    }
    ///Uncompressed video format
    #[inline(always)]
    pub fn is_uncompressed(&self) -> bool {
        *self == JPEG::Uncompressed
    }
    ///This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode
    #[inline(always)]
    pub fn is_jpeg(&self) -> bool {
        *self == JPEG::Jpeg
    }
}
///Field `JPEG` writer - JPEG format
pub type JPEG_W<'a, REG> = crate::BitWriter<'a, REG, JPEG>;
impl<'a, REG> JPEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Uncompressed video format
    #[inline(always)]
    pub fn uncompressed(self) -> &'a mut crate::W<REG> {
        self.variant(JPEG::Uncompressed)
    }
    ///This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode
    #[inline(always)]
    pub fn jpeg(self) -> &'a mut crate::W<REG> {
        self.variant(JPEG::Jpeg)
    }
}
/**Embedded synchronization select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESS {
    ///0: Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals
    Hardware = 0,
    ///1: Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow
    Embedded = 1,
}
impl From<ESS> for bool {
    #[inline(always)]
    fn from(variant: ESS) -> Self {
        variant as u8 != 0
    }
}
///Field `ESS` reader - Embedded synchronization select
pub type ESS_R = crate::BitReader<ESS>;
impl ESS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESS {
        match self.bits {
            false => ESS::Hardware,
            true => ESS::Embedded,
        }
    }
    ///Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == ESS::Hardware
    }
    ///Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == ESS::Embedded
    }
}
///Field `ESS` writer - Embedded synchronization select
pub type ESS_W<'a, REG> = crate::BitWriter<'a, REG, ESS>;
impl<'a, REG> ESS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(ESS::Hardware)
    }
    ///Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow
    #[inline(always)]
    pub fn embedded(self) -> &'a mut crate::W<REG> {
        self.variant(ESS::Embedded)
    }
}
/**Pixel clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKPOL {
    ///0: Falling edge active
    FallingEdge = 0,
    ///1: Rising edge active
    RisingEdge = 1,
}
impl From<PCKPOL> for bool {
    #[inline(always)]
    fn from(variant: PCKPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `PCKPOL` reader - Pixel clock polarity
pub type PCKPOL_R = crate::BitReader<PCKPOL>;
impl PCKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCKPOL {
        match self.bits {
            false => PCKPOL::FallingEdge,
            true => PCKPOL::RisingEdge,
        }
    }
    ///Falling edge active
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCKPOL::FallingEdge
    }
    ///Rising edge active
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCKPOL::RisingEdge
    }
}
///Field `PCKPOL` writer - Pixel clock polarity
pub type PCKPOL_W<'a, REG> = crate::BitWriter<'a, REG, PCKPOL>;
impl<'a, REG> PCKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge active
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCKPOL::FallingEdge)
    }
    ///Rising edge active
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCKPOL::RisingEdge)
    }
}
/**Horizontal synchronization polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSPOL {
    ///0: DCMI_HSYNC active low
    ActiveLow = 0,
    ///1: DCMI_HSYNC active high
    ActiveHigh = 1,
}
impl From<HSPOL> for bool {
    #[inline(always)]
    fn from(variant: HSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `HSPOL` reader - Horizontal synchronization polarity
pub type HSPOL_R = crate::BitReader<HSPOL>;
impl HSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSPOL {
        match self.bits {
            false => HSPOL::ActiveLow,
            true => HSPOL::ActiveHigh,
        }
    }
    ///DCMI_HSYNC active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL::ActiveLow
    }
    ///DCMI_HSYNC active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL::ActiveHigh
    }
}
///Field `HSPOL` writer - Horizontal synchronization polarity
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG, HSPOL>;
impl<'a, REG> HSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCMI_HSYNC active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveLow)
    }
    ///DCMI_HSYNC active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveHigh)
    }
}
/**Vertical synchronization polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSPOL {
    ///0: DCMI_VSYNC active low
    ActiveLow = 0,
    ///1: DCMI_VSYNC active high
    ActiveHigh = 1,
}
impl From<VSPOL> for bool {
    #[inline(always)]
    fn from(variant: VSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `VSPOL` reader - Vertical synchronization polarity
pub type VSPOL_R = crate::BitReader<VSPOL>;
impl VSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSPOL {
        match self.bits {
            false => VSPOL::ActiveLow,
            true => VSPOL::ActiveHigh,
        }
    }
    ///DCMI_VSYNC active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL::ActiveLow
    }
    ///DCMI_VSYNC active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL::ActiveHigh
    }
}
///Field `VSPOL` writer - Vertical synchronization polarity
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG, VSPOL>;
impl<'a, REG> VSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCMI_VSYNC active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveLow)
    }
    ///DCMI_VSYNC active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveHigh)
    }
}
/**Frame capture rate control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCRC {
    ///0: All frames are captured
    All = 0,
    ///1: Every alternate frame captured (50% bandwidth reduction)
    Alternate = 1,
    ///2: One frame out of four captured (75% bandwidth reduction)
    OneOfFour = 2,
}
impl From<FCRC> for u8 {
    #[inline(always)]
    fn from(variant: FCRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCRC {
    type Ux = u8;
}
impl crate::IsEnum for FCRC {}
///Field `FCRC` reader - Frame capture rate control
pub type FCRC_R = crate::FieldReader<FCRC>;
impl FCRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FCRC> {
        match self.bits {
            0 => Some(FCRC::All),
            1 => Some(FCRC::Alternate),
            2 => Some(FCRC::OneOfFour),
            _ => None,
        }
    }
    ///All frames are captured
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == FCRC::All
    }
    ///Every alternate frame captured (50% bandwidth reduction)
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == FCRC::Alternate
    }
    ///One frame out of four captured (75% bandwidth reduction)
    #[inline(always)]
    pub fn is_one_of_four(&self) -> bool {
        *self == FCRC::OneOfFour
    }
}
///Field `FCRC` writer - Frame capture rate control
pub type FCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FCRC>;
impl<'a, REG> FCRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///All frames are captured
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(FCRC::All)
    }
    ///Every alternate frame captured (50% bandwidth reduction)
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(FCRC::Alternate)
    }
    ///One frame out of four captured (75% bandwidth reduction)
    #[inline(always)]
    pub fn one_of_four(self) -> &'a mut crate::W<REG> {
        self.variant(FCRC::OneOfFour)
    }
}
/**Extended data mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDM {
    ///0: Interface captures 8-bit data on every pixel clock
    BitWidth8 = 0,
    ///1: Interface captures 10-bit data on every pixel clock
    BitWidth10 = 1,
    ///2: Interface captures 12-bit data on every pixel clock
    BitWidth12 = 2,
    ///3: Interface captures 14-bit data on every pixel clock
    BitWidth14 = 3,
}
impl From<EDM> for u8 {
    #[inline(always)]
    fn from(variant: EDM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDM {
    type Ux = u8;
}
impl crate::IsEnum for EDM {}
///Field `EDM` reader - Extended data mode
pub type EDM_R = crate::FieldReader<EDM>;
impl EDM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDM {
        match self.bits {
            0 => EDM::BitWidth8,
            1 => EDM::BitWidth10,
            2 => EDM::BitWidth12,
            3 => EDM::BitWidth14,
            _ => unreachable!(),
        }
    }
    ///Interface captures 8-bit data on every pixel clock
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == EDM::BitWidth8
    }
    ///Interface captures 10-bit data on every pixel clock
    #[inline(always)]
    pub fn is_bit_width10(&self) -> bool {
        *self == EDM::BitWidth10
    }
    ///Interface captures 12-bit data on every pixel clock
    #[inline(always)]
    pub fn is_bit_width12(&self) -> bool {
        *self == EDM::BitWidth12
    }
    ///Interface captures 14-bit data on every pixel clock
    #[inline(always)]
    pub fn is_bit_width14(&self) -> bool {
        *self == EDM::BitWidth14
    }
}
///Field `EDM` writer - Extended data mode
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EDM, crate::Safe>;
impl<'a, REG> EDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Interface captures 8-bit data on every pixel clock
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth8)
    }
    ///Interface captures 10-bit data on every pixel clock
    #[inline(always)]
    pub fn bit_width10(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth10)
    }
    ///Interface captures 12-bit data on every pixel clock
    #[inline(always)]
    pub fn bit_width12(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth12)
    }
    ///Interface captures 14-bit data on every pixel clock
    #[inline(always)]
    pub fn bit_width14(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth14)
    }
}
/**DCMI enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    ///0: DCMI disabled
    Disabled = 0,
    ///1: DCMI enabled
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - DCMI enable
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    ///DCMI disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    ///DCMI enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
///Field `ENABLE` writer - DCMI enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCMI disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    ///DCMI enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
impl R {
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("enable", &self.enable())
            .field("edm", &self.edm())
            .field("fcrc", &self.fcrc())
            .field("vspol", &self.vspol())
            .field("hspol", &self.hspol())
            .field("pckpol", &self.pckpol())
            .field("ess", &self.ess())
            .field("jpeg", &self.jpeg())
            .field("crop", &self.crop())
            .field("cm", &self.cm())
            .field("capture", &self.capture())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W<CRrs> {
        CAPTURE_W::new(self, 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<CRrs> {
        CM_W::new(self, 1)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&mut self) -> CROP_W<CRrs> {
        CROP_W::new(self, 2)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W<CRrs> {
        JPEG_W::new(self, 3)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&mut self) -> ESS_W<CRrs> {
        ESS_W::new(self, 4)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&mut self) -> PCKPOL_W<CRrs> {
        PCKPOL_W::new(self, 5)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<CRrs> {
        HSPOL_W::new(self, 6)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<CRrs> {
        VSPOL_W::new(self, 7)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&mut self) -> FCRC_W<CRrs> {
        FCRC_W::new(self, 8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<CRrs> {
        EDM_W::new(self, 10)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 14)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DCMI:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
