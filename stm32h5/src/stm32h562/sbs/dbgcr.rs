#[doc = "Register `DBGCR` reader"]
pub type R = crate::R<DBGCRrs>;
#[doc = "Register `DBGCR` writer"]
pub type W = crate::W<DBGCRrs>;
#[doc = "Field `AP_UNLOCK` reader - access port unlock Write 0xB4 to this bitfield to open the device access port."]
pub type AP_UNLOCK_R = crate::FieldReader;
#[doc = "Field `AP_UNLOCK` writer - access port unlock Write 0xB4 to this bitfield to open the device access port."]
pub type AP_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_UNLOCK` reader - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
pub type DBG_UNLOCK_R = crate::FieldReader;
#[doc = "Field `DBG_UNLOCK` writer - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
pub type DBG_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_AUTH_HDPL` reader - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
pub type DBG_AUTH_HDPL_R = crate::FieldReader;
#[doc = "Field `DBG_AUTH_HDPL` writer - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
pub type DBG_AUTH_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_AUTH_SEC` reader - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure."]
pub type DBG_AUTH_SEC_R = crate::FieldReader;
#[doc = "Field `DBG_AUTH_SEC` writer - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure."]
pub type DBG_AUTH_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port."]
    #[inline(always)]
    pub fn ap_unlock(&self) -> AP_UNLOCK_R {
        AP_UNLOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
    #[inline(always)]
    pub fn dbg_unlock(&self) -> DBG_UNLOCK_R {
        DBG_UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
    #[inline(always)]
    pub fn dbg_auth_hdpl(&self) -> DBG_AUTH_HDPL_R {
        DBG_AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure."]
    #[inline(always)]
    pub fn dbg_auth_sec(&self) -> DBG_AUTH_SEC_R {
        DBG_AUTH_SEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port."]
    #[inline(always)]
    #[must_use]
    pub fn ap_unlock(&mut self) -> AP_UNLOCK_W<DBGCRrs> {
        AP_UNLOCK_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_unlock(&mut self) -> DBG_UNLOCK_W<DBGCRrs> {
        DBG_UNLOCK_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_auth_hdpl(&mut self) -> DBG_AUTH_HDPL_W<DBGCRrs> {
        DBG_AUTH_HDPL_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_auth_sec(&mut self) -> DBG_AUTH_SEC_W<DBGCRrs> {
        DBG_AUTH_SEC_W::new(self, 24)
    }
}
#[doc = "SBS debug control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGCRrs;
impl crate::RegisterSpec for DBGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgcr::R`](R) reader structure"]
impl crate::Readable for DBGCRrs {}
#[doc = "`write(|w| ..)` method takes [`dbgcr::W`](W) writer structure"]
impl crate::Writable for DBGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCR to value 0"]
impl crate::Resettable for DBGCRrs {
    const RESET_VALUE: u32 = 0;
}
