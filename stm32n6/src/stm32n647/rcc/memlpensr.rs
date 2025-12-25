///Register `MEMLPENSR` writer
pub type W = crate::W<MEMLPENSRrs>;
///Field `AXISRAM3LPENS` writer - AXISRAM3 sleep enable
pub type AXISRAM3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4LPENS` writer - AXISRAM4 sleep enable
pub type AXISRAM4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5LPENS` writer - AXISRAM5 sleep enable
pub type AXISRAM5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6LPENS` writer - AXISRAM6 sleep enable
pub type AXISRAM6LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1LPENS` writer - AHBSRAM1 sleep enable
pub type AHBSRAM1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2LPENS` writer - AHBSRAM2 sleep enable
pub type AHBSRAM2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMLPENS` writer - BKPSRAM sleep enable
pub type BKPSRAMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1LPENS` writer - AXISRAM1 sleep enable
pub type AXISRAM1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2LPENS` writer - AXISRAM2 sleep enable
pub type AXISRAM2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMLPENS` writer - FLEXRAM sleep enable
pub type FLEXRAMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMLPENS` writer - NPUCACHERAM sleep enable
pub type NPUCACHERAMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMLPENS` writer - VENCRAM sleep enable
pub type VENCRAMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMLPENS` writer - BOOTROM sleep enable
pub type BOOTROMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 sleep enable
    #[inline(always)]
    pub fn axisram3lpens(&mut self) -> AXISRAM3LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM3LPENS_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 sleep enable
    #[inline(always)]
    pub fn axisram4lpens(&mut self) -> AXISRAM4LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM4LPENS_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 sleep enable
    #[inline(always)]
    pub fn axisram5lpens(&mut self) -> AXISRAM5LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM5LPENS_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 sleep enable
    #[inline(always)]
    pub fn axisram6lpens(&mut self) -> AXISRAM6LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM6LPENS_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 sleep enable
    #[inline(always)]
    pub fn ahbsram1lpens(&mut self) -> AHBSRAM1LPENS_W<'_, MEMLPENSRrs> {
        AHBSRAM1LPENS_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 sleep enable
    #[inline(always)]
    pub fn ahbsram2lpens(&mut self) -> AHBSRAM2LPENS_W<'_, MEMLPENSRrs> {
        AHBSRAM2LPENS_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM sleep enable
    #[inline(always)]
    pub fn bkpsramlpens(&mut self) -> BKPSRAMLPENS_W<'_, MEMLPENSRrs> {
        BKPSRAMLPENS_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 sleep enable
    #[inline(always)]
    pub fn axisram1lpens(&mut self) -> AXISRAM1LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM1LPENS_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 sleep enable
    #[inline(always)]
    pub fn axisram2lpens(&mut self) -> AXISRAM2LPENS_W<'_, MEMLPENSRrs> {
        AXISRAM2LPENS_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM sleep enable
    #[inline(always)]
    pub fn flexramlpens(&mut self) -> FLEXRAMLPENS_W<'_, MEMLPENSRrs> {
        FLEXRAMLPENS_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM sleep enable
    #[inline(always)]
    pub fn npucacheramlpens(&mut self) -> NPUCACHERAMLPENS_W<'_, MEMLPENSRrs> {
        NPUCACHERAMLPENS_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM sleep enable
    #[inline(always)]
    pub fn vencramlpens(&mut self) -> VENCRAMLPENS_W<'_, MEMLPENSRrs> {
        VENCRAMLPENS_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM sleep enable
    #[inline(always)]
    pub fn bootromlpens(&mut self) -> BOOTROMLPENS_W<'_, MEMLPENSRrs> {
        BOOTROMLPENS_W::new(self, 12)
    }
}
/**RCC memory sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMLPENSR)*/
pub struct MEMLPENSRrs;
impl crate::RegisterSpec for MEMLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memlpensr::W`](W) writer structure
impl crate::Writable for MEMLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMLPENSR to value 0
impl crate::Resettable for MEMLPENSRrs {}
