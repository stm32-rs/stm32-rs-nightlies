///Register `C2APB3SMENR` reader
pub type R = crate::R<C2APB3SMENRrs>;
///Register `C2APB3SMENR` writer
pub type W = crate::W<C2APB3SMENRrs>;
///Field `BLESMEN` reader - BLE interface clocks enable during CPU2 Sleep mode
pub type BLESMEN_R = crate::BitReader;
///Field `BLESMEN` writer - BLE interface clocks enable during CPU2 Sleep mode
pub type BLESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMEN802` reader - 802.15.4 interface clocks enable during CPU2 Sleep modes
pub type SMEN802_R = crate::BitReader;
///Field `SMEN802` writer - 802.15.4 interface clocks enable during CPU2 Sleep modes
pub type SMEN802_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BLE interface clocks enable during CPU2 Sleep mode
    #[inline(always)]
    pub fn blesmen(&self) -> BLESMEN_R {
        BLESMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes
    #[inline(always)]
    pub fn smen802(&self) -> SMEN802_R {
        SMEN802_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB3SMENR")
            .field("smen802", &self.smen802())
            .field("blesmen", &self.blesmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - BLE interface clocks enable during CPU2 Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn blesmen(&mut self) -> BLESMEN_W<C2APB3SMENRrs> {
        BLESMEN_W::new(self, 0)
    }
    ///Bit 1 - 802.15.4 interface clocks enable during CPU2 Sleep modes
    #[inline(always)]
    #[must_use]
    pub fn smen802(&mut self) -> SMEN802_W<C2APB3SMENRrs> {
        SMEN802_W::new(self, 1)
    }
}
/**CPU2 APB3SMENR

You can [`read`](crate::Reg::read) this register and get [`c2apb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2APB3SMENR)*/
pub struct C2APB3SMENRrs;
impl crate::RegisterSpec for C2APB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb3smenr::R`](R) reader structure
impl crate::Readable for C2APB3SMENRrs {}
///`write(|w| ..)` method takes [`c2apb3smenr::W`](W) writer structure
impl crate::Writable for C2APB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2APB3SMENR to value 0x03
impl crate::Resettable for C2APB3SMENRrs {
    const RESET_VALUE: u32 = 0x03;
}
