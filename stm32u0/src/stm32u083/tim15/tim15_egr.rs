///Register `TIM15_EGR` reader
pub type R = crate::R<TIM15_EGRrs>;
///Register `TIM15_EGR` writer
pub type W = crate::W<TIM15_EGRrs>;
///Field `UG` writer - Update generation
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - Capture/Compare 1 generation
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` writer - Capture/Compare 2 generation
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` reader - Capture/Compare control update generation
pub type COMG_R = crate::BitReader;
///Field `COMG` writer - Capture/Compare control update generation
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - Trigger generation
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` writer - Break generation
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&self) -> COMG_R {
        COMG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_EGR")
            .field("comg", &self.comg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<TIM15_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<TIM15_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<TIM15_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<TIM15_EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<TIM15_EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - Break generation
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<TIM15_EGRrs> {
        BG_W::new(self, 7)
    }
}
/**TIM15 event generation register

You can [`read`](crate::Reg::read) this register and get [`tim15_egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM15:TIM15_EGR)*/
pub struct TIM15_EGRrs;
impl crate::RegisterSpec for TIM15_EGRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_egr::R`](R) reader structure
impl crate::Readable for TIM15_EGRrs {}
///`write(|w| ..)` method takes [`tim15_egr::W`](W) writer structure
impl crate::Writable for TIM15_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM15_EGR to value 0
impl crate::Resettable for TIM15_EGRrs {
    const RESET_VALUE: u16 = 0;
}
