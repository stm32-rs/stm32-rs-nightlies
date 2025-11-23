///Register `SECCR` reader
pub type R = crate::R<SECCRrs>;
///Register `SECCR` writer
pub type W = crate::W<SECCRrs>;
///Field `SECPG` reader - SECPG
pub type SECPG_R = crate::BitReader;
///Field `SECPG` writer - SECPG
pub type SECPG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECPER` reader - SECPER
pub type SECPER_R = crate::BitReader;
///Field `SECPER` writer - SECPER
pub type SECPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECMER1` reader - SECMER1
pub type SECMER1_R = crate::BitReader;
///Field `SECMER1` writer - SECMER1
pub type SECMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECPNB` reader - SECPNB
pub type SECPNB_R = crate::FieldReader;
///Field `SECPNB` writer - SECPNB
pub type SECPNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SECBKER` reader - SECBKER
pub type SECBKER_R = crate::BitReader;
///Field `SECBKER` writer - SECBKER
pub type SECBKER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECMER2` reader - SECMER2
pub type SECMER2_R = crate::BitReader;
///Field `SECMER2` writer - SECMER2
pub type SECMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECSTRT` reader - SECSTRT
pub type SECSTRT_R = crate::BitReader;
///Field `SECSTRT` writer - SECSTRT
pub type SECSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECEOPIE` reader - SECEOPIE
pub type SECEOPIE_R = crate::BitReader;
///Field `SECEOPIE` writer - SECEOPIE
pub type SECEOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECERRIE` reader - SECERRIE
pub type SECERRIE_R = crate::BitReader;
///Field `SECERRIE` writer - SECERRIE
pub type SECERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECRDERRIE` reader - SECRDERRIE
pub type SECRDERRIE_R = crate::BitReader;
///Field `SECRDERRIE` writer - SECRDERRIE
pub type SECRDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECINV` reader - SECINV
pub type SECINV_R = crate::BitReader;
///Field `SECINV` writer - SECINV
pub type SECINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECLOCK` reader - SECLOCK
pub type SECLOCK_R = crate::BitReader;
///Field `SECLOCK` writer - SECLOCK
pub type SECLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SECPG
    #[inline(always)]
    pub fn secpg(&self) -> SECPG_R {
        SECPG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SECPER
    #[inline(always)]
    pub fn secper(&self) -> SECPER_R {
        SECPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SECMER1
    #[inline(always)]
    pub fn secmer1(&self) -> SECMER1_R {
        SECMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - SECPNB
    #[inline(always)]
    pub fn secpnb(&self) -> SECPNB_R {
        SECPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - SECBKER
    #[inline(always)]
    pub fn secbker(&self) -> SECBKER_R {
        SECBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - SECMER2
    #[inline(always)]
    pub fn secmer2(&self) -> SECMER2_R {
        SECMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SECSTRT
    #[inline(always)]
    pub fn secstrt(&self) -> SECSTRT_R {
        SECSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - SECEOPIE
    #[inline(always)]
    pub fn seceopie(&self) -> SECEOPIE_R {
        SECEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SECERRIE
    #[inline(always)]
    pub fn secerrie(&self) -> SECERRIE_R {
        SECERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SECRDERRIE
    #[inline(always)]
    pub fn secrderrie(&self) -> SECRDERRIE_R {
        SECRDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - SECINV
    #[inline(always)]
    pub fn secinv(&self) -> SECINV_R {
        SECINV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - SECLOCK
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCR")
            .field("secpg", &self.secpg())
            .field("secper", &self.secper())
            .field("secmer1", &self.secmer1())
            .field("secpnb", &self.secpnb())
            .field("secbker", &self.secbker())
            .field("secmer2", &self.secmer2())
            .field("secstrt", &self.secstrt())
            .field("seceopie", &self.seceopie())
            .field("secerrie", &self.secerrie())
            .field("secrderrie", &self.secrderrie())
            .field("secinv", &self.secinv())
            .field("seclock", &self.seclock())
            .finish()
    }
}
impl W {
    ///Bit 0 - SECPG
    #[inline(always)]
    pub fn secpg(&mut self) -> SECPG_W<'_, SECCRrs> {
        SECPG_W::new(self, 0)
    }
    ///Bit 1 - SECPER
    #[inline(always)]
    pub fn secper(&mut self) -> SECPER_W<'_, SECCRrs> {
        SECPER_W::new(self, 1)
    }
    ///Bit 2 - SECMER1
    #[inline(always)]
    pub fn secmer1(&mut self) -> SECMER1_W<'_, SECCRrs> {
        SECMER1_W::new(self, 2)
    }
    ///Bits 3:9 - SECPNB
    #[inline(always)]
    pub fn secpnb(&mut self) -> SECPNB_W<'_, SECCRrs> {
        SECPNB_W::new(self, 3)
    }
    ///Bit 11 - SECBKER
    #[inline(always)]
    pub fn secbker(&mut self) -> SECBKER_W<'_, SECCRrs> {
        SECBKER_W::new(self, 11)
    }
    ///Bit 15 - SECMER2
    #[inline(always)]
    pub fn secmer2(&mut self) -> SECMER2_W<'_, SECCRrs> {
        SECMER2_W::new(self, 15)
    }
    ///Bit 16 - SECSTRT
    #[inline(always)]
    pub fn secstrt(&mut self) -> SECSTRT_W<'_, SECCRrs> {
        SECSTRT_W::new(self, 16)
    }
    ///Bit 24 - SECEOPIE
    #[inline(always)]
    pub fn seceopie(&mut self) -> SECEOPIE_W<'_, SECCRrs> {
        SECEOPIE_W::new(self, 24)
    }
    ///Bit 25 - SECERRIE
    #[inline(always)]
    pub fn secerrie(&mut self) -> SECERRIE_W<'_, SECCRrs> {
        SECERRIE_W::new(self, 25)
    }
    ///Bit 26 - SECRDERRIE
    #[inline(always)]
    pub fn secrderrie(&mut self) -> SECRDERRIE_W<'_, SECCRrs> {
        SECRDERRIE_W::new(self, 26)
    }
    ///Bit 29 - SECINV
    #[inline(always)]
    pub fn secinv(&mut self) -> SECINV_W<'_, SECCRrs> {
        SECINV_W::new(self, 29)
    }
    ///Bit 31 - SECLOCK
    #[inline(always)]
    pub fn seclock(&mut self) -> SECLOCK_W<'_, SECCRrs> {
        SECLOCK_W::new(self, 31)
    }
}
/**Flash secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECCR)*/
pub struct SECCRrs;
impl crate::RegisterSpec for SECCRrs {
    type Ux = u32;
}
///`read()` method returns [`seccr::R`](R) reader structure
impl crate::Readable for SECCRrs {}
///`write(|w| ..)` method takes [`seccr::W`](W) writer structure
impl crate::Writable for SECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCR to value 0x8000_0000
impl crate::Resettable for SECCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
