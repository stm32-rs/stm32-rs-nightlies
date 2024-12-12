///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    ///0: Error interrupt disabled
    Disabled = 0,
    ///1: Error interrupt enabled
    Enabled = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::Disabled,
            true => EIE::Enabled,
        }
    }
    ///Error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE::Disabled
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE::Enabled
    }
}
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Disabled)
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Enabled)
    }
}
/**IrDA mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    ///0: IrDA disabled
    Disabled = 0,
    ///1: IrDA enabled
    Enabled = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
///Field `IREN` reader - IrDA mode enable
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::Disabled,
            true => IREN::Enabled,
        }
    }
    ///IrDA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN::Disabled
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN::Enabled
    }
}
///Field `IREN` writer - IrDA mode enable
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Disabled)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Enabled)
    }
}
/**IrDA low-power

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP {
    ///0: Normal mode
    Normal = 0,
    ///1: Low-power mode
    LowPower = 1,
}
impl From<IRLP> for bool {
    #[inline(always)]
    fn from(variant: IRLP) -> Self {
        variant as u8 != 0
    }
}
///Field `IRLP` reader - IrDA low-power
pub type IRLP_R = crate::BitReader<IRLP>;
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRLP {
        match self.bits {
            false => IRLP::Normal,
            true => IRLP::LowPower,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP::Normal
    }
    ///Low-power mode
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP::LowPower
    }
}
///Field `IRLP` writer - IrDA low-power
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG, IRLP>;
impl<'a, REG> IRLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::Normal)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::LowPower)
    }
}
/**Half-duplex selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    ///0: Half duplex mode is not selected
    FullDuplex = 0,
    ///1: Half duplex mode is selected
    HalfDuplex = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::FullDuplex,
            true => HDSEL::HalfDuplex,
        }
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == HDSEL::FullDuplex
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == HDSEL::HalfDuplex
    }
}
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::FullDuplex)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::HalfDuplex)
    }
}
/**DMA enable receiver

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    ///0: DMA mode is disabled for reception
    Disabled = 0,
    ///1: DMA mode is enabled for reception
    Enabled = 1,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            false => DMAR::Disabled,
            true => DMAR::Enabled,
        }
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::Disabled
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::Enabled
    }
}
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Disabled)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Enabled)
    }
}
/**DMA enable transmitter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    ///0: DMA mode is disabled for transmission
    Disabled = 0,
    ///1: DMA mode is enabled for transmission
    Enabled = 1,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            false => DMAT::Disabled,
            true => DMAT::Enabled,
        }
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT::Disabled
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT::Enabled
    }
}
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Disabled)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Enabled)
    }
}
/**One sample bit method enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT {
    ///0: Three sample bit method
    Sample3 = 0,
    ///1: One sample bit method
    Sample1 = 1,
}
impl From<ONEBIT> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT) -> Self {
        variant as u8 != 0
    }
}
///Field `ONEBIT` reader - One sample bit method enable
pub type ONEBIT_R = crate::BitReader<ONEBIT>;
impl ONEBIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ONEBIT {
        match self.bits {
            false => ONEBIT::Sample3,
            true => ONEBIT::Sample1,
        }
    }
    ///Three sample bit method
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT::Sample3
    }
    ///One sample bit method
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT::Sample1
    }
}
///Field `ONEBIT` writer - One sample bit method enable
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG, ONEBIT>;
impl<'a, REG> ONEBIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Three sample bit method
    #[inline(always)]
    pub fn sample3(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample3)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn sample1(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample1)
    }
}
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("onebit", &self.onebit())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("hdsel", &self.hdsel())
            .field("irlp", &self.irlp())
            .field("iren", &self.iren())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#UART4:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
