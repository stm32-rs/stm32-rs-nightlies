///Register `REG4_CFGR` reader
pub type R = crate::R<REG4_CFGRrs>;
///Register `REG4_CFGR` writer
pub type W = crate::W<REG4_CFGRrs>;
///Field `BREN` reader - base region enable
pub type BREN_R = crate::BitReader;
///Field `BREN` writer - base region enable
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - secure region
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - secure region
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC0` reader - privileged access for compartment y
pub type PRIVC0_R = crate::BitReader;
///Field `PRIVC0` writer - privileged access for compartment y
pub type PRIVC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC1` reader - privileged access for compartment y
pub type PRIVC1_R = crate::BitReader;
///Field `PRIVC1` writer - privileged access for compartment y
pub type PRIVC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC2` reader - privileged access for compartment y
pub type PRIVC2_R = crate::BitReader;
///Field `PRIVC2` writer - privileged access for compartment y
pub type PRIVC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC3` reader - privileged access for compartment y
pub type PRIVC3_R = crate::BitReader;
///Field `PRIVC3` writer - privileged access for compartment y
pub type PRIVC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC4` reader - privileged access for compartment y
pub type PRIVC4_R = crate::BitReader;
///Field `PRIVC4` writer - privileged access for compartment y
pub type PRIVC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC5` reader - privileged access for compartment y
pub type PRIVC5_R = crate::BitReader;
///Field `PRIVC5` writer - privileged access for compartment y
pub type PRIVC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC6` reader - privileged access for compartment y
pub type PRIVC6_R = crate::BitReader;
///Field `PRIVC6` writer - privileged access for compartment y
pub type PRIVC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIVC7` reader - privileged access for compartment y
pub type PRIVC7_R = crate::BitReader;
///Field `PRIVC7` writer - privileged access for compartment y
pub type PRIVC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - base region enable
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - secure region
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - privileged access for compartment y
    #[inline(always)]
    pub fn privc0(&self) -> PRIVC0_R {
        PRIVC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access for compartment y
    #[inline(always)]
    pub fn privc1(&self) -> PRIVC1_R {
        PRIVC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access for compartment y
    #[inline(always)]
    pub fn privc2(&self) -> PRIVC2_R {
        PRIVC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access for compartment y
    #[inline(always)]
    pub fn privc3(&self) -> PRIVC3_R {
        PRIVC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged access for compartment y
    #[inline(always)]
    pub fn privc4(&self) -> PRIVC4_R {
        PRIVC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - privileged access for compartment y
    #[inline(always)]
    pub fn privc5(&self) -> PRIVC5_R {
        PRIVC5_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - privileged access for compartment y
    #[inline(always)]
    pub fn privc6(&self) -> PRIVC6_R {
        PRIVC6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - privileged access for compartment y
    #[inline(always)]
    pub fn privc7(&self) -> PRIVC7_R {
        PRIVC7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG4_CFGR")
            .field("bren", &self.bren())
            .field("sec", &self.sec())
            .field("privc0", &self.privc0())
            .field("privc1", &self.privc1())
            .field("privc2", &self.privc2())
            .field("privc3", &self.privc3())
            .field("privc4", &self.privc4())
            .field("privc5", &self.privc5())
            .field("privc6", &self.privc6())
            .field("privc7", &self.privc7())
            .finish()
    }
}
impl W {
    ///Bit 0 - base region enable
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<'_, REG4_CFGRrs> {
        BREN_W::new(self, 0)
    }
    ///Bit 8 - secure region
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, REG4_CFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 16 - privileged access for compartment y
    #[inline(always)]
    pub fn privc0(&mut self) -> PRIVC0_W<'_, REG4_CFGRrs> {
        PRIVC0_W::new(self, 16)
    }
    ///Bit 17 - privileged access for compartment y
    #[inline(always)]
    pub fn privc1(&mut self) -> PRIVC1_W<'_, REG4_CFGRrs> {
        PRIVC1_W::new(self, 17)
    }
    ///Bit 18 - privileged access for compartment y
    #[inline(always)]
    pub fn privc2(&mut self) -> PRIVC2_W<'_, REG4_CFGRrs> {
        PRIVC2_W::new(self, 18)
    }
    ///Bit 19 - privileged access for compartment y
    #[inline(always)]
    pub fn privc3(&mut self) -> PRIVC3_W<'_, REG4_CFGRrs> {
        PRIVC3_W::new(self, 19)
    }
    ///Bit 20 - privileged access for compartment y
    #[inline(always)]
    pub fn privc4(&mut self) -> PRIVC4_W<'_, REG4_CFGRrs> {
        PRIVC4_W::new(self, 20)
    }
    ///Bit 21 - privileged access for compartment y
    #[inline(always)]
    pub fn privc5(&mut self) -> PRIVC5_W<'_, REG4_CFGRrs> {
        PRIVC5_W::new(self, 21)
    }
    ///Bit 22 - privileged access for compartment y
    #[inline(always)]
    pub fn privc6(&mut self) -> PRIVC6_W<'_, REG4_CFGRrs> {
        PRIVC6_W::new(self, 22)
    }
    ///Bit 23 - privileged access for compartment y
    #[inline(always)]
    pub fn privc7(&mut self) -> PRIVC7_W<'_, REG4_CFGRrs> {
        PRIVC7_W::new(self, 23)
    }
}
/**RISAF region 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg4_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG4_CFGR)*/
pub struct REG4_CFGRrs;
impl crate::RegisterSpec for REG4_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`reg4_cfgr::R`](R) reader structure
impl crate::Readable for REG4_CFGRrs {}
///`write(|w| ..)` method takes [`reg4_cfgr::W`](W) writer structure
impl crate::Writable for REG4_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG4_CFGR to value 0
impl crate::Resettable for REG4_CFGRrs {}
