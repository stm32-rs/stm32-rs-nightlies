///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**lin break detection length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL {
    ///0: 10-bit break detection
    Lbdl10 = 0,
    ///1: 11-bit break detection
    Lbdl11 = 1,
}
impl From<LBDL> for bool {
    #[inline(always)]
    fn from(variant: LBDL) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDL` reader - lin break detection length
pub type LBDL_R = crate::BitReader<LBDL>;
impl LBDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDL {
        match self.bits {
            false => LBDL::Lbdl10,
            true => LBDL::Lbdl11,
        }
    }
    ///10-bit break detection
    #[inline(always)]
    pub fn is_lbdl10(&self) -> bool {
        *self == LBDL::Lbdl10
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn is_lbdl11(&self) -> bool {
        *self == LBDL::Lbdl11
    }
}
///Field `LBDL` writer - lin break detection length
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG, LBDL>;
impl<'a, REG> LBDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///10-bit break detection
    #[inline(always)]
    pub fn lbdl10(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Lbdl10)
    }
    ///11-bit break detection
    #[inline(always)]
    pub fn lbdl11(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Lbdl11)
    }
}
/**LIN break detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE {
    ///0: LIN break detection interrupt disabled
    Disabled = 0,
    ///1: LIN break detection interrupt enabled
    Enabled = 1,
}
impl From<LBDIE> for bool {
    #[inline(always)]
    fn from(variant: LBDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDIE` reader - LIN break detection interrupt enable
pub type LBDIE_R = crate::BitReader<LBDIE>;
impl LBDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDIE {
        match self.bits {
            false => LBDIE::Disabled,
            true => LBDIE::Enabled,
        }
    }
    ///LIN break detection interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE::Disabled
    }
    ///LIN break detection interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE::Enabled
    }
}
///Field `LBDIE` writer - LIN break detection interrupt enable
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG, LBDIE>;
impl<'a, REG> LBDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LIN break detection interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Disabled)
    }
    ///LIN break detection interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Enabled)
    }
}
/**Last bit clock pulse

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCL {
    ///0: The clock pulse of the last data bit is not output to the CK pin
    Disabled = 0,
    ///1: The clock pulse of the last data bit is output to the CK pin
    Enabled = 1,
}
impl From<LBCL> for bool {
    #[inline(always)]
    fn from(variant: LBCL) -> Self {
        variant as u8 != 0
    }
}
///Field `LBCL` reader - Last bit clock pulse
pub type LBCL_R = crate::BitReader<LBCL>;
impl LBCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBCL {
        match self.bits {
            false => LBCL::Disabled,
            true => LBCL::Enabled,
        }
    }
    ///The clock pulse of the last data bit is not output to the CK pin
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBCL::Disabled
    }
    ///The clock pulse of the last data bit is output to the CK pin
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBCL::Enabled
    }
}
///Field `LBCL` writer - Last bit clock pulse
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG, LBCL>;
impl<'a, REG> LBCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The clock pulse of the last data bit is not output to the CK pin
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::Disabled)
    }
    ///The clock pulse of the last data bit is output to the CK pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::Enabled)
    }
}
/**Clock phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    ///0: The first clock transition is the first data capture edge
    First = 0,
    ///1: The second clock transition is the first data capture edge
    Second = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::First,
            true => CPHA::Second,
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA::First
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA::Second
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::First)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::Second)
    }
}
/**Clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    ///0: Steady low value on CK pin outside transmission window
    Low = 0,
    ///1: Steady high value on CK pin outside transmission window
    High = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::Low,
            true => CPOL::High,
        }
    }
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL::Low
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL::High
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Steady low value on CK pin outside transmission window
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::Low)
    }
    ///Steady high value on CK pin outside transmission window
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::High)
    }
}
/**Clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    ///0: CK pin disabled
    Disabled = 0,
    ///1: CK pin enabled
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - Clock enable
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    ///CK pin disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
///Field `CLKEN` writer - Clock enable
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
/**STOP bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    ///0: 1 stop bit
    Stop1 = 0,
    ///1: 0.5 stop bits
    Stop0p5 = 1,
    ///2: 2 stop bits
    Stop2 = 2,
    ///3: 1.5 stop bits
    Stop1p5 = 3,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
impl crate::IsEnum for STOP {}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            0 => STOP::Stop1,
            1 => STOP::Stop0p5,
            2 => STOP::Stop2,
            3 => STOP::Stop1p5,
            _ => unreachable!(),
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    ///0.5 stop bits
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP::Stop0p5
    }
    ///2 stop bits
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
    ///1.5 stop bits
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP::Stop1p5
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP, crate::Safe>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    ///0.5 stop bits
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop0p5)
    }
    ///2 stop bits
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
    ///1.5 stop bits
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1p5)
    }
}
/**LIN mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN {
    ///0: LIN mode disabled
    Disabled = 0,
    ///1: LIN mode enabled
    Enabled = 1,
}
impl From<LINEN> for bool {
    #[inline(always)]
    fn from(variant: LINEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LINEN` reader - LIN mode enable
pub type LINEN_R = crate::BitReader<LINEN>;
impl LINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LINEN {
        match self.bits {
            false => LINEN::Disabled,
            true => LINEN::Enabled,
        }
    }
    ///LIN mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN::Disabled
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN::Enabled
    }
}
///Field `LINEN` writer - LIN mode enable
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINEN>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LIN mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Disabled)
    }
    ///LIN mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Enabled)
    }
}
impl R {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("linen", &self.linen())
            .field("stop", &self.stop())
            .field("clken", &self.clken())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .field("lbcl", &self.lbcl())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 0)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<'_, CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<'_, CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W<'_, CR2rs> {
        LBCL_W::new(self, 8)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CR2rs> {
        CPHA_W::new(self, 9)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CR2rs> {
        CPOL_W::new(self, 10)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CR2rs> {
        CLKEN_W::new(self, 11)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CR2rs> {
        LINEN_W::new(self, 14)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#USART1:CR2)*/
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
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
