///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `EWIC` reader - Watchdog early interrupt acknowledge
pub type EWIC_R = crate::BitReader;
///Field `EWIC` writer - Watchdog early interrupt acknowledge
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - Watchdog early interrupt acknowledge
    #[inline(always)]
    pub fn ewic(&self) -> EWIC_R {
        EWIC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR").field("ewic", &self.ewic()).finish()
    }
}
impl W {
    ///Bit 15 - Watchdog early interrupt acknowledge
    #[inline(always)]
    pub fn ewic(&mut self) -> EWIC_W<'_, ICRrs> {
        EWIC_W::new(self, 15)
    }
}
/**IWDG interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#IWDG:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
