///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - This bit triggers the mass erase (all user pages) when set
pub type MER_R = crate::BitReader;
///Field `MER` writer - This bit triggers the mass erase (all user pages) when set
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Page number selection
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page number selection
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Options modification start
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Options modification start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSTPG` reader - Fast programming
pub type FSTPG_R = crate::BitReader;
///Field `FSTPG` writer - Fast programming
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - End of operation interrupt enable
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - End of operation interrupt enable
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERRIE` reader - PCROP read error interrupt enable
pub type RDERRIE_R = crate::BitReader;
///Field `RDERRIE` writer - PCROP read error interrupt enable
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBL_LAUNCH` reader - Force the option byte loading
pub type OBL_LAUNCH_R = crate::BitReader;
///Field `OBL_LAUNCH` writer - Force the option byte loading
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTLOCK` reader - Options Lock
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Options Lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - FLASH_CR Lock
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - FLASH_CR Lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - This bit triggers the mass erase (all user pages) when set
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:10 - Page number selection
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("strt", &self.strt())
            .field("optstrt", &self.optstrt())
            .field("fstpg", &self.fstpg())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .field("obl_launch", &self.obl_launch())
            .field("optlock", &self.optlock())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - This bit triggers the mass erase (all user pages) when set
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<CRrs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:10 - Page number selection
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<CRrs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<CRrs> {
        FSTPG_W::new(self, 18)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<CRrs> {
        RDERRIE_W::new(self, 26)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<CRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<CRrs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Flash control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0xc000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
