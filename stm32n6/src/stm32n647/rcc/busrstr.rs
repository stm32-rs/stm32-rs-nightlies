///Register `BUSRSTR` reader
pub type R = crate::R<BUSRSTRrs>;
///Register `BUSRSTR` writer
pub type W = crate::W<BUSRSTRrs>;
///Field `ACLKNRST` reader - ACLKN reset
pub type ACLKNRST_R = crate::BitReader;
///Field `ACLKNRST` writer - ACLKN reset
pub type ACLKNRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMRST` reader - AHBM reset
pub type AHBMRST_R = crate::BitReader;
///Field `AHBMRST` writer - AHBM reset
pub type AHBMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1RST` reader - AHB1 reset
pub type AHB1RST_R = crate::BitReader;
///Field `AHB1RST` writer - AHB1 reset
pub type AHB1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2RST` reader - AHB2 reset
pub type AHB2RST_R = crate::BitReader;
///Field `AHB2RST` writer - AHB2 reset
pub type AHB2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3RST` reader - AHB3 reset
pub type AHB3RST_R = crate::BitReader;
///Field `AHB3RST` writer - AHB3 reset
pub type AHB3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4RST` reader - AHB4 reset
pub type AHB4RST_R = crate::BitReader;
///Field `AHB4RST` writer - AHB4 reset
pub type AHB4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5RST` reader - AHB5 reset
pub type AHB5RST_R = crate::BitReader;
///Field `AHB5RST` writer - AHB5 reset
pub type AHB5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1RST` reader - APB1 reset
pub type APB1RST_R = crate::BitReader;
///Field `APB1RST` writer - APB1 reset
pub type APB1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2RST` reader - APB2 reset
pub type APB2RST_R = crate::BitReader;
///Field `APB2RST` writer - APB2 reset
pub type APB2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3RST` reader - APB3 reset
pub type APB3RST_R = crate::BitReader;
///Field `APB3RST` writer - APB3 reset
pub type APB3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4RST` reader - APB4 reset
pub type APB4RST_R = crate::BitReader;
///Field `APB4RST` writer - APB4 reset
pub type APB4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5RST` reader - APB5 reset
pub type APB5RST_R = crate::BitReader;
///Field `APB5RST` writer - APB5 reset
pub type APB5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCRST` reader - NOC reset
pub type NOCRST_R = crate::BitReader;
///Field `NOCRST` writer - NOC reset
pub type NOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ACLKN reset
    #[inline(always)]
    pub fn aclknrst(&self) -> ACLKNRST_R {
        ACLKNRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - AHBM reset
    #[inline(always)]
    pub fn ahbmrst(&self) -> AHBMRST_R {
        AHBMRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AHB1 reset
    #[inline(always)]
    pub fn ahb1rst(&self) -> AHB1RST_R {
        AHB1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB2 reset
    #[inline(always)]
    pub fn ahb2rst(&self) -> AHB2RST_R {
        AHB2RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHB3 reset
    #[inline(always)]
    pub fn ahb3rst(&self) -> AHB3RST_R {
        AHB3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHB4 reset
    #[inline(always)]
    pub fn ahb4rst(&self) -> AHB4RST_R {
        AHB4RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AHB5 reset
    #[inline(always)]
    pub fn ahb5rst(&self) -> AHB5RST_R {
        AHB5RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - APB1 reset
    #[inline(always)]
    pub fn apb1rst(&self) -> APB1RST_R {
        APB1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - APB2 reset
    #[inline(always)]
    pub fn apb2rst(&self) -> APB2RST_R {
        APB2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - APB3 reset
    #[inline(always)]
    pub fn apb3rst(&self) -> APB3RST_R {
        APB3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - APB4 reset
    #[inline(always)]
    pub fn apb4rst(&self) -> APB4RST_R {
        APB4RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - APB5 reset
    #[inline(always)]
    pub fn apb5rst(&self) -> APB5RST_R {
        APB5RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NOC reset
    #[inline(always)]
    pub fn nocrst(&self) -> NOCRST_R {
        NOCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSRSTR")
            .field("aclknrst", &self.aclknrst())
            .field("ahbmrst", &self.ahbmrst())
            .field("ahb1rst", &self.ahb1rst())
            .field("ahb2rst", &self.ahb2rst())
            .field("ahb3rst", &self.ahb3rst())
            .field("ahb4rst", &self.ahb4rst())
            .field("ahb5rst", &self.ahb5rst())
            .field("apb1rst", &self.apb1rst())
            .field("apb2rst", &self.apb2rst())
            .field("apb3rst", &self.apb3rst())
            .field("apb4rst", &self.apb4rst())
            .field("apb5rst", &self.apb5rst())
            .field("nocrst", &self.nocrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - ACLKN reset
    #[inline(always)]
    pub fn aclknrst(&mut self) -> ACLKNRST_W<'_, BUSRSTRrs> {
        ACLKNRST_W::new(self, 0)
    }
    ///Bit 2 - AHBM reset
    #[inline(always)]
    pub fn ahbmrst(&mut self) -> AHBMRST_W<'_, BUSRSTRrs> {
        AHBMRST_W::new(self, 2)
    }
    ///Bit 3 - AHB1 reset
    #[inline(always)]
    pub fn ahb1rst(&mut self) -> AHB1RST_W<'_, BUSRSTRrs> {
        AHB1RST_W::new(self, 3)
    }
    ///Bit 4 - AHB2 reset
    #[inline(always)]
    pub fn ahb2rst(&mut self) -> AHB2RST_W<'_, BUSRSTRrs> {
        AHB2RST_W::new(self, 4)
    }
    ///Bit 5 - AHB3 reset
    #[inline(always)]
    pub fn ahb3rst(&mut self) -> AHB3RST_W<'_, BUSRSTRrs> {
        AHB3RST_W::new(self, 5)
    }
    ///Bit 6 - AHB4 reset
    #[inline(always)]
    pub fn ahb4rst(&mut self) -> AHB4RST_W<'_, BUSRSTRrs> {
        AHB4RST_W::new(self, 6)
    }
    ///Bit 7 - AHB5 reset
    #[inline(always)]
    pub fn ahb5rst(&mut self) -> AHB5RST_W<'_, BUSRSTRrs> {
        AHB5RST_W::new(self, 7)
    }
    ///Bit 8 - APB1 reset
    #[inline(always)]
    pub fn apb1rst(&mut self) -> APB1RST_W<'_, BUSRSTRrs> {
        APB1RST_W::new(self, 8)
    }
    ///Bit 9 - APB2 reset
    #[inline(always)]
    pub fn apb2rst(&mut self) -> APB2RST_W<'_, BUSRSTRrs> {
        APB2RST_W::new(self, 9)
    }
    ///Bit 10 - APB3 reset
    #[inline(always)]
    pub fn apb3rst(&mut self) -> APB3RST_W<'_, BUSRSTRrs> {
        APB3RST_W::new(self, 10)
    }
    ///Bit 11 - APB4 reset
    #[inline(always)]
    pub fn apb4rst(&mut self) -> APB4RST_W<'_, BUSRSTRrs> {
        APB4RST_W::new(self, 11)
    }
    ///Bit 12 - APB5 reset
    #[inline(always)]
    pub fn apb5rst(&mut self) -> APB5RST_W<'_, BUSRSTRrs> {
        APB5RST_W::new(self, 12)
    }
    ///Bit 13 - NOC reset
    #[inline(always)]
    pub fn nocrst(&mut self) -> NOCRST_W<'_, BUSRSTRrs> {
        NOCRST_W::new(self, 13)
    }
}
/**RCC SoC buses reset register

You can [`read`](crate::Reg::read) this register and get [`busrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSRSTR)*/
pub struct BUSRSTRrs;
impl crate::RegisterSpec for BUSRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`busrstr::R`](R) reader structure
impl crate::Readable for BUSRSTRrs {}
///`write(|w| ..)` method takes [`busrstr::W`](W) writer structure
impl crate::Writable for BUSRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSRSTR to value 0
impl crate::Resettable for BUSRSTRrs {}
