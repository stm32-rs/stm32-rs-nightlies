///Register `PWR_CR4` reader
pub type R = crate::R<PWR_CR4rs>;
///Register `PWR_CR4` writer
pub type W = crate::W<PWR_CR4rs>;
///Field `WP1` reader - Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
pub type WP1_R = crate::BitReader;
///Field `WP1` writer - Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP2` reader - Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
pub type WP2_R = crate::BitReader;
///Field `WP2` writer - Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP3` reader - Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
pub type WP3_R = crate::BitReader;
///Field `WP3` writer - Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP4` reader - Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
pub type WP4_R = crate::BitReader;
///Field `WP4` writer - Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
pub type WP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP5` reader - Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
pub type WP5_R = crate::BitReader;
///Field `WP5` writer - Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
pub type WP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WP7` reader - Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7
pub type WP7_R = crate::BitReader;
///Field `WP7` writer - Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7
pub type WP7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBE` reader - V<sub>BAT</sub> battery charging enable
pub type VBE_R = crate::BitReader;
///Field `VBE` writer - V<sub>BAT</sub> battery charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBRS` reader - V<sub>BAT</sub> battery charging resistor selection
pub type VBRS_R = crate::BitReader;
///Field `VBRS` writer - V<sub>BAT</sub> battery charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7
    #[inline(always)]
    pub fn wp7(&self) -> WP7_R {
        WP7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - V<sub>BAT</sub> battery charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - V<sub>BAT</sub> battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR4")
            .field("wp1", &self.wp1())
            .field("wp2", &self.wp2())
            .field("wp3", &self.wp3())
            .field("wp4", &self.wp4())
            .field("wp5", &self.wp5())
            .field("wp7", &self.wp7())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W<PWR_CR4rs> {
        WP1_W::new(self, 0)
    }
    ///Bit 1 - Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W<PWR_CR4rs> {
        WP2_W::new(self, 1)
    }
    ///Bit 2 - Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W<PWR_CR4rs> {
        WP3_W::new(self, 2)
    }
    ///Bit 3 - Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4
    #[inline(always)]
    pub fn wp4(&mut self) -> WP4_W<PWR_CR4rs> {
        WP4_W::new(self, 3)
    }
    ///Bit 4 - Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W<PWR_CR4rs> {
        WP5_W::new(self, 4)
    }
    ///Bit 6 - Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7
    #[inline(always)]
    pub fn wp7(&mut self) -> WP7_W<PWR_CR4rs> {
        WP7_W::new(self, 6)
    }
    ///Bit 8 - V<sub>BAT</sub> battery charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<PWR_CR4rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - V<sub>BAT</sub> battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<PWR_CR4rs> {
        VBRS_W::new(self, 9)
    }
}
/**Power control register 4

You can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#PWR:PWR_CR4)*/
pub struct PWR_CR4rs;
impl crate::RegisterSpec for PWR_CR4rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr4::R`](R) reader structure
impl crate::Readable for PWR_CR4rs {}
///`write(|w| ..)` method takes [`pwr_cr4::W`](W) writer structure
impl crate::Writable for PWR_CR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_CR4 to value 0
impl crate::Resettable for PWR_CR4rs {
    const RESET_VALUE: u32 = 0;
}
