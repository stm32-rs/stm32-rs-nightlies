#[doc = "Register `CLKCR` reader"]
pub type R = crate::R<CLKCRrs>;
#[doc = "Register `CLKCR` writer"]
pub type W = crate::W<CLKCRrs>;
#[doc = "Field `CLKDIV` reader - Clock divide factor"]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - Clock divide factor"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PWRSAV` reader - Power saving configuration bit"]
pub type PWRSAV_R = crate::BitReader;
#[doc = "Field `PWRSAV` writer - Power saving configuration bit"]
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit"]
pub type WIDBUS_R = crate::FieldReader;
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit"]
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NEGEDGE` reader - SDMMC_CK dephasing selection bit for data and command"]
pub type NEGEDGE_R = crate::BitReader;
#[doc = "Field `NEGEDGE` writer - SDMMC_CK dephasing selection bit for data and command"]
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWFC_EN` reader - Hardware flow control enable"]
pub type HWFC_EN_R = crate::BitReader;
#[doc = "Field `HWFC_EN` writer - Hardware flow control enable"]
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR` reader - Data rate signaling selection"]
pub type DDR_R = crate::BitReader;
#[doc = "Field `DDR` writer - Data rate signaling selection"]
pub type DDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSSPEED` reader - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
pub type BUSSPEED_R = crate::BitReader;
#[doc = "Field `BUSSPEED` writer - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
pub type BUSSPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELCLKRX` reader - Receive clock selection"]
pub type SELCLKRX_R = crate::FieldReader;
#[doc = "Field `SELCLKRX` writer - Receive clock selection"]
pub type SELCLKRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and command"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data rate signaling selection"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Receive clock selection"]
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 12 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<CLKCRrs> {
        PWRSAV_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<CLKCRrs> {
        WIDBUS_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and command"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<CLKCRrs> {
        NEGEDGE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<CLKCRrs> {
        HWFC_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Data rate signaling selection"]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<CLKCRrs> {
        DDR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
    #[inline(always)]
    #[must_use]
    pub fn busspeed(&mut self) -> BUSSPEED_W<CLKCRrs> {
        BUSSPEED_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Receive clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn selclkrx(&mut self) -> SELCLKRX_W<CLKCRrs> {
        SELCLKRX_W::new(self, 20)
    }
}
#[doc = "SDI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
