///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**RXP interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPIE {
    ///0: RX data available interrupt masked
    Masked = 0,
    ///1: RX data available interrupt not masked
    NotMasked = 1,
}
impl From<RXPIE> for bool {
    #[inline(always)]
    fn from(variant: RXPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXPIE` reader - RXP interrupt enable
pub type RXPIE_R = crate::BitReader<RXPIE>;
impl RXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXPIE {
        match self.bits {
            false => RXPIE::Masked,
            true => RXPIE::NotMasked,
        }
    }
    ///RX data available interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXPIE::Masked
    }
    ///RX data available interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXPIE::NotMasked
    }
}
///Field `RXPIE` writer - RXP interrupt enable
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG, RXPIE>;
impl<'a, REG> RXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RX data available interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::Masked)
    }
    ///RX data available interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXPIE::NotMasked)
    }
}
/**TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPIE {
    ///0: TX space available interrupt masked
    Masked = 0,
    ///1: TX space available interrupt not masked
    NotMasked = 1,
}
impl From<TXPIE> for bool {
    #[inline(always)]
    fn from(variant: TXPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXPIE` reader - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
pub type TXPIE_R = crate::BitReader<TXPIE>;
impl TXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXPIE {
        match self.bits {
            false => TXPIE::Masked,
            true => TXPIE::NotMasked,
        }
    }
    ///TX space available interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXPIE::Masked
    }
    ///TX space available interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXPIE::NotMasked
    }
}
///Field `TXPIE` writer - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG, TXPIE>;
impl<'a, REG> TXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX space available interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE::Masked)
    }
    ///TX space available interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXPIE::NotMasked)
    }
}
/**DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXPIE {
    ///0: Duplex transfer complete interrupt masked
    Masked = 0,
    ///1: Duplex transfer complete interrupt not masked
    NotMasked = 1,
}
impl From<DXPIE> for bool {
    #[inline(always)]
    fn from(variant: DXPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `DXPIE` reader - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
pub type DXPIE_R = crate::BitReader<DXPIE>;
impl DXPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DXPIE {
        match self.bits {
            false => DXPIE::Masked,
            true => DXPIE::NotMasked,
        }
    }
    ///Duplex transfer complete interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DXPIE::Masked
    }
    ///Duplex transfer complete interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == DXPIE::NotMasked
    }
}
///Field `DXPIE` writer - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG, DXPIE>;
impl<'a, REG> DXPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Duplex transfer complete interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE::Masked)
    }
    ///Duplex transfer complete interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(DXPIE::NotMasked)
    }
}
/**EOT, SUSP and TXC interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTIE {
    ///0: End-of-transfer interrupt masked
    Masked = 0,
    ///1: End-of-transfer interrupt not masked
    NotMasked = 1,
}
impl From<EOTIE> for bool {
    #[inline(always)]
    fn from(variant: EOTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable
pub type EOTIE_R = crate::BitReader<EOTIE>;
impl EOTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOTIE {
        match self.bits {
            false => EOTIE::Masked,
            true => EOTIE::NotMasked,
        }
    }
    ///End-of-transfer interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EOTIE::Masked
    }
    ///End-of-transfer interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == EOTIE::NotMasked
    }
}
///Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG, EOTIE>;
impl<'a, REG> EOTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End-of-transfer interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE::Masked)
    }
    ///End-of-transfer interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(EOTIE::NotMasked)
    }
}
/**TXTFIE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTFIE {
    ///0: Transmission transfer filled interrupt masked
    Masked = 0,
    ///1: Transmission transfer filled interrupt not masked
    NotMasked = 1,
}
impl From<TXTFIE> for bool {
    #[inline(always)]
    fn from(variant: TXTFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXTFIE` reader - TXTFIE interrupt enable
pub type TXTFIE_R = crate::BitReader<TXTFIE>;
impl TXTFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXTFIE {
        match self.bits {
            false => TXTFIE::Masked,
            true => TXTFIE::NotMasked,
        }
    }
    ///Transmission transfer filled interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXTFIE::Masked
    }
    ///Transmission transfer filled interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXTFIE::NotMasked
    }
}
///Field `TXTFIE` writer - TXTFIE interrupt enable
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG, TXTFIE>;
impl<'a, REG> TXTFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmission transfer filled interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE::Masked)
    }
    ///Transmission transfer filled interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXTFIE::NotMasked)
    }
}
/**UDR interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRIE {
    ///0: Underrun interrupt masked
    Masked = 0,
    ///1: Underrun interrupt not masked
    NotMasked = 1,
}
impl From<UDRIE> for bool {
    #[inline(always)]
    fn from(variant: UDRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UDRIE` reader - UDR interrupt enable
pub type UDRIE_R = crate::BitReader<UDRIE>;
impl UDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRIE {
        match self.bits {
            false => UDRIE::Masked,
            true => UDRIE::NotMasked,
        }
    }
    ///Underrun interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == UDRIE::Masked
    }
    ///Underrun interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == UDRIE::NotMasked
    }
}
///Field `UDRIE` writer - UDR interrupt enable
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG, UDRIE>;
impl<'a, REG> UDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underrun interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE::Masked)
    }
    ///Underrun interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(UDRIE::NotMasked)
    }
}
/**OVR interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt masked
    Masked = 0,
    ///1: Overrun interrupt not masked
    NotMasked = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - OVR interrupt enable
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::Masked,
            true => OVRIE::NotMasked,
        }
    }
    ///Overrun interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == OVRIE::Masked
    }
    ///Overrun interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == OVRIE::NotMasked
    }
}
///Field `OVRIE` writer - OVR interrupt enable
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::Masked)
    }
    ///Overrun interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::NotMasked)
    }
}
/**CRC error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEIE {
    ///0: CRC error interrupt masked
    Masked = 0,
    ///1: CRC error interrupt not masked
    NotMasked = 1,
}
impl From<CRCEIE> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEIE` reader - CRC error interrupt enable
pub type CRCEIE_R = crate::BitReader<CRCEIE>;
impl CRCEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEIE {
        match self.bits {
            false => CRCEIE::Masked,
            true => CRCEIE::NotMasked,
        }
    }
    ///CRC error interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CRCEIE::Masked
    }
    ///CRC error interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == CRCEIE::NotMasked
    }
}
///Field `CRCEIE` writer - CRC error interrupt enable
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG, CRCEIE>;
impl<'a, REG> CRCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::Masked)
    }
    ///CRC error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::NotMasked)
    }
}
/**TIFRE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFREIE {
    ///0: TI frame format error interrupt masked
    Masked = 0,
    ///1: TI frame format error interrupt not masked
    NotMasked = 1,
}
impl From<TIFREIE> for bool {
    #[inline(always)]
    fn from(variant: TIFREIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIFREIE` reader - TIFRE interrupt enable
pub type TIFREIE_R = crate::BitReader<TIFREIE>;
impl TIFREIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIFREIE {
        match self.bits {
            false => TIFREIE::Masked,
            true => TIFREIE::NotMasked,
        }
    }
    ///TI frame format error interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TIFREIE::Masked
    }
    ///TI frame format error interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TIFREIE::NotMasked
    }
}
///Field `TIFREIE` writer - TIFRE interrupt enable
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG, TIFREIE>;
impl<'a, REG> TIFREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TI frame format error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE::Masked)
    }
    ///TI frame format error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TIFREIE::NotMasked)
    }
}
/**mode Fault interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFIE {
    ///0: Mode fault interrupt masked
    Masked = 0,
    ///1: Mode fault interrupt not masked
    NotMasked = 1,
}
impl From<MODFIE> for bool {
    #[inline(always)]
    fn from(variant: MODFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `MODFIE` reader - mode Fault interrupt enable
pub type MODFIE_R = crate::BitReader<MODFIE>;
impl MODFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODFIE {
        match self.bits {
            false => MODFIE::Masked,
            true => MODFIE::NotMasked,
        }
    }
    ///Mode fault interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MODFIE::Masked
    }
    ///Mode fault interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == MODFIE::NotMasked
    }
}
///Field `MODFIE` writer - mode Fault interrupt enable
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG, MODFIE>;
impl<'a, REG> MODFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mode fault interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE::Masked)
    }
    ///Mode fault interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(MODFIE::NotMasked)
    }
}
impl R {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rxpie", &self.rxpie())
            .field("txpie", &self.txpie())
            .field("dxpie", &self.dxpie())
            .field("eotie", &self.eotie())
            .field("txtfie", &self.txtfie())
            .field("udrie", &self.udrie())
            .field("ovrie", &self.ovrie())
            .field("crceie", &self.crceie())
            .field("tifreie", &self.tifreie())
            .field("modfie", &self.modfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<IERrs> {
        RXPIE_W::new(self, 0)
    }
    ///Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<IERrs> {
        TXPIE_W::new(self, 1)
    }
    ///Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event.
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<IERrs> {
        DXPIE_W::new(self, 2)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<IERrs> {
        EOTIE_W::new(self, 3)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<IERrs> {
        TXTFIE_W::new(self, 4)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<IERrs> {
        UDRIE_W::new(self, 5)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<IERrs> {
        OVRIE_W::new(self, 6)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<IERrs> {
        CRCEIE_W::new(self, 7)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<IERrs> {
        TIFREIE_W::new(self, 8)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<IERrs> {
        MODFIE_W::new(self, 9)
    }
}
/**SPI/I2S interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SPI1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
