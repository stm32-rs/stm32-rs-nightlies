///Register `PLL4CR` reader
pub type R = crate::R<PLL4CRrs>;
///Register `PLL4CR` writer
pub type W = crate::W<PLL4CRrs>;
///Field `PLLON` reader - PLLON
pub type PLLON_R = crate::BitReader;
///Field `PLLON` writer - PLLON
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4RDY` reader - PLL4RDY
pub type PLL4RDY_R = crate::BitReader;
///Field `SSCG_CTRL` reader - SSCG_CTRL
pub type SSCG_CTRL_R = crate::BitReader;
///Field `SSCG_CTRL` writer - SSCG_CTRL
pub type SSCG_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVPEN` reader - DIVPEN
pub type DIVPEN_R = crate::BitReader;
///Field `DIVPEN` writer - DIVPEN
pub type DIVPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVQEN` reader - DIVQEN
pub type DIVQEN_R = crate::BitReader;
///Field `DIVQEN` writer - DIVQEN
pub type DIVQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVREN` reader - DIVREN
pub type DIVREN_R = crate::BitReader;
///Field `DIVREN` writer - DIVREN
pub type DIVREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLLON
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL4RDY
    #[inline(always)]
    pub fn pll4rdy(&self) -> PLL4RDY_R {
        PLL4RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SSCG_CTRL
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - DIVPEN
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DIVQEN
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DIVREN
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL4CR")
            .field("pllon", &self.pllon())
            .field("pll4rdy", &self.pll4rdy())
            .field("sscg_ctrl", &self.sscg_ctrl())
            .field("divpen", &self.divpen())
            .field("divqen", &self.divqen())
            .field("divren", &self.divren())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLLON
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<PLL4CRrs> {
        PLLON_W::new(self, 0)
    }
    ///Bit 2 - SSCG_CTRL
    #[inline(always)]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W<PLL4CRrs> {
        SSCG_CTRL_W::new(self, 2)
    }
    ///Bit 4 - DIVPEN
    #[inline(always)]
    pub fn divpen(&mut self) -> DIVPEN_W<PLL4CRrs> {
        DIVPEN_W::new(self, 4)
    }
    ///Bit 5 - DIVQEN
    #[inline(always)]
    pub fn divqen(&mut self) -> DIVQEN_W<PLL4CRrs> {
        DIVQEN_W::new(self, 5)
    }
    ///Bit 6 - DIVREN
    #[inline(always)]
    pub fn divren(&mut self) -> DIVREN_W<PLL4CRrs> {
        DIVREN_W::new(self, 6)
    }
}
/**This register is used to control the PLL4.

You can [`read`](crate::Reg::read) this register and get [`pll4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PLL4CR)*/
pub struct PLL4CRrs;
impl crate::RegisterSpec for PLL4CRrs {
    type Ux = u32;
}
///`read()` method returns [`pll4cr::R`](R) reader structure
impl crate::Readable for PLL4CRrs {}
///`write(|w| ..)` method takes [`pll4cr::W`](W) writer structure
impl crate::Writable for PLL4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL4CR to value 0
impl crate::Resettable for PLL4CRrs {}
