#[doc = "Register `OTGHSPHYCR` reader"]
pub type R = crate::R<OTGHSPHYCRrs>;
#[doc = "Register `OTGHSPHYCR` writer"]
pub type W = crate::W<OTGHSPHYCRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCTRL` reader - PDCTRL"]
pub type PDCTRL_R = crate::BitReader;
#[doc = "Field `PDCTRL` writer - PDCTRL"]
pub type PDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - CLKSEL"]
pub type CLKSEL_R = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - CLKSEL"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDCTRL"]
    #[inline(always)]
    pub fn pdctrl(&self) -> PDCTRL_R {
        PDCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CLKSEL"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<OTGHSPHYCRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PDCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pdctrl(&mut self) -> PDCTRL_W<OTGHSPHYCRrs> {
        PDCTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<OTGHSPHYCRrs> {
        CLKSEL_W::new(self, 2)
    }
}
#[doc = "SYSCFG USB OTG_HS PHY register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otghsphycr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otghsphycr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTGHSPHYCRrs;
impl crate::RegisterSpec for OTGHSPHYCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otghsphycr::R`](R) reader structure"]
impl crate::Readable for OTGHSPHYCRrs {}
#[doc = "`write(|w| ..)` method takes [`otghsphycr::W`](W) writer structure"]
impl crate::Writable for OTGHSPHYCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTGHSPHYCR to value 0"]
impl crate::Resettable for OTGHSPHYCRrs {
    const RESET_VALUE: u32 = 0;
}
