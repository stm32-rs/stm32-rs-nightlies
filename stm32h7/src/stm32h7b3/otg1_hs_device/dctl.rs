#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTLrs>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTLrs>;
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RWUSIG_R = crate::BitReader;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SDIS_R = crate::BitReader;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GINSTS_R = crate::BitReader;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GONSTS_R = crate::BitReader;
#[doc = "Field `TCTL` reader - Test control"]
pub type TCTL_R = crate::FieldReader;
#[doc = "Field `TCTL` writer - Test control"]
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type POPRGDNE_R = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rwusig(&mut self) -> RWUSIG_W<DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<DCTLrs> {
        SDIS_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    #[must_use]
    pub fn tctl(&mut self) -> TCTL_W<DCTLrs> {
        TCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    #[must_use]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
}
#[doc = "OTG_HS device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTLrs;
impl crate::RegisterSpec for DCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTLrs {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTLrs {
    const RESET_VALUE: u32 = 0;
}
