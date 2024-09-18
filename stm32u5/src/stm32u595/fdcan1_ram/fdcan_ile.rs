///Register `FDCAN_ILE` reader
pub type R = crate::R<FDCAN_ILErs>;
///Register `FDCAN_ILE` writer
pub type W = crate::W<FDCAN_ILErs>;
///Field `EINT0` reader - Enable Interrupt Line 0
pub type EINT0_R = crate::BitReader;
///Field `EINT0` writer - Enable Interrupt Line 0
pub type EINT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EINT1` reader - Enable Interrupt Line 1
pub type EINT1_R = crate::BitReader;
///Field `EINT1` writer - Enable Interrupt Line 1
pub type EINT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Interrupt Line 0
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Interrupt Line 1
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILE")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Interrupt Line 0
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<FDCAN_ILErs> {
        EINT0_W::new(self, 0)
    }
    ///Bit 1 - Enable Interrupt Line 1
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<FDCAN_ILErs> {
        EINT1_W::new(self, 1)
    }
}
/**FDCAN Interrupt Line Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:FDCAN_ILE)*/
pub struct FDCAN_ILErs;
impl crate::RegisterSpec for FDCAN_ILErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ile::R`](R) reader structure
impl crate::Readable for FDCAN_ILErs {}
///`write(|w| ..)` method takes [`fdcan_ile::W`](W) writer structure
impl crate::Writable for FDCAN_ILErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_ILE to value 0
impl crate::Resettable for FDCAN_ILErs {
    const RESET_VALUE: u32 = 0;
}
