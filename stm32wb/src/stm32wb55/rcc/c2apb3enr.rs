///Register `C2APB3ENR` reader
pub type R = crate::R<C2APB3ENRrs>;
///Register `C2APB3ENR` writer
pub type W = crate::W<C2APB3ENRrs>;
/**CPU2 BLE interface clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLEEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<BLEEN> for bool {
    #[inline(always)]
    fn from(variant: BLEEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BLEEN` reader - CPU2 BLE interface clock enable
pub type BLEEN_R = crate::BitReader<BLEEN>;
impl BLEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BLEEN {
        match self.bits {
            false => BLEEN::Disabled,
            true => BLEEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLEEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLEEN::Enabled
    }
}
///Field `BLEEN` writer - CPU2 BLE interface clock enable
pub type BLEEN_W<'a, REG> = crate::BitWriter<'a, REG, BLEEN>;
impl<'a, REG> BLEEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLEEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLEEN::Enabled)
    }
}
///Field `EN802` reader - CPU2 802.15.4 interface clock enable
pub use BLEEN_R as EN802_R;
///Field `EN802` writer - CPU2 802.15.4 interface clock enable
pub use BLEEN_W as EN802_W;
impl R {
    ///Bit 0 - CPU2 BLE interface clock enable
    #[inline(always)]
    pub fn bleen(&self) -> BLEEN_R {
        BLEEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 802.15.4 interface clock enable
    #[inline(always)]
    pub fn en802(&self) -> EN802_R {
        EN802_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB3ENR")
            .field("bleen", &self.bleen())
            .field("en802", &self.en802())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 BLE interface clock enable
    #[inline(always)]
    pub fn bleen(&mut self) -> BLEEN_W<'_, C2APB3ENRrs> {
        BLEEN_W::new(self, 0)
    }
    ///Bit 1 - CPU2 802.15.4 interface clock enable
    #[inline(always)]
    pub fn en802(&mut self) -> EN802_W<'_, C2APB3ENRrs> {
        EN802_W::new(self, 1)
    }
}
/**CPU2 APB3ENR

You can [`read`](crate::Reg::read) this register and get [`c2apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2APB3ENR)*/
pub struct C2APB3ENRrs;
impl crate::RegisterSpec for C2APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb3enr::R`](R) reader structure
impl crate::Readable for C2APB3ENRrs {}
///`write(|w| ..)` method takes [`c2apb3enr::W`](W) writer structure
impl crate::Writable for C2APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB3ENR to value 0
impl crate::Resettable for C2APB3ENRrs {}
