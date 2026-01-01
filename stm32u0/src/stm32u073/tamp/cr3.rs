///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
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
impl R {
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("itamp3pom", &self.itamp3pom())
            .field("itamp4pom", &self.itamp4pom())
            .field("itamp5pom", &self.itamp5pom())
            .field("itamp6pom", &self.itamp6pom())
            .finish()
    }
}
impl W {
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
}
/**TAMP control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TAMP:CR3)*/
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
