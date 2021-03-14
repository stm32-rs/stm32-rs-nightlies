#[doc = "Reader of register TZSC_SECCFGR1"]
pub type R = crate::R<u32, super::TZSC_SECCFGR1>;
#[doc = "Writer for register TZSC_SECCFGR1"]
pub type W = crate::W<u32, super::TZSC_SECCFGR1>;
#[doc = "Register TZSC_SECCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_SECCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM2SEC`"]
pub type TIM2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2SEC`"]
pub struct TIM2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TIM3SEC`"]
pub type TIM3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM3SEC`"]
pub struct TIM3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TIM4SEC`"]
pub type TIM4SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM4SEC`"]
pub struct TIM4SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIM5SEC`"]
pub type TIM5SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM5SEC`"]
pub struct TIM5SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIM6SEC`"]
pub type TIM6SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM6SEC`"]
pub struct TIM6SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TIM7SEC`"]
pub type TIM7SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM7SEC`"]
pub struct TIM7SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WWDGSEC`"]
pub type WWDGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGSEC`"]
pub struct WWDGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IWDGSEC`"]
pub type IWDGSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDGSEC`"]
pub struct IWDGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI2SEC`"]
pub type SPI2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2SEC`"]
pub struct SPI2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI3SEC`"]
pub type SPI3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI3SEC`"]
pub struct SPI3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USART2SEC`"]
pub type USART2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2SEC`"]
pub struct USART2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USART3SEC`"]
pub type USART3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3SEC`"]
pub struct USART3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UART4SEC`"]
pub type UART4SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART4SEC`"]
pub struct UART4SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `UART5SEC`"]
pub type UART5SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART5SEC`"]
pub struct UART5SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2C1SEC`"]
pub type I2C1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SEC`"]
pub struct I2C1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `I2C2SEC`"]
pub type I2C2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2SEC`"]
pub struct I2C2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `I2C3SEC`"]
pub type I2C3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3SEC`"]
pub struct I2C3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CRSSEC`"]
pub type CRSSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSSEC`"]
pub struct CRSSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DACSEC`"]
pub type DACSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACSEC`"]
pub struct DACSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OPAMPSEC`"]
pub type OPAMPSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAMPSEC`"]
pub struct OPAMPSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `LPTIM1SEC`"]
pub type LPTIM1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1SEC`"]
pub struct LPTIM1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LPUART1SEC`"]
pub type LPUART1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1SEC`"]
pub struct LPUART1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `I2C4SEC`"]
pub type I2C4SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4SEC`"]
pub struct I2C4SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LPTIM2SEC`"]
pub type LPTIM2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2SEC`"]
pub struct LPTIM2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LPTIM3SEC`"]
pub type LPTIM3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3SEC`"]
pub struct LPTIM3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FDCAN1SEC`"]
pub type FDCAN1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDCAN1SEC`"]
pub struct FDCAN1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `USBFSSEC`"]
pub type USBFSSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFSSEC`"]
pub struct USBFSSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `UCPD1SEC`"]
pub type UCPD1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD1SEC`"]
pub struct UCPD1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `VREFBUFSEC`"]
pub type VREFBUFSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFBUFSEC`"]
pub struct VREFBUFSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `COMPSEC`"]
pub type COMPSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPSEC`"]
pub struct COMPSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPSEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIM1SEC`"]
pub type TIM1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1SEC`"]
pub struct TIM1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI1SEC`"]
pub type SPI1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1SEC`"]
pub struct SPI1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM2SEC"]
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3SEC"]
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4SEC"]
    #[inline(always)]
    pub fn tim4sec(&self) -> TIM4SEC_R {
        TIM4SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5SEC"]
    #[inline(always)]
    pub fn tim5sec(&self) -> TIM5SEC_R {
        TIM5SEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6SEC"]
    #[inline(always)]
    pub fn tim6sec(&self) -> TIM6SEC_R {
        TIM6SEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7SEC"]
    #[inline(always)]
    pub fn tim7sec(&self) -> TIM7SEC_R {
        TIM7SEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGSEC"]
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGSEC"]
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2SEC"]
    #[inline(always)]
    pub fn spi2sec(&self) -> SPI2SEC_R {
        SPI2SEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3SEC"]
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2SEC"]
    #[inline(always)]
    pub fn usart2sec(&self) -> USART2SEC_R {
        USART2SEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3SEC"]
    #[inline(always)]
    pub fn usart3sec(&self) -> USART3SEC_R {
        USART3SEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4SEC"]
    #[inline(always)]
    pub fn uart4sec(&self) -> UART4SEC_R {
        UART4SEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5SEC"]
    #[inline(always)]
    pub fn uart5sec(&self) -> UART5SEC_R {
        UART5SEC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1SEC"]
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2SEC"]
    #[inline(always)]
    pub fn i2c2sec(&self) -> I2C2SEC_R {
        I2C2SEC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3SEC"]
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSSEC"]
    #[inline(always)]
    pub fn crssec(&self) -> CRSSEC_R {
        CRSSEC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACSEC"]
    #[inline(always)]
    pub fn dacsec(&self) -> DACSEC_R {
        DACSEC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPSEC"]
    #[inline(always)]
    pub fn opampsec(&self) -> OPAMPSEC_R {
        OPAMPSEC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1SEC"]
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1SEC"]
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4SEC"]
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2SEC"]
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3SEC"]
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1SEC"]
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSSEC"]
    #[inline(always)]
    pub fn usbfssec(&self) -> USBFSSEC_R {
        USBFSSEC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1SEC"]
    #[inline(always)]
    pub fn ucpd1sec(&self) -> UCPD1SEC_R {
        UCPD1SEC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFSEC"]
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPSEC"]
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1SEC"]
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1SEC"]
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2SEC"]
    #[inline(always)]
    pub fn tim2sec(&mut self) -> TIM2SEC_W {
        TIM2SEC_W { w: self }
    }
    #[doc = "Bit 1 - TIM3SEC"]
    #[inline(always)]
    pub fn tim3sec(&mut self) -> TIM3SEC_W {
        TIM3SEC_W { w: self }
    }
    #[doc = "Bit 2 - TIM4SEC"]
    #[inline(always)]
    pub fn tim4sec(&mut self) -> TIM4SEC_W {
        TIM4SEC_W { w: self }
    }
    #[doc = "Bit 3 - TIM5SEC"]
    #[inline(always)]
    pub fn tim5sec(&mut self) -> TIM5SEC_W {
        TIM5SEC_W { w: self }
    }
    #[doc = "Bit 4 - TIM6SEC"]
    #[inline(always)]
    pub fn tim6sec(&mut self) -> TIM6SEC_W {
        TIM6SEC_W { w: self }
    }
    #[doc = "Bit 5 - TIM7SEC"]
    #[inline(always)]
    pub fn tim7sec(&mut self) -> TIM7SEC_W {
        TIM7SEC_W { w: self }
    }
    #[doc = "Bit 6 - WWDGSEC"]
    #[inline(always)]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W {
        WWDGSEC_W { w: self }
    }
    #[doc = "Bit 7 - IWDGSEC"]
    #[inline(always)]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W {
        IWDGSEC_W { w: self }
    }
    #[doc = "Bit 8 - SPI2SEC"]
    #[inline(always)]
    pub fn spi2sec(&mut self) -> SPI2SEC_W {
        SPI2SEC_W { w: self }
    }
    #[doc = "Bit 9 - SPI3SEC"]
    #[inline(always)]
    pub fn spi3sec(&mut self) -> SPI3SEC_W {
        SPI3SEC_W { w: self }
    }
    #[doc = "Bit 10 - USART2SEC"]
    #[inline(always)]
    pub fn usart2sec(&mut self) -> USART2SEC_W {
        USART2SEC_W { w: self }
    }
    #[doc = "Bit 11 - USART3SEC"]
    #[inline(always)]
    pub fn usart3sec(&mut self) -> USART3SEC_W {
        USART3SEC_W { w: self }
    }
    #[doc = "Bit 12 - UART4SEC"]
    #[inline(always)]
    pub fn uart4sec(&mut self) -> UART4SEC_W {
        UART4SEC_W { w: self }
    }
    #[doc = "Bit 13 - UART5SEC"]
    #[inline(always)]
    pub fn uart5sec(&mut self) -> UART5SEC_W {
        UART5SEC_W { w: self }
    }
    #[doc = "Bit 14 - I2C1SEC"]
    #[inline(always)]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W {
        I2C1SEC_W { w: self }
    }
    #[doc = "Bit 15 - I2C2SEC"]
    #[inline(always)]
    pub fn i2c2sec(&mut self) -> I2C2SEC_W {
        I2C2SEC_W { w: self }
    }
    #[doc = "Bit 16 - I2C3SEC"]
    #[inline(always)]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W {
        I2C3SEC_W { w: self }
    }
    #[doc = "Bit 17 - CRSSEC"]
    #[inline(always)]
    pub fn crssec(&mut self) -> CRSSEC_W {
        CRSSEC_W { w: self }
    }
    #[doc = "Bit 18 - DACSEC"]
    #[inline(always)]
    pub fn dacsec(&mut self) -> DACSEC_W {
        DACSEC_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPSEC"]
    #[inline(always)]
    pub fn opampsec(&mut self) -> OPAMPSEC_W {
        OPAMPSEC_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1SEC"]
    #[inline(always)]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W {
        LPTIM1SEC_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1SEC"]
    #[inline(always)]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W {
        LPUART1SEC_W { w: self }
    }
    #[doc = "Bit 22 - I2C4SEC"]
    #[inline(always)]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W {
        I2C4SEC_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2SEC"]
    #[inline(always)]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W {
        LPTIM2SEC_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3SEC"]
    #[inline(always)]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W {
        LPTIM3SEC_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1SEC"]
    #[inline(always)]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W {
        FDCAN1SEC_W { w: self }
    }
    #[doc = "Bit 26 - USBFSSEC"]
    #[inline(always)]
    pub fn usbfssec(&mut self) -> USBFSSEC_W {
        USBFSSEC_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1SEC"]
    #[inline(always)]
    pub fn ucpd1sec(&mut self) -> UCPD1SEC_W {
        UCPD1SEC_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFSEC"]
    #[inline(always)]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W {
        VREFBUFSEC_W { w: self }
    }
    #[doc = "Bit 29 - COMPSEC"]
    #[inline(always)]
    pub fn compsec(&mut self) -> COMPSEC_W {
        COMPSEC_W { w: self }
    }
    #[doc = "Bit 30 - TIM1SEC"]
    #[inline(always)]
    pub fn tim1sec(&mut self) -> TIM1SEC_W {
        TIM1SEC_W { w: self }
    }
    #[doc = "Bit 31 - SPI1SEC"]
    #[inline(always)]
    pub fn spi1sec(&mut self) -> SPI1SEC_W {
        SPI1SEC_W { w: self }
    }
}
