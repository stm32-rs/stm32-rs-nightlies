///Register `MISCLPENCR` writer
pub type W = crate::W<MISCLPENCRrs>;
///Field `DBGLPENC` writer - DBG sleep enable
pub type DBGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHYCOMPLPENC` writer - XSPIPHYCOMP sleep enable
pub type XSPIPHYCOMPLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERLPENC` writer - PER sleep enable
pub type PERLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG sleep enable
    #[inline(always)]
    pub fn dbglpenc(&mut self) -> DBGLPENC_W<'_, MISCLPENCRrs> {
        DBGLPENC_W::new(self, 0)
    }
    ///Bit 3 - XSPIPHYCOMP sleep enable
    #[inline(always)]
    pub fn xspiphycomplpenc(&mut self) -> XSPIPHYCOMPLPENC_W<'_, MISCLPENCRrs> {
        XSPIPHYCOMPLPENC_W::new(self, 3)
    }
    ///Bit 6 - PER sleep enable
    #[inline(always)]
    pub fn perlpenc(&mut self) -> PERLPENC_W<'_, MISCLPENCRrs> {
        PERLPENC_W::new(self, 6)
    }
}
/**RCC miscellaneous Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MISCLPENCR)*/
pub struct MISCLPENCRrs;
impl crate::RegisterSpec for MISCLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`misclpencr::W`](W) writer structure
impl crate::Writable for MISCLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCLPENCR to value 0
impl crate::Resettable for MISCLPENCRrs {}
