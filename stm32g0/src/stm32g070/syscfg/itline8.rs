///Register `ITLINE8` reader
pub type R = crate::R<ITLINE8rs>;
/**USB

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<USB> for bool {
    #[inline(always)]
    fn from(variant: USB) -> Self {
        variant as u8 != 0
    }
}
///Field `USB` reader - USB
pub type USB_R = crate::BitReader<USB>;
impl USB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USB {
        match self.bits {
            false => USB::NotInterrupted,
            true => USB::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == USB::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == USB::Interrupted
    }
}
impl R {
    ///Bit 2 - USB
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE8").field("usb", &self.usb()).finish()
    }
}
/**interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`itline8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE8)*/
pub struct ITLINE8rs;
impl crate::RegisterSpec for ITLINE8rs {
    type Ux = u32;
}
///`read()` method returns [`itline8::R`](R) reader structure
impl crate::Readable for ITLINE8rs {}
///`reset()` method sets ITLINE8 to value 0
impl crate::Resettable for ITLINE8rs {}
