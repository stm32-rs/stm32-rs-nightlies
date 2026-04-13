///Register `MEMLPENCR` writer
pub type W = crate::W<MEMLPENCRrs>;
///Field `AXISRAM3LPENC` writer - AXISRAM3 sleep enable
pub type AXISRAM3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4LPENC` writer - AXISRAM4 sleep enable
pub type AXISRAM4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5LPENC` writer - AXISRAM5 sleep enable
pub type AXISRAM5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6LPENC` writer - AXISRAM6 sleep enable
pub type AXISRAM6LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1LPENC` writer - AHBSRAM1 sleep enable
pub type AHBSRAM1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2LPENC` writer - AHBSRAM2 sleep enable
pub type AHBSRAM2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMLPENC` writer - BKPSRAM sleep enable
pub type BKPSRAMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1LPENC` writer - AXISRAM1 sleep enable
pub type AXISRAM1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2LPENC` writer - AXISRAM2 sleep enable
pub type AXISRAM2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMLPENC` writer - FLEXRAM sleep enable
pub type FLEXRAMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMLPENC` writer - NPUCACHERAM sleep enable
pub type NPUCACHERAMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMLPENC` writer - VENCRAM sleep enable
pub type VENCRAMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMLPENC` writer - BOOTROM sleep enable
pub type BOOTROMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 sleep enable
    #[inline(always)]
    pub fn axisram3lpenc(&mut self) -> AXISRAM3LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM3LPENC_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 sleep enable
    #[inline(always)]
    pub fn axisram4lpenc(&mut self) -> AXISRAM4LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM4LPENC_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 sleep enable
    #[inline(always)]
    pub fn axisram5lpenc(&mut self) -> AXISRAM5LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM5LPENC_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 sleep enable
    #[inline(always)]
    pub fn axisram6lpenc(&mut self) -> AXISRAM6LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM6LPENC_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 sleep enable
    #[inline(always)]
    pub fn ahbsram1lpenc(&mut self) -> AHBSRAM1LPENC_W<'_, MEMLPENCRrs> {
        AHBSRAM1LPENC_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 sleep enable
    #[inline(always)]
    pub fn ahbsram2lpenc(&mut self) -> AHBSRAM2LPENC_W<'_, MEMLPENCRrs> {
        AHBSRAM2LPENC_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM sleep enable
    #[inline(always)]
    pub fn bkpsramlpenc(&mut self) -> BKPSRAMLPENC_W<'_, MEMLPENCRrs> {
        BKPSRAMLPENC_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 sleep enable
    #[inline(always)]
    pub fn axisram1lpenc(&mut self) -> AXISRAM1LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM1LPENC_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 sleep enable
    #[inline(always)]
    pub fn axisram2lpenc(&mut self) -> AXISRAM2LPENC_W<'_, MEMLPENCRrs> {
        AXISRAM2LPENC_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM sleep enable
    #[inline(always)]
    pub fn flexramlpenc(&mut self) -> FLEXRAMLPENC_W<'_, MEMLPENCRrs> {
        FLEXRAMLPENC_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM sleep enable
    #[inline(always)]
    pub fn npucacheramlpenc(&mut self) -> NPUCACHERAMLPENC_W<'_, MEMLPENCRrs> {
        NPUCACHERAMLPENC_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM sleep enable
    #[inline(always)]
    pub fn vencramlpenc(&mut self) -> VENCRAMLPENC_W<'_, MEMLPENCRrs> {
        VENCRAMLPENC_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM sleep enable
    #[inline(always)]
    pub fn bootromlpenc(&mut self) -> BOOTROMLPENC_W<'_, MEMLPENCRrs> {
        BOOTROMLPENC_W::new(self, 12)
    }
}
/**RCC memory Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:MEMLPENCR)*/
pub struct MEMLPENCRrs;
impl crate::RegisterSpec for MEMLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memlpencr::W`](W) writer structure
impl crate::Writable for MEMLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMLPENCR to value 0
impl crate::Resettable for MEMLPENCRrs {}
