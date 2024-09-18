///Register `RCC_MP_APB4ENSETR` reader
pub type R = crate::R<RCC_MP_APB4ENSETRrs>;
///Register `RCC_MP_APB4ENSETR` writer
pub type W = crate::W<RCC_MP_APB4ENSETRrs>;
///Field `LTDCEN` reader - LTDCEN
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LTDCEN
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSIEN` reader - DSIEN
pub type DSIEN_R = crate::BitReader;
///Field `DSIEN` writer - DSIEN
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPERFMEN` reader - DDRPERFMEN
pub type DDRPERFMEN_R = crate::BitReader;
///Field `DDRPERFMEN` writer - DDRPERFMEN
pub type DDRPERFMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG2APBEN` reader - IWDG2APBEN
pub type IWDG2APBEN_R = crate::BitReader;
///Field `IWDG2APBEN` writer - IWDG2APBEN
pub type IWDG2APBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBPHYEN` reader - USBPHYEN
pub type USBPHYEN_R = crate::BitReader;
///Field `USBPHYEN` writer - USBPHYEN
pub type USBPHYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENROEN` reader - STGENROEN
pub type STGENROEN_R = crate::BitReader;
///Field `STGENROEN` writer - STGENROEN
pub type STGENROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LTDCEN
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DSIEN
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DDRPERFMEN
    #[inline(always)]
    pub fn ddrperfmen(&self) -> DDRPERFMEN_R {
        DDRPERFMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - IWDG2APBEN
    #[inline(always)]
    pub fn iwdg2apben(&self) -> IWDG2APBEN_R {
        IWDG2APBEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - USBPHYEN
    #[inline(always)]
    pub fn usbphyen(&self) -> USBPHYEN_R {
        USBPHYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENROEN
    #[inline(always)]
    pub fn stgenroen(&self) -> STGENROEN_R {
        STGENROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MP_APB4ENSETR")
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .field("ddrperfmen", &self.ddrperfmen())
            .field("iwdg2apben", &self.iwdg2apben())
            .field("usbphyen", &self.usbphyen())
            .field("stgenroen", &self.stgenroen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDCEN
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<RCC_MP_APB4ENSETRrs> {
        LTDCEN_W::new(self, 0)
    }
    ///Bit 4 - DSIEN
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<RCC_MP_APB4ENSETRrs> {
        DSIEN_W::new(self, 4)
    }
    ///Bit 8 - DDRPERFMEN
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W<RCC_MP_APB4ENSETRrs> {
        DDRPERFMEN_W::new(self, 8)
    }
    ///Bit 15 - IWDG2APBEN
    #[inline(always)]
    #[must_use]
    pub fn iwdg2apben(&mut self) -> IWDG2APBEN_W<RCC_MP_APB4ENSETRrs> {
        IWDG2APBEN_W::new(self, 15)
    }
    ///Bit 16 - USBPHYEN
    #[inline(always)]
    #[must_use]
    pub fn usbphyen(&mut self) -> USBPHYEN_W<RCC_MP_APB4ENSETRrs> {
        USBPHYEN_W::new(self, 16)
    }
    ///Bit 20 - STGENROEN
    #[inline(always)]
    #[must_use]
    pub fn stgenroen(&mut self) -> STGENROEN_W<RCC_MP_APB4ENSETRrs> {
        STGENROEN_W::new(self, 20)
    }
}
/**This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`rcc_mp_apb4ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_mp_apb4ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_MP_APB4ENSETR)*/
pub struct RCC_MP_APB4ENSETRrs;
impl crate::RegisterSpec for RCC_MP_APB4ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_mp_apb4ensetr::R`](R) reader structure
impl crate::Readable for RCC_MP_APB4ENSETRrs {}
///`write(|w| ..)` method takes [`rcc_mp_apb4ensetr::W`](W) writer structure
impl crate::Writable for RCC_MP_APB4ENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_MP_APB4ENSETR to value 0
impl crate::Resettable for RCC_MP_APB4ENSETRrs {
    const RESET_VALUE: u32 = 0;
}
