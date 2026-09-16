///Register `MC_APB4ENCLRR` reader
pub type R = crate::R<MC_APB4ENCLRRrs>;
///Register `MC_APB4ENCLRR` writer
pub type W = crate::W<MC_APB4ENCLRRrs>;
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
        f.debug_struct("MC_APB4ENCLRR")
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .field("ddrperfmen", &self.ddrperfmen())
            .field("usbphyen", &self.usbphyen())
            .field("stgenroen", &self.stgenroen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDCEN
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, MC_APB4ENCLRRrs> {
        LTDCEN_W::new(self, 0)
    }
    ///Bit 4 - DSIEN
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<'_, MC_APB4ENCLRRrs> {
        DSIEN_W::new(self, 4)
    }
    ///Bit 8 - DDRPERFMEN
    #[inline(always)]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W<'_, MC_APB4ENCLRRrs> {
        DDRPERFMEN_W::new(self, 8)
    }
    ///Bit 16 - USBPHYEN
    #[inline(always)]
    pub fn usbphyen(&mut self) -> USBPHYEN_W<'_, MC_APB4ENCLRRrs> {
        USBPHYEN_W::new(self, 16)
    }
    ///Bit 20 - STGENROEN
    #[inline(always)]
    pub fn stgenroen(&mut self) -> STGENROEN_W<'_, MC_APB4ENCLRRrs> {
        STGENROEN_W::new(self, 20)
    }
}
/**This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb4enclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4enclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_APB4ENCLRR)*/
pub struct MC_APB4ENCLRRrs;
impl crate::RegisterSpec for MC_APB4ENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_apb4enclrr::R`](R) reader structure
impl crate::Readable for MC_APB4ENCLRRrs {}
///`write(|w| ..)` method takes [`mc_apb4enclrr::W`](W) writer structure
impl crate::Writable for MC_APB4ENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_APB4ENCLRR to value 0
impl crate::Resettable for MC_APB4ENCLRRrs {}
