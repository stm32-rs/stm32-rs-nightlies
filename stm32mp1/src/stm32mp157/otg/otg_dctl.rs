#[doc = "Register `OTG_DCTL` reader"]
pub type R = crate::R<OTG_DCTLrs>;
#[doc = "Register `OTG_DCTL` writer"]
pub type W = crate::W<OTG_DCTLrs>;
#[doc = "Field `RWUSIG` reader - RWUSIG"]
pub type RWUSIG_R = crate::BitReader;
#[doc = "Field `RWUSIG` writer - RWUSIG"]
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - SDIS"]
pub type SDIS_R = crate::BitReader;
#[doc = "Field `SDIS` writer - SDIS"]
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - GINSTS"]
pub type GINSTS_R = crate::BitReader;
#[doc = "Field `GONSTS` reader - GONSTS"]
pub type GONSTS_R = crate::BitReader;
#[doc = "Field `TCTL` reader - TCTL"]
pub type TCTL_R = crate::FieldReader;
#[doc = "Field `TCTL` writer - TCTL"]
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` writer - SGINAK"]
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - CGINAK"]
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - SGONAK"]
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - CGONAK"]
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - POPRGDNE"]
pub type POPRGDNE_R = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - POPRGDNE"]
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSBESLRJCT` reader - DSBESLRJCT"]
pub type DSBESLRJCT_R = crate::BitReader;
#[doc = "Field `DSBESLRJCT` writer - DSBESLRJCT"]
pub type DSBESLRJCT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GINSTS"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GONSTS"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    pub fn dsbeslrjct(&self) -> DSBESLRJCT_R {
        DSBESLRJCT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    #[must_use]
    pub fn rwusig(&mut self) -> RWUSIG_W<OTG_DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<OTG_DCTLrs> {
        SDIS_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    #[must_use]
    pub fn tctl(&mut self) -> TCTL_W<OTG_DCTLrs> {
        TCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - SGINAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<OTG_DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - CGINAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<OTG_DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - SGONAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<OTG_DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - CGONAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<OTG_DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    #[must_use]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<OTG_DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    #[must_use]
    pub fn dsbeslrjct(&mut self) -> DSBESLRJCT_W<OTG_DCTLrs> {
        DSBESLRJCT_W::new(self, 18)
    }
}
#[doc = "OTG device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DCTLrs;
impl crate::RegisterSpec for OTG_DCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dctl::R`](R) reader structure"]
impl crate::Readable for OTG_DCTLrs {}
#[doc = "`write(|w| ..)` method takes [`otg_dctl::W`](W) writer structure"]
impl crate::Writable for OTG_DCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DCTL to value 0x02"]
impl crate::Resettable for OTG_DCTLrs {
    const RESET_VALUE: u32 = 0x02;
}
