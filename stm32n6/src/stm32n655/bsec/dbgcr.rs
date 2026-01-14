///Register `DBGCR` reader
pub type R = crate::R<DBGCRrs>;
///Register `DBGCR` writer
pub type W = crate::W<DBGCRrs>;
///Field `UNLOCK` reader - any other value: debug not authorized (provided BSEC state is not OPEN)
pub type UNLOCK_R = crate::FieldReader;
///Field `UNLOCK` writer - any other value: debug not authorized (provided BSEC state is not OPEN)
pub type UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AUTH_HDPL` reader - level at which debug may be opened.
pub type AUTH_HDPL_R = crate::FieldReader;
///Field `AUTH_HDPL` writer - level at which debug may be opened.
pub type AUTH_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AUTH_SEC` reader - any other value: secure debug not authorized (provided BSEC state is not OPEN)
pub type AUTH_SEC_R = crate::FieldReader;
///Field `AUTH_SEC` writer - any other value: secure debug not authorized (provided BSEC state is not OPEN)
pub type AUTH_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 8:15 - any other value: debug not authorized (provided BSEC state is not OPEN)
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - level at which debug may be opened.
    #[inline(always)]
    pub fn auth_hdpl(&self) -> AUTH_HDPL_R {
        AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - any other value: secure debug not authorized (provided BSEC state is not OPEN)
    #[inline(always)]
    pub fn auth_sec(&self) -> AUTH_SEC_R {
        AUTH_SEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCR")
            .field("unlock", &self.unlock())
            .field("auth_hdpl", &self.auth_hdpl())
            .field("auth_sec", &self.auth_sec())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - any other value: debug not authorized (provided BSEC state is not OPEN)
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, DBGCRrs> {
        UNLOCK_W::new(self, 8)
    }
    ///Bits 16:23 - level at which debug may be opened.
    #[inline(always)]
    pub fn auth_hdpl(&mut self) -> AUTH_HDPL_W<'_, DBGCRrs> {
        AUTH_HDPL_W::new(self, 16)
    }
    ///Bits 24:31 - any other value: secure debug not authorized (provided BSEC state is not OPEN)
    #[inline(always)]
    pub fn auth_sec(&mut self) -> AUTH_SEC_W<'_, DBGCRrs> {
        AUTH_SEC_W::new(self, 24)
    }
}
/**BSEC Debug

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:DBGCR)*/
pub struct DBGCRrs;
impl crate::RegisterSpec for DBGCRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgcr::R`](R) reader structure
impl crate::Readable for DBGCRrs {}
///`write(|w| ..)` method takes [`dbgcr::W`](W) writer structure
impl crate::Writable for DBGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGCR to value 0
impl crate::Resettable for DBGCRrs {}
