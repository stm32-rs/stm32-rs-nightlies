#[doc = "Register `OTG_DTHRCTL` reader"]
pub type R = crate::R<OTG_DTHRCTLrs>;
#[doc = "Register `OTG_DTHRCTL` writer"]
pub type W = crate::W<OTG_DTHRCTLrs>;
#[doc = "Field `NONISOTHREN` reader - NONISOTHREN"]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - NONISOTHREN"]
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISOTHREN"]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISOTHREN"]
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - TXTHRLEN"]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - TXTHRLEN"]
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RXTHREN` reader - RXTHREN"]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - RXTHREN"]
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - RXTHRLEN"]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - RXTHRLEN"]
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ARPEN_R = crate::BitReader;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NONISOTHREN"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISOTHREN"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - TXTHRLEN"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - RXTHREN"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - RXTHRLEN"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NONISOTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<OTG_DTHRCTLrs> {
        NONISOTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ISOTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<OTG_DTHRCTLrs> {
        ISOTHREN_W::new(self, 1)
    }
    #[doc = "Bits 2:10 - TXTHRLEN"]
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<OTG_DTHRCTLrs> {
        TXTHRLEN_W::new(self, 2)
    }
    #[doc = "Bit 16 - RXTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<OTG_DTHRCTLrs> {
        RXTHREN_W::new(self, 16)
    }
    #[doc = "Bits 17:25 - RXTHRLEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<OTG_DTHRCTLrs> {
        RXTHRLEN_W::new(self, 17)
    }
    #[doc = "Bit 27 - ARPEN"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<OTG_DTHRCTLrs> {
        ARPEN_W::new(self, 27)
    }
}
#[doc = "OTG device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DTHRCTLrs;
impl crate::RegisterSpec for OTG_DTHRCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dthrctl::R`](R) reader structure"]
impl crate::Readable for OTG_DTHRCTLrs {}
#[doc = "`write(|w| ..)` method takes [`otg_dthrctl::W`](W) writer structure"]
impl crate::Writable for OTG_DTHRCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DTHRCTL to value 0"]
impl crate::Resettable for OTG_DTHRCTLrs {
    const RESET_VALUE: u32 = 0;
}
