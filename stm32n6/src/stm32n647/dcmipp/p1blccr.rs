///Register `P1BLCCR` reader
pub type R = crate::R<P1BLCCRrs>;
///Register `P1BLCCR` writer
pub type W = crate::W<P1BLCCRrs>;
///Field `ENABLE` reader - Black level calibration
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Black level calibration
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLCB` reader - Black level calibration - Blue
pub type BLCB_R = crate::FieldReader;
///Field `BLCB` writer - Black level calibration - Blue
pub type BLCB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BLCG` reader - Black level calibration - Green
pub type BLCG_R = crate::FieldReader;
///Field `BLCG` writer - Black level calibration - Green
pub type BLCG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BLCR` reader - Black level calibration - Red
pub type BLCR_R = crate::FieldReader;
///Field `BLCR` writer - Black level calibration - Red
pub type BLCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Black level calibration
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Black level calibration - Blue
    #[inline(always)]
    pub fn blcb(&self) -> BLCB_R {
        BLCB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Black level calibration - Green
    #[inline(always)]
    pub fn blcg(&self) -> BLCG_R {
        BLCG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Black level calibration - Red
    #[inline(always)]
    pub fn blcr(&self) -> BLCR_R {
        BLCR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1BLCCR")
            .field("enable", &self.enable())
            .field("blcb", &self.blcb())
            .field("blcg", &self.blcg())
            .field("blcr", &self.blcr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Black level calibration
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1BLCCRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 8:15 - Black level calibration - Blue
    #[inline(always)]
    pub fn blcb(&mut self) -> BLCB_W<'_, P1BLCCRrs> {
        BLCB_W::new(self, 8)
    }
    ///Bits 16:23 - Black level calibration - Green
    #[inline(always)]
    pub fn blcg(&mut self) -> BLCG_W<'_, P1BLCCRrs> {
        BLCG_W::new(self, 16)
    }
    ///Bits 24:31 - Black level calibration - Red
    #[inline(always)]
    pub fn blcr(&mut self) -> BLCR_W<'_, P1BLCCRrs> {
        BLCR_W::new(self, 24)
    }
}
/**DCMIPP Pipe1 black level calibration control register

You can [`read`](crate::Reg::read) this register and get [`p1blccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1blccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1BLCCR)*/
pub struct P1BLCCRrs;
impl crate::RegisterSpec for P1BLCCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1blccr::R`](R) reader structure
impl crate::Readable for P1BLCCRrs {}
///`write(|w| ..)` method takes [`p1blccr::W`](W) writer structure
impl crate::Writable for P1BLCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1BLCCR to value 0
impl crate::Resettable for P1BLCCRrs {}
