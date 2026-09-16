///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Enable Enable the QUADSPI.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: QUADSPI is disabled
    Disabled = 0,
    ///1: QUADSPI is enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Enable Enable the QUADSPI.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///QUADSPI is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///QUADSPI is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Enable Enable the QUADSPI.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///QUADSPI is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///QUADSPI is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT {
    ///0: No abort requested
    NoAbortRequested = 0,
    ///1: Abort requested
    AbortRequested = 1,
}
impl From<ABORT> for bool {
    #[inline(always)]
    fn from(variant: ABORT) -> Self {
        variant as u8 != 0
    }
}
///Field `ABORT` reader - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit.
pub type ABORT_R = crate::BitReader<ABORT>;
impl ABORT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABORT {
        match self.bits {
            false => ABORT::NoAbortRequested,
            true => ABORT::AbortRequested,
        }
    }
    ///No abort requested
    #[inline(always)]
    pub fn is_no_abort_requested(&self) -> bool {
        *self == ABORT::NoAbortRequested
    }
    ///Abort requested
    #[inline(always)]
    pub fn is_abort_requested(&self) -> bool {
        *self == ABORT::AbortRequested
    }
}
///Field `ABORT` writer - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit.
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No abort requested
    #[inline(always)]
    pub fn no_abort_requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::NoAbortRequested)
    }
    ///Abort requested
    #[inline(always)]
    pub fn abort_requested(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::AbortRequested)
    }
}
/**DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA is disabled for indirect mode
    Disabled = 0,
    ///1: DMA is enabled for indirect mode
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set.
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///DMA is disabled for indirect mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///DMA is enabled for indirect mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set.
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA is disabled for indirect mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///DMA is enabled for indirect mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
/**Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\[15:0\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\[3\]) so that nCS is released after a period of TIMEOUT\[15:0\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCEN {
    ///0: Timeout counter is disabled, and thus the chip select (nCS) remains active indefinitely after an access in memory-mapped mode.
    Disabled = 0,
    ///1: Timeout counter is enabled, and thus the chip select is released in memory-mapped mode after TIMEOUT\[15:0\] cycles of Flash memory inactivity.
    Enabled = 1,
}
impl From<TCEN> for bool {
    #[inline(always)]
    fn from(variant: TCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCEN` reader - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\[15:0\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\[3\]) so that nCS is released after a period of TIMEOUT\[15:0\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0.
pub type TCEN_R = crate::BitReader<TCEN>;
impl TCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCEN {
        match self.bits {
            false => TCEN::Disabled,
            true => TCEN::Enabled,
        }
    }
    ///Timeout counter is disabled, and thus the chip select (nCS) remains active indefinitely after an access in memory-mapped mode.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCEN::Disabled
    }
    ///Timeout counter is enabled, and thus the chip select is released in memory-mapped mode after TIMEOUT\[15:0\] cycles of Flash memory inactivity.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCEN::Enabled
    }
}
///Field `TCEN` writer - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\[15:0\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\[3\]) so that nCS is released after a period of TIMEOUT\[15:0\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0.
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG, TCEN>;
impl<'a, REG> TCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timeout counter is disabled, and thus the chip select (nCS) remains active indefinitely after an access in memory-mapped mode.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Disabled)
    }
    ///Timeout counter is enabled, and thus the chip select is released in memory-mapped mode after TIMEOUT\[15:0\] cycles of Flash memory inactivity.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCEN::Enabled)
    }
}
/**Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSHIFT {
    ///0: No shift
    NoShift = 0,
    ///1: 1/2 cycle shift
    OneHalfCycleShift = 1,
}
impl From<SSHIFT> for bool {
    #[inline(always)]
    fn from(variant: SSHIFT) -> Self {
        variant as u8 != 0
    }
}
///Field `SSHIFT` reader - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0.
pub type SSHIFT_R = crate::BitReader<SSHIFT>;
impl SSHIFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSHIFT {
        match self.bits {
            false => SSHIFT::NoShift,
            true => SSHIFT::OneHalfCycleShift,
        }
    }
    ///No shift
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == SSHIFT::NoShift
    }
    ///1/2 cycle shift
    #[inline(always)]
    pub fn is_one_half_cycle_shift(&self) -> bool {
        *self == SSHIFT::OneHalfCycleShift
    }
}
///Field `SSHIFT` writer - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0.
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG, SSHIFT>;
impl<'a, REG> SSHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No shift
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::NoShift)
    }
    ///1/2 cycle shift
    #[inline(always)]
    pub fn one_half_cycle_shift(self) -> &'a mut crate::W<REG> {
        self.variant(SSHIFT::OneHalfCycleShift)
    }
}
/**Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFM {
    ///0: Dual-flash mode disabled
    Disabled = 0,
    ///1: Dual-flash mode enabled
    Enabled = 1,
}
impl From<DFM> for bool {
    #[inline(always)]
    fn from(variant: DFM) -> Self {
        variant as u8 != 0
    }
}
///Field `DFM` reader - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0.
pub type DFM_R = crate::BitReader<DFM>;
impl DFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFM {
        match self.bits {
            false => DFM::Disabled,
            true => DFM::Enabled,
        }
    }
    ///Dual-flash mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFM::Disabled
    }
    ///Dual-flash mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFM::Enabled
    }
}
///Field `DFM` writer - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0.
pub type DFM_W<'a, REG> = crate::BitWriter<'a, REG, DFM>;
impl<'a, REG> DFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Dual-flash mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFM::Disabled)
    }
    ///Dual-flash mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFM::Enabled)
    }
}
/**Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEL {
    ///0: FLASH 1 selected
    SelectFlash1 = 0,
    ///1: FLASH 2 selected
    SelectFlash2 = 1,
}
impl From<FSEL> for bool {
    #[inline(always)]
    fn from(variant: FSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `FSEL` reader - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1.
pub type FSEL_R = crate::BitReader<FSEL>;
impl FSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSEL {
        match self.bits {
            false => FSEL::SelectFlash1,
            true => FSEL::SelectFlash2,
        }
    }
    ///FLASH 1 selected
    #[inline(always)]
    pub fn is_select_flash1(&self) -> bool {
        *self == FSEL::SelectFlash1
    }
    ///FLASH 2 selected
    #[inline(always)]
    pub fn is_select_flash2(&self) -> bool {
        *self == FSEL::SelectFlash2
    }
}
///Field `FSEL` writer - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1.
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG, FSEL>;
impl<'a, REG> FSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLASH 1 selected
    #[inline(always)]
    pub fn select_flash1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL::SelectFlash1)
    }
    ///FLASH 2 selected
    #[inline(always)]
    pub fn select_flash2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL::SelectFlash2)
    }
}
///Field `FTHRES` reader - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\[2\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value.
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\[2\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value.
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Transfer error interrupt enable This bit enables the transfer error interrupt.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: Interrupt disable
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transfer error interrupt enable This bit enables the transfer error interrupt.
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    ///Interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
///Field `TEIE` writer - Transfer error interrupt enable This bit enables the transfer error interrupt.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
pub use TEIE_R as TCIE_R;
///Field `FTIE` reader - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
pub use TEIE_R as FTIE_R;
///Field `SMIE` reader - Status match interrupt enable This bit enables the status match interrupt.
pub use TEIE_R as SMIE_R;
///Field `TOIE` reader - TimeOut interrupt enable This bit enables the TimeOut interrupt.
pub use TEIE_R as TOIE_R;
///Field `TCIE` writer - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
pub use TEIE_W as TCIE_W;
///Field `FTIE` writer - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
pub use TEIE_W as FTIE_W;
///Field `SMIE` writer - Status match interrupt enable This bit enables the status match interrupt.
pub use TEIE_W as SMIE_W;
///Field `TOIE` writer - TimeOut interrupt enable This bit enables the TimeOut interrupt.
pub use TEIE_W as TOIE_W;
/**Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APMS {
    ///0: Automatic polling mode is stopped only by abort or by disabling the QUADSPI.
    NotStopOnMatch = 0,
    ///1: Automatic polling mode stops as soon as there is a match.
    StopOnMatch = 1,
}
impl From<APMS> for bool {
    #[inline(always)]
    fn from(variant: APMS) -> Self {
        variant as u8 != 0
    }
}
///Field `APMS` reader - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0.
pub type APMS_R = crate::BitReader<APMS>;
impl APMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APMS {
        match self.bits {
            false => APMS::NotStopOnMatch,
            true => APMS::StopOnMatch,
        }
    }
    ///Automatic polling mode is stopped only by abort or by disabling the QUADSPI.
    #[inline(always)]
    pub fn is_not_stop_on_match(&self) -> bool {
        *self == APMS::NotStopOnMatch
    }
    ///Automatic polling mode stops as soon as there is a match.
    #[inline(always)]
    pub fn is_stop_on_match(&self) -> bool {
        *self == APMS::StopOnMatch
    }
}
///Field `APMS` writer - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0.
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG, APMS>;
impl<'a, REG> APMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic polling mode is stopped only by abort or by disabling the QUADSPI.
    #[inline(always)]
    pub fn not_stop_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::NotStopOnMatch)
    }
    ///Automatic polling mode stops as soon as there is a match.
    #[inline(always)]
    pub fn stop_on_match(self) -> &'a mut crate::W<REG> {
        self.variant(APMS::StopOnMatch)
    }
}
/**Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMM {
    ///0: AND match mode. SMF is set if all the unmasked bits received from the Flash memory match the corresponding bits in the match register.
    AndMatch = 0,
    ///1: OR match mode. SMF is set if any one of the unmasked bits received from the Flash memory matches its corresponding bit in the match register.
    OrMatch = 1,
}
impl From<PMM> for bool {
    #[inline(always)]
    fn from(variant: PMM) -> Self {
        variant as u8 != 0
    }
}
///Field `PMM` reader - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0.
pub type PMM_R = crate::BitReader<PMM>;
impl PMM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMM {
        match self.bits {
            false => PMM::AndMatch,
            true => PMM::OrMatch,
        }
    }
    ///AND match mode. SMF is set if all the unmasked bits received from the Flash memory match the corresponding bits in the match register.
    #[inline(always)]
    pub fn is_and_match(&self) -> bool {
        *self == PMM::AndMatch
    }
    ///OR match mode. SMF is set if any one of the unmasked bits received from the Flash memory matches its corresponding bit in the match register.
    #[inline(always)]
    pub fn is_or_match(&self) -> bool {
        *self == PMM::OrMatch
    }
}
///Field `PMM` writer - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0.
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG, PMM>;
impl<'a, REG> PMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AND match mode. SMF is set if all the unmasked bits received from the Flash memory match the corresponding bits in the match register.
    #[inline(always)]
    pub fn and_match(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::AndMatch)
    }
    ///OR match mode. SMF is set if any one of the unmasked bits received from the Flash memory matches its corresponding bit in the match register.
    #[inline(always)]
    pub fn or_match(self) -> &'a mut crate::W<REG> {
        self.variant(PMM::OrMatch)
    }
}
///Field `PRESCALER` reader - clock prescaler
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - clock prescaler
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bit 0 - Enable Enable the QUADSPI.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit.
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\[15:0\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\[3\]) so that nCS is released after a period of TIMEOUT\[15:0\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1.
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\[2\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value.
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match interrupt enable This bit enables the status match interrupt.
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt.
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("abort", &self.abort())
            .field("dmaen", &self.dmaen())
            .field("tcen", &self.tcen())
            .field("sshift", &self.sshift())
            .field("dfm", &self.dfm())
            .field("fsel", &self.fsel())
            .field("fthres", &self.fthres())
            .field("teie", &self.teie())
            .field("tcie", &self.tcie())
            .field("ftie", &self.ftie())
            .field("smie", &self.smie())
            .field("toie", &self.toie())
            .field("apms", &self.apms())
            .field("pmm", &self.pmm())
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Enable the QUADSPI.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit.
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<'_, CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\[15:0\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\[3\]) so that nCS is released after a period of TIMEOUT\[15:0\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<'_, CRrs> {
        SSHIFT_W::new(self, 4)
    }
    ///Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn dfm(&mut self) -> DFM_W<'_, CRrs> {
        DFM_W::new(self, 6)
    }
    ///Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1.
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W<'_, CRrs> {
        FSEL_W::new(self, 7)
    }
    ///Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\[2\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value.
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<'_, CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt.
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<'_, CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - Status match interrupt enable This bit enables the status match interrupt.
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<'_, CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt.
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<'_, CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<'_, CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<'_, CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bits 24:31 - clock prescaler
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<'_, CRrs> {
        PRESCALER_W::new(self, 24)
    }
}
/**QUADSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#QUADSPI:CR)*/
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
