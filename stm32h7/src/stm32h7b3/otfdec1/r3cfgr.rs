#[doc = "Register `R3CFGR` reader"]
pub type R = crate::R<R3CFGRrs>;
#[doc = "Register `R3CFGR` writer"]
pub type W = crate::W<R3CFGRrs>;
#[doc = "Field `REG_EN` reader - region on-the-fly decryption enable"]
pub type REG_EN_R = crate::BitReader;
#[doc = "Field `REG_EN` writer - region on-the-fly decryption enable"]
pub type REG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIGLOCK` reader - region config lock"]
pub type CONFIGLOCK_R = crate::BitReader;
#[doc = "Field `CONFIGLOCK` writer - region config lock"]
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYLOCK` reader - region key lock"]
pub type KEYLOCK_R = crate::BitReader;
#[doc = "Field `KEYLOCK` writer - region key lock"]
pub type KEYLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - operating mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - operating mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCRC` reader - region key 8-bit CRC"]
pub type KEYCRC_R = crate::FieldReader;
#[doc = "Field `REGx_VERSION` reader - region firmware version"]
pub type REGX_VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `REGx_VERSION` writer - region firmware version"]
pub type REGX_VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - region key 8-bit CRC"]
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<R3CFGRrs> {
        REG_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<R3CFGRrs> {
        CONFIGLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    #[must_use]
    pub fn keylock(&mut self) -> KEYLOCK_W<R3CFGRrs> {
        KEYLOCK_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<R3CFGRrs> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    #[must_use]
    pub fn regx_version(&mut self) -> REGX_VERSION_W<R3CFGRrs> {
        REGX_VERSION_W::new(self, 16)
    }
}
#[doc = "OTFDEC region x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r3cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r3cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R3CFGRrs;
impl crate::RegisterSpec for R3CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r3cfgr::R`](R) reader structure"]
impl crate::Readable for R3CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`r3cfgr::W`](W) writer structure"]
impl crate::Writable for R3CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R3CFGR to value 0"]
impl crate::Resettable for R3CFGRrs {
    const RESET_VALUE: u32 = 0;
}
