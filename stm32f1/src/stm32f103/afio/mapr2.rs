///Register `MAPR2` reader
pub type R = crate::R<MAPR2rs>;
///Register `MAPR2` writer
pub type W = crate::W<MAPR2rs>;
///Field `TIM9_REMAP` reader - TIM9 remapping
pub type TIM9_REMAP_R = crate::BitReader;
///Field `TIM9_REMAP` writer - TIM9 remapping
pub type TIM9_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10_REMAP` reader - TIM10 remapping
pub type TIM10_REMAP_R = crate::BitReader;
///Field `TIM10_REMAP` writer - TIM10 remapping
pub type TIM10_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11_REMAP` reader - TIM11 remapping
pub type TIM11_REMAP_R = crate::BitReader;
///Field `TIM11_REMAP` writer - TIM11 remapping
pub type TIM11_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13_REMAP` reader - TIM13 remapping
pub type TIM13_REMAP_R = crate::BitReader;
///Field `TIM13_REMAP` writer - TIM13 remapping
pub type TIM13_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14_REMAP` reader - TIM14 remapping
pub type TIM14_REMAP_R = crate::BitReader;
///Field `TIM14_REMAP` writer - TIM14 remapping
pub type TIM14_REMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSMC_NADV` reader - NADV connect/disconnect
pub type FSMC_NADV_R = crate::BitReader;
///Field `FSMC_NADV` writer - NADV connect/disconnect
pub type FSMC_NADV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - TIM9 remapping
    #[inline(always)]
    pub fn tim9_remap(&self) -> TIM9_REMAP_R {
        TIM9_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM10 remapping
    #[inline(always)]
    pub fn tim10_remap(&self) -> TIM10_REMAP_R {
        TIM10_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM11 remapping
    #[inline(always)]
    pub fn tim11_remap(&self) -> TIM11_REMAP_R {
        TIM11_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAPR2")
            .field("tim9_remap", &self.tim9_remap())
            .field("tim10_remap", &self.tim10_remap())
            .field("tim11_remap", &self.tim11_remap())
            .field("tim13_remap", &self.tim13_remap())
            .field("tim14_remap", &self.tim14_remap())
            .field("fsmc_nadv", &self.fsmc_nadv())
            .finish()
    }
}
impl W {
    ///Bit 5 - TIM9 remapping
    #[inline(always)]
    pub fn tim9_remap(&mut self) -> TIM9_REMAP_W<'_, MAPR2rs> {
        TIM9_REMAP_W::new(self, 5)
    }
    ///Bit 6 - TIM10 remapping
    #[inline(always)]
    pub fn tim10_remap(&mut self) -> TIM10_REMAP_W<'_, MAPR2rs> {
        TIM10_REMAP_W::new(self, 6)
    }
    ///Bit 7 - TIM11 remapping
    #[inline(always)]
    pub fn tim11_remap(&mut self) -> TIM11_REMAP_W<'_, MAPR2rs> {
        TIM11_REMAP_W::new(self, 7)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W<'_, MAPR2rs> {
        TIM13_REMAP_W::new(self, 8)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W<'_, MAPR2rs> {
        TIM14_REMAP_W::new(self, 9)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W<'_, MAPR2rs> {
        FSMC_NADV_W::new(self, 10)
    }
}
/**AF remap and debug I/O configuration register

You can [`read`](crate::Reg::read) this register and get [`mapr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#AFIO:MAPR2)*/
pub struct MAPR2rs;
impl crate::RegisterSpec for MAPR2rs {
    type Ux = u32;
}
///`read()` method returns [`mapr2::R`](R) reader structure
impl crate::Readable for MAPR2rs {}
///`write(|w| ..)` method takes [`mapr2::W`](W) writer structure
impl crate::Writable for MAPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAPR2 to value 0
impl crate::Resettable for MAPR2rs {}
