#[doc = "Register `USBPHYC_PLL` reader"]
pub type R = crate::R<USBPHYC_PLLrs>;
#[doc = "Register `USBPHYC_PLL` writer"]
pub type W = crate::W<USBPHYC_PLLrs>;
#[doc = "Field `PLLNDIV` reader - PLLNDIV"]
pub type PLLNDIV_R = crate::FieldReader;
#[doc = "Field `PLLNDIV` writer - PLLNDIV"]
pub type PLLNDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLLODF` reader - PLLODF"]
pub type PLLODF_R = crate::FieldReader;
#[doc = "Field `PLLODF` writer - PLLODF"]
pub type PLLODF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLFRACIN` reader - PLLFRACIN"]
pub type PLLFRACIN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLFRACIN` writer - PLLFRACIN"]
pub type PLLFRACIN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTRB` reader - PLLSTRB"]
pub type PLLSTRB_R = crate::BitReader;
#[doc = "Field `PLLSTRB` writer - PLLSTRB"]
pub type PLLSTRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTRBYP` reader - PLLSTRBYP"]
pub type PLLSTRBYP_R = crate::BitReader;
#[doc = "Field `PLLSTRBYP` writer - PLLSTRBYP"]
pub type PLLSTRBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLFRACCTL` reader - PLLFRACCTL"]
pub type PLLFRACCTL_R = crate::BitReader;
#[doc = "Field `PLLFRACCTL` writer - PLLFRACCTL"]
pub type PLLFRACCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDITHEN0` reader - PLLDITHEN0"]
pub type PLLDITHEN0_R = crate::BitReader;
#[doc = "Field `PLLDITHEN0` writer - PLLDITHEN0"]
pub type PLLDITHEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDITHEN1` reader - PLLDITHEN1"]
pub type PLLDITHEN1_R = crate::BitReader;
#[doc = "Field `PLLDITHEN1` writer - PLLDITHEN1"]
pub type PLLDITHEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&self) -> PLLNDIV_R {
        PLLNDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&self) -> PLLODF_R {
        PLLODF_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&self) -> PLLFRACIN_R {
        PLLFRACIN_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&self) -> PLLSTRB_R {
        PLLSTRB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&self) -> PLLSTRBYP_R {
        PLLSTRBYP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&self) -> PLLFRACCTL_R {
        PLLFRACCTL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&self) -> PLLDITHEN0_R {
        PLLDITHEN0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&self) -> PLLDITHEN1_R {
        PLLDITHEN1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    #[must_use]
    pub fn pllndiv(&mut self) -> PLLNDIV_W<USBPHYC_PLLrs> {
        PLLNDIV_W::new(self, 0)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    #[must_use]
    pub fn pllodf(&mut self) -> PLLODF_W<USBPHYC_PLLrs> {
        PLLODF_W::new(self, 7)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    #[must_use]
    pub fn pllfracin(&mut self) -> PLLFRACIN_W<USBPHYC_PLLrs> {
        PLLFRACIN_W::new(self, 10)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<USBPHYC_PLLrs> {
        PLLEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    #[must_use]
    pub fn pllstrb(&mut self) -> PLLSTRB_W<USBPHYC_PLLrs> {
        PLLSTRB_W::new(self, 27)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    #[must_use]
    pub fn pllstrbyp(&mut self) -> PLLSTRBYP_W<USBPHYC_PLLrs> {
        PLLSTRBYP_W::new(self, 28)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    #[must_use]
    pub fn pllfracctl(&mut self) -> PLLFRACCTL_W<USBPHYC_PLLrs> {
        PLLFRACCTL_W::new(self, 29)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    #[must_use]
    pub fn plldithen0(&mut self) -> PLLDITHEN0_W<USBPHYC_PLLrs> {
        PLLDITHEN0_W::new(self, 30)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    #[must_use]
    pub fn plldithen1(&mut self) -> PLLDITHEN1_W<USBPHYC_PLLrs> {
        PLLDITHEN1_W::new(self, 31)
    }
}
#[doc = "This register is used to control the PLL of the HS PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_pll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_pll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHYC_PLLrs;
impl crate::RegisterSpec for USBPHYC_PLLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphyc_pll::R`](R) reader structure"]
impl crate::Readable for USBPHYC_PLLrs {}
#[doc = "`write(|w| ..)` method takes [`usbphyc_pll::W`](W) writer structure"]
impl crate::Writable for USBPHYC_PLLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHYC_PLL to value 0xc000_0000"]
impl crate::Resettable for USBPHYC_PLLrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
