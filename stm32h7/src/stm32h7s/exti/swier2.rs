///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
///Field `SWIER34` reader - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER34_R = crate::BitReader;
///Field `SWIER34` writer - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER46` reader - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER46_R = crate::BitReader;
///Field `SWIER46` writer - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER49` reader - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER49_R = crate::BitReader;
///Field `SWIER49` writer - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER51` reader - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER51_R = crate::BitReader;
///Field `SWIER51` writer - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER54` reader - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER54_R = crate::BitReader;
///Field `SWIER54` writer - Software interrupt on line x+32 Always returns 0 when read.
pub type SWIER54_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier34(&self) -> SWIER34_R {
        SWIER34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier46(&self) -> SWIER46_R {
        SWIER46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier54(&self) -> SWIER54_R {
        SWIER54_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swier34", &self.swier34())
            .field("swier46", &self.swier46())
            .field("swier49", &self.swier49())
            .field("swier51", &self.swier51())
            .field("swier54", &self.swier54())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier34(&mut self) -> SWIER34_W<'_, SWIER2rs> {
        SWIER34_W::new(self, 2)
    }
    ///Bit 14 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier46(&mut self) -> SWIER46_W<'_, SWIER2rs> {
        SWIER46_W::new(self, 14)
    }
    ///Bit 17 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier49(&mut self) -> SWIER49_W<'_, SWIER2rs> {
        SWIER49_W::new(self, 17)
    }
    ///Bit 19 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier51(&mut self) -> SWIER51_W<'_, SWIER2rs> {
        SWIER51_W::new(self, 19)
    }
    ///Bit 22 - Software interrupt on line x+32 Always returns 0 when read.
    #[inline(always)]
    pub fn swier54(&mut self) -> SWIER54_W<'_, SWIER2rs> {
        SWIER54_W::new(self, 22)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {}
