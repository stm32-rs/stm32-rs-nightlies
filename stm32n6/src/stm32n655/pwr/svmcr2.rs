///Register `SVMCR2` reader
pub type R = crate::R<SVMCR2rs>;
///Register `SVMCR2` writer
pub type W = crate::W<SVMCR2rs>;
///Field `VDDIO5VMEN` reader - Vless thansub>DDIO5 less than/sub>independent voltage monitor enable
pub type VDDIO5VMEN_R = crate::BitReader;
///Field `VDDIO5VMEN` writer - Vless thansub>DDIO5 less than/sub>independent voltage monitor enable
pub type VDDIO5VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO5SV` reader - Vless thansub>DDIO5 less than/sub>independent supply valid
pub type VDDIO5SV_R = crate::BitReader;
///Field `VDDIO5SV` writer - Vless thansub>DDIO5 less than/sub>independent supply valid
pub type VDDIO5SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO5RDY` reader - Vless thansub>DDIO5 less than/sub>ready
pub type VDDIO5RDY_R = crate::BitReader;
///Field `VDDIO5VRSEL` reader - Vless thansub>DDIO5less than/sub> I/O voltage range selection
pub type VDDIO5VRSEL_R = crate::BitReader;
///Field `VDDIO5VRSEL` writer - Vless thansub>DDIO5less than/sub> I/O voltage range selection
pub type VDDIO5VRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO5VRSTBY` reader - Vless thansub>DDIO5less than/sub> I/O voltage range Standby mode
pub type VDDIO5VRSTBY_R = crate::BitReader;
///Field `VDDIO5VRSTBY` writer - Vless thansub>DDIO5less than/sub> I/O voltage range Standby mode
pub type VDDIO5VRSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Vless thansub>DDIO5 less than/sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio5vmen(&self) -> VDDIO5VMEN_R {
        VDDIO5VMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Vless thansub>DDIO5 less than/sub>independent supply valid
    #[inline(always)]
    pub fn vddio5sv(&self) -> VDDIO5SV_R {
        VDDIO5SV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Vless thansub>DDIO5 less than/sub>ready
    #[inline(always)]
    pub fn vddio5rdy(&self) -> VDDIO5RDY_R {
        VDDIO5RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Vless thansub>DDIO5less than/sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio5vrsel(&self) -> VDDIO5VRSEL_R {
        VDDIO5VRSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Vless thansub>DDIO5less than/sub> I/O voltage range Standby mode
    #[inline(always)]
    pub fn vddio5vrstby(&self) -> VDDIO5VRSTBY_R {
        VDDIO5VRSTBY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMCR2")
            .field("vddio5vmen", &self.vddio5vmen())
            .field("vddio5sv", &self.vddio5sv())
            .field("vddio5rdy", &self.vddio5rdy())
            .field("vddio5vrsel", &self.vddio5vrsel())
            .field("vddio5vrstby", &self.vddio5vrstby())
            .finish()
    }
}
impl W {
    ///Bit 0 - Vless thansub>DDIO5 less than/sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio5vmen(&mut self) -> VDDIO5VMEN_W<'_, SVMCR2rs> {
        VDDIO5VMEN_W::new(self, 0)
    }
    ///Bit 8 - Vless thansub>DDIO5 less than/sub>independent supply valid
    #[inline(always)]
    pub fn vddio5sv(&mut self) -> VDDIO5SV_W<'_, SVMCR2rs> {
        VDDIO5SV_W::new(self, 8)
    }
    ///Bit 24 - Vless thansub>DDIO5less than/sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio5vrsel(&mut self) -> VDDIO5VRSEL_W<'_, SVMCR2rs> {
        VDDIO5VRSEL_W::new(self, 24)
    }
    ///Bit 25 - Vless thansub>DDIO5less than/sub> I/O voltage range Standby mode
    #[inline(always)]
    pub fn vddio5vrstby(&mut self) -> VDDIO5VRSTBY_W<'_, SVMCR2rs> {
        VDDIO5VRSTBY_W::new(self, 25)
    }
}
/**PWR supply voltage monitoring control register 2

You can [`read`](crate::Reg::read) this register and get [`svmcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PWR:SVMCR2)*/
pub struct SVMCR2rs;
impl crate::RegisterSpec for SVMCR2rs {
    type Ux = u32;
}
///`read()` method returns [`svmcr2::R`](R) reader structure
impl crate::Readable for SVMCR2rs {}
///`write(|w| ..)` method takes [`svmcr2::W`](W) writer structure
impl crate::Writable for SVMCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SVMCR2 to value 0
impl crate::Resettable for SVMCR2rs {}
