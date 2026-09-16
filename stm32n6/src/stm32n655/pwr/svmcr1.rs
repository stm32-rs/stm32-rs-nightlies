///Register `SVMCR1` reader
pub type R = crate::R<SVMCR1rs>;
///Register `SVMCR1` writer
pub type W = crate::W<SVMCR1rs>;
///Field `VDDIO4VMEN` reader - Vless thansub>DDIO4 less than/sub>independent I/O voltage monitor enable
pub type VDDIO4VMEN_R = crate::BitReader;
///Field `VDDIO4VMEN` writer - Vless thansub>DDIO4 less than/sub>independent I/O voltage monitor enable
pub type VDDIO4VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO4SV` reader - Vless thansub>DDIO4 less than/sub>independent I/O supply valid.
pub type VDDIO4SV_R = crate::BitReader;
///Field `VDDIO4SV` writer - Vless thansub>DDIO4 less than/sub>independent I/O supply valid.
pub type VDDIO4SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO4RDY` reader - Vless thansub>DDIO4 less than/sub>ready
pub type VDDIO4RDY_R = crate::BitReader;
///Field `VDDIO4VRSEL` reader - Vless thansub>DDIO4less than/sub> I/O voltage range selection
pub type VDDIO4VRSEL_R = crate::BitReader;
///Field `VDDIO4VRSEL` writer - Vless thansub>DDIO4less than/sub> I/O voltage range selection
pub type VDDIO4VRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO4VRSTBY` reader - Vless thansub>DDIO4less than/sub> I/O voltage range Standby mode
pub type VDDIO4VRSTBY_R = crate::BitReader;
///Field `VDDIO4VRSTBY` writer - Vless thansub>DDIO4less than/sub> I/O voltage range Standby mode
pub type VDDIO4VRSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Vless thansub>DDIO4 less than/sub>independent I/O voltage monitor enable
    #[inline(always)]
    pub fn vddio4vmen(&self) -> VDDIO4VMEN_R {
        VDDIO4VMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Vless thansub>DDIO4 less than/sub>independent I/O supply valid.
    #[inline(always)]
    pub fn vddio4sv(&self) -> VDDIO4SV_R {
        VDDIO4SV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Vless thansub>DDIO4 less than/sub>ready
    #[inline(always)]
    pub fn vddio4rdy(&self) -> VDDIO4RDY_R {
        VDDIO4RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Vless thansub>DDIO4less than/sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio4vrsel(&self) -> VDDIO4VRSEL_R {
        VDDIO4VRSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Vless thansub>DDIO4less than/sub> I/O voltage range Standby mode
    #[inline(always)]
    pub fn vddio4vrstby(&self) -> VDDIO4VRSTBY_R {
        VDDIO4VRSTBY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMCR1")
            .field("vddio4vmen", &self.vddio4vmen())
            .field("vddio4sv", &self.vddio4sv())
            .field("vddio4rdy", &self.vddio4rdy())
            .field("vddio4vrsel", &self.vddio4vrsel())
            .field("vddio4vrstby", &self.vddio4vrstby())
            .finish()
    }
}
impl W {
    ///Bit 0 - Vless thansub>DDIO4 less than/sub>independent I/O voltage monitor enable
    #[inline(always)]
    pub fn vddio4vmen(&mut self) -> VDDIO4VMEN_W<'_, SVMCR1rs> {
        VDDIO4VMEN_W::new(self, 0)
    }
    ///Bit 8 - Vless thansub>DDIO4 less than/sub>independent I/O supply valid.
    #[inline(always)]
    pub fn vddio4sv(&mut self) -> VDDIO4SV_W<'_, SVMCR1rs> {
        VDDIO4SV_W::new(self, 8)
    }
    ///Bit 24 - Vless thansub>DDIO4less than/sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio4vrsel(&mut self) -> VDDIO4VRSEL_W<'_, SVMCR1rs> {
        VDDIO4VRSEL_W::new(self, 24)
    }
    ///Bit 25 - Vless thansub>DDIO4less than/sub> I/O voltage range Standby mode
    #[inline(always)]
    pub fn vddio4vrstby(&mut self) -> VDDIO4VRSTBY_W<'_, SVMCR1rs> {
        VDDIO4VRSTBY_W::new(self, 25)
    }
}
/**PWR supply voltage monitoring control register 1

You can [`read`](crate::Reg::read) this register and get [`svmcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PWR:SVMCR1)*/
pub struct SVMCR1rs;
impl crate::RegisterSpec for SVMCR1rs {
    type Ux = u32;
}
///`read()` method returns [`svmcr1::R`](R) reader structure
impl crate::Readable for SVMCR1rs {}
///`write(|w| ..)` method takes [`svmcr1::W`](W) writer structure
impl crate::Writable for SVMCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SVMCR1 to value 0
impl crate::Resettable for SVMCR1rs {}
