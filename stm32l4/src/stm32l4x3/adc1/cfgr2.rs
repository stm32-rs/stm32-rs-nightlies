#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `ROVSE` reader - DMAEN"]
pub type ROVSE_R = crate::BitReader;
#[doc = "Field `ROVSE` writer - DMAEN"]
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVSE` reader - DMACFG"]
pub type JOVSE_R = crate::BitReader;
#[doc = "Field `JOVSE` writer - DMACFG"]
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR` reader - RES"]
pub type OVSR_R = crate::FieldReader;
#[doc = "Field `OVSR` writer - RES"]
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVSS` reader - ALIGN"]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - ALIGN"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOVS` reader - EXTSEL"]
pub type TOVS_R = crate::BitReader;
#[doc = "Field `TOVS` writer - EXTSEL"]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVSM` reader - EXTEN"]
pub type ROVSM_R = crate::BitReader;
#[doc = "Field `ROVSM` writer - EXTEN"]
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - EXTSEL"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - EXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bit 10 - EXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
