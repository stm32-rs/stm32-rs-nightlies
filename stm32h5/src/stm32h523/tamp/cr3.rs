///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `ITAMP1POM` reader - Internal tamper 1 potential mode
pub type ITAMP1POM_R = crate::BitReader;
///Field `ITAMP1POM` writer - Internal tamper 1 potential mode
pub type ITAMP1POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2POM` reader - Internal tamper 2 potential mode
pub type ITAMP2POM_R = crate::BitReader;
///Field `ITAMP2POM` writer - Internal tamper 2 potential mode
pub type ITAMP2POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3POM` reader - Internal tamper 3 potential mode
pub type ITAMP3POM_R = crate::BitReader;
///Field `ITAMP3POM` writer - Internal tamper 3 potential mode
pub type ITAMP3POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP4POM` reader - Internal tamper 4 potential mode
pub type ITAMP4POM_R = crate::BitReader;
///Field `ITAMP4POM` writer - Internal tamper 4 potential mode
pub type ITAMP4POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5POM` reader - Internal tamper 5 potential mode
pub type ITAMP5POM_R = crate::BitReader;
///Field `ITAMP5POM` writer - Internal tamper 5 potential mode
pub type ITAMP5POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP6POM` reader - Internal tamper 6 potential mode
pub type ITAMP6POM_R = crate::BitReader;
///Field `ITAMP6POM` writer - Internal tamper 6 potential mode
pub type ITAMP6POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP7POM` reader - Internal tamper 7 potential mode
pub type ITAMP7POM_R = crate::BitReader;
///Field `ITAMP7POM` writer - Internal tamper 7 potential mode
pub type ITAMP7POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8POM` reader - Internal tamper 8 potential mode
pub type ITAMP8POM_R = crate::BitReader;
///Field `ITAMP8POM` writer - Internal tamper 8 potential mode
pub type ITAMP8POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP9POM` reader - Internal tamper 9 potential mode
pub type ITAMP9POM_R = crate::BitReader;
///Field `ITAMP9POM` writer - Internal tamper 9 potential mode
pub type ITAMP9POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP11POM` reader - Internal tamper 11 potential mode
pub type ITAMP11POM_R = crate::BitReader;
///Field `ITAMP11POM` writer - Internal tamper 11 potential mode
pub type ITAMP11POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP12POM` reader - Internal tamper 12 potential mode
pub type ITAMP12POM_R = crate::BitReader;
///Field `ITAMP12POM` writer - Internal tamper 12 potential mode
pub type ITAMP12POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP13POM` reader - Internal tamper 13 potential mode
pub type ITAMP13POM_R = crate::BitReader;
///Field `ITAMP13POM` writer - Internal tamper 13 potential mode
pub type ITAMP13POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP15POM` reader - Internal tamper 15 potential mode
pub type ITAMP15POM_R = crate::BitReader;
///Field `ITAMP15POM` writer - Internal tamper 15 potential mode
pub type ITAMP15POM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Internal tamper 1 potential mode
    #[inline(always)]
    pub fn itamp1pom(&self) -> ITAMP1POM_R {
        ITAMP1POM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal tamper 2 potential mode
    #[inline(always)]
    pub fn itamp2pom(&self) -> ITAMP2POM_R {
        ITAMP2POM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Internal tamper 3 potential mode
    #[inline(always)]
    pub fn itamp3pom(&self) -> ITAMP3POM_R {
        ITAMP3POM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal tamper 4 potential mode
    #[inline(always)]
    pub fn itamp4pom(&self) -> ITAMP4POM_R {
        ITAMP4POM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Internal tamper 5 potential mode
    #[inline(always)]
    pub fn itamp5pom(&self) -> ITAMP5POM_R {
        ITAMP5POM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal tamper 6 potential mode
    #[inline(always)]
    pub fn itamp6pom(&self) -> ITAMP6POM_R {
        ITAMP6POM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Internal tamper 7 potential mode
    #[inline(always)]
    pub fn itamp7pom(&self) -> ITAMP7POM_R {
        ITAMP7POM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Internal tamper 8 potential mode
    #[inline(always)]
    pub fn itamp8pom(&self) -> ITAMP8POM_R {
        ITAMP8POM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal tamper 9 potential mode
    #[inline(always)]
    pub fn itamp9pom(&self) -> ITAMP9POM_R {
        ITAMP9POM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Internal tamper 11 potential mode
    #[inline(always)]
    pub fn itamp11pom(&self) -> ITAMP11POM_R {
        ITAMP11POM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Internal tamper 12 potential mode
    #[inline(always)]
    pub fn itamp12pom(&self) -> ITAMP12POM_R {
        ITAMP12POM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Internal tamper 13 potential mode
    #[inline(always)]
    pub fn itamp13pom(&self) -> ITAMP13POM_R {
        ITAMP13POM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Internal tamper 15 potential mode
    #[inline(always)]
    pub fn itamp15pom(&self) -> ITAMP15POM_R {
        ITAMP15POM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp1pom", &self.itamp1pom())
            .field("itamp2pom", &self.itamp2pom())
            .field("itamp3pom", &self.itamp3pom())
            .field("itamp4pom", &self.itamp4pom())
            .field("itamp5pom", &self.itamp5pom())
            .field("itamp6pom", &self.itamp6pom())
            .field("itamp7pom", &self.itamp7pom())
            .field("itamp8pom", &self.itamp8pom())
            .field("itamp9pom", &self.itamp9pom())
            .field("itamp11pom", &self.itamp11pom())
            .field("itamp12pom", &self.itamp12pom())
            .field("itamp13pom", &self.itamp13pom())
            .field("itamp15pom", &self.itamp15pom())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal tamper 1 potential mode
    #[inline(always)]
    pub fn itamp1pom(&mut self) -> ITAMP1POM_W<'_, CR3rs> {
        ITAMP1POM_W::new(self, 0)
    }
    ///Bit 1 - Internal tamper 2 potential mode
    #[inline(always)]
    pub fn itamp2pom(&mut self) -> ITAMP2POM_W<'_, CR3rs> {
        ITAMP2POM_W::new(self, 1)
    }
    ///Bit 2 - Internal tamper 3 potential mode
    #[inline(always)]
    pub fn itamp3pom(&mut self) -> ITAMP3POM_W<'_, CR3rs> {
        ITAMP3POM_W::new(self, 2)
    }
    ///Bit 3 - Internal tamper 4 potential mode
    #[inline(always)]
    pub fn itamp4pom(&mut self) -> ITAMP4POM_W<'_, CR3rs> {
        ITAMP4POM_W::new(self, 3)
    }
    ///Bit 4 - Internal tamper 5 potential mode
    #[inline(always)]
    pub fn itamp5pom(&mut self) -> ITAMP5POM_W<'_, CR3rs> {
        ITAMP5POM_W::new(self, 4)
    }
    ///Bit 5 - Internal tamper 6 potential mode
    #[inline(always)]
    pub fn itamp6pom(&mut self) -> ITAMP6POM_W<'_, CR3rs> {
        ITAMP6POM_W::new(self, 5)
    }
    ///Bit 6 - Internal tamper 7 potential mode
    #[inline(always)]
    pub fn itamp7pom(&mut self) -> ITAMP7POM_W<'_, CR3rs> {
        ITAMP7POM_W::new(self, 6)
    }
    ///Bit 7 - Internal tamper 8 potential mode
    #[inline(always)]
    pub fn itamp8pom(&mut self) -> ITAMP8POM_W<'_, CR3rs> {
        ITAMP8POM_W::new(self, 7)
    }
    ///Bit 8 - Internal tamper 9 potential mode
    #[inline(always)]
    pub fn itamp9pom(&mut self) -> ITAMP9POM_W<'_, CR3rs> {
        ITAMP9POM_W::new(self, 8)
    }
    ///Bit 10 - Internal tamper 11 potential mode
    #[inline(always)]
    pub fn itamp11pom(&mut self) -> ITAMP11POM_W<'_, CR3rs> {
        ITAMP11POM_W::new(self, 10)
    }
    ///Bit 11 - Internal tamper 12 potential mode
    #[inline(always)]
    pub fn itamp12pom(&mut self) -> ITAMP12POM_W<'_, CR3rs> {
        ITAMP12POM_W::new(self, 11)
    }
    ///Bit 12 - Internal tamper 13 potential mode
    #[inline(always)]
    pub fn itamp13pom(&mut self) -> ITAMP13POM_W<'_, CR3rs> {
        ITAMP13POM_W::new(self, 12)
    }
    ///Bit 14 - Internal tamper 15 potential mode
    #[inline(always)]
    pub fn itamp15pom(&mut self) -> ITAMP15POM_W<'_, CR3rs> {
        ITAMP15POM_W::new(self, 14)
    }
}
/**TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#TAMP:CR3)*/
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
