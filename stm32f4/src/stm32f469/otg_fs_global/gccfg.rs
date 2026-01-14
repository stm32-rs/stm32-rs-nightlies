///Register `GCCFG` reader
pub type R = crate::R<GCCFGrs>;
///Register `GCCFG` writer
pub type W = crate::W<GCCFGrs>;
///Field `PWRDWN` reader - Power down
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power down
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBDEN` reader - USB VBUS detection enable
pub type VBDEN_R = crate::BitReader;
///Field `VBDEN` writer - USB VBUS detection enable
pub type VBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdwn", &self.pwrdwn())
            .field("vbden", &self.vbden())
            .finish()
    }
}
impl W {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, GCCFGrs> {
        PWRDWN_W::new(self, 16)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W<'_, GCCFGrs> {
        VBDEN_W::new(self, 21)
    }
}
/**OTG_FS general core configuration register (OTG_FS_GCCFG)

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#OTG_FS_GLOBAL:GCCFG)*/
pub struct GCCFGrs;
impl crate::RegisterSpec for GCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gccfg::R`](R) reader structure
impl crate::Readable for GCCFGrs {}
///`write(|w| ..)` method takes [`gccfg::W`](W) writer structure
impl crate::Writable for GCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCCFG to value 0
impl crate::Resettable for GCCFGrs {}
