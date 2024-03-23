#[doc = "Register `CLKCR` reader"]
pub type R = crate::R<CLKCRrs>;
#[doc = "Register `CLKCR` writer"]
pub type W = crate::W<CLKCRrs>;
#[doc = "Field `CLKDIV` reader - Clock divide factor"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide factor"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSAV` reader - Power saving configuration bit"]
pub type PWRSAV_R = crate::BitReader;
#[doc = "Field `PWRSAV` writer - Power saving configuration bit"]
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - Clock divider bypass enable bit"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - Clock divider bypass enable bit"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit"]
pub type WIDBUS_R = crate::FieldReader;
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit"]
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NEGEDGE` reader - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_R = crate::BitReader;
#[doc = "Field `NEGEDGE` writer - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWFC_EN` reader - HW Flow Control enable"]
pub type HWFC_EN_R = crate::BitReader;
#[doc = "Field `HWFC_EN` writer - HW Flow Control enable"]
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCRrs> {
        CLKEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<CLKCRrs> {
        PWRSAV_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CLKCRrs> {
        BYPASS_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<CLKCRrs> {
        WIDBUS_W::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<CLKCRrs> {
        NEGEDGE_W::new(self, 13)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<CLKCRrs> {
        HWFC_EN_W::new(self, 14)
    }
}
#[doc = "SDI clock control register (SDIO_CLKCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcr::R`](R) reader structure"]
impl crate::Readable for CLKCRrs {}
#[doc = "`write(|w| ..)` method takes [`clkcr::W`](W) writer structure"]
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCRrs {
    const RESET_VALUE: u32 = 0;
}
