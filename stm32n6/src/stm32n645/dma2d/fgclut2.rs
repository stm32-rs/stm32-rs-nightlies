///Register `FGCLUT2` reader
pub type R = crate::R<FGCLUT2rs>;
///Register `FGCLUT2` writer
pub type W = crate::W<FGCLUT2rs>;
///Field `BLUE` reader - Blue
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - Green
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - Red
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ALPHA` reader - Alpha
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Blue
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Alpha
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FGCLUT2")
            .field("blue", &self.blue())
            .field("green", &self.green())
            .field("red", &self.red())
            .field("alpha", &self.alpha())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Blue
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<'_, FGCLUT2rs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<'_, FGCLUT2rs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<'_, FGCLUT2rs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<'_, FGCLUT2rs> {
        ALPHA_W::new(self, 24)
    }
}
/**DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DMA2D:FGCLUT2)*/
pub struct FGCLUT2rs;
impl crate::RegisterSpec for FGCLUT2rs {
    type Ux = u32;
}
///`read()` method returns [`fgclut2::R`](R) reader structure
impl crate::Readable for FGCLUT2rs {}
///`write(|w| ..)` method takes [`fgclut2::W`](W) writer structure
impl crate::Writable for FGCLUT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGCLUT2 to value 0
impl crate::Resettable for FGCLUT2rs {}
