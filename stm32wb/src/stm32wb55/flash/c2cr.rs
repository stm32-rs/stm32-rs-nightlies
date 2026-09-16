///Register `C2CR` reader
pub type R = crate::R<C2CRrs>;
///Register `C2CR` writer
pub type W = crate::W<C2CRrs>;
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - Masse erase
pub type MER_R = crate::BitReader;
///Field `MER` writer - Masse erase
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Page Number selection
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page Number selection
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - Masse erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:10 - Page Number selection
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("strt", &self.strt())
            .field("fstpg", &self.fstpg())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, C2CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, C2CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Masse erase
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, C2CRrs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:10 - Page Number selection
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, C2CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, C2CRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<'_, C2CRrs> {
        FSTPG_W::new(self, 18)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, C2CRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, C2CRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<'_, C2CRrs> {
        RDERRIE_W::new(self, 26)
    }
}
/**CPU2 cortex M0 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:C2CR)*/
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
///`read()` method returns [`c2cr::R`](R) reader structure
impl crate::Readable for C2CRrs {}
///`write(|w| ..)` method takes [`c2cr::W`](W) writer structure
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2CR to value 0
impl crate::Resettable for C2CRrs {}
