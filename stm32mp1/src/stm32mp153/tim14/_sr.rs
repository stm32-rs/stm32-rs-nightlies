///Register `_SR` reader
pub type R = crate::R<_SRrs>;
///Register `_SR` writer
pub type W = crate::W<_SRrs>;
///Field `UIF` reader - UIF
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - UIF
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - CC1IF
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - CC1IF
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - CC1OF
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - CC1OF
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UIF
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IF
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - CC1OF
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc1of", &self.cc1of())
            .finish()
    }
}
impl W {
    ///Bit 0 - UIF
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, _SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - CC1IF
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, _SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 9 - CC1OF
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, _SRrs> {
        CC1OF_W::new(self, 9)
    }
}
/**TIM14 status register

You can [`read`](crate::Reg::read) this register and get [`_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM14:_SR)*/
pub struct _SRrs;
impl crate::RegisterSpec for _SRrs {
    type Ux = u16;
}
///`read()` method returns [`_sr::R`](R) reader structure
impl crate::Readable for _SRrs {}
///`write(|w| ..)` method takes [`_sr::W`](W) writer structure
impl crate::Writable for _SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _SR to value 0
impl crate::Resettable for _SRrs {}
