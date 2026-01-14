///Register `P1EXCR2` reader
pub type R = crate::R<P1EXCR2rs>;
///Register `P1EXCR2` writer
pub type W = crate::W<P1EXCR2rs>;
///Field `MULTB` reader - Exposure multiplier - Blue
pub type MULTB_R = crate::FieldReader;
///Field `MULTB` writer - Exposure multiplier - Blue
pub type MULTB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SHFB` reader - Exposure shift - Blue
pub type SHFB_R = crate::FieldReader;
///Field `SHFB` writer - Exposure shift - Blue
pub type SHFB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MULTG` reader - Exposure multiplier - Green
pub type MULTG_R = crate::FieldReader;
///Field `MULTG` writer - Exposure multiplier - Green
pub type MULTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SHFG` reader - Exposure shift - Green
pub type SHFG_R = crate::FieldReader;
///Field `SHFG` writer - Exposure shift - Green
pub type SHFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 4:11 - Exposure multiplier - Blue
    #[inline(always)]
    pub fn multb(&self) -> MULTB_R {
        MULTB_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bits 12:14 - Exposure shift - Blue
    #[inline(always)]
    pub fn shfb(&self) -> SHFB_R {
        SHFB_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 20:27 - Exposure multiplier - Green
    #[inline(always)]
    pub fn multg(&self) -> MULTG_R {
        MULTG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bits 28:30 - Exposure shift - Green
    #[inline(always)]
    pub fn shfg(&self) -> SHFG_R {
        SHFG_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1EXCR2")
            .field("multb", &self.multb())
            .field("shfb", &self.shfb())
            .field("multg", &self.multg())
            .field("shfg", &self.shfg())
            .finish()
    }
}
impl W {
    ///Bits 4:11 - Exposure multiplier - Blue
    #[inline(always)]
    pub fn multb(&mut self) -> MULTB_W<'_, P1EXCR2rs> {
        MULTB_W::new(self, 4)
    }
    ///Bits 12:14 - Exposure shift - Blue
    #[inline(always)]
    pub fn shfb(&mut self) -> SHFB_W<'_, P1EXCR2rs> {
        SHFB_W::new(self, 12)
    }
    ///Bits 20:27 - Exposure multiplier - Green
    #[inline(always)]
    pub fn multg(&mut self) -> MULTG_W<'_, P1EXCR2rs> {
        MULTG_W::new(self, 20)
    }
    ///Bits 28:30 - Exposure shift - Green
    #[inline(always)]
    pub fn shfg(&mut self) -> SHFG_W<'_, P1EXCR2rs> {
        SHFG_W::new(self, 28)
    }
}
/**DCMIPP Pipe1 exposure control register 2

You can [`read`](crate::Reg::read) this register and get [`p1excr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1excr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1EXCR2)*/
pub struct P1EXCR2rs;
impl crate::RegisterSpec for P1EXCR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1excr2::R`](R) reader structure
impl crate::Readable for P1EXCR2rs {}
///`write(|w| ..)` method takes [`p1excr2::W`](W) writer structure
impl crate::Writable for P1EXCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1EXCR2 to value 0
impl crate::Resettable for P1EXCR2rs {}
