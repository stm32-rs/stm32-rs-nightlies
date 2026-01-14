///Register `P1YUVCR` reader
pub type R = crate::R<P1YUVCRrs>;
///Register `P1YUVCR` writer
pub type W = crate::W<P1YUVCRrs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPE` reader - Output samples type used while CLAMP is activated
pub type TYPE_R = crate::BitReader;
///Field `TYPE` writer - Output samples type used while CLAMP is activated
pub type TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLAMP` reader - Clamp the output samples
pub type CLAMP_R = crate::BitReader;
///Field `CLAMP` writer - Clamp the output samples
pub type CLAMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output samples type used while CLAMP is activated
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clamp the output samples
    #[inline(always)]
    pub fn clamp(&self) -> CLAMP_R {
        CLAMP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1YUVCR")
            .field("enable", &self.enable())
            .field("type_", &self.type_())
            .field("clamp", &self.clamp())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1YUVCRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - Output samples type used while CLAMP is activated
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<'_, P1YUVCRrs> {
        TYPE_W::new(self, 1)
    }
    ///Bit 2 - Clamp the output samples
    #[inline(always)]
    pub fn clamp(&mut self) -> CLAMP_W<'_, P1YUVCRrs> {
        CLAMP_W::new(self, 2)
    }
}
/**DCMIPP Pipe1 YUVConv configuration register

You can [`read`](crate::Reg::read) this register and get [`p1yuvcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1YUVCR)*/
pub struct P1YUVCRrs;
impl crate::RegisterSpec for P1YUVCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1yuvcr::R`](R) reader structure
impl crate::Readable for P1YUVCRrs {}
///`write(|w| ..)` method takes [`p1yuvcr::W`](W) writer structure
impl crate::Writable for P1YUVCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1YUVCR to value 0
impl crate::Resettable for P1YUVCRrs {}
