///Register `ITLINE8` reader
pub type R = crate::R<ITLINE8rs>;
/**UCPD1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPD1 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<UCPD1> for bool {
    #[inline(always)]
    fn from(variant: UCPD1) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPD1` reader - UCPD1
pub type UCPD1_R = crate::BitReader<UCPD1>;
impl UCPD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UCPD1 {
        match self.bits {
            false => UCPD1::NotInterrupted,
            true => UCPD1::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == UCPD1::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == UCPD1::Interrupted
    }
}
///Field `UCPD2` reader - UCPD2
pub use UCPD1_R as UCPD2_R;
///Field `USB` reader - USB
pub use UCPD1_R as USB_R;
impl R {
    ///Bit 0 - UCPD1
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD2
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - USB
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE8")
            .field("ucpd1", &self.ucpd1())
            .field("ucpd2", &self.ucpd2())
            .field("usb", &self.usb())
            .finish()
    }
}
/**interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`itline8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE8)*/
pub struct ITLINE8rs;
impl crate::RegisterSpec for ITLINE8rs {
    type Ux = u32;
}
///`read()` method returns [`itline8::R`](R) reader structure
impl crate::Readable for ITLINE8rs {}
///`reset()` method sets ITLINE8 to value 0
impl crate::Resettable for ITLINE8rs {}
