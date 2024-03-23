#[doc = "Register `PWR_CR2` reader"]
pub type R = crate::R<PWR_CR2rs>;
#[doc = "Register `PWR_CR2` writer"]
pub type W = crate::W<PWR_CR2rs>;
#[doc = "Field `BREN` reader - BREN"]
pub type BREN_R = crate::BitReader;
#[doc = "Field `BREN` writer - BREN"]
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RREN` reader - RREN"]
pub type RREN_R = crate::BitReader;
#[doc = "Field `RREN` writer - RREN"]
pub type RREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONEN` reader - MONEN"]
pub type MONEN_R = crate::BitReader;
#[doc = "Field `MONEN` writer - MONEN"]
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRRDY` reader - BRRDY"]
pub type BRRDY_R = crate::BitReader;
#[doc = "Field `RRRDY` reader - RRRDY"]
pub type RRRDY_R = crate::BitReader;
#[doc = "Field `VBATL` reader - VBATL"]
pub type VBATL_R = crate::BitReader;
#[doc = "Field `VBATH` reader - VBATH"]
pub type VBATH_R = crate::BitReader;
#[doc = "Field `TEMPL` reader - TEMPL"]
pub type TEMPL_R = crate::BitReader;
#[doc = "Field `TEMPH` reader - TEMPH"]
pub type TEMPH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MONEN"]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - BRRDY"]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RRRDY"]
    #[inline(always)]
    pub fn rrrdy(&self) -> RRRDY_R {
        RRRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - VBATL"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBATH"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TEMPL"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEMPH"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<PWR_CR2rs> {
        BREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    #[must_use]
    pub fn rren(&mut self) -> RREN_W<PWR_CR2rs> {
        RREN_W::new(self, 1)
    }
    #[doc = "Bit 4 - MONEN"]
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<PWR_CR2rs> {
        MONEN_W::new(self, 4)
    }
}
#[doc = "Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CR2rs;
impl crate::RegisterSpec for PWR_CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr2::R`](R) reader structure"]
impl crate::Readable for PWR_CR2rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure"]
impl crate::Writable for PWR_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PWR_CR2rs {
    const RESET_VALUE: u32 = 0;
}
