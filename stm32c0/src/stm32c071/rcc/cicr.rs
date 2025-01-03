///Register `CICR` writer
pub type W = crate::W<CICRrs>;
///Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIUSB48RDYC` writer - HSIUSB48 ready interrupt clear This bit is set software to clear the HSIUSB48RDYF flag. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI48 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag.
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag.
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    ///Bit 2 - HSIUSB48 ready interrupt clear This bit is set software to clear the HSIUSB48RDYF flag. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdyc(&mut self) -> HSIUSB48RDYC_W<CICRrs> {
        HSIUSB48RDYC_W::new(self, 2)
    }
    ///Bit 3 - HSI48 ready interrupt clear This bit is set software to clear the HSIRDYF flag.
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag.
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    ///Bit 8 - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag.
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<CICRrs> {
        CSSC_W::new(self, 8)
    }
    ///Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag.
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICRrs> {
        LSECSSC_W::new(self, 9)
    }
}
/**RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RCC:CICR)*/
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cicr::W`](W) writer structure
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICRrs {
    const RESET_VALUE: u32 = 0;
}
