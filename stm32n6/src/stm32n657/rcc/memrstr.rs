///Register `MEMRSTR` reader
pub type R = crate::R<MEMRSTRrs>;
///Register `MEMRSTR` writer
pub type W = crate::W<MEMRSTRrs>;
///Field `AXISRAM3RST` reader - AXISRAM3 reset
pub type AXISRAM3RST_R = crate::BitReader;
///Field `AXISRAM3RST` writer - AXISRAM3 reset
pub type AXISRAM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4RST` reader - AXISRAM4reset
pub type AXISRAM4RST_R = crate::BitReader;
///Field `AXISRAM4RST` writer - AXISRAM4reset
pub type AXISRAM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5RST` reader - AXISRAM5 reset
pub type AXISRAM5RST_R = crate::BitReader;
///Field `AXISRAM5RST` writer - AXISRAM5 reset
pub type AXISRAM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6RST` reader - AXISRAM6 reset
pub type AXISRAM6RST_R = crate::BitReader;
///Field `AXISRAM6RST` writer - AXISRAM6 reset
pub type AXISRAM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1RST` reader - AHBSRAM1 reset
pub type AHBSRAM1RST_R = crate::BitReader;
///Field `AHBSRAM1RST` writer - AHBSRAM1 reset
pub type AHBSRAM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2RST` reader - AHBSRAM2 reset
pub type AHBSRAM2RST_R = crate::BitReader;
///Field `AHBSRAM2RST` writer - AHBSRAM2 reset
pub type AHBSRAM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1RST` reader - AXISRAM1 reset
pub type AXISRAM1RST_R = crate::BitReader;
///Field `AXISRAM1RST` writer - AXISRAM1 reset
pub type AXISRAM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2RST` reader - AXISRAM2 reset
pub type AXISRAM2RST_R = crate::BitReader;
///Field `AXISRAM2RST` writer - AXISRAM2 reset
pub type AXISRAM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMRST` reader - FLEXRAM reset
pub type FLEXRAMRST_R = crate::BitReader;
///Field `FLEXRAMRST` writer - FLEXRAM reset
pub type FLEXRAMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMRST` reader - NPUCACHERAM reset
pub type NPUCACHERAMRST_R = crate::BitReader;
///Field `NPUCACHERAMRST` writer - NPUCACHERAM reset
pub type NPUCACHERAMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMRST` reader - VENCRAM reset
pub type VENCRAMRST_R = crate::BitReader;
///Field `VENCRAMRST` writer - VENCRAM reset
pub type VENCRAMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMRST` reader - BOOTROM reset
pub type BOOTROMRST_R = crate::BitReader;
///Field `BOOTROMRST` writer - BOOTROM reset
pub type BOOTROMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXISRAM3 reset
    #[inline(always)]
    pub fn axisram3rst(&self) -> AXISRAM3RST_R {
        AXISRAM3RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXISRAM4reset
    #[inline(always)]
    pub fn axisram4rst(&self) -> AXISRAM4RST_R {
        AXISRAM4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXISRAM5 reset
    #[inline(always)]
    pub fn axisram5rst(&self) -> AXISRAM5RST_R {
        AXISRAM5RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXISRAM6 reset
    #[inline(always)]
    pub fn axisram6rst(&self) -> AXISRAM6RST_R {
        AXISRAM6RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHBSRAM1 reset
    #[inline(always)]
    pub fn ahbsram1rst(&self) -> AHBSRAM1RST_R {
        AHBSRAM1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHBSRAM2 reset
    #[inline(always)]
    pub fn ahbsram2rst(&self) -> AHBSRAM2RST_R {
        AHBSRAM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - AXISRAM1 reset
    #[inline(always)]
    pub fn axisram1rst(&self) -> AXISRAM1RST_R {
        AXISRAM1RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXISRAM2 reset
    #[inline(always)]
    pub fn axisram2rst(&self) -> AXISRAM2RST_R {
        AXISRAM2RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FLEXRAM reset
    #[inline(always)]
    pub fn flexramrst(&self) -> FLEXRAMRST_R {
        FLEXRAMRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NPUCACHERAM reset
    #[inline(always)]
    pub fn npucacheramrst(&self) -> NPUCACHERAMRST_R {
        NPUCACHERAMRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - VENCRAM reset
    #[inline(always)]
    pub fn vencramrst(&self) -> VENCRAMRST_R {
        VENCRAMRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BOOTROM reset
    #[inline(always)]
    pub fn bootromrst(&self) -> BOOTROMRST_R {
        BOOTROMRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRSTR")
            .field("axisram3rst", &self.axisram3rst())
            .field("axisram4rst", &self.axisram4rst())
            .field("axisram5rst", &self.axisram5rst())
            .field("axisram6rst", &self.axisram6rst())
            .field("ahbsram1rst", &self.ahbsram1rst())
            .field("ahbsram2rst", &self.ahbsram2rst())
            .field("axisram1rst", &self.axisram1rst())
            .field("axisram2rst", &self.axisram2rst())
            .field("flexramrst", &self.flexramrst())
            .field("npucacheramrst", &self.npucacheramrst())
            .field("vencramrst", &self.vencramrst())
            .field("bootromrst", &self.bootromrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXISRAM3 reset
    #[inline(always)]
    pub fn axisram3rst(&mut self) -> AXISRAM3RST_W<'_, MEMRSTRrs> {
        AXISRAM3RST_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4reset
    #[inline(always)]
    pub fn axisram4rst(&mut self) -> AXISRAM4RST_W<'_, MEMRSTRrs> {
        AXISRAM4RST_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 reset
    #[inline(always)]
    pub fn axisram5rst(&mut self) -> AXISRAM5RST_W<'_, MEMRSTRrs> {
        AXISRAM5RST_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 reset
    #[inline(always)]
    pub fn axisram6rst(&mut self) -> AXISRAM6RST_W<'_, MEMRSTRrs> {
        AXISRAM6RST_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 reset
    #[inline(always)]
    pub fn ahbsram1rst(&mut self) -> AHBSRAM1RST_W<'_, MEMRSTRrs> {
        AHBSRAM1RST_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 reset
    #[inline(always)]
    pub fn ahbsram2rst(&mut self) -> AHBSRAM2RST_W<'_, MEMRSTRrs> {
        AHBSRAM2RST_W::new(self, 5)
    }
    ///Bit 7 - AXISRAM1 reset
    #[inline(always)]
    pub fn axisram1rst(&mut self) -> AXISRAM1RST_W<'_, MEMRSTRrs> {
        AXISRAM1RST_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 reset
    #[inline(always)]
    pub fn axisram2rst(&mut self) -> AXISRAM2RST_W<'_, MEMRSTRrs> {
        AXISRAM2RST_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM reset
    #[inline(always)]
    pub fn flexramrst(&mut self) -> FLEXRAMRST_W<'_, MEMRSTRrs> {
        FLEXRAMRST_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM reset
    #[inline(always)]
    pub fn npucacheramrst(&mut self) -> NPUCACHERAMRST_W<'_, MEMRSTRrs> {
        NPUCACHERAMRST_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM reset
    #[inline(always)]
    pub fn vencramrst(&mut self) -> VENCRAMRST_W<'_, MEMRSTRrs> {
        VENCRAMRST_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM reset
    #[inline(always)]
    pub fn bootromrst(&mut self) -> BOOTROMRST_W<'_, MEMRSTRrs> {
        BOOTROMRST_W::new(self, 12)
    }
}
/**RCC memories reset register

You can [`read`](crate::Reg::read) this register and get [`memrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMRSTR)*/
pub struct MEMRSTRrs;
impl crate::RegisterSpec for MEMRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`memrstr::R`](R) reader structure
impl crate::Readable for MEMRSTRrs {}
///`write(|w| ..)` method takes [`memrstr::W`](W) writer structure
impl crate::Writable for MEMRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRSTR to value 0
impl crate::Resettable for MEMRSTRrs {}
