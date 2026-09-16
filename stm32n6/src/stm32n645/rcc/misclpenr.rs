///Register `MISCLPENR` reader
pub type R = crate::R<MISCLPENRrs>;
///Register `MISCLPENR` writer
pub type W = crate::W<MISCLPENRrs>;
///Field `DBGLPEN` reader - DBG sleep enable
pub type DBGLPEN_R = crate::BitReader;
///Field `DBGLPEN` writer - DBG sleep enable
pub type DBGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPLPEN` reader - XSPIPHYCOMP sleep enable
pub type XSPIPHYCOMPLPEN_R = crate::BitReader;
///Field `XSPIPHYCOMPLPEN` writer - XSPIPHYCOMP sleep enable
pub type XSPIPHYCOMPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERLPEN` reader - PER sleep enable
pub type PERLPEN_R = crate::BitReader;
///Field `PERLPEN` writer - PER sleep enable
pub type PERLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG sleep enable
    #[inline(always)]
    pub fn dbglpen(&self) -> DBGLPEN_R {
        DBGLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - XSPIPHYCOMP sleep enable
    #[inline(always)]
    pub fn xspiphycomplpen(&self) -> XSPIPHYCOMPLPEN_R {
        XSPIPHYCOMPLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - PER sleep enable
    #[inline(always)]
    pub fn perlpen(&self) -> PERLPEN_R {
        PERLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCLPENR")
            .field("dbglpen", &self.dbglpen())
            .field("xspiphycomplpen", &self.xspiphycomplpen())
            .field("perlpen", &self.perlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG sleep enable
    #[inline(always)]
    pub fn dbglpen(&mut self) -> DBGLPEN_W<'_, MISCLPENRrs> {
        DBGLPEN_W::new(self, 0)
    }
    ///Bit 3 - XSPIPHYCOMP sleep enable
    #[inline(always)]
    pub fn xspiphycomplpen(&mut self) -> XSPIPHYCOMPLPEN_W<'_, MISCLPENRrs> {
        XSPIPHYCOMPLPEN_W::new(self, 3)
    }
    ///Bit 6 - PER sleep enable
    #[inline(always)]
    pub fn perlpen(&mut self) -> PERLPEN_W<'_, MISCLPENRrs> {
        PERLPEN_W::new(self, 6)
    }
}
/**RCC miscellaneous configurations Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`misclpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:MISCLPENR)*/
pub struct MISCLPENRrs;
impl crate::RegisterSpec for MISCLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`misclpenr::R`](R) reader structure
impl crate::Readable for MISCLPENRrs {}
///`write(|w| ..)` method takes [`misclpenr::W`](W) writer structure
impl crate::Writable for MISCLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCLPENR to value 0
impl crate::Resettable for MISCLPENRrs {}
