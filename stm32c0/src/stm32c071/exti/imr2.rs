///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
///Field `IM34` reader - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
pub type IM34_R = crate::BitReader;
///Field `IM34` writer - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM36` reader - CPU wake-up with interrupt mask on line 36
pub type IM36_R = crate::BitReader;
///Field `IM36` writer - CPU wake-up with interrupt mask on line 36
pub type IM36_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on line 36
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im34", &self.im34())
            .field("im36", &self.im36())
            .finish()
    }
}
impl W {
    ///Bit 2 - CPU wakeup with interrupt mask on line 34 Setting/clearing the bit unmasks/masks the CPU wakeup with interrupt request from the line 34.
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on line 36
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<IMR2rs> {
        IM36_W::new(self, 4)
    }
}
/**EXTI CPU wakeup with interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMR2 to value 0
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0;
}
