///Register `SVMCR3` reader
pub type R = crate::R<SVMCR3rs>;
///Register `SVMCR3` writer
pub type W = crate::W<SVMCR3rs>;
///Field `VDDIO2VMEN` reader - V less than sub>DDIO2 less than /sub>independent voltage monitor enable
pub type VDDIO2VMEN_R = crate::BitReader;
///Field `VDDIO2VMEN` writer - V less than sub>DDIO2 less than /sub>independent voltage monitor enable
pub type VDDIO2VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO3VMEN` reader - V less than sub>DDIO3 less than /sub>independent voltage monitor enable
pub type VDDIO3VMEN_R = crate::BitReader;
///Field `VDDIO3VMEN` writer - V less than sub>DDIO3 less than /sub>independent voltage monitor enable
pub type VDDIO3VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33VMEN` reader - V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable.
pub type USB33VMEN_R = crate::BitReader;
///Field `USB33VMEN` writer - V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable.
pub type USB33VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVMEN` reader - V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable
pub type AVMEN_R = crate::BitReader;
///Field `AVMEN` writer - V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable
pub type AVMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO2SV` reader - V less than sub>DDIO2 less than /sub>independent supply valid.
pub type VDDIO2SV_R = crate::BitReader;
///Field `VDDIO2SV` writer - V less than sub>DDIO2 less than /sub>independent supply valid.
pub type VDDIO2SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO3SV` reader - V less than sub>DDIO3 less than /sub>independent supply valid
pub type VDDIO3SV_R = crate::BitReader;
///Field `VDDIO3SV` writer - V less than sub>DDIO3 less than /sub>independent supply valid
pub type VDDIO3SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33SV` reader - V less than sub>DD33USB less than /sub>independent supply valid
pub type USB33SV_R = crate::BitReader;
///Field `USB33SV` writer - V less than sub>DD33USB less than /sub>independent supply valid
pub type USB33SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASV` reader - V less than sub>DDA18ADC less than /sub>independent supply valid
pub type ASV_R = crate::BitReader;
///Field `ASV` writer - V less than sub>DDA18ADC less than /sub>independent supply valid
pub type ASV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO2RDY` reader - V less than sub>DDIO2 less than /sub>ready
pub type VDDIO2RDY_R = crate::BitReader;
///Field `VDDIO3RDY` reader - V less than sub>DDIO3 less than /sub>ready
pub type VDDIO3RDY_R = crate::BitReader;
///Field `USB33RDY` reader - V less than sub>DD33USB less than /sub>ready
pub type USB33RDY_R = crate::BitReader;
///Field `ARDY` reader - V less than sub>DDA18ADC less than /sub>ready
pub type ARDY_R = crate::BitReader;
///Field `VDDIOVRSEL` reader - V less than sub>DD less than /sub> I/O voltage range selection
pub type VDDIOVRSEL_R = crate::BitReader;
///Field `VDDIOVRSEL` writer - V less than sub>DD less than /sub> I/O voltage range selection
pub type VDDIOVRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO2VRSEL` reader - V less than sub>DDIO2 less than /sub> I/O voltage range selection
pub type VDDIO2VRSEL_R = crate::BitReader;
///Field `VDDIO2VRSEL` writer - V less than sub>DDIO2 less than /sub> I/O voltage range selection
pub type VDDIO2VRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO3VRSEL` reader - V less than sub>DDIO3 less than /sub> I/O voltage range selection
pub type VDDIO3VRSEL_R = crate::BitReader;
///Field `VDDIO3VRSEL` writer - V less than sub>DDIO3 less than /sub> I/O voltage range selection
pub type VDDIO3VRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - V less than sub>DDIO2 less than /sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio2vmen(&self) -> VDDIO2VMEN_R {
        VDDIO2VMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - V less than sub>DDIO3 less than /sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio3vmen(&self) -> VDDIO3VMEN_R {
        VDDIO3VMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable.
    #[inline(always)]
    pub fn usb33vmen(&self) -> USB33VMEN_R {
        USB33VMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable
    #[inline(always)]
    pub fn avmen(&self) -> AVMEN_R {
        AVMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - V less than sub>DDIO2 less than /sub>independent supply valid.
    #[inline(always)]
    pub fn vddio2sv(&self) -> VDDIO2SV_R {
        VDDIO2SV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - V less than sub>DDIO3 less than /sub>independent supply valid
    #[inline(always)]
    pub fn vddio3sv(&self) -> VDDIO3SV_R {
        VDDIO3SV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - V less than sub>DD33USB less than /sub>independent supply valid
    #[inline(always)]
    pub fn usb33sv(&self) -> USB33SV_R {
        USB33SV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - V less than sub>DDA18ADC less than /sub>independent supply valid
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - V less than sub>DDIO2 less than /sub>ready
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - V less than sub>DDIO3 less than /sub>ready
    #[inline(always)]
    pub fn vddio3rdy(&self) -> VDDIO3RDY_R {
        VDDIO3RDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - V less than sub>DD33USB less than /sub>ready
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - V less than sub>DDA18ADC less than /sub>ready
    #[inline(always)]
    pub fn ardy(&self) -> ARDY_R {
        ARDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - V less than sub>DD less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddiovrsel(&self) -> VDDIOVRSEL_R {
        VDDIOVRSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - V less than sub>DDIO2 less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio2vrsel(&self) -> VDDIO2VRSEL_R {
        VDDIO2VRSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - V less than sub>DDIO3 less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio3vrsel(&self) -> VDDIO3VRSEL_R {
        VDDIO3VRSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMCR3")
            .field("vddio2vmen", &self.vddio2vmen())
            .field("vddio3vmen", &self.vddio3vmen())
            .field("usb33vmen", &self.usb33vmen())
            .field("avmen", &self.avmen())
            .field("vddio2sv", &self.vddio2sv())
            .field("vddio3sv", &self.vddio3sv())
            .field("usb33sv", &self.usb33sv())
            .field("asv", &self.asv())
            .field("vddio2rdy", &self.vddio2rdy())
            .field("vddio3rdy", &self.vddio3rdy())
            .field("usb33rdy", &self.usb33rdy())
            .field("ardy", &self.ardy())
            .field("vddiovrsel", &self.vddiovrsel())
            .field("vddio2vrsel", &self.vddio2vrsel())
            .field("vddio3vrsel", &self.vddio3vrsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - V less than sub>DDIO2 less than /sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio2vmen(&mut self) -> VDDIO2VMEN_W<'_, SVMCR3rs> {
        VDDIO2VMEN_W::new(self, 0)
    }
    ///Bit 1 - V less than sub>DDIO3 less than /sub>independent voltage monitor enable
    #[inline(always)]
    pub fn vddio3vmen(&mut self) -> VDDIO3VMEN_W<'_, SVMCR3rs> {
        VDDIO3VMEN_W::new(self, 1)
    }
    ///Bit 2 - V less than sub>DD33USB less than /sub>independent USB 33 voltage monitor enable.
    #[inline(always)]
    pub fn usb33vmen(&mut self) -> USB33VMEN_W<'_, SVMCR3rs> {
        USB33VMEN_W::new(self, 2)
    }
    ///Bit 4 - V less than sub>DDA18ADC less than /sub>independent ADC voltage monitor enable
    #[inline(always)]
    pub fn avmen(&mut self) -> AVMEN_W<'_, SVMCR3rs> {
        AVMEN_W::new(self, 4)
    }
    ///Bit 8 - V less than sub>DDIO2 less than /sub>independent supply valid.
    #[inline(always)]
    pub fn vddio2sv(&mut self) -> VDDIO2SV_W<'_, SVMCR3rs> {
        VDDIO2SV_W::new(self, 8)
    }
    ///Bit 9 - V less than sub>DDIO3 less than /sub>independent supply valid
    #[inline(always)]
    pub fn vddio3sv(&mut self) -> VDDIO3SV_W<'_, SVMCR3rs> {
        VDDIO3SV_W::new(self, 9)
    }
    ///Bit 10 - V less than sub>DD33USB less than /sub>independent supply valid
    #[inline(always)]
    pub fn usb33sv(&mut self) -> USB33SV_W<'_, SVMCR3rs> {
        USB33SV_W::new(self, 10)
    }
    ///Bit 12 - V less than sub>DDA18ADC less than /sub>independent supply valid
    #[inline(always)]
    pub fn asv(&mut self) -> ASV_W<'_, SVMCR3rs> {
        ASV_W::new(self, 12)
    }
    ///Bit 24 - V less than sub>DD less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddiovrsel(&mut self) -> VDDIOVRSEL_W<'_, SVMCR3rs> {
        VDDIOVRSEL_W::new(self, 24)
    }
    ///Bit 25 - V less than sub>DDIO2 less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio2vrsel(&mut self) -> VDDIO2VRSEL_W<'_, SVMCR3rs> {
        VDDIO2VRSEL_W::new(self, 25)
    }
    ///Bit 26 - V less than sub>DDIO3 less than /sub> I/O voltage range selection
    #[inline(always)]
    pub fn vddio3vrsel(&mut self) -> VDDIO3VRSEL_W<'_, SVMCR3rs> {
        VDDIO3VRSEL_W::new(self, 26)
    }
}
/**PWR supply voltage monitoring control register 3

You can [`read`](crate::Reg::read) this register and get [`svmcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:SVMCR3)*/
pub struct SVMCR3rs;
impl crate::RegisterSpec for SVMCR3rs {
    type Ux = u32;
}
///`read()` method returns [`svmcr3::R`](R) reader structure
impl crate::Readable for SVMCR3rs {}
///`write(|w| ..)` method takes [`svmcr3::W`](W) writer structure
impl crate::Writable for SVMCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SVMCR3 to value 0
impl crate::Resettable for SVMCR3rs {}
