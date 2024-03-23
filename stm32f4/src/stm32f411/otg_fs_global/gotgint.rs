#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINTrs>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINTrs>;
#[doc = "Field `SEDET` reader - Session end detected"]
pub type SEDET_R = crate::BitReader;
#[doc = "Field `SEDET` writer - Session end detected"]
pub type SEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRSSCHG` reader - Session request success status change"]
pub type SRSSCHG_R = crate::BitReader;
#[doc = "Field `SRSSCHG` writer - Session request success status change"]
pub type SRSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNSSCHG` reader - Host negotiation success status change"]
pub type HNSSCHG_R = crate::BitReader;
#[doc = "Field `HNSSCHG` writer - Host negotiation success status change"]
pub type HNSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGDET` reader - Host negotiation detected"]
pub type HNGDET_R = crate::BitReader;
#[doc = "Field `HNGDET` writer - Host negotiation detected"]
pub type HNGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTOCHG` reader - A-device timeout change"]
pub type ADTOCHG_R = crate::BitReader;
#[doc = "Field `ADTOCHG` writer - A-device timeout change"]
pub type ADTOCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCDNE` reader - Debounce done"]
pub type DBCDNE_R = crate::BitReader;
#[doc = "Field `DBCDNE` writer - Debounce done"]
pub type DBCDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SEDET_W<GOTGINTrs> {
        SEDET_W::new(self, 2)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SRSSCHG_W<GOTGINTrs> {
        SRSSCHG_W::new(self, 8)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<GOTGINTrs> {
        HNSSCHG_W::new(self, 9)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HNGDET_W<GOTGINTrs> {
        HNGDET_W::new(self, 17)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> ADTOCHG_W<GOTGINTrs> {
        ADTOCHG_W::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DBCDNE_W<GOTGINTrs> {
        DBCDNE_W::new(self, 19)
    }
}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINTrs;
impl crate::RegisterSpec for GOTGINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINTrs {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINTrs {
    const RESET_VALUE: u32 = 0;
}
