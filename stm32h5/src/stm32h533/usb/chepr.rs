///Register `CHEP%sR` reader
pub type R = crate::R<CHEPRrs>;
///Register `CHEP%sR` writer
pub type W = crate::W<CHEPRrs>;
///Field `EA` reader - endpoint/channel address
pub type EA_R = crate::FieldReader;
///Field `EA` writer - endpoint/channel address
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Status bits, for transmission transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATTXR {
    ///0: All transmission requests addressed to this endpoint/channel are ignored.
    Disabled = 0,
    ///1: Device mode: the endpoint is stalled and all transmission requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    Stall = 1,
    ///2: Device mode: the endpoint is NAKed and all transmission requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the transmission request.
    Nak = 2,
    ///3: This endpoint/channel is enabled for transmission.
    Valid = 3,
}
impl From<STATTXR> for u8 {
    #[inline(always)]
    fn from(variant: STATTXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATTXR {
    type Ux = u8;
}
impl crate::IsEnum for STATTXR {}
///Field `STATTX` reader - Status bits, for transmission transfers
pub type STATTX_R = crate::FieldReader<STATTXR>;
impl STATTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STATTXR {
        match self.bits {
            0 => STATTXR::Disabled,
            1 => STATTXR::Stall,
            2 => STATTXR::Nak,
            3 => STATTXR::Valid,
            _ => unreachable!(),
        }
    }
    ///All transmission requests addressed to this endpoint/channel are ignored.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATTXR::Disabled
    }
    ///Device mode: the endpoint is stalled and all transmission requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STATTXR::Stall
    }
    ///Device mode: the endpoint is NAKed and all transmission requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the transmission request.
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STATTXR::Nak
    }
    ///This endpoint/channel is enabled for transmission.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STATTXR::Valid
    }
}
/**Status bits, for transmission transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATTXW {
    ///0: Do not change bits
    Keep = 0,
}
impl From<STATTXW> for u8 {
    #[inline(always)]
    fn from(variant: STATTXW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATTXW {
    type Ux = u8;
}
impl crate::IsEnum for STATTXW {}
///Field `STATTX` writer - Status bits, for transmission transfers
pub type STATTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STATTXW>;
impl<'a, REG> STATTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not change bits
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(STATTXW::Keep)
    }
}
/**Data toggle, for transmission transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOGTXW {
    ///1: Flip bit
    Toggle = 1,
}
impl From<DTOGTXW> for bool {
    #[inline(always)]
    fn from(variant: DTOGTXW) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOGTX` writer - Data toggle, for transmission transfers
pub type DTOGTX_W<'a, REG> = crate::BitWriter1T<'a, REG, DTOGTXW>;
impl<'a, REG> DTOGTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flip bit
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(DTOGTXW::Toggle)
    }
}
/**Valid USB transaction transmitted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTTXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<VTTXW> for bool {
    #[inline(always)]
    fn from(variant: VTTXW) -> Self {
        variant as u8 != 0
    }
}
///Field `VTTX` reader - Valid USB transaction transmitted
pub type VTTX_R = crate::BitReader<VTTXW>;
impl VTTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VTTXW> {
        match self.bits {
            false => Some(VTTXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == VTTXW::Clear
    }
}
///Field `VTTX` writer - Valid USB transaction transmitted
pub type VTTX_W<'a, REG> = crate::BitWriter0C<'a, REG, VTTXW>;
impl<'a, REG> VTTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VTTXW::Clear)
    }
}
///Field `EPKIND` reader - endpoint/channel kind
pub type EPKIND_R = crate::BitReader;
///Field `EPKIND` writer - endpoint/channel kind
pub type EPKIND_W<'a, REG> = crate::BitWriter<'a, REG>;
/**USB type of transaction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTYPE {
    ///0: Bulk endpoint
    Bulk = 0,
    ///1: Control endpoint
    Control = 1,
    ///2: Isochronous endpoint
    Iso = 2,
    ///3: Interrupt endpoint
    Interrupt = 3,
}
impl From<UTYPE> for u8 {
    #[inline(always)]
    fn from(variant: UTYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UTYPE {
    type Ux = u8;
}
impl crate::IsEnum for UTYPE {}
///Field `UTYPE` reader - USB type of transaction
pub type UTYPE_R = crate::FieldReader<UTYPE>;
impl UTYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UTYPE {
        match self.bits {
            0 => UTYPE::Bulk,
            1 => UTYPE::Control,
            2 => UTYPE::Iso,
            3 => UTYPE::Interrupt,
            _ => unreachable!(),
        }
    }
    ///Bulk endpoint
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == UTYPE::Bulk
    }
    ///Control endpoint
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == UTYPE::Control
    }
    ///Isochronous endpoint
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == UTYPE::Iso
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == UTYPE::Interrupt
    }
}
///Field `UTYPE` writer - USB type of transaction
pub type UTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UTYPE, crate::Safe>;
impl<'a, REG> UTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Bulk endpoint
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Bulk)
    }
    ///Control endpoint
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Control)
    }
    ///Isochronous endpoint
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Iso)
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Interrupt)
    }
}
///Field `SETUP` reader - Setup transaction completed
pub type SETUP_R = crate::BitReader;
/**Status bits, for reception transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATRXR {
    ///0: All reception requests addressed to this endpoint/channel are ignored.
    Disabled = 0,
    ///1: Device mode: the endpoint is stalled and all reception requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    Stall = 1,
    ///2: Device mode: the endpoint is NAKed and all reception requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the reception request.
    Nak = 2,
    ///3: This endpoint/channel is enabled for reception.
    Valid = 3,
}
impl From<STATRXR> for u8 {
    #[inline(always)]
    fn from(variant: STATRXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATRXR {
    type Ux = u8;
}
impl crate::IsEnum for STATRXR {}
///Field `STATRX` reader - Status bits, for reception transfers
pub type STATRX_R = crate::FieldReader<STATRXR>;
impl STATRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STATRXR {
        match self.bits {
            0 => STATRXR::Disabled,
            1 => STATRXR::Stall,
            2 => STATRXR::Nak,
            3 => STATRXR::Valid,
            _ => unreachable!(),
        }
    }
    ///All reception requests addressed to this endpoint/channel are ignored.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATRXR::Disabled
    }
    ///Device mode: the endpoint is stalled and all reception requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STATRXR::Stall
    }
    ///Device mode: the endpoint is NAKed and all reception requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the reception request.
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STATRXR::Nak
    }
    ///This endpoint/channel is enabled for reception.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STATRXR::Valid
    }
}
/**Status bits, for reception transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATRXW {
    ///0: Do not change bits
    Keep = 0,
}
impl From<STATRXW> for u8 {
    #[inline(always)]
    fn from(variant: STATRXW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATRXW {
    type Ux = u8;
}
impl crate::IsEnum for STATRXW {}
///Field `STATRX` writer - Status bits, for reception transfers
pub type STATRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STATRXW>;
impl<'a, REG> STATRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not change bits
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(STATRXW::Keep)
    }
}
/**Data Toggle, for reception transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOGRXW {
    ///1: Flip bit
    Toggle = 1,
}
impl From<DTOGRXW> for bool {
    #[inline(always)]
    fn from(variant: DTOGRXW) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOGRX` writer - Data Toggle, for reception transfers
pub type DTOGRX_W<'a, REG> = crate::BitWriter1T<'a, REG, DTOGRXW>;
impl<'a, REG> DTOGRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flip bit
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(DTOGRXW::Toggle)
    }
}
/**USB valid transaction received

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTRXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<VTRXW> for bool {
    #[inline(always)]
    fn from(variant: VTRXW) -> Self {
        variant as u8 != 0
    }
}
///Field `VTRX` reader - USB valid transaction received
pub type VTRX_R = crate::BitReader<VTRXW>;
impl VTRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VTRXW> {
        match self.bits {
            false => Some(VTRXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == VTRXW::Clear
    }
}
///Field `VTRX` writer - USB valid transaction received
pub type VTRX_W<'a, REG> = crate::BitWriter0C<'a, REG, VTRXW>;
impl<'a, REG> VTRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VTRXW::Clear)
    }
}
///Field `DEVADDR` reader - Host mode
pub type DEVADDR_R = crate::FieldReader;
///Field `DEVADDR` writer - Host mode
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Host mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKW {
    ///0: Clear flag
    Clear = 0,
}
impl From<NAKW> for bool {
    #[inline(always)]
    fn from(variant: NAKW) -> Self {
        variant as u8 != 0
    }
}
///Field `NAK` reader - Host mode
pub type NAK_R = crate::BitReader<NAKW>;
impl NAK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NAKW> {
        match self.bits {
            false => Some(NAKW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == NAKW::Clear
    }
}
///Field `NAK` writer - Host mode
pub type NAK_W<'a, REG> = crate::BitWriter0C<'a, REG, NAKW>;
impl<'a, REG> NAK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NAKW::Clear)
    }
}
///Field `LS_EP` reader - Low speed endpoint host with HUB only
pub type LS_EP_R = crate::BitReader;
///Field `LS_EP` writer - Low speed endpoint host with HUB only
pub type LS_EP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Received error for an OUT/SETUP transaction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_TXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERR_TXW> for bool {
    #[inline(always)]
    fn from(variant: ERR_TXW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_TX` reader - Received error for an OUT/SETUP transaction
pub type ERR_TX_R = crate::BitReader<ERR_TXW>;
impl ERR_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERR_TXW> {
        match self.bits {
            false => Some(ERR_TXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERR_TXW::Clear
    }
}
///Field `ERR_TX` writer - Received error for an OUT/SETUP transaction
pub type ERR_TX_W<'a, REG> = crate::BitWriter0C<'a, REG, ERR_TXW>;
impl<'a, REG> ERR_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_TXW::Clear)
    }
}
/**Received error for an IN transaction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_RXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERR_RXW> for bool {
    #[inline(always)]
    fn from(variant: ERR_RXW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_RX` reader - Received error for an IN transaction
pub type ERR_RX_R = crate::BitReader<ERR_RXW>;
impl ERR_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERR_RXW> {
        match self.bits {
            false => Some(ERR_RXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERR_RXW::Clear
    }
}
///Field `ERR_RX` writer - Received error for an IN transaction
pub type ERR_RX_W<'a, REG> = crate::BitWriter0C<'a, REG, ERR_RXW>;
impl<'a, REG> ERR_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_RXW::Clear)
    }
}
///Field `THREE_ERR_TX` reader - Three errors for an OUT or SETUP transaction
pub type THREE_ERR_TX_R = crate::FieldReader;
///Field `THREE_ERR_TX` writer - Three errors for an OUT or SETUP transaction
pub type THREE_ERR_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `THREE_ERR_RX` reader - Three errors for an IN transaction
pub type THREE_ERR_RX_R = crate::FieldReader;
///Field `THREE_ERR_RX` writer - Three errors for an IN transaction
pub type THREE_ERR_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - endpoint/channel address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stattx(&self) -> STATTX_R {
        STATTX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Valid USB transaction transmitted
    #[inline(always)]
    pub fn vttx(&self) -> VTTX_R {
        VTTX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - endpoint/channel kind
    #[inline(always)]
    pub fn epkind(&self) -> EPKIND_R {
        EPKIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - USB type of transaction
    #[inline(always)]
    pub fn utype(&self) -> UTYPE_R {
        UTYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn statrx(&self) -> STATRX_R {
        STATRX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - USB valid transaction received
    #[inline(always)]
    pub fn vtrx(&self) -> VTRX_R {
        VTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - Host mode
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Host mode
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Low speed endpoint host with HUB only
    #[inline(always)]
    pub fn ls_ep(&self) -> LS_EP_R {
        LS_EP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Received error for an OUT/SETUP transaction
    #[inline(always)]
    pub fn err_tx(&self) -> ERR_TX_R {
        ERR_TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Received error for an IN transaction
    #[inline(always)]
    pub fn err_rx(&self) -> ERR_RX_R {
        ERR_RX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - Three errors for an OUT or SETUP transaction
    #[inline(always)]
    pub fn three_err_tx(&self) -> THREE_ERR_TX_R {
        THREE_ERR_TX_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bits 29:30 - Three errors for an IN transaction
    #[inline(always)]
    pub fn three_err_rx(&self) -> THREE_ERR_RX_R {
        THREE_ERR_RX_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEPR")
            .field("ea", &self.ea())
            .field("stattx", &self.stattx())
            .field("vttx", &self.vttx())
            .field("epkind", &self.epkind())
            .field("utype", &self.utype())
            .field("setup", &self.setup())
            .field("statrx", &self.statrx())
            .field("vtrx", &self.vtrx())
            .field("devaddr", &self.devaddr())
            .field("nak", &self.nak())
            .field("ls_ep", &self.ls_ep())
            .field("err_tx", &self.err_tx())
            .field("err_rx", &self.err_rx())
            .field("three_err_tx", &self.three_err_tx())
            .field("three_err_rx", &self.three_err_rx())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - endpoint/channel address
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<'_, CHEPRrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stattx(&mut self) -> STATTX_W<'_, CHEPRrs> {
        STATTX_W::new(self, 4)
    }
    ///Bit 6 - Data toggle, for transmission transfers
    #[inline(always)]
    pub fn dtogtx(&mut self) -> DTOGTX_W<'_, CHEPRrs> {
        DTOGTX_W::new(self, 6)
    }
    ///Bit 7 - Valid USB transaction transmitted
    #[inline(always)]
    pub fn vttx(&mut self) -> VTTX_W<'_, CHEPRrs> {
        VTTX_W::new(self, 7)
    }
    ///Bit 8 - endpoint/channel kind
    #[inline(always)]
    pub fn epkind(&mut self) -> EPKIND_W<'_, CHEPRrs> {
        EPKIND_W::new(self, 8)
    }
    ///Bits 9:10 - USB type of transaction
    #[inline(always)]
    pub fn utype(&mut self) -> UTYPE_W<'_, CHEPRrs> {
        UTYPE_W::new(self, 9)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn statrx(&mut self) -> STATRX_W<'_, CHEPRrs> {
        STATRX_W::new(self, 12)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtogrx(&mut self) -> DTOGRX_W<'_, CHEPRrs> {
        DTOGRX_W::new(self, 14)
    }
    ///Bit 15 - USB valid transaction received
    #[inline(always)]
    pub fn vtrx(&mut self) -> VTRX_W<'_, CHEPRrs> {
        VTRX_W::new(self, 15)
    }
    ///Bits 16:22 - Host mode
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<'_, CHEPRrs> {
        DEVADDR_W::new(self, 16)
    }
    ///Bit 23 - Host mode
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, CHEPRrs> {
        NAK_W::new(self, 23)
    }
    ///Bit 24 - Low speed endpoint host with HUB only
    #[inline(always)]
    pub fn ls_ep(&mut self) -> LS_EP_W<'_, CHEPRrs> {
        LS_EP_W::new(self, 24)
    }
    ///Bit 25 - Received error for an OUT/SETUP transaction
    #[inline(always)]
    pub fn err_tx(&mut self) -> ERR_TX_W<'_, CHEPRrs> {
        ERR_TX_W::new(self, 25)
    }
    ///Bit 26 - Received error for an IN transaction
    #[inline(always)]
    pub fn err_rx(&mut self) -> ERR_RX_W<'_, CHEPRrs> {
        ERR_RX_W::new(self, 26)
    }
    ///Bits 27:28 - Three errors for an OUT or SETUP transaction
    #[inline(always)]
    pub fn three_err_tx(&mut self) -> THREE_ERR_TX_W<'_, CHEPRrs> {
        THREE_ERR_TX_W::new(self, 27)
    }
    ///Bits 29:30 - Three errors for an IN transaction
    #[inline(always)]
    pub fn three_err_rx(&mut self) -> THREE_ERR_RX_W<'_, CHEPRrs> {
        THREE_ERR_RX_W::new(self, 29)
    }
}
/**USB endpoint/channel %s register

You can [`read`](crate::Reg::read) this register and get [`chepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USB:CHEP[0]R)*/
pub struct CHEPRrs;
impl crate::RegisterSpec for CHEPRrs {
    type Ux = u32;
}
///`read()` method returns [`chepr::R`](R) reader structure
impl crate::Readable for CHEPRrs {}
///`write(|w| ..)` method takes [`chepr::W`](W) writer structure
impl crate::Writable for CHEPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0680_8080;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7070;
}
///`reset()` method sets CHEP%sR to value 0
impl crate::Resettable for CHEPRrs {}
