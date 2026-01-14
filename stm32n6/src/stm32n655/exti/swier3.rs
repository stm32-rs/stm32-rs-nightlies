///Register `SWIER3` reader
pub type R = crate::R<SWIER3rs>;
///Register `SWIER3` writer
pub type W = crate::W<SWIER3rs>;
///Field `SWI66` reader - Software interrupt on event 66
pub type SWI66_R = crate::BitReader;
///Field `SWI66` writer - Software interrupt on event 66
pub type SWI66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI68` reader - Software interrupt on event x
pub type SWI68_R = crate::BitReader;
///Field `SWI68` writer - Software interrupt on event x
pub type SWI68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI69` reader - Software interrupt on event x
pub type SWI69_R = crate::BitReader;
///Field `SWI69` writer - Software interrupt on event x
pub type SWI69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI70` reader - Software interrupt on event x
pub type SWI70_R = crate::BitReader;
///Field `SWI70` writer - Software interrupt on event x
pub type SWI70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI71` reader - Software interrupt on event x
pub type SWI71_R = crate::BitReader;
///Field `SWI71` writer - Software interrupt on event x
pub type SWI71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI72` reader - Software interrupt on event x
pub type SWI72_R = crate::BitReader;
///Field `SWI72` writer - Software interrupt on event x
pub type SWI72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI73` reader - Software interrupt on event x
pub type SWI73_R = crate::BitReader;
///Field `SWI73` writer - Software interrupt on event x
pub type SWI73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI74` reader - Software interrupt on event x
pub type SWI74_R = crate::BitReader;
///Field `SWI74` writer - Software interrupt on event x
pub type SWI74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Software interrupt on event 66
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Software interrupt on event x
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software interrupt on event x
    #[inline(always)]
    pub fn swi69(&self) -> SWI69_R {
        SWI69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software interrupt on event x
    #[inline(always)]
    pub fn swi70(&self) -> SWI70_R {
        SWI70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi71(&self) -> SWI71_R {
        SWI71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi72(&self) -> SWI72_R {
        SWI72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software interrupt on event x
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software interrupt on event x
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER3")
            .field("swi66", &self.swi66())
            .field("swi68", &self.swi68())
            .field("swi69", &self.swi69())
            .field("swi70", &self.swi70())
            .field("swi71", &self.swi71())
            .field("swi72", &self.swi72())
            .field("swi73", &self.swi73())
            .field("swi74", &self.swi74())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software interrupt on event 66
    #[inline(always)]
    pub fn swi66(&mut self) -> SWI66_W<'_, SWIER3rs> {
        SWI66_W::new(self, 2)
    }
    ///Bit 4 - Software interrupt on event x
    #[inline(always)]
    pub fn swi68(&mut self) -> SWI68_W<'_, SWIER3rs> {
        SWI68_W::new(self, 4)
    }
    ///Bit 5 - Software interrupt on event x
    #[inline(always)]
    pub fn swi69(&mut self) -> SWI69_W<'_, SWIER3rs> {
        SWI69_W::new(self, 5)
    }
    ///Bit 6 - Software interrupt on event x
    #[inline(always)]
    pub fn swi70(&mut self) -> SWI70_W<'_, SWIER3rs> {
        SWI70_W::new(self, 6)
    }
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi71(&mut self) -> SWI71_W<'_, SWIER3rs> {
        SWI71_W::new(self, 7)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi72(&mut self) -> SWI72_W<'_, SWIER3rs> {
        SWI72_W::new(self, 8)
    }
    ///Bit 9 - Software interrupt on event x
    #[inline(always)]
    pub fn swi73(&mut self) -> SWI73_W<'_, SWIER3rs> {
        SWI73_W::new(self, 9)
    }
    ///Bit 10 - Software interrupt on event x
    #[inline(always)]
    pub fn swi74(&mut self) -> SWI74_W<'_, SWIER3rs> {
        SWI74_W::new(self, 10)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#EXTI:SWIER3)*/
pub struct SWIER3rs;
impl crate::RegisterSpec for SWIER3rs {
    type Ux = u32;
}
///`read()` method returns [`swier3::R`](R) reader structure
impl crate::Readable for SWIER3rs {}
///`write(|w| ..)` method takes [`swier3::W`](W) writer structure
impl crate::Writable for SWIER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER3 to value 0
impl crate::Resettable for SWIER3rs {}
