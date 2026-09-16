///Register `MEMLPENR` reader
pub type R = crate::R<MEMLPENRrs>;
///Register `MEMLPENR` writer
pub type W = crate::W<MEMLPENRrs>;
///Field `AXISRAM3LPEN` reader - AXISRAM3 sleep enable
pub type AXISRAM3LPEN_R = crate::BitReader;
///Field `AXISRAM3LPEN` writer - AXISRAM3 sleep enable
pub type AXISRAM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4LPEN` reader - AXISRAM4 sleep enable
pub type AXISRAM4LPEN_R = crate::BitReader;
///Field `AXISRAM4LPEN` writer - AXISRAM4 sleep enable
pub type AXISRAM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5LPEN` reader - AXISRAM5 sleep enable
pub type AXISRAM5LPEN_R = crate::BitReader;
///Field `AXISRAM5LPEN` writer - AXISRAM5 sleep enable
pub type AXISRAM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6LPEN` reader - AXISRAM6 sleep enable
pub type AXISRAM6LPEN_R = crate::BitReader;
///Field `AXISRAM6LPEN` writer - AXISRAM6 sleep enable
pub type AXISRAM6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1LPEN` reader - AHBSRAM1 sleep enable
pub type AHBSRAM1LPEN_R = crate::BitReader;
///Field `AHBSRAM1LPEN` writer - AHBSRAM1 sleep enable
pub type AHBSRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2LPEN` reader - AHBSRAM2 sleep enable
pub type AHBSRAM2LPEN_R = crate::BitReader;
///Field `AHBSRAM2LPEN` writer - AHBSRAM2 sleep enable
pub type AHBSRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMLPEN` reader - BKPSRAM sleep enable
pub type BKPSRAMLPEN_R = crate::BitReader;
///Field `BKPSRAMLPEN` writer - BKPSRAM sleep enable
pub type BKPSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1LPEN` reader - AXISRAM1 sleep enable
pub type AXISRAM1LPEN_R = crate::BitReader;
///Field `AXISRAM1LPEN` writer - AXISRAM1 sleep enable
pub type AXISRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2LPEN` reader - AXISRAM2 sleep enable
pub type AXISRAM2LPEN_R = crate::BitReader;
///Field `AXISRAM2LPEN` writer - AXISRAM2 sleep enable
pub type AXISRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMLPEN` reader - FLEXRAM sleep enable
pub type FLEXRAMLPEN_R = crate::BitReader;
///Field `FLEXRAMLPEN` writer - FLEXRAM sleep enable
pub type FLEXRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMLPEN` reader - NPUCACHERAM sleep enable
pub type NPUCACHERAMLPEN_R = crate::BitReader;
///Field `NPUCACHERAMLPEN` writer - NPUCACHERAM sleep enable
pub type NPUCACHERAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMLPEN` reader - VENCRAM sleep enable
pub type VENCRAMLPEN_R = crate::BitReader;
///Field `VENCRAMLPEN` writer - VENCRAM sleep enable
pub type VENCRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMLPEN` reader - BOOTROM sleep enable
pub type BOOTROMLPEN_R = crate::BitReader;
///Field `BOOTROMLPEN` writer - BOOTROM sleep enable
pub type BOOTROMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXISRAM3 sleep enable
    #[inline(always)]
    pub fn axisram3lpen(&self) -> AXISRAM3LPEN_R {
        AXISRAM3LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXISRAM4 sleep enable
    #[inline(always)]
    pub fn axisram4lpen(&self) -> AXISRAM4LPEN_R {
        AXISRAM4LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXISRAM5 sleep enable
    #[inline(always)]
    pub fn axisram5lpen(&self) -> AXISRAM5LPEN_R {
        AXISRAM5LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXISRAM6 sleep enable
    #[inline(always)]
    pub fn axisram6lpen(&self) -> AXISRAM6LPEN_R {
        AXISRAM6LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHBSRAM1 sleep enable
    #[inline(always)]
    pub fn ahbsram1lpen(&self) -> AHBSRAM1LPEN_R {
        AHBSRAM1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHBSRAM2 sleep enable
    #[inline(always)]
    pub fn ahbsram2lpen(&self) -> AHBSRAM2LPEN_R {
        AHBSRAM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BKPSRAM sleep enable
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXISRAM1 sleep enable
    #[inline(always)]
    pub fn axisram1lpen(&self) -> AXISRAM1LPEN_R {
        AXISRAM1LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXISRAM2 sleep enable
    #[inline(always)]
    pub fn axisram2lpen(&self) -> AXISRAM2LPEN_R {
        AXISRAM2LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FLEXRAM sleep enable
    #[inline(always)]
    pub fn flexramlpen(&self) -> FLEXRAMLPEN_R {
        FLEXRAMLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NPUCACHERAM sleep enable
    #[inline(always)]
    pub fn npucacheramlpen(&self) -> NPUCACHERAMLPEN_R {
        NPUCACHERAMLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - VENCRAM sleep enable
    #[inline(always)]
    pub fn vencramlpen(&self) -> VENCRAMLPEN_R {
        VENCRAMLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BOOTROM sleep enable
    #[inline(always)]
    pub fn bootromlpen(&self) -> BOOTROMLPEN_R {
        BOOTROMLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMLPENR")
            .field("axisram3lpen", &self.axisram3lpen())
            .field("axisram4lpen", &self.axisram4lpen())
            .field("axisram5lpen", &self.axisram5lpen())
            .field("axisram6lpen", &self.axisram6lpen())
            .field("ahbsram1lpen", &self.ahbsram1lpen())
            .field("ahbsram2lpen", &self.ahbsram2lpen())
            .field("bkpsramlpen", &self.bkpsramlpen())
            .field("axisram1lpen", &self.axisram1lpen())
            .field("axisram2lpen", &self.axisram2lpen())
            .field("flexramlpen", &self.flexramlpen())
            .field("npucacheramlpen", &self.npucacheramlpen())
            .field("vencramlpen", &self.vencramlpen())
            .field("bootromlpen", &self.bootromlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXISRAM3 sleep enable
    #[inline(always)]
    pub fn axisram3lpen(&mut self) -> AXISRAM3LPEN_W<'_, MEMLPENRrs> {
        AXISRAM3LPEN_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 sleep enable
    #[inline(always)]
    pub fn axisram4lpen(&mut self) -> AXISRAM4LPEN_W<'_, MEMLPENRrs> {
        AXISRAM4LPEN_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 sleep enable
    #[inline(always)]
    pub fn axisram5lpen(&mut self) -> AXISRAM5LPEN_W<'_, MEMLPENRrs> {
        AXISRAM5LPEN_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 sleep enable
    #[inline(always)]
    pub fn axisram6lpen(&mut self) -> AXISRAM6LPEN_W<'_, MEMLPENRrs> {
        AXISRAM6LPEN_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 sleep enable
    #[inline(always)]
    pub fn ahbsram1lpen(&mut self) -> AHBSRAM1LPEN_W<'_, MEMLPENRrs> {
        AHBSRAM1LPEN_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 sleep enable
    #[inline(always)]
    pub fn ahbsram2lpen(&mut self) -> AHBSRAM2LPEN_W<'_, MEMLPENRrs> {
        AHBSRAM2LPEN_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM sleep enable
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<'_, MEMLPENRrs> {
        BKPSRAMLPEN_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 sleep enable
    #[inline(always)]
    pub fn axisram1lpen(&mut self) -> AXISRAM1LPEN_W<'_, MEMLPENRrs> {
        AXISRAM1LPEN_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 sleep enable
    #[inline(always)]
    pub fn axisram2lpen(&mut self) -> AXISRAM2LPEN_W<'_, MEMLPENRrs> {
        AXISRAM2LPEN_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM sleep enable
    #[inline(always)]
    pub fn flexramlpen(&mut self) -> FLEXRAMLPEN_W<'_, MEMLPENRrs> {
        FLEXRAMLPEN_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM sleep enable
    #[inline(always)]
    pub fn npucacheramlpen(&mut self) -> NPUCACHERAMLPEN_W<'_, MEMLPENRrs> {
        NPUCACHERAMLPEN_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM sleep enable
    #[inline(always)]
    pub fn vencramlpen(&mut self) -> VENCRAMLPEN_W<'_, MEMLPENRrs> {
        VENCRAMLPEN_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM sleep enable
    #[inline(always)]
    pub fn bootromlpen(&mut self) -> BOOTROMLPEN_W<'_, MEMLPENRrs> {
        BOOTROMLPEN_W::new(self, 12)
    }
}
/**RCC memory Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`memlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMLPENR)*/
pub struct MEMLPENRrs;
impl crate::RegisterSpec for MEMLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`memlpenr::R`](R) reader structure
impl crate::Readable for MEMLPENRrs {}
///`write(|w| ..)` method takes [`memlpenr::W`](W) writer structure
impl crate::Writable for MEMLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMLPENR to value 0
impl crate::Resettable for MEMLPENRrs {}
