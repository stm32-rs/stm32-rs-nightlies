///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
///Field `EM32` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM32_R = crate::BitReader;
///Field `EM32` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM33` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM33_R = crate::BitReader;
///Field `EM33` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM34` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM34_R = crate::BitReader;
///Field `EM34` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM35` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM35_R = crate::BitReader;
///Field `EM35` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM36` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM36_R = crate::BitReader;
///Field `EM36` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM37` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM37_R = crate::BitReader;
///Field `EM37` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
pub type EM37_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("em32", &self.em32())
            .field("em33", &self.em33())
            .field("em34", &self.em34())
            .field("em35", &self.em35())
            .field("em36", &self.em36())
            .field("em37", &self.em37())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em32(&mut self) -> EM32_W<'_, EMR2rs> {
        EM32_W::new(self, 0)
    }
    ///Bit 1 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em33(&mut self) -> EM33_W<'_, EMR2rs> {
        EM33_W::new(self, 1)
    }
    ///Bit 2 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W<'_, EMR2rs> {
        EM34_W::new(self, 2)
    }
    ///Bit 3 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em35(&mut self) -> EM35_W<'_, EMR2rs> {
        EM35_W::new(self, 3)
    }
    ///Bit 4 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em36(&mut self) -> EM36_W<'_, EMR2rs> {
        EM36_W::new(self, 4)
    }
    ///Bit 5 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.
    #[inline(always)]
    pub fn em37(&mut self) -> EM37_W<'_, EMR2rs> {
        EM37_W::new(self, 5)
    }
}
/**EXTI CPU wake-up with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EMR2)*/
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`emr2::R`](R) reader structure
impl crate::Readable for EMR2rs {}
///`write(|w| ..)` method takes [`emr2::W`](W) writer structure
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR2 to value 0
impl crate::Resettable for EMR2rs {}
