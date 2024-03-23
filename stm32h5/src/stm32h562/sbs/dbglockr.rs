#[doc = "Register `DBGLOCKR` reader"]
pub type R = crate::R<DBGLOCKRrs>;
#[doc = "Register `DBGLOCKR` writer"]
pub type W = crate::W<DBGLOCKRrs>;
#[doc = "Field `DBGCFG_LOCK` reader - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_R = crate::FieldReader;
#[doc = "Field `DBGCFG_LOCK` writer - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    pub fn dbgcfg_lock(&self) -> DBGCFG_LOCK_R {
        DBGCFG_LOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    #[must_use]
    pub fn dbgcfg_lock(&mut self) -> DBGCFG_LOCK_W<DBGLOCKRrs> {
        DBGCFG_LOCK_W::new(self, 0)
    }
}
#[doc = "SBS debug lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbglockr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbglockr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGLOCKRrs;
impl crate::RegisterSpec for DBGLOCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbglockr::R`](R) reader structure"]
impl crate::Readable for DBGLOCKRrs {}
#[doc = "`write(|w| ..)` method takes [`dbglockr::W`](W) writer structure"]
impl crate::Writable for DBGLOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGLOCKR to value 0xb4"]
impl crate::Resettable for DBGLOCKRrs {
    const RESET_VALUE: u32 = 0xb4;
}
