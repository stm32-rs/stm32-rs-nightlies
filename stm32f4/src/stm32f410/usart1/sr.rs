///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Parity error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: No parity error
    NoError = 0,
    ///1: Parity error
    Error = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::NoError,
            true => PE::Error,
        }
    }
    ///No parity error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PE::NoError
    }
    ///Parity error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PE::Error
    }
}
/**Framing error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE {
    ///0: No Framing error is detected
    NoError = 0,
    ///1: Framing error or break character is detected
    Error = 1,
}
impl From<FE> for bool {
    #[inline(always)]
    fn from(variant: FE) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader<FE>;
impl FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FE {
        match self.bits {
            false => FE::NoError,
            true => FE::Error,
        }
    }
    ///No Framing error is detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FE::NoError
    }
    ///Framing error or break character is detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FE::Error
    }
}
/**Noise detected flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NF {
    ///0: No noise is detected
    NoNoise = 0,
    ///1: Noise is detected
    Noise = 1,
}
impl From<NF> for bool {
    #[inline(always)]
    fn from(variant: NF) -> Self {
        variant as u8 != 0
    }
}
///Field `NF` reader - Noise detected flag
pub type NF_R = crate::BitReader<NF>;
impl NF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NF {
        match self.bits {
            false => NF::NoNoise,
            true => NF::Noise,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NF::NoNoise
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NF::Noise
    }
}
/**Overrun error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE {
    ///0: No Overrun error
    NoOverrun = 0,
    ///1: Overrun error is detected
    Overrun = 1,
}
impl From<ORE> for bool {
    #[inline(always)]
    fn from(variant: ORE) -> Self {
        variant as u8 != 0
    }
}
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader<ORE>;
impl ORE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORE {
        match self.bits {
            false => ORE::NoOverrun,
            true => ORE::Overrun,
        }
    }
    ///No Overrun error
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == ORE::NoOverrun
    }
    ///Overrun error is detected
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ORE::Overrun
    }
}
/**IDLE line detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE {
    ///0: No Idle Line is detected
    NoIdle = 0,
    ///1: Idle Line is detected
    Idle = 1,
}
impl From<IDLE> for bool {
    #[inline(always)]
    fn from(variant: IDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLE` reader - IDLE line detected
pub type IDLE_R = crate::BitReader<IDLE>;
impl IDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLE {
        match self.bits {
            false => IDLE::NoIdle,
            true => IDLE::Idle,
        }
    }
    ///No Idle Line is detected
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLE::NoIdle
    }
    ///Idle Line is detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE::Idle
    }
}
/**Read data register not empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNER {
    ///0: Data is not received
    NoData = 0,
    ///1: Received data is ready to be read
    DataReady = 1,
}
impl From<RXNER> for bool {
    #[inline(always)]
    fn from(variant: RXNER) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Read data register not empty
pub type RXNE_R = crate::BitReader<RXNER>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNER {
        match self.bits {
            false => RXNER::NoData,
            true => RXNER::DataReady,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == RXNER::NoData
    }
    ///Received data is ready to be read
    #[inline(always)]
    pub fn is_data_ready(&self) -> bool {
        *self == RXNER::DataReady
    }
}
/**Read data register not empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEW {
    ///0: Clear received data ready flag
    Clear = 0,
}
impl From<RXNEW> for bool {
    #[inline(always)]
    fn from(variant: RXNEW) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` writer - Read data register not empty
pub type RXNE_W<'a, REG> = crate::BitWriter0C<'a, REG, RXNEW>;
impl<'a, REG> RXNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear received data ready flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEW::Clear)
    }
}
/**Transmission complete

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCR {
    ///0: Transmission is not complete
    TxNotComplete = 0,
    ///1: Transmission is complete
    TxComplete = 1,
}
impl From<TCR> for bool {
    #[inline(always)]
    fn from(variant: TCR) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader<TCR>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCR {
        match self.bits {
            false => TCR::TxNotComplete,
            true => TCR::TxComplete,
        }
    }
    ///Transmission is not complete
    #[inline(always)]
    pub fn is_tx_not_complete(&self) -> bool {
        *self == TCR::TxNotComplete
    }
    ///Transmission is complete
    #[inline(always)]
    pub fn is_tx_complete(&self) -> bool {
        *self == TCR::TxComplete
    }
}
/**Transmission complete

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCW {
    ///0: Clear transmission complete flag
    Clear = 0,
}
impl From<TCW> for bool {
    #[inline(always)]
    fn from(variant: TCW) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` writer - Transmission complete
pub type TC_W<'a, REG> = crate::BitWriter0C<'a, REG, TCW>;
impl<'a, REG> TC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear transmission complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCW::Clear)
    }
}
/**Transmit data register empty

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Data is not transferred to the shift register
    TxNotEmpty = 0,
    ///1: Data is transferred to the shift register
    TxEmpty = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit data register empty
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::TxNotEmpty,
            true => TXE::TxEmpty,
        }
    }
    ///Data is not transferred to the shift register
    #[inline(always)]
    pub fn is_tx_not_empty(&self) -> bool {
        *self == TXE::TxNotEmpty
    }
    ///Data is transferred to the shift register
    #[inline(always)]
    pub fn is_tx_empty(&self) -> bool {
        *self == TXE::TxEmpty
    }
}
/**LIN break detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDR {
    ///0: LIN break not detected
    NotDetected = 0,
    ///1: LIN break detected
    Detected = 1,
}
impl From<LBDR> for bool {
    #[inline(always)]
    fn from(variant: LBDR) -> Self {
        variant as u8 != 0
    }
}
///Field `LBD` reader - LIN break detection flag
pub type LBD_R = crate::BitReader<LBDR>;
impl LBD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDR {
        match self.bits {
            false => LBDR::NotDetected,
            true => LBDR::Detected,
        }
    }
    ///LIN break not detected
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LBDR::NotDetected
    }
    ///LIN break detected
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LBDR::Detected
    }
}
/**LIN break detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDW {
    ///0: Clear LIN break detection flag
    Clear = 0,
}
impl From<LBDW> for bool {
    #[inline(always)]
    fn from(variant: LBDW) -> Self {
        variant as u8 != 0
    }
}
///Field `LBD` writer - LIN break detection flag
pub type LBD_W<'a, REG> = crate::BitWriter0C<'a, REG, LBDW>;
impl<'a, REG> LBD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear LIN break detection flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LBDW::Clear)
    }
}
/**CTS flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSR {
    ///0: No change occurred on the CTS status line
    NotChanged = 0,
    ///1: A change occurred on the CTS status line
    Changed = 1,
}
impl From<CTSR> for bool {
    #[inline(always)]
    fn from(variant: CTSR) -> Self {
        variant as u8 != 0
    }
}
///Field `CTS` reader - CTS flag
pub type CTS_R = crate::BitReader<CTSR>;
impl CTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSR {
        match self.bits {
            false => CTSR::NotChanged,
            true => CTSR::Changed,
        }
    }
    ///No change occurred on the CTS status line
    #[inline(always)]
    pub fn is_not_changed(&self) -> bool {
        *self == CTSR::NotChanged
    }
    ///A change occurred on the CTS status line
    #[inline(always)]
    pub fn is_changed(&self) -> bool {
        *self == CTSR::Changed
    }
}
/**CTS flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSW {
    ///0: Clear CTS toggle detection flag
    Clear = 0,
}
impl From<CTSW> for bool {
    #[inline(always)]
    fn from(variant: CTSW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTS` writer - CTS flag
pub type CTS_W<'a, REG> = crate::BitWriter0C<'a, REG, CTSW>;
impl<'a, REG> CTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear CTS toggle detection flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSW::Clear)
    }
}
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected flag
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS flag
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("cts", &self.cts())
            .field("lbd", &self.lbd())
            .field("txe", &self.txe())
            .field("tc", &self.tc())
            .field("rxne", &self.rxne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("nf", &self.nf())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<'_, SRrs> {
        RXNE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, SRrs> {
        TC_W::new(self, 6)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&mut self) -> LBD_W<'_, SRrs> {
        LBD_W::new(self, 8)
    }
    ///Bit 9 - CTS flag
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W<'_, SRrs> {
        CTS_W::new(self, 9)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#USART1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x0360;
}
///`reset()` method sets SR to value 0xc0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0xc0;
}
