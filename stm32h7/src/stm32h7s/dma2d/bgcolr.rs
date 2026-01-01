///Register `BGCOLR` reader
pub type R = crate::R<BGCOLRrs>;
///Register `BGCOLR` writer
pub type W = crate::W<BGCOLRrs>;
///Field `BLUE` reader - Blue value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - Green value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - Red value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Blue value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGCOLR")
            .field("blue", &self.blue())
            .field("green", &self.green())
            .field("red", &self.red())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Blue value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<'_, BGCOLRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<'_, BGCOLRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red value for the A4 or A8 mode of the background Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG (FG fetch only with FG and BG PFC active).
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<'_, BGCOLRrs> {
        RED_W::new(self, 16)
    }
}
/**DMA2D background color register

You can [`read`](crate::Reg::read) this register and get [`bgcolr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DMA2D:BGCOLR)*/
pub struct BGCOLRrs;
impl crate::RegisterSpec for BGCOLRrs {
    type Ux = u32;
}
///`read()` method returns [`bgcolr::R`](R) reader structure
impl crate::Readable for BGCOLRrs {}
///`write(|w| ..)` method takes [`bgcolr::W`](W) writer structure
impl crate::Writable for BGCOLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGCOLR to value 0
impl crate::Resettable for BGCOLRrs {}
