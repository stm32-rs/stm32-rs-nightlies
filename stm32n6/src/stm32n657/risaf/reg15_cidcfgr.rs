///Register `REG15_CIDCFGR` reader
pub type R = crate::R<REG15_CIDCFGRrs>;
///Register `REG15_CIDCFGR` writer
pub type W = crate::W<REG15_CIDCFGRrs>;
///Field `RDENC0` reader - read enable for compartment y
pub type RDENC0_R = crate::BitReader;
///Field `RDENC0` writer - read enable for compartment y
pub type RDENC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC1` reader - read enable for compartment y
pub type RDENC1_R = crate::BitReader;
///Field `RDENC1` writer - read enable for compartment y
pub type RDENC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC2` reader - read enable for compartment y
pub type RDENC2_R = crate::BitReader;
///Field `RDENC2` writer - read enable for compartment y
pub type RDENC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC3` reader - read enable for compartment y
pub type RDENC3_R = crate::BitReader;
///Field `RDENC3` writer - read enable for compartment y
pub type RDENC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC4` reader - read enable for compartment y
pub type RDENC4_R = crate::BitReader;
///Field `RDENC4` writer - read enable for compartment y
pub type RDENC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC5` reader - read enable for compartment y
pub type RDENC5_R = crate::BitReader;
///Field `RDENC5` writer - read enable for compartment y
pub type RDENC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC6` reader - read enable for compartment y
pub type RDENC6_R = crate::BitReader;
///Field `RDENC6` writer - read enable for compartment y
pub type RDENC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDENC7` reader - read enable for compartment y
pub type RDENC7_R = crate::BitReader;
///Field `RDENC7` writer - read enable for compartment y
pub type RDENC7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC0` reader - write enable for compartment y
pub type WRENC0_R = crate::BitReader;
///Field `WRENC0` writer - write enable for compartment y
pub type WRENC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC1` reader - write enable for compartment y
pub type WRENC1_R = crate::BitReader;
///Field `WRENC1` writer - write enable for compartment y
pub type WRENC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC2` reader - write enable for compartment y
pub type WRENC2_R = crate::BitReader;
///Field `WRENC2` writer - write enable for compartment y
pub type WRENC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC3` reader - write enable for compartment y
pub type WRENC3_R = crate::BitReader;
///Field `WRENC3` writer - write enable for compartment y
pub type WRENC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC4` reader - write enable for compartment y
pub type WRENC4_R = crate::BitReader;
///Field `WRENC4` writer - write enable for compartment y
pub type WRENC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC5` reader - write enable for compartment y
pub type WRENC5_R = crate::BitReader;
///Field `WRENC5` writer - write enable for compartment y
pub type WRENC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC6` reader - write enable for compartment y
pub type WRENC6_R = crate::BitReader;
///Field `WRENC6` writer - write enable for compartment y
pub type WRENC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRENC7` reader - write enable for compartment y
pub type WRENC7_R = crate::BitReader;
///Field `WRENC7` writer - write enable for compartment y
pub type WRENC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc0(&self) -> RDENC0_R {
        RDENC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc1(&self) -> RDENC1_R {
        RDENC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc2(&self) -> RDENC2_R {
        RDENC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc3(&self) -> RDENC3_R {
        RDENC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc4(&self) -> RDENC4_R {
        RDENC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc5(&self) -> RDENC5_R {
        RDENC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc6(&self) -> RDENC6_R {
        RDENC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc7(&self) -> RDENC7_R {
        RDENC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc0(&self) -> WRENC0_R {
        WRENC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc1(&self) -> WRENC1_R {
        WRENC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc2(&self) -> WRENC2_R {
        WRENC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc3(&self) -> WRENC3_R {
        WRENC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc4(&self) -> WRENC4_R {
        WRENC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc5(&self) -> WRENC5_R {
        WRENC5_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc6(&self) -> WRENC6_R {
        WRENC6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc7(&self) -> WRENC7_R {
        WRENC7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG15_CIDCFGR")
            .field("rdenc0", &self.rdenc0())
            .field("rdenc1", &self.rdenc1())
            .field("rdenc2", &self.rdenc2())
            .field("rdenc3", &self.rdenc3())
            .field("rdenc4", &self.rdenc4())
            .field("rdenc5", &self.rdenc5())
            .field("rdenc6", &self.rdenc6())
            .field("rdenc7", &self.rdenc7())
            .field("wrenc0", &self.wrenc0())
            .field("wrenc1", &self.wrenc1())
            .field("wrenc2", &self.wrenc2())
            .field("wrenc3", &self.wrenc3())
            .field("wrenc4", &self.wrenc4())
            .field("wrenc5", &self.wrenc5())
            .field("wrenc6", &self.wrenc6())
            .field("wrenc7", &self.wrenc7())
            .finish()
    }
}
impl W {
    ///Bit 0 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc0(&mut self) -> RDENC0_W<'_, REG15_CIDCFGRrs> {
        RDENC0_W::new(self, 0)
    }
    ///Bit 1 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc1(&mut self) -> RDENC1_W<'_, REG15_CIDCFGRrs> {
        RDENC1_W::new(self, 1)
    }
    ///Bit 2 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc2(&mut self) -> RDENC2_W<'_, REG15_CIDCFGRrs> {
        RDENC2_W::new(self, 2)
    }
    ///Bit 3 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc3(&mut self) -> RDENC3_W<'_, REG15_CIDCFGRrs> {
        RDENC3_W::new(self, 3)
    }
    ///Bit 4 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc4(&mut self) -> RDENC4_W<'_, REG15_CIDCFGRrs> {
        RDENC4_W::new(self, 4)
    }
    ///Bit 5 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc5(&mut self) -> RDENC5_W<'_, REG15_CIDCFGRrs> {
        RDENC5_W::new(self, 5)
    }
    ///Bit 6 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc6(&mut self) -> RDENC6_W<'_, REG15_CIDCFGRrs> {
        RDENC6_W::new(self, 6)
    }
    ///Bit 7 - read enable for compartment y
    #[inline(always)]
    pub fn rdenc7(&mut self) -> RDENC7_W<'_, REG15_CIDCFGRrs> {
        RDENC7_W::new(self, 7)
    }
    ///Bit 16 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc0(&mut self) -> WRENC0_W<'_, REG15_CIDCFGRrs> {
        WRENC0_W::new(self, 16)
    }
    ///Bit 17 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc1(&mut self) -> WRENC1_W<'_, REG15_CIDCFGRrs> {
        WRENC1_W::new(self, 17)
    }
    ///Bit 18 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc2(&mut self) -> WRENC2_W<'_, REG15_CIDCFGRrs> {
        WRENC2_W::new(self, 18)
    }
    ///Bit 19 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc3(&mut self) -> WRENC3_W<'_, REG15_CIDCFGRrs> {
        WRENC3_W::new(self, 19)
    }
    ///Bit 20 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc4(&mut self) -> WRENC4_W<'_, REG15_CIDCFGRrs> {
        WRENC4_W::new(self, 20)
    }
    ///Bit 21 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc5(&mut self) -> WRENC5_W<'_, REG15_CIDCFGRrs> {
        WRENC5_W::new(self, 21)
    }
    ///Bit 22 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc6(&mut self) -> WRENC6_W<'_, REG15_CIDCFGRrs> {
        WRENC6_W::new(self, 22)
    }
    ///Bit 23 - write enable for compartment y
    #[inline(always)]
    pub fn wrenc7(&mut self) -> WRENC7_W<'_, REG15_CIDCFGRrs> {
        WRENC7_W::new(self, 23)
    }
}
/**RISAF region 15 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg15_cidcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_cidcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RISAF:REG15_CIDCFGR)*/
pub struct REG15_CIDCFGRrs;
impl crate::RegisterSpec for REG15_CIDCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`reg15_cidcfgr::R`](R) reader structure
impl crate::Readable for REG15_CIDCFGRrs {}
///`write(|w| ..)` method takes [`reg15_cidcfgr::W`](W) writer structure
impl crate::Writable for REG15_CIDCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG15_CIDCFGR to value 0
impl crate::Resettable for REG15_CIDCFGRrs {}
