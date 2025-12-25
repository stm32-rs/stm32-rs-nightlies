///Register `CLKCR` reader
pub type R = crate::R<CLKCRrs>;
///Register `CLKCR` writer
pub type W = crate::W<CLKCRrs>;
///Field `CLKDIV` reader - Clock divide factor
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide factor
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLKEN` reader - Clock enable bit
pub type CLKEN_R = crate::BitReader;
///Field `CLKEN` writer - Clock enable bit
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSAV` reader - Power saving configuration bit
pub type PWRSAV_R = crate::BitReader;
///Field `PWRSAV` writer - Power saving configuration bit
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPASS` reader - Clock divider bypass enable bit
pub type BYPASS_R = crate::BitReader;
///Field `BYPASS` writer - Clock divider bypass enable bit
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIDBUS` reader - Wide bus mode enable bit
pub type WIDBUS_R = crate::FieldReader;
///Field `WIDBUS` writer - Wide bus mode enable bit
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NEGEDGE` reader - SDIO_CK dephasing selection bit
pub type NEGEDGE_R = crate::BitReader;
///Field `NEGEDGE` writer - SDIO_CK dephasing selection bit
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HWFC_EN` reader - HW Flow Control enable
pub type HWFC_EN_R = crate::BitReader;
///Field `HWFC_EN` writer - HW Flow Control enable
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCR")
            .field("hwfc_en", &self.hwfc_en())
            .field("negedge", &self.negedge())
            .field("widbus", &self.widbus())
            .field("bypass", &self.bypass())
            .field("pwrsav", &self.pwrsav())
            .field("clken", &self.clken())
            .field("clkdiv", &self.clkdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CLKCRrs> {
        CLKEN_W::new(self, 8)
    }
    ///Bit 9 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W<'_, CLKCRrs> {
        PWRSAV_W::new(self, 9)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<'_, CLKCRrs> {
        BYPASS_W::new(self, 10)
    }
    ///Bits 11:12 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W<'_, CLKCRrs> {
        WIDBUS_W::new(self, 11)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W<'_, CLKCRrs> {
        NEGEDGE_W::new(self, 13)
    }
    ///Bit 14 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<'_, CLKCRrs> {
        HWFC_EN_W::new(self, 14)
    }
}
/**SDI clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:CLKCR)*/
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
///`read()` method returns [`clkcr::R`](R) reader structure
impl crate::Readable for CLKCRrs {}
///`write(|w| ..)` method takes [`clkcr::W`](W) writer structure
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCRrs {}
