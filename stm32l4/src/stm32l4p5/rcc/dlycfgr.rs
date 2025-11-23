///Register `DLYCFGR` reader
pub type R = crate::R<DLYCFGRrs>;
///Register `DLYCFGR` writer
pub type W = crate::W<DLYCFGRrs>;
///Field `OCTOSPI1_DLY` reader - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
pub type OCTOSPI1_DLY_R = crate::FieldReader;
///Field `OCTOSPI1_DLY` writer - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
pub type OCTOSPI1_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `OCTOSPI2_DLY` reader - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
pub type OCTOSPI2_DLY_R = crate::FieldReader;
///Field `OCTOSPI2_DLY` writer - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
pub type OCTOSPI2_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    #[inline(always)]
    pub fn octospi1_dly(&self) -> OCTOSPI1_DLY_R {
        OCTOSPI1_DLY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    #[inline(always)]
    pub fn octospi2_dly(&self) -> OCTOSPI2_DLY_R {
        OCTOSPI2_DLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLYCFGR")
            .field("octospi2_dly", &self.octospi2_dly())
            .field("octospi1_dly", &self.octospi1_dly())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    #[inline(always)]
    pub fn octospi1_dly(&mut self) -> OCTOSPI1_DLY_W<'_, DLYCFGRrs> {
        OCTOSPI1_DLY_W::new(self, 0)
    }
    ///Bits 4:7 - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe
    #[inline(always)]
    pub fn octospi2_dly(&mut self) -> OCTOSPI2_DLY_W<'_, DLYCFGRrs> {
        OCTOSPI2_DLY_W::new(self, 4)
    }
}
/**delay configuration register

You can [`read`](crate::Reg::read) this register and get [`dlycfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlycfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:DLYCFGR)*/
pub struct DLYCFGRrs;
impl crate::RegisterSpec for DLYCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`dlycfgr::R`](R) reader structure
impl crate::Readable for DLYCFGRrs {}
///`write(|w| ..)` method takes [`dlycfgr::W`](W) writer structure
impl crate::Writable for DLYCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLYCFGR to value 0
impl crate::Resettable for DLYCFGRrs {}
