///Register `CC1CFGR` reader
pub type R = crate::R<CC1CFGRrs>;
///Register `CC1CFGR` writer
pub type W = crate::W<CC1CFGRrs>;
///Field `CCEN` reader - Cipher context enable
pub type CCEN_R = crate::BitReader;
///Field `CCEN` writer - Cipher context enable
pub type CCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCLOCK` reader - Cipher context lock
pub type CCLOCK_R = crate::BitReader;
///Field `CCLOCK` writer - Cipher context lock
pub type CCLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYLOCK` reader - Key lock
pub type KEYLOCK_R = crate::BitReader;
///Field `KEYLOCK` writer - Key lock
pub type KEYLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - Authorized cipher mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - Authorized cipher mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYCRC` reader - Key CRC
pub type KEYCRC_R = crate::FieldReader;
///Field `VERSION` reader - Version
pub type VERSION_R = crate::FieldReader<u16>;
///Field `VERSION` writer - Version
pub type VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Cipher context enable
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Cipher context lock
    #[inline(always)]
    pub fn cclock(&self) -> CCLOCK_R {
        CCLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key lock
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Authorized cipher mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:15 - Key CRC
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - Version
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC1CFGR")
            .field("ccen", &self.ccen())
            .field("cclock", &self.cclock())
            .field("keylock", &self.keylock())
            .field("mode", &self.mode())
            .field("keycrc", &self.keycrc())
            .field("version", &self.version())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cipher context enable
    #[inline(always)]
    pub fn ccen(&mut self) -> CCEN_W<'_, CC1CFGRrs> {
        CCEN_W::new(self, 0)
    }
    ///Bit 1 - Cipher context lock
    #[inline(always)]
    pub fn cclock(&mut self) -> CCLOCK_W<'_, CC1CFGRrs> {
        CCLOCK_W::new(self, 1)
    }
    ///Bit 2 - Key lock
    #[inline(always)]
    pub fn keylock(&mut self) -> KEYLOCK_W<'_, CC1CFGRrs> {
        KEYLOCK_W::new(self, 2)
    }
    ///Bits 4:5 - Authorized cipher mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CC1CFGRrs> {
        MODE_W::new(self, 4)
    }
    ///Bits 16:31 - Version
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W<'_, CC1CFGRrs> {
        VERSION_W::new(self, 16)
    }
}
/**MCE cipher context 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`cc1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:CC1CFGR)*/
pub struct CC1CFGRrs;
impl crate::RegisterSpec for CC1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cc1cfgr::R`](R) reader structure
impl crate::Readable for CC1CFGRrs {}
///`write(|w| ..)` method takes [`cc1cfgr::W`](W) writer structure
impl crate::Writable for CC1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CC1CFGR to value 0
impl crate::Resettable for CC1CFGRrs {}
