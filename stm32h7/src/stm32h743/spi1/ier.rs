#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "RXP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPIE {
    #[doc = "0: RX data available interrupt masked"]
    Masked = 0,
    #[doc = "1: RX data available interrupt not masked"]
    NotMasked = 1,
}
impl From<RXPIE> for bool {
    #[inline(always)]
    fn from(variant: RXPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPIE` reader - RXP Interrupt Enable"]
pub type RXPIE_R = crate::BitReader<RXPIE>;
impl RXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXPIE {
        match self.bits {
            false => RXPIE::Masked,
            true => RXPIE::NotMasked,
        }
    }
    #[doc = "RX data available interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXPIE::Masked
    }
    #[doc = "RX data available interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXPIE::NotMasked
    }
}
#[doc = "Field `RXPIE` writer - RXP Interrupt Enable"]
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG, RXPIE>;
impl<'a, REG> RXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX data available interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::Masked)
    }
    #[doc = "RX data available interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::NotMasked)
    }
}
#[doc = "TXP interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPIE {
    #[doc = "0: TX space available interrupt masked"]
    Masked = 0,
    #[doc = "1: TX space available interrupt not masked"]
    NotMasked = 1,
}
impl From<TXPIE> for bool {
    #[inline(always)]
    fn from(variant: TXPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPIE` reader - TXP interrupt enable"]
pub type TXPIE_R = crate::BitReader<TXPIE>;
impl TXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXPIE {
        match self.bits {
            false => TXPIE::Masked,
            true => TXPIE::NotMasked,
        }
    }
    #[doc = "TX space available interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXPIE::Masked
    }
    #[doc = "TX space available interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXPIE::NotMasked
    }
}
#[doc = "Field `TXPIE` writer - TXP interrupt enable"]
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG, TXPIE>;
impl<'a, REG> TXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX space available interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE::Masked)
    }
    #[doc = "TX space available interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE::NotMasked)
    }
}
#[doc = "DXP interrupt enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXPIE {
    #[doc = "0: Duplex transfer complete interrupt masked"]
    Masked = 0,
    #[doc = "1: Duplex transfer complete interrupt not masked"]
    NotMasked = 1,
}
impl From<DXPIE> for bool {
    #[inline(always)]
    fn from(variant: DXPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXPIE` reader - DXP interrupt enabled"]
pub type DXPIE_R = crate::BitReader<DXPIE>;
impl DXPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DXPIE {
        match self.bits {
            false => DXPIE::Masked,
            true => DXPIE::NotMasked,
        }
    }
    #[doc = "Duplex transfer complete interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DXPIE::Masked
    }
    #[doc = "Duplex transfer complete interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == DXPIE::NotMasked
    }
}
#[doc = "Field `DXPIE` writer - DXP interrupt enabled"]
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG, DXPIE>;
impl<'a, REG> DXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Duplex transfer complete interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE::Masked)
    }
    #[doc = "Duplex transfer complete interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE::NotMasked)
    }
}
#[doc = "EOT, SUSP and TXC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTIE {
    #[doc = "0: End-of-transfer interrupt masked"]
    Masked = 0,
    #[doc = "1: End-of-transfer interrupt not masked"]
    NotMasked = 1,
}
impl From<EOTIE> for bool {
    #[inline(always)]
    fn from(variant: EOTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_R = crate::BitReader<EOTIE>;
impl EOTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOTIE {
        match self.bits {
            false => EOTIE::Masked,
            true => EOTIE::NotMasked,
        }
    }
    #[doc = "End-of-transfer interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EOTIE::Masked
    }
    #[doc = "End-of-transfer interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == EOTIE::NotMasked
    }
}
#[doc = "Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG, EOTIE>;
impl<'a, REG> EOTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End-of-transfer interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE::Masked)
    }
    #[doc = "End-of-transfer interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE::NotMasked)
    }
}
#[doc = "TXTFIE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTFIE {
    #[doc = "0: Transmission transfer filled interrupt masked"]
    Masked = 0,
    #[doc = "1: Transmission transfer filled interrupt not masked"]
    NotMasked = 1,
}
impl From<TXTFIE> for bool {
    #[inline(always)]
    fn from(variant: TXTFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTFIE` reader - TXTFIE interrupt enable"]
pub type TXTFIE_R = crate::BitReader<TXTFIE>;
impl TXTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXTFIE {
        match self.bits {
            false => TXTFIE::Masked,
            true => TXTFIE::NotMasked,
        }
    }
    #[doc = "Transmission transfer filled interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXTFIE::Masked
    }
    #[doc = "Transmission transfer filled interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXTFIE::NotMasked
    }
}
#[doc = "Field `TXTFIE` writer - TXTFIE interrupt enable"]
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG, TXTFIE>;
impl<'a, REG> TXTFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission transfer filled interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE::Masked)
    }
    #[doc = "Transmission transfer filled interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE::NotMasked)
    }
}
#[doc = "UDR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE {
    #[doc = "0: Underrun interrupt masked"]
    Masked = 0,
    #[doc = "1: Underrun interrupt not masked"]
    NotMasked = 1,
}
impl From<UDRIE> for bool {
    #[inline(always)]
    fn from(variant: UDRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDRIE` reader - UDR interrupt enable"]
pub type UDRIE_R = crate::BitReader<UDRIE>;
impl UDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDRIE {
        match self.bits {
            false => UDRIE::Masked,
            true => UDRIE::NotMasked,
        }
    }
    #[doc = "Underrun interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == UDRIE::Masked
    }
    #[doc = "Underrun interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == UDRIE::NotMasked
    }
}
#[doc = "Field `UDRIE` writer - UDR interrupt enable"]
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG, UDRIE>;
impl<'a, REG> UDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Underrun interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE::Masked)
    }
    #[doc = "Underrun interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE::NotMasked)
    }
}
#[doc = "OVR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    #[doc = "0: Overrun interrupt masked"]
    Masked = 0,
    #[doc = "1: Overrun interrupt not masked"]
    NotMasked = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRIE` reader - OVR interrupt enable"]
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Masked,
            true => OVRIE::NotMasked,
        }
    }
    #[doc = "Overrun interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == OVRIE::Masked
    }
    #[doc = "Overrun interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == OVRIE::NotMasked
    }
}
#[doc = "Field `OVRIE` writer - OVR interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Masked)
    }
    #[doc = "Overrun interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::NotMasked)
    }
}
#[doc = "CRC Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEIE {
    #[doc = "0: CRC error interrupt masked"]
    Masked = 0,
    #[doc = "1: CRC error interrupt not masked"]
    NotMasked = 1,
}
impl From<CRCEIE> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEIE` reader - CRC Interrupt enable"]
pub type CRCEIE_R = crate::BitReader<CRCEIE>;
impl CRCEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEIE {
        match self.bits {
            false => CRCEIE::Masked,
            true => CRCEIE::NotMasked,
        }
    }
    #[doc = "CRC error interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CRCEIE::Masked
    }
    #[doc = "CRC error interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == CRCEIE::NotMasked
    }
}
#[doc = "Field `CRCEIE` writer - CRC Interrupt enable"]
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG, CRCEIE>;
impl<'a, REG> CRCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::Masked)
    }
    #[doc = "CRC error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::NotMasked)
    }
}
#[doc = "TIFRE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFREIE {
    #[doc = "0: TI frame format error interrupt masked"]
    Masked = 0,
    #[doc = "1: TI frame format error interrupt not masked"]
    NotMasked = 1,
}
impl From<TIFREIE> for bool {
    #[inline(always)]
    fn from(variant: TIFREIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIFREIE` reader - TIFRE interrupt enable"]
