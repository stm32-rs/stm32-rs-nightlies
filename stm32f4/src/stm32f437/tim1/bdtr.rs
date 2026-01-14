///Register `BDTR` reader
pub type R = crate::R<BDTRrs>;
///Register `BDTR` writer
pub type W = crate::W<BDTRrs>;
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LOCK` reader - Lock configuration
pub type LOCK_R = crate::FieldReader;
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSSI` reader - Off-state selection for Idle mode
pub type OSSI_R = crate::BitReader;
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSSR` reader - Off-state selection for Run mode
pub type OSSR_R = crate::BitReader;
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader;
///Field `BKE` writer - Break enable
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKP` reader - Break polarity
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Break polarity
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOE` reader - Automatic output enable
pub type AOE_R = crate::BitReader;
///Field `AOE` writer - Automatic output enable
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOE` reader - Main output enable
pub type MOE_R = crate::BitReader;
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTR")
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("bkp", &self.bkp())
            .field("bke", &self.bke())
            .field("ossr", &self.ossr())
            .field("ossi", &self.ossi())
            .field("lock", &self.lock())
            .field("dtg", &self.dtg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<'_, BDTRrs> {
        DTG_W::new(self, 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, BDTRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<'_, BDTRrs> {
        OSSI_W::new(self, 10)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<'_, BDTRrs> {
        OSSR_W::new(self, 11)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<'_, BDTRrs> {
        BKE_W::new(self, 12)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, BDTRrs> {
        BKP_W::new(self, 13)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<'_, BDTRrs> {
        AOE_W::new(self, 14)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<'_, BDTRrs> {
        MOE_W::new(self, 15)
    }
}
/**break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#TIM1:BDTR)*/
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtr::R`](R) reader structure
impl crate::Readable for BDTRrs {}
///`write(|w| ..)` method takes [`bdtr::W`](W) writer structure
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTRrs {}
