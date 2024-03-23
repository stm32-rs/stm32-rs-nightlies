#[doc = "Register `DBG_CR` reader"]
pub type R = crate::R<DBG_CRrs>;
#[doc = "Register `DBG_CR` writer"]
pub type W = crate::W<DBG_CRrs>;
#[doc = "Field `DBG_STOP` reader - Debug Stop mode"]
pub type DBG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Debug Stop mode"]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - Debug Standby and Shutdown modes"]
pub type DBG_STANDBY_R = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby and Shutdown modes"]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<DBG_CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<DBG_CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
}
#[doc = "DBG configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_CRrs;
impl crate::RegisterSpec for DBG_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cr::R`](R) reader structure"]
impl crate::Readable for DBG_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dbg_cr::W`](W) writer structure"]
impl crate::Writable for DBG_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CR to value 0"]
impl crate::Resettable for DBG_CRrs {
    const RESET_VALUE: u32 = 0;
}
