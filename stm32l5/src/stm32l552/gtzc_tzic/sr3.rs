///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Register `SR3` writer
pub type W = crate::W<SR3rs>;
///Field `TZSCF` reader - TZSCF
pub type TZSCF_R = crate::BitReader;
///Field `TZSCF` writer - TZSCF
pub type TZSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZICF` reader - TZICF
pub type TZICF_R = crate::BitReader;
///Field `TZICF` writer - TZICF
pub type TZICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCWM1F` reader - MPCWM1F
pub type MPCWM1F_R = crate::BitReader;
///Field `MPCWM1F` writer - MPCWM1F
pub type MPCWM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCWM2F` reader - MPCWM2F
pub type MPCWM2F_R = crate::BitReader;
///Field `MPCWM2F` writer - MPCWM2F
pub type MPCWM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1F` reader - MPCBB1F
pub type MPCBB1F_R = crate::BitReader;
///Field `MPCBB1F` writer - MPCBB1F
pub type MPCBB1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1_REGF` reader - MPCBB1_REGF
pub type MPCBB1_REGF_R = crate::BitReader;
///Field `MPCBB1_REGF` writer - MPCBB1_REGF
pub type MPCBB1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2F` reader - MPCBB2F
pub type MPCBB2F_R = crate::BitReader;
///Field `MPCBB2F` writer - MPCBB2F
pub type MPCBB2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2_REGF` reader - MPCBB2_REGF
pub type MPCBB2_REGF_R = crate::BitReader;
///Field `MPCBB2_REGF` writer - MPCBB2_REGF
pub type MPCBB2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZSCF
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZICF
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MPCWM1F
    #[inline(always)]
    pub fn mpcwm1f(&self) -> MPCWM1F_R {
        MPCWM1F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MPCWM2F
    #[inline(always)]
    pub fn mpcwm2f(&self) -> MPCWM2F_R {
        MPCWM2F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MPCBB1F
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MPCBB1_REGF
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MPCBB2F
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MPCBB2_REGF
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("tzscf", &self.tzscf())
            .field("tzicf", &self.tzicf())
            .field("mpcwm1f", &self.mpcwm1f())
            .field("mpcwm2f", &self.mpcwm2f())
            .field("mpcbb1f", &self.mpcbb1f())
            .field("mpcbb1_regf", &self.mpcbb1_regf())
            .field("mpcbb2f", &self.mpcbb2f())
            .field("mpcbb2_regf", &self.mpcbb2_regf())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZSCF
    #[inline(always)]
    pub fn tzscf(&mut self) -> TZSCF_W<'_, SR3rs> {
        TZSCF_W::new(self, 0)
    }
    ///Bit 1 - TZICF
    #[inline(always)]
    pub fn tzicf(&mut self) -> TZICF_W<'_, SR3rs> {
        TZICF_W::new(self, 1)
    }
    ///Bit 2 - MPCWM1F
    #[inline(always)]
    pub fn mpcwm1f(&mut self) -> MPCWM1F_W<'_, SR3rs> {
        MPCWM1F_W::new(self, 2)
    }
    ///Bit 3 - MPCWM2F
    #[inline(always)]
    pub fn mpcwm2f(&mut self) -> MPCWM2F_W<'_, SR3rs> {
        MPCWM2F_W::new(self, 3)
    }
    ///Bit 4 - MPCBB1F
    #[inline(always)]
    pub fn mpcbb1f(&mut self) -> MPCBB1F_W<'_, SR3rs> {
        MPCBB1F_W::new(self, 4)
    }
    ///Bit 5 - MPCBB1_REGF
    #[inline(always)]
    pub fn mpcbb1_regf(&mut self) -> MPCBB1_REGF_W<'_, SR3rs> {
        MPCBB1_REGF_W::new(self, 5)
    }
    ///Bit 6 - MPCBB2F
    #[inline(always)]
    pub fn mpcbb2f(&mut self) -> MPCBB2F_W<'_, SR3rs> {
        MPCBB2F_W::new(self, 6)
    }
    ///Bit 7 - MPCBB2_REGF
    #[inline(always)]
    pub fn mpcbb2_regf(&mut self) -> MPCBB2_REGF_W<'_, SR3rs> {
        MPCBB2_REGF_W::new(self, 7)
    }
}
/**TZIC interrupt status register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_TZIC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`write(|w| ..)` method takes [`sr3::W`](W) writer structure
impl crate::Writable for SR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3rs {}
