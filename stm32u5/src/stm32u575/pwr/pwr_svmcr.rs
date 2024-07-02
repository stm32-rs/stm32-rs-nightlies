///Register `PWR_SVMCR` reader
pub type R = crate::R<PWR_SVMCRrs>;
///Register `PWR_SVMCR` writer
pub type W = crate::W<PWR_SVMCRrs>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UVMEN` reader - VDDUSB independent USB voltage monitor enable
pub type UVMEN_R = crate::BitReader;
///Field `UVMEN` writer - VDDUSB independent USB voltage monitor enable
pub type UVMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO2VMEN` reader - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_R = crate::BitReader;
///Field `IO2VMEN` writer - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVM1EN` reader - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_R = crate::BitReader;
///Field `AVM1EN` writer - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVM2EN` reader - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_R = crate::BitReader;
///Field `AVM2EN` writer - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USV` reader - VDDUSB independent USB supply valid
pub type USV_R = crate::BitReader;
///Field `USV` writer - VDDUSB independent USB supply valid
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO2SV` reader - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_R = crate::BitReader;
///Field `IO2SV` writer - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASV` reader - VDDA independent analog supply valid
pub type ASV_R = crate::BitReader;
///Field `ASV` writer - VDDA independent analog supply valid
pub type ASV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    pub fn uvmen(&self) -> UVMEN_R {
        UVMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    pub fn io2vmen(&self) -> IO2VMEN_R {
        IO2VMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    pub fn avm1en(&self) -> AVM1EN_R {
        AVM1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    pub fn avm2en(&self) -> AVM2EN_R {
        AVM2EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - VDDUSB independent USB supply valid
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn io2sv(&self) -> IO2SV_R {
        IO2SV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - VDDA independent analog supply valid
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SVMCR")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("uvmen", &self.uvmen())
            .field("io2vmen", &self.io2vmen())
            .field("avm1en", &self.avm1en())
            .field("avm2en", &self.avm2en())
            .field("usv", &self.usv())
            .field("io2sv", &self.io2sv())
            .field("asv", &self.asv())
            .finish()
    }
}
impl W {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<PWR_SVMCRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    #[must_use]
    pub fn pvdls(&mut self) -> PVDLS_W<PWR_SVMCRrs> {
        PVDLS_W::new(self, 5)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    #[must_use]
    pub fn uvmen(&mut self) -> UVMEN_W<PWR_SVMCRrs> {
        UVMEN_W::new(self, 24)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    #[must_use]
    pub fn io2vmen(&mut self) -> IO2VMEN_W<PWR_SVMCRrs> {
        IO2VMEN_W::new(self, 25)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    #[must_use]
    pub fn avm1en(&mut self) -> AVM1EN_W<PWR_SVMCRrs> {
        AVM1EN_W::new(self, 26)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    #[must_use]
    pub fn avm2en(&mut self) -> AVM2EN_W<PWR_SVMCRrs> {
        AVM2EN_W::new(self, 27)
    }
    ///Bit 28 - VDDUSB independent USB supply valid
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<PWR_SVMCRrs> {
        USV_W::new(self, 28)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    #[must_use]
    pub fn io2sv(&mut self) -> IO2SV_W<PWR_SVMCRrs> {
        IO2SV_W::new(self, 29)
    }
    ///Bit 30 - VDDA independent analog supply valid
    #[inline(always)]
    #[must_use]
    pub fn asv(&mut self) -> ASV_W<PWR_SVMCRrs> {
        ASV_W::new(self, 30)
    }
}
/**PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`pwr_svmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_svmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PWR_SVMCR)*/
pub struct PWR_SVMCRrs;
impl crate::RegisterSpec for PWR_SVMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_svmcr::R`](R) reader structure
impl crate::Readable for PWR_SVMCRrs {}
///`write(|w| ..)` method takes [`pwr_svmcr::W`](W) writer structure
impl crate::Writable for PWR_SVMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_SVMCR to value 0
impl crate::Resettable for PWR_SVMCRrs {
    const RESET_VALUE: u32 = 0;
}
