#[doc = "Register `PWR_BDCR2` reader"]
pub type R = crate::R<PWR_BDCR2rs>;
#[doc = "Register `PWR_BDCR2` writer"]
pub type W = crate::W<PWR_BDCR2rs>;
#[doc = "Field `VBE` reader - VBAT charging enable"]
pub type VBE_R = crate::BitReader;
#[doc = "Field `VBE` writer - VBAT charging enable"]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBAT charging resistor selection"]
pub type VBRS_R = crate::BitReader;
#[doc = "Field `VBRS` writer - VBAT charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBAT charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<PWR_BDCR2rs> {
        VBE_W::new(self, 0)
    }
    #[doc = "Bit 1 - VBAT charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<PWR_BDCR2rs> {
        VBRS_W::new(self, 1)
    }
}
#[doc = "PWR Backup domain control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_bdcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_bdcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_BDCR2rs;
impl crate::RegisterSpec for PWR_BDCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_bdcr2::R`](R) reader structure"]
impl crate::Readable for PWR_BDCR2rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_bdcr2::W`](W) writer structure"]
impl crate::Writable for PWR_BDCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_BDCR2 to value 0"]
impl crate::Resettable for PWR_BDCR2rs {
    const RESET_VALUE: u32 = 0;
}
