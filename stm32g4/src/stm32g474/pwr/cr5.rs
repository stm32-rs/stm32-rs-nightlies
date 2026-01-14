///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
///Field `R1MODE` reader - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to
pub type R1MODE_R = crate::BitReader;
///Field `R1MODE` writer - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to
pub type R1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR5")
            .field("r1mode", &self.r1mode())
            .finish()
    }
}
impl W {
    ///Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to
    #[inline(always)]
    pub fn r1mode(&mut self) -> R1MODE_W<'_, CR5rs> {
        R1MODE_W::new(self, 8)
    }
}
/**Power control register

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#PWR:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0x0100
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x0100;
}
