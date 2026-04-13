///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
///Field `WUPP1` reader - Wakeup pin WKUP1 polarity
pub type WUPP1_R = crate::BitReader;
///Field `WUPP1` writer - Wakeup pin WKUP1 polarity
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP2` reader - Wakeup pin WKUP2 polarity
pub type WUPP2_R = crate::BitReader;
///Field `WUPP2` writer - Wakeup pin WKUP2 polarity
pub type WUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP3` reader - Wakeup pin WKUP3 polarity
pub type WUPP3_R = crate::BitReader;
///Field `WUPP3` writer - Wakeup pin WKUP3 polarity
pub type WUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP4` reader - Wakeup pin WKUP4 polarity
pub type WUPP4_R = crate::BitReader;
///Field `WUPP4` writer - Wakeup pin WKUP4 polarity
pub type WUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP5` reader - Wakeup pin WKUP5 polarity
pub type WUPP5_R = crate::BitReader;
///Field `WUPP5` writer - Wakeup pin WKUP5 polarity
pub type WUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBE` reader - VBAT battery charging enable
pub type VBE_R = crate::BitReader;
///Field `VBE` writer - VBAT battery charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBRS` reader - VBAT battery charging resistor selection
pub type VBRS_R = crate::BitReader;
///Field `VBRS` writer - VBAT battery charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSBYP` reader - SMPSBYP
pub type SMPSBYP_R = crate::BitReader;
///Field `SMPSBYP` writer - SMPSBYP
pub type SMPSBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTSMPSEN` reader - EXTSMPSEN
pub type EXTSMPSEN_R = crate::BitReader;
///Field `EXTSMPSEN` writer - EXTSMPSEN
pub type EXTSMPSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSFSTEN` reader - SMPSFSTEN
pub type SMPSFSTEN_R = crate::BitReader;
///Field `SMPSFSTEN` writer - SMPSFSTEN
pub type SMPSFSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSLPEN` reader - SMPSLPEN
pub type SMPSLPEN_R = crate::BitReader;
///Field `SMPSLPEN` writer - SMPSLPEN
pub type SMPSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SMPSBYP
    #[inline(always)]
    pub fn smpsbyp(&self) -> SMPSBYP_R {
        SMPSBYP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EXTSMPSEN
    #[inline(always)]
    pub fn extsmpsen(&self) -> EXTSMPSEN_R {
        EXTSMPSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SMPSFSTEN
    #[inline(always)]
    pub fn smpsfsten(&self) -> SMPSFSTEN_R {
        SMPSFSTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SMPSLPEN
    #[inline(always)]
    pub fn smpslpen(&self) -> SMPSLPEN_R {
        SMPSLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("smpslpen", &self.smpslpen())
            .field("smpsfsten", &self.smpsfsten())
            .field("extsmpsen", &self.extsmpsen())
            .field("smpsbyp", &self.smpsbyp())
            .field("vbrs", &self.vbrs())
            .field("vbe", &self.vbe())
            .field("wupp5", &self.wupp5())
            .field("wupp4", &self.wupp4())
            .field("wupp3", &self.wupp3())
            .field("wupp2", &self.wupp2())
            .field("wupp1", &self.wupp1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W<'_, CR4rs> {
        WUPP1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W<'_, CR4rs> {
        WUPP2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W<'_, CR4rs> {
        WUPP3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W<'_, CR4rs> {
        WUPP4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W<'_, CR4rs> {
        WUPP5_W::new(self, 4)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, CR4rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, CR4rs> {
        VBRS_W::new(self, 9)
    }
    ///Bit 12 - SMPSBYP
    #[inline(always)]
    pub fn smpsbyp(&mut self) -> SMPSBYP_W<'_, CR4rs> {
        SMPSBYP_W::new(self, 12)
    }
    ///Bit 13 - EXTSMPSEN
    #[inline(always)]
    pub fn extsmpsen(&mut self) -> EXTSMPSEN_W<'_, CR4rs> {
        EXTSMPSEN_W::new(self, 13)
    }
    ///Bit 14 - SMPSFSTEN
    #[inline(always)]
    pub fn smpsfsten(&mut self) -> SMPSFSTEN_W<'_, CR4rs> {
        SMPSFSTEN_W::new(self, 14)
    }
    ///Bit 15 - SMPSLPEN
    #[inline(always)]
    pub fn smpslpen(&mut self) -> SMPSLPEN_W<'_, CR4rs> {
        SMPSLPEN_W::new(self, 15)
    }
}
/**Power control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#PWR:CR4)*/
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
///`read()` method returns [`cr4::R`](R) reader structure
impl crate::Readable for CR4rs {}
///`write(|w| ..)` method takes [`cr4::W`](W) writer structure
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4rs {}
