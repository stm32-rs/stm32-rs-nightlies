///Register `PHYCTRL_DIG_USR` reader
pub type R = crate::R<PHYCTRL_DIG_USRrs>;
///Register `PHYCTRL_DIG_USR` writer
pub type W = crate::W<PHYCTRL_DIG_USRrs>;
///Field `RXTXPHY` reader - RXTXPHY selection.
pub type RXTXPHY_R = crate::FieldReader;
///Field `RXTXPHY` writer - RXTXPHY selection.
pub type RXTXPHY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - RXTXPHY selection.
    #[inline(always)]
    pub fn rxtxphy(&self) -> RXTXPHY_R {
        RXTXPHY_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHYCTRL_DIG_USR")
            .field("rxtxphy", &self.rxtxphy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - RXTXPHY selection.
    #[inline(always)]
    pub fn rxtxphy(&mut self) -> RXTXPHY_W<'_, PHYCTRL_DIG_USRrs> {
        RXTXPHY_W::new(self, 0)
    }
}
/**PHYCTRL_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`phyctrl_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phyctrl_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:PHYCTRL_DIG_USR)*/
pub struct PHYCTRL_DIG_USRrs;
impl crate::RegisterSpec for PHYCTRL_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`phyctrl_dig_usr::R`](R) reader structure
impl crate::Readable for PHYCTRL_DIG_USRrs {}
///`write(|w| ..)` method takes [`phyctrl_dig_usr::W`](W) writer structure
impl crate::Writable for PHYCTRL_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHYCTRL_DIG_USR to value 0
impl crate::Resettable for PHYCTRL_DIG_USRrs {}
