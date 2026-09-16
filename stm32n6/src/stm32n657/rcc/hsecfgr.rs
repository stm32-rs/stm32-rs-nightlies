///Register `HSECFGR` reader
pub type R = crate::R<HSECFGRrs>;
///Register `HSECFGR` writer
pub type W = crate::W<HSECFGRrs>;
///Field `HSEDIV2BYP` reader - HSE div2 oscillator clock in Bypass mode
pub type HSEDIV2BYP_R = crate::BitReader;
///Field `HSEDIV2BYP` writer - HSE div2 oscillator clock in Bypass mode
pub type HSEDIV2BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSON` reader - HSE clock security system (CSS) enable
pub type HSECSSON_R = crate::BitReader;
///Field `HSECSSON` writer - HSE clock security system (CSS) enable
pub type HSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSRA` reader - HSE clock security system (CSS) re-arm function
pub type HSECSSRA_R = crate::BitReader;
///Field `HSECSSRA` writer - HSE clock security system (CSS) re-arm function
pub type HSECSSRA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSD` reader - HSE clock security system (CSS) failure detection
pub type HSECSSD_R = crate::BitReader;
///Field `HSECSSBYP` reader - HSE clock security system (CSS) bypass enable
pub type HSECSSBYP_R = crate::BitReader;
///Field `HSECSSBYP` writer - HSE clock security system (CSS) bypass enable
pub type HSECSSBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSECSSBPRE` reader - HSE clock security system (CSS) bypass divider
pub type HSECSSBPRE_R = crate::FieldReader;
///Field `HSECSSBPRE` writer - HSE clock security system (CSS) bypass divider
pub type HSECSSBPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSEBYP` reader - HSE clock bypass
pub type HSEBYP_R = crate::BitReader;
///Field `HSEBYP` writer - HSE clock bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEEXT` reader - HSE clock type in Bypass mode
pub type HSEEXT_R = crate::BitReader;
///Field `HSEEXT` writer - HSE clock type in Bypass mode
pub type HSEEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEGFON` reader - HSE clock glitch filter enable
pub type HSEGFON_R = crate::BitReader;
///Field `HSEGFON` writer - HSE clock glitch filter enable
pub type HSEGFON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEDRV` reader - HSE oscillator driving capability
pub type HSEDRV_R = crate::FieldReader;
///Field `HSEDRV` writer - HSE oscillator driving capability
pub type HSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 6 - HSE div2 oscillator clock in Bypass mode
    #[inline(always)]
    pub fn hsediv2byp(&self) -> HSEDIV2BYP_R {
        HSEDIV2BYP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - HSE clock security system (CSS) enable
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSE clock security system (CSS) re-arm function
    #[inline(always)]
    pub fn hsecssra(&self) -> HSECSSRA_R {
        HSECSSRA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSE clock security system (CSS) failure detection
    #[inline(always)]
    pub fn hsecssd(&self) -> HSECSSD_R {
        HSECSSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSE clock security system (CSS) bypass enable
    #[inline(always)]
    pub fn hsecssbyp(&self) -> HSECSSBYP_R {
        HSECSSBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:14 - HSE clock security system (CSS) bypass divider
    #[inline(always)]
    pub fn hsecssbpre(&self) -> HSECSSBPRE_R {
        HSECSSBPRE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HSE clock type in Bypass mode
    #[inline(always)]
    pub fn hseext(&self) -> HSEEXT_R {
        HSEEXT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock glitch filter enable
    #[inline(always)]
    pub fn hsegfon(&self) -> HSEGFON_R {
        HSEGFON_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - HSE oscillator driving capability
    #[inline(always)]
    pub fn hsedrv(&self) -> HSEDRV_R {
        HSEDRV_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSECFGR")
            .field("hsediv2byp", &self.hsediv2byp())
            .field("hsecsson", &self.hsecsson())
            .field("hsecssra", &self.hsecssra())
            .field("hsecssd", &self.hsecssd())
            .field("hsecssbyp", &self.hsecssbyp())
            .field("hsecssbpre", &self.hsecssbpre())
            .field("hsebyp", &self.hsebyp())
            .field("hseext", &self.hseext())
            .field("hsegfon", &self.hsegfon())
            .field("hsedrv", &self.hsedrv())
            .finish()
    }
}
impl W {
    ///Bit 6 - HSE div2 oscillator clock in Bypass mode
    #[inline(always)]
    pub fn hsediv2byp(&mut self) -> HSEDIV2BYP_W<'_, HSECFGRrs> {
        HSEDIV2BYP_W::new(self, 6)
    }
    ///Bit 7 - HSE clock security system (CSS) enable
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W<'_, HSECFGRrs> {
        HSECSSON_W::new(self, 7)
    }
    ///Bit 8 - HSE clock security system (CSS) re-arm function
    #[inline(always)]
    pub fn hsecssra(&mut self) -> HSECSSRA_W<'_, HSECFGRrs> {
        HSECSSRA_W::new(self, 8)
    }
    ///Bit 10 - HSE clock security system (CSS) bypass enable
    #[inline(always)]
    pub fn hsecssbyp(&mut self) -> HSECSSBYP_W<'_, HSECFGRrs> {
        HSECSSBYP_W::new(self, 10)
    }
    ///Bits 11:14 - HSE clock security system (CSS) bypass divider
    #[inline(always)]
    pub fn hsecssbpre(&mut self) -> HSECSSBPRE_W<'_, HSECFGRrs> {
        HSECSSBPRE_W::new(self, 11)
    }
    ///Bit 15 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, HSECFGRrs> {
        HSEBYP_W::new(self, 15)
    }
    ///Bit 16 - HSE clock type in Bypass mode
    #[inline(always)]
    pub fn hseext(&mut self) -> HSEEXT_W<'_, HSECFGRrs> {
        HSEEXT_W::new(self, 16)
    }
    ///Bit 17 - HSE clock glitch filter enable
    #[inline(always)]
    pub fn hsegfon(&mut self) -> HSEGFON_W<'_, HSECFGRrs> {
        HSEGFON_W::new(self, 17)
    }
    ///Bits 18:19 - HSE oscillator driving capability
    #[inline(always)]
    pub fn hsedrv(&mut self) -> HSEDRV_W<'_, HSECFGRrs> {
        HSEDRV_W::new(self, 18)
    }
}
/**RCC HSE configuration register

You can [`read`](crate::Reg::read) this register and get [`hsecfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:HSECFGR)*/
pub struct HSECFGRrs;
impl crate::RegisterSpec for HSECFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hsecfgr::R`](R) reader structure
impl crate::Readable for HSECFGRrs {}
///`write(|w| ..)` method takes [`hsecfgr::W`](W) writer structure
impl crate::Writable for HSECFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSECFGR to value 0x0800
impl crate::Resettable for HSECFGRrs {
    const RESET_VALUE: u32 = 0x0800;
}
