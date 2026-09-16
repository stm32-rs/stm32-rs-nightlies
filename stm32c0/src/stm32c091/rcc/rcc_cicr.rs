///Register `RCC_CICR` writer
pub type W = crate::W<RCC_CICRrs>;
/**LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear LSIRDYF flag
    B0x1 = 1,
}
impl From<LSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::B0x0)
    }
    ///Clear LSIRDYF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::B0x1)
    }
}
/**LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear LSERDYF flag
    B0x1 = 1,
}
impl From<LSERDYC> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYC>;
impl<'a, REG> LSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC::B0x0)
    }
    ///Clear LSERDYF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC::B0x1)
    }
}
/**HSIUSB48 ready interrupt clear This bit is set software to clear the HSIUSB48RDYF flag. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIUSB48RDYC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear HSIUSB48RDYF flag
    B0x1 = 1,
}
impl From<HSIUSB48RDYC> for bool {
    #[inline(always)]
    fn from(variant: HSIUSB48RDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIUSB48RDYC` writer - HSIUSB48 ready interrupt clear This bit is set software to clear the HSIUSB48RDYF flag. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIUSB48RDYC>;
impl<'a, REG> HSIUSB48RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDYC::B0x0)
    }
    ///Clear HSIUSB48RDYF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDYC::B0x1)
    }
}
/**HSI48 ready interrupt clear This bit is set software to clear the HSIRDYF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear HSIRDYF flag
    B0x1 = 1,
}
impl From<HSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYC` writer - HSI48 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYC>;
impl<'a, REG> HSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC::B0x0)
    }
    ///Clear HSIRDYF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC::B0x1)
    }
}
/**HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear HSERDYF flag
    B0x1 = 1,
}
impl From<HSERDYC> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYC>;
impl<'a, REG> HSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC::B0x0)
    }
    ///Clear HSERDYF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC::B0x1)
    }
}
/**Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear CSSF flag
    B0x1 = 1,
}
impl From<CSSC> for bool {
    #[inline(always)]
    fn from(variant: CSSC) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG, CSSC>;
impl<'a, REG> CSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSC::B0x0)
    }
    ///Clear CSSF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSC::B0x1)
    }
}
/**LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear LSECSSF flag
    B0x1 = 1,
}
impl From<LSECSSC> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSC>;
impl<'a, REG> LSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSC::B0x0)
    }
    ///Clear LSECSSF flag
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSC::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RCC_CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, RCC_CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, RCC_CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - HSIUSB48 ready interrupt clear This bit is set software to clear the HSIUSB48RDYF flag. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdyc(&mut self) -> HSIUSB48RDYC_W<'_, RCC_CICRrs> {
        HSIUSB48RDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI48 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, RCC_CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, RCC_CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 8 - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, RCC_CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<'_, RCC_CICRrs> {
        LSECSSC_W::new(self, 9)
    }
}
/**RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_CICR)*/
pub struct RCC_CICRrs;
impl crate::RegisterSpec for RCC_CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rcc_cicr::W`](W) writer structure
impl crate::Writable for RCC_CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CICR to value 0
impl crate::Resettable for RCC_CICRrs {}
