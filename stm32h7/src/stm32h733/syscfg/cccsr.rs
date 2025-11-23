///Register `CCCSR` reader
pub type R = crate::R<CCCSRrs>;
///Register `CCCSR` writer
pub type W = crate::W<CCCSRrs>;
///Field `EN` reader - enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS` reader - Code selection
pub type CS_R = crate::BitReader;
///Field `CS` writer - Code selection
pub type CS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READY` reader - Compensation cell ready flag
pub type READY_R = crate::BitReader;
///Field `READY` writer - Compensation cell ready flag
pub type READY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSLV` reader - High-speed at low-voltage
pub type HSLV_R = crate::BitReader;
///Field `HSLV` writer - High-speed at low-voltage
pub type HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Code selection
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Compensation cell ready flag
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - High-speed at low-voltage
    #[inline(always)]
    pub fn hslv(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCSR")
            .field("en", &self.en())
            .field("cs", &self.cs())
            .field("ready", &self.ready())
            .field("hslv", &self.hslv())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CCCSRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Code selection
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<'_, CCCSRrs> {
        CS_W::new(self, 1)
    }
    ///Bit 8 - Compensation cell ready flag
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<'_, CCCSRrs> {
        READY_W::new(self, 8)
    }
    ///Bit 16 - High-speed at low-voltage
    #[inline(always)]
    pub fn hslv(&mut self) -> HSLV_W<'_, CCCSRrs> {
        HSLV_W::new(self, 16)
    }
}
/**compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SYSCFG:CCCSR)*/
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
///`read()` method returns [`cccsr::R`](R) reader structure
impl crate::Readable for CCCSRrs {}
///`write(|w| ..)` method takes [`cccsr::W`](W) writer structure
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCSR to value 0
impl crate::Resettable for CCCSRrs {}
