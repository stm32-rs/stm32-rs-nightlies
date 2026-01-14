///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
///Field `IM32` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM32_R = crate::BitReader;
///Field `IM32` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM33` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM33_R = crate::BitReader;
///Field `IM33` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM34` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM34_R = crate::BitReader;
///Field `IM34` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM35` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM35_R = crate::BitReader;
///Field `IM35` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM36` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM36_R = crate::BitReader;
///Field `IM36` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IM37` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM37_R = crate::BitReader;
///Field `IM37` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im32", &self.im32())
            .field("im33", &self.im33())
            .field("im34", &self.im34())
            .field("im35", &self.im35())
            .field("im36", &self.im36())
            .field("im37", &self.im37())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im32(&mut self) -> IM32_W<'_, IMR2rs> {
        IM32_W::new(self, 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im33(&mut self) -> IM33_W<'_, IMR2rs> {
        IM33_W::new(self, 1)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<'_, IMR2rs> {
        IM34_W::new(self, 2)
    }
    ///Bit 3 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im35(&mut self) -> IM35_W<'_, IMR2rs> {
        IM35_W::new(self, 3)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<'_, IMR2rs> {
        IM36_W::new(self, 4)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<'_, IMR2rs> {
        IM37_W::new(self, 5)
    }
}
/**EXTI CPU wake-up with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#EXTI:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR2 to value 0xffff_ffff
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
