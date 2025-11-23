///Register `DBGCR` reader
pub type R = crate::R<DBGCRrs>;
///Register `DBGCR` writer
pub type W = crate::W<DBGCRrs>;
///Field `AP_UNLOCK` reader - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_R = crate::FieldReader;
///Field `AP_UNLOCK` writer - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DBG_UNLOCK` reader - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_R = crate::FieldReader;
///Field `DBG_UNLOCK` writer - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DBG_AUTH_HDPL` reader - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_R = crate::FieldReader;
///Field `DBG_AUTH_HDPL` writer - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DBG_AUTH_SEC` reader - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
pub type DBG_AUTH_SEC_R = crate::FieldReader;
///Field `DBG_AUTH_SEC` writer - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
pub type DBG_AUTH_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    pub fn ap_unlock(&self) -> AP_UNLOCK_R {
        AP_UNLOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    pub fn dbg_unlock(&self) -> DBG_UNLOCK_R {
        DBG_UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    pub fn dbg_auth_hdpl(&self) -> DBG_AUTH_HDPL_R {
        DBG_AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
    #[inline(always)]
    pub fn dbg_auth_sec(&self) -> DBG_AUTH_SEC_R {
        DBG_AUTH_SEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCR")
            .field("ap_unlock", &self.ap_unlock())
            .field("dbg_unlock", &self.dbg_unlock())
            .field("dbg_auth_hdpl", &self.dbg_auth_hdpl())
            .field("dbg_auth_sec", &self.dbg_auth_sec())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    pub fn ap_unlock(&mut self) -> AP_UNLOCK_W<'_, DBGCRrs> {
        AP_UNLOCK_W::new(self, 0)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    pub fn dbg_unlock(&mut self) -> DBG_UNLOCK_W<'_, DBGCRrs> {
        DBG_UNLOCK_W::new(self, 8)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    pub fn dbg_auth_hdpl(&mut self) -> DBG_AUTH_HDPL_W<'_, DBGCRrs> {
        DBG_AUTH_HDPL_W::new(self, 16)
    }
    ///Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
    #[inline(always)]
    pub fn dbg_auth_sec(&mut self) -> DBG_AUTH_SEC_W<'_, DBGCRrs> {
        DBG_AUTH_SEC_W::new(self, 24)
    }
}
/**SBS debug control register

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SBS:DBGCR)*/
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
