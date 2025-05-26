///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV0` reader - I/O pin y of Port x privilege configuration
pub type PRIV0_R = crate::BitReader;
///Field `PRIV0` writer - I/O pin y of Port x privilege configuration
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV1` reader - I/O pin y of Port x privilege configuration
pub type PRIV1_R = crate::BitReader;
///Field `PRIV1` writer - I/O pin y of Port x privilege configuration
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV2` reader - I/O pin y of Port x privilege configuration
pub type PRIV2_R = crate::BitReader;
///Field `PRIV2` writer - I/O pin y of Port x privilege configuration
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV3` reader - I/O pin y of Port x privilege configuration
pub type PRIV3_R = crate::BitReader;
///Field `PRIV3` writer - I/O pin y of Port x privilege configuration
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV4` reader - I/O pin y of Port x privilege configuration
pub type PRIV4_R = crate::BitReader;
///Field `PRIV4` writer - I/O pin y of Port x privilege configuration
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV5` reader - I/O pin y of Port x privilege configuration
pub type PRIV5_R = crate::BitReader;
///Field `PRIV5` writer - I/O pin y of Port x privilege configuration
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV6` reader - I/O pin y of Port x privilege configuration
pub type PRIV6_R = crate::BitReader;
///Field `PRIV6` writer - I/O pin y of Port x privilege configuration
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV7` reader - I/O pin y of Port x privilege configuration
pub type PRIV7_R = crate::BitReader;
///Field `PRIV7` writer - I/O pin y of Port x privilege configuration
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV8` reader - I/O pin y of Port x privilege configuration
pub type PRIV8_R = crate::BitReader;
///Field `PRIV8` writer - I/O pin y of Port x privilege configuration
pub type PRIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV9` reader - I/O pin y of Port x privilege configuration
pub type PRIV9_R = crate::BitReader;
///Field `PRIV9` writer - I/O pin y of Port x privilege configuration
pub type PRIV9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV10` reader - I/O pin y of Port x privilege configuration
pub type PRIV10_R = crate::BitReader;
///Field `PRIV10` writer - I/O pin y of Port x privilege configuration
pub type PRIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV11` reader - I/O pin y of Port x privilege configuration
pub type PRIV11_R = crate::BitReader;
///Field `PRIV11` writer - I/O pin y of Port x privilege configuration
pub type PRIV11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV12` reader - I/O pin y of Port x privilege configuration
pub type PRIV12_R = crate::BitReader;
///Field `PRIV12` writer - I/O pin y of Port x privilege configuration
pub type PRIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV13` reader - I/O pin y of Port x privilege configuration
pub type PRIV13_R = crate::BitReader;
///Field `PRIV13` writer - I/O pin y of Port x privilege configuration
pub type PRIV13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV14` reader - I/O pin y of Port x privilege configuration
pub type PRIV14_R = crate::BitReader;
///Field `PRIV14` writer - I/O pin y of Port x privilege configuration
pub type PRIV14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV15` reader - I/O pin y of Port x privilege configuration
pub type PRIV15_R = crate::BitReader;
///Field `PRIV15` writer - I/O pin y of Port x privilege configuration
pub type PRIV15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .field("priv4", &self.priv4())
            .field("priv5", &self.priv5())
            .field("priv6", &self.priv6())
            .field("priv7", &self.priv7())
            .field("priv8", &self.priv8())
            .field("priv9", &self.priv9())
            .field("priv10", &self.priv10())
            .field("priv11", &self.priv11())
            .field("priv12", &self.priv12())
            .field("priv13", &self.priv13())
            .field("priv14", &self.priv14())
            .field("priv15", &self.priv15())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W<PRIVCFGRrs> {
        PRIV0_W::new(self, 0)
    }
    ///Bit 1 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W<PRIVCFGRrs> {
        PRIV1_W::new(self, 1)
    }
    ///Bit 2 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W<PRIVCFGRrs> {
        PRIV2_W::new(self, 2)
    }
    ///Bit 3 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W<PRIVCFGRrs> {
        PRIV3_W::new(self, 3)
    }
    ///Bit 4 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W<PRIVCFGRrs> {
        PRIV4_W::new(self, 4)
    }
    ///Bit 5 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W<PRIVCFGRrs> {
        PRIV5_W::new(self, 5)
    }
    ///Bit 6 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W<PRIVCFGRrs> {
        PRIV6_W::new(self, 6)
    }
    ///Bit 7 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W<PRIVCFGRrs> {
        PRIV7_W::new(self, 7)
    }
    ///Bit 8 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv8(&mut self) -> PRIV8_W<PRIVCFGRrs> {
        PRIV8_W::new(self, 8)
    }
    ///Bit 9 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv9(&mut self) -> PRIV9_W<PRIVCFGRrs> {
        PRIV9_W::new(self, 9)
    }
    ///Bit 10 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv10(&mut self) -> PRIV10_W<PRIVCFGRrs> {
        PRIV10_W::new(self, 10)
    }
    ///Bit 11 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv11(&mut self) -> PRIV11_W<PRIVCFGRrs> {
        PRIV11_W::new(self, 11)
    }
    ///Bit 12 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv12(&mut self) -> PRIV12_W<PRIVCFGRrs> {
        PRIV12_W::new(self, 12)
    }
    ///Bit 13 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv13(&mut self) -> PRIV13_W<PRIVCFGRrs> {
        PRIV13_W::new(self, 13)
    }
    ///Bit 14 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv14(&mut self) -> PRIV14_W<PRIVCFGRrs> {
        PRIV14_W::new(self, 14)
    }
    ///Bit 15 - I/O pin y of Port x privilege configuration
    #[inline(always)]
    pub fn priv15(&mut self) -> PRIV15_W<PRIVCFGRrs> {
        PRIV15_W::new(self, 15)
    }
}
/**GPIO port Q privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOQ:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0xffff
impl crate::Resettable for PRIVCFGRrs {
    const RESET_VALUE: u32 = 0xffff;
}
