///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `ITAMP1NOER` reader - Internal Tamper 1 no erase
pub type ITAMP1NOER_R = crate::BitReader;
///Field `ITAMP1NOER` writer - Internal Tamper 1 no erase
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2NOER` reader - Internal Tamper 2 no erase
pub type ITAMP2NOER_R = crate::BitReader;
///Field `ITAMP2NOER` writer - Internal Tamper 2 no erase
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3NOER` reader - Internal Tamper 3 no erase
pub type ITAMP3NOER_R = crate::BitReader;
///Field `ITAMP3NOER` writer - Internal Tamper 3 no erase
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP4NOER` reader - Internal Tamper 4 no erase
pub type ITAMP4NOER_R = crate::BitReader;
///Field `ITAMP4NOER` writer - Internal Tamper 4 no erase
pub type ITAMP4NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5NOER` reader - Internal Tamper 5 no erase
pub type ITAMP5NOER_R = crate::BitReader;
///Field `ITAMP5NOER` writer - Internal Tamper 5 no erase
pub type ITAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP6NOER` reader - Internal Tamper 6 no erase
pub type ITAMP6NOER_R = crate::BitReader;
///Field `ITAMP6NOER` writer - Internal Tamper 6 no erase
pub type ITAMP6NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP7NOER` reader - Internal Tamper 7 no erase
pub type ITAMP7NOER_R = crate::BitReader;
///Field `ITAMP7NOER` writer - Internal Tamper 7 no erase
pub type ITAMP7NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8NOER` reader - Internal Tamper 8 no erase
pub type ITAMP8NOER_R = crate::BitReader;
///Field `ITAMP8NOER` writer - Internal Tamper 8 no erase
pub type ITAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP9NOER` reader - Internal Tamper 9 no erase
pub type ITAMP9NOER_R = crate::BitReader;
///Field `ITAMP9NOER` writer - Internal Tamper 9 no erase
pub type ITAMP9NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP11NOER` reader - Internal Tamper 11 no erase
pub type ITAMP11NOER_R = crate::BitReader;
///Field `ITAMP11NOER` writer - Internal Tamper 11 no erase
pub type ITAMP11NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP15NOER` reader - Internal Tamper 15 no erase
pub type ITAMP15NOER_R = crate::BitReader;
///Field `ITAMP15NOER` writer - Internal Tamper 15 no erase
pub type ITAMP15NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Internal Tamper 1 no erase
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal Tamper 2 no erase
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Internal Tamper 3 no erase
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal Tamper 4 no erase
    #[inline(always)]
    pub fn itamp4noer(&self) -> ITAMP4NOER_R {
        ITAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Internal Tamper 5 no erase
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal Tamper 6 no erase
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Internal Tamper 7 no erase
    #[inline(always)]
    pub fn itamp7noer(&self) -> ITAMP7NOER_R {
        ITAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Internal Tamper 8 no erase
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal Tamper 9 no erase
    #[inline(always)]
    pub fn itamp9noer(&self) -> ITAMP9NOER_R {
        ITAMP9NOER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Internal Tamper 11 no erase
    #[inline(always)]
    pub fn itamp11noer(&self) -> ITAMP11NOER_R {
        ITAMP11NOER_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Internal Tamper 15 no erase
    #[inline(always)]
    pub fn itamp15noer(&self) -> ITAMP15NOER_R {
        ITAMP15NOER_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp1noer", &self.itamp1noer())
            .field("itamp2noer", &self.itamp2noer())
            .field("itamp3noer", &self.itamp3noer())
            .field("itamp4noer", &self.itamp4noer())
            .field("itamp5noer", &self.itamp5noer())
            .field("itamp6noer", &self.itamp6noer())
            .field("itamp7noer", &self.itamp7noer())
            .field("itamp8noer", &self.itamp8noer())
            .field("itamp9noer", &self.itamp9noer())
            .field("itamp11noer", &self.itamp11noer())
            .field("itamp15noer", &self.itamp15noer())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal Tamper 1 no erase
    #[inline(always)]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<CR3rs> {
        ITAMP1NOER_W::new(self, 0)
    }
    ///Bit 1 - Internal Tamper 2 no erase
    #[inline(always)]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<CR3rs> {
        ITAMP2NOER_W::new(self, 1)
    }
    ///Bit 2 - Internal Tamper 3 no erase
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    ///Bit 3 - Internal Tamper 4 no erase
    #[inline(always)]
    pub fn itamp4noer(&mut self) -> ITAMP4NOER_W<CR3rs> {
        ITAMP4NOER_W::new(self, 3)
    }
    ///Bit 4 - Internal Tamper 5 no erase
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    ///Bit 5 - Internal Tamper 6 no erase
    #[inline(always)]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<CR3rs> {
        ITAMP6NOER_W::new(self, 5)
    }
    ///Bit 6 - Internal Tamper 7 no erase
    #[inline(always)]
    pub fn itamp7noer(&mut self) -> ITAMP7NOER_W<CR3rs> {
        ITAMP7NOER_W::new(self, 6)
    }
    ///Bit 7 - Internal Tamper 8 no erase
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
    ///Bit 8 - Internal Tamper 9 no erase
    #[inline(always)]
    pub fn itamp9noer(&mut self) -> ITAMP9NOER_W<CR3rs> {
        ITAMP9NOER_W::new(self, 8)
    }
    ///Bit 10 - Internal Tamper 11 no erase
    #[inline(always)]
    pub fn itamp11noer(&mut self) -> ITAMP11NOER_W<CR3rs> {
        ITAMP11NOER_W::new(self, 10)
    }
    ///Bit 14 - Internal Tamper 15 no erase
    #[inline(always)]
    pub fn itamp15noer(&mut self) -> ITAMP15NOER_W<CR3rs> {
        ITAMP15NOER_W::new(self, 14)
    }
}
/**TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#TAMP:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
