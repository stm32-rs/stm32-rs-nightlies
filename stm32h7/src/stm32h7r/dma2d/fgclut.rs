///Register `FGCLUT%s` reader
pub type R = crate::R<FGCLUTrs>;
///Register `FGCLUT%s` writer
pub type W = crate::W<FGCLUTrs>;
///Field `BLUE` reader - Blue Blue value for index 0 for the foreground
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue Blue value for index 0 for the foreground
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - Green Green value for index 0 for the foreground
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green Green value for index 0 for the foreground
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - Red Red value for index 0 for the foreground
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red Red value for index 0 for the foreground
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ALPHA` reader - Alpha Alpha value for index 0 for the foreground
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha Alpha value for index 0 for the foreground
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Blue Blue value for index 0 for the foreground
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green Green value for index 0 for the foreground
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red Red value for index 0 for the foreground
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Alpha Alpha value for index 0 for the foreground
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FGCLUT")
            .field("blue", &self.blue())
            .field("green", &self.green())
            .field("red", &self.red())
            .field("alpha", &self.alpha())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Blue Blue value for index 0 for the foreground
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<'_, FGCLUTrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green Green value for index 0 for the foreground
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<'_, FGCLUTrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red Red value for index 0 for the foreground
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<'_, FGCLUTrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha Alpha value for index 0 for the foreground
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<'_, FGCLUTrs> {
        ALPHA_W::new(self, 24)
    }
}
/**DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DMA2D:FGCLUT[0])*/
pub struct FGCLUTrs;
impl crate::RegisterSpec for FGCLUTrs {
    type Ux = u32;
}
///`read()` method returns [`fgclut::R`](R) reader structure
impl crate::Readable for FGCLUTrs {}
///`write(|w| ..)` method takes [`fgclut::W`](W) writer structure
impl crate::Writable for FGCLUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGCLUT%s to value 0
impl crate::Resettable for FGCLUTrs {}
