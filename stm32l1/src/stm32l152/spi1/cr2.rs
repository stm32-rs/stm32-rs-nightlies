///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Rx buffer DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: Rx buffer DMA disabled
    Disabled = 0,
    ///1: Rx buffer DMA enabled
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Rx buffer DMA enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
///Field `RXDMAEN` writer - Rx buffer DMA enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
/**Tx buffer DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: Tx buffer DMA disabled
    Disabled = 0,
    ///1: Tx buffer DMA enabled
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Tx buffer DMA enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
///Field `TXDMAEN` writer - Tx buffer DMA enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
/**SS output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    ///0: SS output is disabled in master mode
    Disabled = 0,
    ///1: SS output is enabled in master mode
    Enabled = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOE` reader - SS output enable
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::Disabled,
            true => SSOE::Enabled,
        }
    }
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE::Disabled
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE::Enabled
    }
}
///Field `SSOE` writer - SS output enable
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Disabled)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Enabled)
    }
}
/**Frame format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF {
    ///0: SPI Motorola mode
    Motorola = 0,
    ///1: SPI TI mode
    Ti = 1,
}
impl From<FRF> for bool {
    #[inline(always)]
    fn from(variant: FRF) -> Self {
        variant as u8 != 0
    }
}
///Field `FRF` reader - Frame format
pub type FRF_R = crate::BitReader<FRF>;
impl FRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRF {
        match self.bits {
            false => FRF::Motorola,
            true => FRF::Ti,
        }
    }
    ///SPI Motorola mode
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF::Motorola
    }
    ///SPI TI mode
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF::Ti
    }
}
///Field `FRF` writer - Frame format
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG, FRF>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI Motorola mode
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Motorola)
    }
    ///SPI TI mode
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Ti)
    }
}
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: Error interrupt masked
    Masked = 0,
    ///1: Error interrupt not masked
    NotMasked = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Masked,
            true => ERRIE::NotMasked,
        }
    }
    ///Error interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE::Masked
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE::NotMasked
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Masked)
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::NotMasked)
    }
}
/**RX buffer not empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    ///0: RXE interrupt masked
    Masked = 0,
    ///1: RXE interrupt not masked
    NotMasked = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RX buffer not empty interrupt enable
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Masked,
            true => RXNEIE::NotMasked,
        }
    }
    ///RXE interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE::Masked
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE::NotMasked
    }
}
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Masked)
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::NotMasked)
    }
}
/**Tx buffer empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    ///0: TXE interrupt masked
    Masked = 0,
    ///1: TXE interrupt not masked
    NotMasked = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - Tx buffer empty interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Masked,
            true => TXEIE::NotMasked,
        }
    }
    ///TXE interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE::Masked
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE::NotMasked
    }
}
///Field `TXEIE` writer - Tx buffer empty interrupt enable
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Masked)
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::NotMasked)
    }
}
impl R {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("txeie", &self.txeie())
            .field("rxneie", &self.rxneie())
            .field("errie", &self.errie())
            .field("frf", &self.frf())
            .field("ssoe", &self.ssoe())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<'_, CR2rs> {
        SSOE_W::new(self, 2)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<'_, CR2rs> {
        FRF_W::new(self, 4)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CR2rs> {
        ERRIE_W::new(self, 5)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR2rs> {
        TXEIE_W::new(self, 7)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#SPI1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0x0700
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u16 = 0x0700;
}
