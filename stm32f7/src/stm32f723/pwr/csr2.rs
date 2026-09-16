///Register `CSR2` reader
pub type R = crate::R<CSR2rs>;
///Register `CSR2` writer
pub type W = crate::W<CSR2rs>;
///Field `WUPF1` reader - Wakeup Pin flag for PA0
pub type WUPF1_R = crate::BitReader;
///Field `WUPF2` reader - Wakeup Pin flag for PA2
pub type WUPF2_R = crate::BitReader;
///Field `WUPF3` reader - Wakeup Pin flag for PC1
pub type WUPF3_R = crate::BitReader;
///Field `WUPF4` reader - Wakeup Pin flag for PC13
pub type WUPF4_R = crate::BitReader;
///Field `WUPF5` reader - Wakeup Pin flag for PI8
pub type WUPF5_R = crate::BitReader;
///Field `WUPF6` reader - Wakeup Pin flag for PI11
pub type WUPF6_R = crate::BitReader;
///Field `EWUP1` reader - Enable Wakeup pin for PA0
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable Wakeup pin for PA0
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable Wakeup pin for PA2
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable Wakeup pin for PA2
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP3` reader - Enable Wakeup pin for PC1
pub type EWUP3_R = crate::BitReader;
///Field `EWUP3` writer - Enable Wakeup pin for PC1
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable Wakeup pin for PC13
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable Wakeup pin for PC13
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable Wakeup pin for PI8
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable Wakeup pin for PI8
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP6` reader - Enable Wakeup pin for PI11
pub type EWUP6_R = crate::BitReader;
///Field `EWUP6` writer - Enable Wakeup pin for PI11
pub type EWUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wakeup Pin flag for PA0
    #[inline(always)]
    pub fn wupf1(&self) -> WUPF1_R {
        WUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup Pin flag for PA2
    #[inline(always)]
    pub fn wupf2(&self) -> WUPF2_R {
        WUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup Pin flag for PC1
    #[inline(always)]
    pub fn wupf3(&self) -> WUPF3_R {
        WUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup Pin flag for PC13
    #[inline(always)]
    pub fn wupf4(&self) -> WUPF4_R {
        WUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup Pin flag for PI8
    #[inline(always)]
    pub fn wupf5(&self) -> WUPF5_R {
        WUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup Pin flag for PI11
    #[inline(always)]
    pub fn wupf6(&self) -> WUPF6_R {
        WUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Enable Wakeup pin for PA0
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable Wakeup pin for PA2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable Wakeup pin for PC1
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable Wakeup pin for PC13
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable Wakeup pin for PI8
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable Wakeup pin for PI11
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR2")
            .field("wupf1", &self.wupf1())
            .field("wupf2", &self.wupf2())
            .field("wupf3", &self.wupf3())
            .field("wupf4", &self.wupf4())
            .field("wupf5", &self.wupf5())
            .field("wupf6", &self.wupf6())
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("ewup6", &self.ewup6())
            .finish()
    }
}
impl W {
    ///Bit 8 - Enable Wakeup pin for PA0
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CSR2rs> {
        EWUP1_W::new(self, 8)
    }
    ///Bit 9 - Enable Wakeup pin for PA2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CSR2rs> {
        EWUP2_W::new(self, 9)
    }
    ///Bit 10 - Enable Wakeup pin for PC1
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CSR2rs> {
        EWUP3_W::new(self, 10)
    }
    ///Bit 11 - Enable Wakeup pin for PC13
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, CSR2rs> {
        EWUP4_W::new(self, 11)
    }
    ///Bit 12 - Enable Wakeup pin for PI8
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, CSR2rs> {
        EWUP5_W::new(self, 12)
    }
    ///Bit 13 - Enable Wakeup pin for PI11
    #[inline(always)]
    pub fn ewup6(&mut self) -> EWUP6_W<'_, CSR2rs> {
        EWUP6_W::new(self, 13)
    }
}
/**power control/status register

You can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#PWR:CSR2)*/
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`csr2::R`](R) reader structure
impl crate::Readable for CSR2rs {}
///`write(|w| ..)` method takes [`csr2::W`](W) writer structure
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR2 to value 0
impl crate::Resettable for CSR2rs {}
