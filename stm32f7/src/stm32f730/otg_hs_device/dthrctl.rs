#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DTHRCTLrs>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DTHRCTLrs>;
#[doc = "Field `NONISOTHREN` reader - Nonisochronous IN endpoints threshold enable"]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - Nonisochronous IN endpoints threshold enable"]
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISO IN endpoint threshold enable"]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISO IN endpoint threshold enable"]
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - Transmit threshold length"]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit threshold length"]
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RXTHREN` reader - Receive threshold enable"]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - Receive threshold enable"]
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - Receive threshold length"]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - Receive threshold length"]
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARPEN` reader - Arbiter parking enable"]
pub type ARPEN_R = crate::BitReader;
#[doc = "Field `ARPEN` writer - Arbiter parking enable"]
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Nonisochronous IN endpoints threshold enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN endpoint threshold enable"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Receive threshold enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive threshold length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter parking enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Nonisochronous IN endpoints threshold enable"]
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<DTHRCTLrs> {
        NONISOTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ISO IN endpoint threshold enable"]
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<DTHRCTLrs> {
        ISOTHREN_W::new(self, 1)
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<DTHRCTLrs> {
        TXTHRLEN_W::new(self, 2)
    }
    #[doc = "Bit 16 - Receive threshold enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<DTHRCTLrs> {
        RXTHREN_W::new(self, 16)
    }
    #[doc = "Bits 17:25 - Receive threshold length"]
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<DTHRCTLrs> {
        RXTHRLEN_W::new(self, 17)
    }
    #[doc = "Bit 27 - Arbiter parking enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<DTHRCTLrs> {
        ARPEN_W::new(self, 27)
    }
}
#[doc = "OTG_HS Device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTHRCTLrs;
impl crate::RegisterSpec for DTHRCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DTHRCTLrs {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DTHRCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTHRCTL to value 0"]
impl crate::Resettable for DTHRCTLrs {
    const RESET_VALUE: u32 = 0;
}
