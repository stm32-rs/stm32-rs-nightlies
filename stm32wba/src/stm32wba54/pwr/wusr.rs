///Register `WUSR` reader
pub type R = crate::R<WUSRrs>;
///Field `WUF1` reader - Wakeup and interrupt pending flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR or by hardware when WUPEN1 = 0.
pub type WUF1_R = crate::BitReader;
///Field `WUF2` reader - Wakeup and interrupt pending flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR or by hardware when WUPEN2 = 0.
pub type WUF2_R = crate::BitReader;
///Field `WUF3` reader - Wakeup and interrupt pending flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR or by hardware when WUPEN3 = 0.
pub type WUF3_R = crate::BitReader;
///Field `WUF4` reader - Wakeup and interrupt pending flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR or by hardware when WUPEN4 = 0.
pub type WUF4_R = crate::BitReader;
///Field `WUF5` reader - Wakeup and interrupt pending flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR or by hardware when WUPEN5 = 0.
pub type WUF5_R = crate::BitReader;
///Field `WUF6` reader - Wakeup and interrupt pending flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL6 different 11, or by hardware when WUPEN6 = 0. When WUSEL6 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL6 = 11, no WKUP interrupt is generated
pub type WUF6_R = crate::BitReader;
///Field `WUF7` reader - Wakeup and interrupt pending flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL7 different 11, or by hardware when WUPEN7 = 0. When WUSEL7 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL7 = 11, no WKUP interrupt is generated.
pub type WUF7_R = crate::BitReader;
///Field `WUF8` reader - Wakeup and interrupt pending flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL8 different 11, or by hardware when WUPEN8 = 0. When WUSEL8 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL8 = 11, no WKUP interrupt is generated
pub type WUF8_R = crate::BitReader;
impl R {
    ///Bit 0 - Wakeup and interrupt pending flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR or by hardware when WUPEN1 = 0.
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup and interrupt pending flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR or by hardware when WUPEN2 = 0.
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup and interrupt pending flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR or by hardware when WUPEN3 = 0.
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup and interrupt pending flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR or by hardware when WUPEN4 = 0.
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup and interrupt pending flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR or by hardware when WUPEN5 = 0.
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup and interrupt pending flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL6 different 11, or by hardware when WUPEN6 = 0. When WUSEL6 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL6 = 11, no WKUP interrupt is generated
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup and interrupt pending flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL7 different 11, or by hardware when WUPEN7 = 0. When WUSEL7 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL7 = 11, no WKUP interrupt is generated.
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup and interrupt pending flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL8 different 11, or by hardware when WUPEN8 = 0. When WUSEL8 = 11, this bit is cleared by hardware when all associated internal wakeup source are cleared. When WUSEL8 = 11, no WKUP interrupt is generated
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUSR")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("wuf6", &self.wuf6())
            .field("wuf7", &self.wuf7())
            .field("wuf8", &self.wuf8())
            .finish()
    }
}
/**PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#PWR:WUSR)*/
pub struct WUSRrs;
impl crate::RegisterSpec for WUSRrs {
    type Ux = u32;
}
///`read()` method returns [`wusr::R`](R) reader structure
impl crate::Readable for WUSRrs {}
///`reset()` method sets WUSR to value 0
impl crate::Resettable for WUSRrs {}
