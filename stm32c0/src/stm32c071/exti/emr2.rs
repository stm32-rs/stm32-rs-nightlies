///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
///Field `EM34` reader - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
pub type EM34_R = crate::BitReader;
///Field `EM34` writer - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
pub type EM34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM36` reader - CPU wake-up with event generation mask on line 36
pub type EM36_R = crate::BitReader;
///Field `EM36` writer - CPU wake-up with event generation mask on line 36
pub type EM36_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with event generation mask on line 36
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("em34", &self.em34())
            .field("em36", &self.em36())
            .finish()
    }
}
impl W {
    ///Bit 2 - CPU wakeup with event generation mask on line 34 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the line 34.
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W<EMR2rs> {
        EM34_W::new(self, 2)
    }
    ///Bit 4 - CPU wake-up with event generation mask on line 36
    #[inline(always)]
    pub fn em36(&mut self) -> EM36_W<EMR2rs> {
        EM36_W::new(self, 4)
    }
}
/**EXTI CPU wakeup with event mask register 2

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:EMR2)*/
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`emr2::R`](R) reader structure
impl crate::Readable for EMR2rs {}
///`write(|w| ..)` method takes [`emr2::W`](W) writer structure
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EMR2 to value 0
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0;
}
