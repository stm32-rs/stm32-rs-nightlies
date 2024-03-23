#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `RELOAD` reader - Counter reload value"]
pub type RELOAD_R = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value"]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit"]
pub type FELIM_R = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit"]
pub type FELIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCDIV` reader - SYNCDIV"]
pub type SYNCDIV_R = crate::FieldReader;
#[doc = "Field `SYNCDIV` writer - SYNCDIV"]
pub type SYNCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection"]
pub type SYNCSRC_R = crate::FieldReader;
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection"]
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection"]
pub type SYNCPOL_R = crate::BitReader;
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection"]
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNCDIV"]
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<CFGRrs> {
        RELOAD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    #[must_use]
    pub fn felim(&mut self) -> FELIM_W<CFGRrs> {
        FELIM_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNCDIV"]
    #[inline(always)]
    #[must_use]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<CFGRrs> {
        SYNCDIV_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<CFGRrs> {
        SYNCSRC_W::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CFGRrs> {
        SYNCPOL_W::new(self, 31)
    }
}
#[doc = "CRS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