pub type TIFREIE_R = crate::BitReader<TIFREIE>;
impl TIFREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFREIE {
        match self.bits {
            false => TIFREIE::Masked,
            true => TIFREIE::NotMasked,
        }
    }
    #[doc = "TI frame format error interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TIFREIE::Masked
    }
    #[doc = "TI frame format error interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TIFREIE::NotMasked
    }
}
#[doc = "Field `TIFREIE` writer - TIFRE interrupt enable"]
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG, TIFREIE>;
impl<'a, REG> TIFREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TI frame format error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE::Masked)
    }
    #[doc = "TI frame format error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE::NotMasked)
    }
}
#[doc = "Mode Fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFIE {
    #[doc = "0: Mode fault interrupt masked"]
    Masked = 0,
    #[doc = "1: Mode fault interrupt not masked"]
    NotMasked = 1,
}
impl From<MODFIE> for bool {
    #[inline(always)]
    fn from(variant: MODFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODFIE` reader - Mode Fault interrupt enable"]
pub type MODFIE_R = crate::BitReader<MODFIE>;
impl MODFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODFIE {
        match self.bits {
            false => MODFIE::Masked,
            true => MODFIE::NotMasked,
        }
    }
    #[doc = "Mode fault interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MODFIE::Masked
    }
    #[doc = "Mode fault interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == MODFIE::NotMasked
    }
}
#[doc = "Field `MODFIE` writer - Mode Fault interrupt enable"]
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG, MODFIE>;
impl<'a, REG> MODFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode fault interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE::Masked)
    }
    #[doc = "Mode fault interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE::NotMasked)
    }
}
#[doc = "Additional number of transactions reload interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSERFIE {
    #[doc = "0: TSER loaded interrupt masked"]
    Masked = 0,
    #[doc = "1: TSER loaded interrupt not masked"]
    NotMasked = 1,
}
impl From<TSERFIE> for bool {
    #[inline(always)]
    fn from(variant: TSERFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSERFIE` reader - Additional number of transactions reload interrupt enable"]
pub type TSERFIE_R = crate::BitReader<TSERFIE>;
impl TSERFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSERFIE {
        match self.bits {
            false => TSERFIE::Masked,
            true => TSERFIE::NotMasked,
        }
    }
    #[doc = "TSER loaded interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSERFIE::Masked
    }
    #[doc = "TSER loaded interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TSERFIE::NotMasked
    }
}
#[doc = "Field `TSERFIE` writer - Additional number of transactions reload interrupt enable"]
pub type TSERFIE_W<'a, REG> = crate::BitWriter<'a, REG, TSERFIE>;
impl<'a, REG> TSERFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSER loaded interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TSERFIE::Masked)
    }
    #[doc = "TSER loaded interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TSERFIE::NotMasked)
    }
}
impl R {
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<IERrs> {
        RXPIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<IERrs> {
        TXPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DXP interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<IERrs> {
        DXPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<IERrs> {
        EOTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<IERrs> {
        TXTFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<IERrs> {
        UDRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<IERrs> {
        CRCEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<IERrs> {
        TIFREIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<IERrs> {
        MODFIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tserfie(&mut self) -> TSERFIE_W<IERrs> {
        TSERFIE_W::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
