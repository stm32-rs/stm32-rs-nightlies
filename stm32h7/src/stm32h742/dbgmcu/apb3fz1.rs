///Register `APB3FZ1` reader
pub type R = crate::R<APB3FZ1rs>;
///Register `APB3FZ1` writer
pub type W = crate::W<APB3FZ1rs>;
///Field `WWDG1` reader - WWDG1 stop in debug
pub type WWDG1_R = crate::BitReader;
///Field `WWDG1` writer - WWDG1 stop in debug
pub type WWDG1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - WWDG1 stop in debug
    #[inline(always)]
    pub fn wwdg1(&self) -> WWDG1_R {
        WWDG1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3FZ1")
            .field("wwdg1", &self.wwdg1())
            .finish()
    }
}
impl W {
    ///Bit 6 - WWDG1 stop in debug
    #[inline(always)]
    pub fn wwdg1(&mut self) -> WWDG1_W<'_, APB3FZ1rs> {
        WWDG1_W::new(self, 6)
    }
}
/**DBGMCU APB3 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb3fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#DBGMCU:APB3FZ1)*/
pub struct APB3FZ1rs;
impl crate::RegisterSpec for APB3FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb3fz1::R`](R) reader structure
impl crate::Readable for APB3FZ1rs {}
///`write(|w| ..)` method takes [`apb3fz1::W`](W) writer structure
impl crate::Writable for APB3FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3FZ1 to value 0
impl crate::Resettable for APB3FZ1rs {}
