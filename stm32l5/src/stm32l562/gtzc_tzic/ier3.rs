///Register `IER3` reader
pub type R = crate::R<IER3rs>;
///Register `IER3` writer
pub type W = crate::W<IER3rs>;
///Field `TZSCIE` reader - TZSCIE
pub type TZSCIE_R = crate::BitReader;
///Field `TZSCIE` writer - TZSCIE
pub type TZSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZICIE` reader - TZICIE
pub type TZICIE_R = crate::BitReader;
///Field `TZICIE` writer - TZICIE
pub type TZICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCWM1IE` reader - MPCWM1IE
pub type MPCWM1IE_R = crate::BitReader;
///Field `MPCWM1IE` writer - MPCWM1IE
pub type MPCWM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCWM2IE` reader - MPCWM2IE
pub type MPCWM2IE_R = crate::BitReader;
///Field `MPCWM2IE` writer - MPCWM2IE
pub type MPCWM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1IE` reader - MPCBB1IE
pub type MPCBB1IE_R = crate::BitReader;
///Field `MPCBB1IE` writer - MPCBB1IE
pub type MPCBB1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1_REGIE` reader - MPCBB1_REGIE
pub type MPCBB1_REGIE_R = crate::BitReader;
///Field `MPCBB1_REGIE` writer - MPCBB1_REGIE
pub type MPCBB1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2IE` reader - MPCBB2IE
pub type MPCBB2IE_R = crate::BitReader;
///Field `MPCBB2IE` writer - MPCBB2IE
pub type MPCBB2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2_REGIE` reader - MPCBB2_REGIE
pub type MPCBB2_REGIE_R = crate::BitReader;
///Field `MPCBB2_REGIE` writer - MPCBB2_REGIE
pub type MPCBB2_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZSCIE
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZICIE
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MPCWM1IE
    #[inline(always)]
    pub fn mpcwm1ie(&self) -> MPCWM1IE_R {
        MPCWM1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MPCWM2IE
    #[inline(always)]
    pub fn mpcwm2ie(&self) -> MPCWM2IE_R {
        MPCWM2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPCBB1IE
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MPCBB1_REGIE
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MPCBB2IE
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MPCBB2_REGIE
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER3")
            .field("tzscie", &self.tzscie())
            .field("tzicie", &self.tzicie())
            .field("mpcwm1ie", &self.mpcwm1ie())
            .field("mpcwm2ie", &self.mpcwm2ie())
            .field("mpcbb1ie", &self.mpcbb1ie())
            .field("mpcbb1_regie", &self.mpcbb1_regie())
            .field("mpcbb2ie", &self.mpcbb2ie())
            .field("mpcbb2_regie", &self.mpcbb2_regie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZSCIE
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W<'_, IER3rs> {
        TZSCIE_W::new(self, 0)
    }
    ///Bit 1 - TZICIE
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W<'_, IER3rs> {
        TZICIE_W::new(self, 1)
    }
    ///Bit 2 - MPCWM1IE
    #[inline(always)]
    pub fn mpcwm1ie(&mut self) -> MPCWM1IE_W<'_, IER3rs> {
        MPCWM1IE_W::new(self, 2)
    }
    ///Bit 3 - MPCWM2IE
    #[inline(always)]
    pub fn mpcwm2ie(&mut self) -> MPCWM2IE_W<'_, IER3rs> {
        MPCWM2IE_W::new(self, 3)
    }
    ///Bit 4 - MPCBB1IE
    #[inline(always)]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W<'_, IER3rs> {
        MPCBB1IE_W::new(self, 4)
    }
    ///Bit 5 - MPCBB1_REGIE
    #[inline(always)]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<'_, IER3rs> {
        MPCBB1_REGIE_W::new(self, 5)
    }
    ///Bit 6 - MPCBB2IE
    #[inline(always)]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W<'_, IER3rs> {
        MPCBB2IE_W::new(self, 6)
    }
    ///Bit 7 - MPCBB2_REGIE
    #[inline(always)]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<'_, IER3rs> {
        MPCBB2_REGIE_W::new(self, 7)
    }
}
/**TZIC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#GTZC_TZIC:IER3)*/
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
///`read()` method returns [`ier3::R`](R) reader structure
impl crate::Readable for IER3rs {}
///`write(|w| ..)` method takes [`ier3::W`](W) writer structure
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER3 to value 0
impl crate::Resettable for IER3rs {}
