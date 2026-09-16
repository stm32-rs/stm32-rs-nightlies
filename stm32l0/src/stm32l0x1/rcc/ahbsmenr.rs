///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
/**DMA clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASMEN {
    ///0: DMA clock disabled in Sleep mode
    Disabled = 0,
    ///1: DMA clock enabled in Sleep mode
    Enabled = 1,
}
impl From<DMASMEN> for bool {
    #[inline(always)]
    fn from(variant: DMASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMASMEN` reader - DMA clock enable during sleep mode bit
pub type DMASMEN_R = crate::BitReader<DMASMEN>;
impl DMASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMASMEN {
        match self.bits {
            false => DMASMEN::Disabled,
            true => DMASMEN::Enabled,
        }
    }
    ///DMA clock disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMASMEN::Disabled
    }
    ///DMA clock enabled in Sleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMASMEN::Enabled
    }
}
///Field `DMASMEN` writer - DMA clock enable during sleep mode bit
pub type DMASMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMASMEN>;
impl<'a, REG> DMASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA clock disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Disabled)
    }
    ///DMA clock enabled in Sleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Enabled)
    }
}
/**NVM interface clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIFSMEN {
    ///0: NVM interface clock disabled in Sleep mode
    Disabled = 0,
    ///1: NVM interface clock enabled in Sleep mode
    Enabled = 1,
}
impl From<MIFSMEN> for bool {
    #[inline(always)]
    fn from(variant: MIFSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `MIFSMEN` reader - NVM interface clock enable during sleep mode bit
pub type MIFSMEN_R = crate::BitReader<MIFSMEN>;
impl MIFSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MIFSMEN {
        match self.bits {
            false => MIFSMEN::Disabled,
            true => MIFSMEN::Enabled,
        }
    }
    ///NVM interface clock disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MIFSMEN::Disabled
    }
    ///NVM interface clock enabled in Sleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MIFSMEN::Enabled
    }
}
///Field `MIFSMEN` writer - NVM interface clock enable during sleep mode bit
pub type MIFSMEN_W<'a, REG> = crate::BitWriter<'a, REG, MIFSMEN>;
impl<'a, REG> MIFSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NVM interface clock disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIFSMEN::Disabled)
    }
    ///NVM interface clock enabled in Sleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIFSMEN::Enabled)
    }
}
/**SRAM interface clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSMEN {
    ///0: NVM interface clock disabled in Sleep mode
    Disabled = 0,
    ///1: NVM interface clock enabled in Sleep mode
    Enabled = 1,
}
impl From<SRAMSMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAMSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMSMEN` reader - SRAM interface clock enable during sleep mode bit
pub type SRAMSMEN_R = crate::BitReader<SRAMSMEN>;
impl SRAMSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAMSMEN {
        match self.bits {
            false => SRAMSMEN::Disabled,
            true => SRAMSMEN::Enabled,
        }
    }
    ///NVM interface clock disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAMSMEN::Disabled
    }
    ///NVM interface clock enabled in Sleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAMSMEN::Enabled
    }
}
///Field `SRAMSMEN` writer - SRAM interface clock enable during sleep mode bit
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAMSMEN>;
impl<'a, REG> SRAMSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NVM interface clock disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::Disabled)
    }
    ///NVM interface clock enabled in Sleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::Enabled)
    }
}
/**CRC clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN {
    ///0: Test integration module clock disabled in Sleep mode
    Disabled = 0,
    ///1: Test integration module clock enabled in Sleep mode (if enabled by CRCEN)
    Enabled = 1,
}
impl From<CRCSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCSMEN` reader - CRC clock enable during sleep mode bit
pub type CRCSMEN_R = crate::BitReader<CRCSMEN>;
impl CRCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSMEN {
        match self.bits {
            false => CRCSMEN::Disabled,
            true => CRCSMEN::Enabled,
        }
    }
    ///Test integration module clock disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCSMEN::Disabled
    }
    ///Test integration module clock enabled in Sleep mode (if enabled by CRCEN)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCSMEN::Enabled
    }
}
///Field `CRCSMEN` writer - CRC clock enable during sleep mode bit
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSMEN>;
impl<'a, REG> CRCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Test integration module clock disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Disabled)
    }
    ///Test integration module clock enabled in Sleep mode (if enabled by CRCEN)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Enabled)
    }
}
/**Crypto clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRYPSMEN {
    ///0: Crypto clock disabled in Sleep mode
    Disabled = 0,
    ///1: Crypto clock enabled in Sleep mode
    Enabled = 1,
}
impl From<CRYPSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRYPSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRYPSMEN` reader - Crypto clock enable during sleep mode bit
pub type CRYPSMEN_R = crate::BitReader<CRYPSMEN>;
impl CRYPSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRYPSMEN {
        match self.bits {
            false => CRYPSMEN::Disabled,
            true => CRYPSMEN::Enabled,
        }
    }
    ///Crypto clock disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRYPSMEN::Disabled
    }
    ///Crypto clock enabled in Sleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRYPSMEN::Enabled
    }
}
///Field `CRYPSMEN` writer - Crypto clock enable during sleep mode bit
pub type CRYPSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRYPSMEN>;
impl<'a, REG> CRYPSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Crypto clock disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRYPSMEN::Disabled)
    }
    ///Crypto clock enabled in Sleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRYPSMEN::Enabled)
    }
}
impl R {
    ///Bit 0 - DMA clock enable during sleep mode bit
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - NVM interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn mifsmen(&self) -> MIFSMEN_R {
        MIFSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during sleep mode bit
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 24 - Crypto clock enable during sleep mode bit
    #[inline(always)]
    pub fn crypsmen(&self) -> CRYPSMEN_R {
        CRYPSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("crypsmen", &self.crypsmen())
            .field("crcsmen", &self.crcsmen())
            .field("sramsmen", &self.sramsmen())
            .field("mifsmen", &self.mifsmen())
            .field("dmasmen", &self.dmasmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA clock enable during sleep mode bit
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DMASMEN_W<'_, AHBSMENRrs> {
        DMASMEN_W::new(self, 0)
    }
    ///Bit 8 - NVM interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn mifsmen(&mut self) -> MIFSMEN_W<'_, AHBSMENRrs> {
        MIFSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<'_, AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during sleep mode bit
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 24 - Crypto clock enable during sleep mode bit
    #[inline(always)]
    pub fn crypsmen(&mut self) -> CRYPSMEN_W<'_, AHBSMENRrs> {
        CRYPSMEN_W::new(self, 24)
    }
}
/**AHB peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:AHBSMENR)*/
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsmenr::R`](R) reader structure
impl crate::Readable for AHBSMENRrs {}
///`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSMENR to value 0x0111_1301
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0111_1301;
}
