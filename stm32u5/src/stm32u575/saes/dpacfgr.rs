#[doc = "Register `DPACFGR` reader"]
pub type R = crate::R<DPACFGRrs>;
#[doc = "Register `DPACFGR` writer"]
pub type W = crate::W<DPACFGRrs>;
#[doc = "Field `REDCFG` reader - REDCFG"]
pub type REDCFG_R = crate::BitReader;
#[doc = "Field `REDCFG` writer - REDCFG"]
pub type REDCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEED` reader - RESEED"]
pub type RESEED_R = crate::BitReader;
#[doc = "Field `RESEED` writer - RESEED"]
pub type RESEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMCFG` reader - TRIMCFG"]
pub type TRIMCFG_R = crate::FieldReader;
#[doc = "Field `TRIMCFG` writer - TRIMCFG"]
pub type TRIMCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONFIGLOCK` reader - CONFIGLOCK"]
pub type CONFIGLOCK_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - REDCFG"]
    #[inline(always)]
    pub fn redcfg(&self) -> REDCFG_R {
        REDCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESEED"]
    #[inline(always)]
    pub fn reseed(&self) -> RESEED_R {
        RESEED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - TRIMCFG"]
    #[inline(always)]
    pub fn trimcfg(&self) -> TRIMCFG_R {
        TRIMCFG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 31 - CONFIGLOCK"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - REDCFG"]
    #[inline(always)]
    #[must_use]
    pub fn redcfg(&mut self) -> REDCFG_W<DPACFGRrs> {
        REDCFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - RESEED"]
    #[inline(always)]
    #[must_use]
    pub fn reseed(&mut self) -> RESEED_W<DPACFGRrs> {
        RESEED_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - TRIMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn trimcfg(&mut self) -> TRIMCFG_W<DPACFGRrs> {
        TRIMCFG_W::new(self, 3)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpacfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpacfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPACFGRrs;
impl crate::RegisterSpec for DPACFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpacfgr::R`](R) reader structure"]
impl crate::Readable for DPACFGRrs {}
#[doc = "`write(|w| ..)` method takes [`dpacfgr::W`](W) writer structure"]
impl crate::Writable for DPACFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPACFGR to value 0x08"]
impl crate::Resettable for DPACFGRrs {
    const RESET_VALUE: u32 = 0x08;
}
