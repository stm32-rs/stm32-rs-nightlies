///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `ITAMP1NOER` reader - ITAMP1NOER
pub type ITAMP1NOER_R = crate::BitReader;
///Field `ITAMP1NOER` writer - ITAMP1NOER
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2NOER` reader - ITAMP2NOER
pub type ITAMP2NOER_R = crate::BitReader;
///Field `ITAMP2NOER` writer - ITAMP2NOER
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3NOER` reader - ITAMP3NOER
pub type ITAMP3NOER_R = crate::BitReader;
///Field `ITAMP3NOER` writer - ITAMP3NOER
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5NOER` reader - ITAMP5NOER
pub type ITAMP5NOER_R = crate::BitReader;
///Field `ITAMP5NOER` writer - ITAMP5NOER
pub type ITAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8NOER` reader - ITAMP8NOER
pub type ITAMP8NOER_R = crate::BitReader;
///Field `ITAMP8NOER` writer - ITAMP8NOER
pub type ITAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ITAMP1NOER
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ITAMP2NOER
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ITAMP5NOER
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - ITAMP8NOER
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp1noer", &self.itamp1noer())
            .field("itamp2noer", &self.itamp2noer())
            .field("itamp3noer", &self.itamp3noer())
            .field("itamp5noer", &self.itamp5noer())
            .field("itamp8noer", &self.itamp8noer())
            .finish()
    }
}
impl W {
    ///Bit 0 - ITAMP1NOER
    #[inline(always)]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<'_, CR3rs> {
        ITAMP1NOER_W::new(self, 0)
    }
    ///Bit 1 - ITAMP2NOER
    #[inline(always)]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<'_, CR3rs> {
        ITAMP2NOER_W::new(self, 1)
    }
    ///Bit 2 - ITAMP3NOER
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<'_, CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    ///Bit 4 - ITAMP5NOER
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<'_, CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    ///Bit 7 - ITAMP8NOER
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<'_, CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
}
/**control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TAMP:CR3)*/
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
