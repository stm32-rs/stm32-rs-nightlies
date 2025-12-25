///Register `BOOTR` reader
pub type R = crate::R<BOOTRrs>;
///Register `BOOTR` writer
pub type W = crate::W<BOOTRrs>;
///Field `BOOT0` reader - BOOT0
pub type BOOT0_R = crate::BitReader;
///Field `BOOT1` reader - BOOT1
pub type BOOT1_R = crate::BitReader;
///Field `BOOT2` reader - BOOT2
pub type BOOT2_R = crate::BitReader;
///Field `BOOT0_PD` reader - BOOT0_PD
pub type BOOT0_PD_R = crate::BitReader;
///Field `BOOT0_PD` writer - BOOT0_PD
pub type BOOT0_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT1_PD` reader - BOOT1_PD
pub type BOOT1_PD_R = crate::BitReader;
///Field `BOOT1_PD` writer - BOOT1_PD
pub type BOOT1_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT2_PD` reader - BOOT2_PD
pub type BOOT2_PD_R = crate::BitReader;
///Field `BOOT2_PD` writer - BOOT2_PD
pub type BOOT2_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BOOT0
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BOOT1
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BOOT2
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - BOOT0_PD
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BOOT1_PD
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BOOT2_PD
    #[inline(always)]
    pub fn boot2_pd(&self) -> BOOT2_PD_R {
        BOOT2_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTR")
            .field("boot0", &self.boot0())
            .field("boot1", &self.boot1())
            .field("boot2", &self.boot2())
            .field("boot0_pd", &self.boot0_pd())
            .field("boot1_pd", &self.boot1_pd())
            .field("boot2_pd", &self.boot2_pd())
            .finish()
    }
}
impl W {
    ///Bit 4 - BOOT0_PD
    #[inline(always)]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W<'_, BOOTRrs> {
        BOOT0_PD_W::new(self, 4)
    }
    ///Bit 5 - BOOT1_PD
    #[inline(always)]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W<'_, BOOTRrs> {
        BOOT1_PD_W::new(self, 5)
    }
    ///Bit 6 - BOOT2_PD
    #[inline(always)]
    pub fn boot2_pd(&mut self) -> BOOT2_PD_W<'_, BOOTRrs> {
        BOOT2_PD_W::new(self, 6)
    }
}
/**This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )

You can [`read`](crate::Reg::read) this register and get [`bootr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:BOOTR)*/
pub struct BOOTRrs;
impl crate::RegisterSpec for BOOTRrs {
    type Ux = u32;
}
///`read()` method returns [`bootr::R`](R) reader structure
impl crate::Readable for BOOTRrs {}
///`write(|w| ..)` method takes [`bootr::W`](W) writer structure
impl crate::Writable for BOOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOTR to value 0
impl crate::Resettable for BOOTRrs {}
