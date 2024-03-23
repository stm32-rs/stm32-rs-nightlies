#[doc = "Register `PMC` reader"]
pub type R = crate::R<PMCrs>;
#[doc = "Register `PMC` writer"]
pub type W = crate::W<PMCrs>;
#[doc = "Field `ADC1DC2` reader - ADC1DC2"]
pub type ADC1DC2_R = crate::BitReader;
#[doc = "Field `ADC1DC2` writer - ADC1DC2"]
pub type ADC1DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2DC2` reader - ADC2DC2"]
pub type ADC2DC2_R = crate::BitReader;
#[doc = "Field `ADC2DC2` writer - ADC2DC2"]
pub type ADC2DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3DC2` reader - ADC3DC2"]
pub type ADC3DC2_R = crate::BitReader;
#[doc = "Field `ADC3DC2` writer - ADC3DC2"]
pub type ADC3DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_RMII_SEL` reader - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL` writer - Ethernet PHY interface selection"]
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W<PMCrs> {
        ADC1DC2_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W<PMCrs> {
        ADC2DC2_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W<PMCrs> {
        ADC3DC2_W::new(self, 18)
    }
    #[doc = "Bit 23 - Ethernet PHY interface selection"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<PMCrs> {
        MII_RMII_SEL_W::new(self, 23)
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc::R`](R) reader structure"]
impl crate::Readable for PMCrs {}
#[doc = "`write(|w| ..)` method takes [`pmc::W`](W) writer structure"]
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMCrs {
    const RESET_VALUE: u32 = 0;
}
