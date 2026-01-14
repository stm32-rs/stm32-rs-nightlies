///Register `ASCR` reader
pub type R = crate::R<ASCRrs>;
///Register `ASCR` writer
pub type W = crate::W<ASCRrs>;
///Field `ASC0` reader - Port analog switch control
pub type ASC0_R = crate::BitReader;
///Field `ASC0` writer - Port analog switch control
pub type ASC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC1` reader - Port analog switch control
pub type ASC1_R = crate::BitReader;
///Field `ASC1` writer - Port analog switch control
pub type ASC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC2` reader - Port analog switch control
pub type ASC2_R = crate::BitReader;
///Field `ASC2` writer - Port analog switch control
pub type ASC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC3` reader - Port analog switch control
pub type ASC3_R = crate::BitReader;
///Field `ASC3` writer - Port analog switch control
pub type ASC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC4` reader - Port analog switch control
pub type ASC4_R = crate::BitReader;
///Field `ASC4` writer - Port analog switch control
pub type ASC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC5` reader - Port analog switch control
pub type ASC5_R = crate::BitReader;
///Field `ASC5` writer - Port analog switch control
pub type ASC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC6` reader - Port analog switch control
pub type ASC6_R = crate::BitReader;
///Field `ASC6` writer - Port analog switch control
pub type ASC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC7` reader - Port analog switch control
pub type ASC7_R = crate::BitReader;
///Field `ASC7` writer - Port analog switch control
pub type ASC7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC8` reader - Port analog switch control
pub type ASC8_R = crate::BitReader;
///Field `ASC8` writer - Port analog switch control
pub type ASC8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC9` reader - Port analog switch control
pub type ASC9_R = crate::BitReader;
///Field `ASC9` writer - Port analog switch control
pub type ASC9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC10` reader - Port analog switch control
pub type ASC10_R = crate::BitReader;
///Field `ASC10` writer - Port analog switch control
pub type ASC10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC11` reader - Port analog switch control
pub type ASC11_R = crate::BitReader;
///Field `ASC11` writer - Port analog switch control
pub type ASC11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC12` reader - Port analog switch control
pub type ASC12_R = crate::BitReader;
///Field `ASC12` writer - Port analog switch control
pub type ASC12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC13` reader - Port analog switch control
pub type ASC13_R = crate::BitReader;
///Field `ASC13` writer - Port analog switch control
pub type ASC13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC14` reader - Port analog switch control
pub type ASC14_R = crate::BitReader;
///Field `ASC14` writer - Port analog switch control
pub type ASC14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASC15` reader - Port analog switch control
pub type ASC15_R = crate::BitReader;
///Field `ASC15` writer - Port analog switch control
pub type ASC15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port analog switch control
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port analog switch control
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port analog switch control
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port analog switch control
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port analog switch control
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port analog switch control
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port analog switch control
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port analog switch control
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port analog switch control
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port analog switch control
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port analog switch control
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port analog switch control
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port analog switch control
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port analog switch control
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port analog switch control
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port analog switch control
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASCR")
            .field("asc0", &self.asc0())
            .field("asc1", &self.asc1())
            .field("asc2", &self.asc2())
            .field("asc3", &self.asc3())
            .field("asc4", &self.asc4())
            .field("asc5", &self.asc5())
            .field("asc6", &self.asc6())
            .field("asc7", &self.asc7())
            .field("asc8", &self.asc8())
            .field("asc9", &self.asc9())
            .field("asc10", &self.asc10())
            .field("asc11", &self.asc11())
            .field("asc12", &self.asc12())
            .field("asc13", &self.asc13())
            .field("asc14", &self.asc14())
            .field("asc15", &self.asc15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port analog switch control
    #[inline(always)]
    pub fn asc0(&mut self) -> ASC0_W<'_, ASCRrs> {
        ASC0_W::new(self, 0)
    }
    ///Bit 1 - Port analog switch control
    #[inline(always)]
    pub fn asc1(&mut self) -> ASC1_W<'_, ASCRrs> {
        ASC1_W::new(self, 1)
    }
    ///Bit 2 - Port analog switch control
    #[inline(always)]
    pub fn asc2(&mut self) -> ASC2_W<'_, ASCRrs> {
        ASC2_W::new(self, 2)
    }
    ///Bit 3 - Port analog switch control
    #[inline(always)]
    pub fn asc3(&mut self) -> ASC3_W<'_, ASCRrs> {
        ASC3_W::new(self, 3)
    }
    ///Bit 4 - Port analog switch control
    #[inline(always)]
    pub fn asc4(&mut self) -> ASC4_W<'_, ASCRrs> {
        ASC4_W::new(self, 4)
    }
    ///Bit 5 - Port analog switch control
    #[inline(always)]
    pub fn asc5(&mut self) -> ASC5_W<'_, ASCRrs> {
        ASC5_W::new(self, 5)
    }
    ///Bit 6 - Port analog switch control
    #[inline(always)]
    pub fn asc6(&mut self) -> ASC6_W<'_, ASCRrs> {
        ASC6_W::new(self, 6)
    }
    ///Bit 7 - Port analog switch control
    #[inline(always)]
    pub fn asc7(&mut self) -> ASC7_W<'_, ASCRrs> {
        ASC7_W::new(self, 7)
    }
    ///Bit 8 - Port analog switch control
    #[inline(always)]
    pub fn asc8(&mut self) -> ASC8_W<'_, ASCRrs> {
        ASC8_W::new(self, 8)
    }
    ///Bit 9 - Port analog switch control
    #[inline(always)]
    pub fn asc9(&mut self) -> ASC9_W<'_, ASCRrs> {
        ASC9_W::new(self, 9)
    }
    ///Bit 10 - Port analog switch control
    #[inline(always)]
    pub fn asc10(&mut self) -> ASC10_W<'_, ASCRrs> {
        ASC10_W::new(self, 10)
    }
    ///Bit 11 - Port analog switch control
    #[inline(always)]
    pub fn asc11(&mut self) -> ASC11_W<'_, ASCRrs> {
        ASC11_W::new(self, 11)
    }
    ///Bit 12 - Port analog switch control
    #[inline(always)]
    pub fn asc12(&mut self) -> ASC12_W<'_, ASCRrs> {
        ASC12_W::new(self, 12)
    }
    ///Bit 13 - Port analog switch control
    #[inline(always)]
    pub fn asc13(&mut self) -> ASC13_W<'_, ASCRrs> {
        ASC13_W::new(self, 13)
    }
    ///Bit 14 - Port analog switch control
    #[inline(always)]
    pub fn asc14(&mut self) -> ASC14_W<'_, ASCRrs> {
        ASC14_W::new(self, 14)
    }
    ///Bit 15 - Port analog switch control
    #[inline(always)]
    pub fn asc15(&mut self) -> ASC15_W<'_, ASCRrs> {
        ASC15_W::new(self, 15)
    }
}
/**GPIO port analog switch control register

You can [`read`](crate::Reg::read) this register and get [`ascr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#GPIOA:ASCR)*/
pub struct ASCRrs;
impl crate::RegisterSpec for ASCRrs {
    type Ux = u32;
}
///`read()` method returns [`ascr::R`](R) reader structure
impl crate::Readable for ASCRrs {}
///`write(|w| ..)` method takes [`ascr::W`](W) writer structure
impl crate::Writable for ASCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASCR to value 0
impl crate::Resettable for ASCRrs {}
