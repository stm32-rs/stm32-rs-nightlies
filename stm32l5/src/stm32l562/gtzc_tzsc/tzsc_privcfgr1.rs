#[doc = "Reader of register TZSC_PRIVCFGR1"]
pub type R = crate::R<u32, super::TZSC_PRIVCFGR1>;
#[doc = "Writer for register TZSC_PRIVCFGR1"]
pub type W = crate::W<u32, super::TZSC_PRIVCFGR1>;
#[doc = "Register TZSC_PRIVCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_PRIVCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM2PRIV`"]
pub type TIM2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2PRIV`"]
pub struct TIM2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2PRIV_W<'a> {
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
#[doc = "Reader of field `TIM3PRIV`"]
pub type TIM3PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM3PRIV`"]
pub struct TIM3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3PRIV_W<'a> {
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
#[doc = "Reader of field `TIM4PRIV`"]
pub type TIM4PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM4PRIV`"]
pub struct TIM4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4PRIV_W<'a> {
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
#[doc = "Reader of field `TIM5PRIV`"]
pub type TIM5PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM5PRIV`"]
pub struct TIM5PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5PRIV_W<'a> {
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
#[doc = "Reader of field `TIM6PRIV`"]
pub type TIM6PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM6PRIV`"]
pub struct TIM6PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6PRIV_W<'a> {
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
#[doc = "Reader of field `TIM7PRIV`"]
pub type TIM7PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM7PRIV`"]
pub struct TIM7PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7PRIV_W<'a> {
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
#[doc = "Reader of field `WWDGPRIV`"]
pub type WWDGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGPRIV`"]
pub struct WWDGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGPRIV_W<'a> {
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
#[doc = "Reader of field `IWDGPRIV`"]
pub type IWDGPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDGPRIV`"]
pub struct IWDGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGPRIV_W<'a> {
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
#[doc = "Reader of field `SPI2PRIV`"]
pub type SPI2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2PRIV`"]
pub struct SPI2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2PRIV_W<'a> {
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
#[doc = "Reader of field `SPI3PRIV`"]
pub type SPI3PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI3PRIV`"]
pub struct SPI3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3PRIV_W<'a> {
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
#[doc = "Reader of field `USART2PRIV`"]
pub type USART2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2PRIV`"]
pub struct USART2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2PRIV_W<'a> {
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
#[doc = "Reader of field `USART3PRIV`"]
pub type USART3PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3PRIV`"]
pub struct USART3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3PRIV_W<'a> {
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
#[doc = "Reader of field `UART4PRIV`"]
pub type UART4PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART4PRIV`"]
pub struct UART4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4PRIV_W<'a> {
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
#[doc = "Reader of field `UART5PRIV`"]
pub type UART5PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART5PRIV`"]
pub struct UART5PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5PRIV_W<'a> {
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
#[doc = "Reader of field `I2C1PRIV`"]
pub type I2C1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1PRIV`"]
pub struct I2C1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1PRIV_W<'a> {
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
#[doc = "Reader of field `I2C2PRIV`"]
pub type I2C2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2PRIV`"]
pub struct I2C2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2PRIV_W<'a> {
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
#[doc = "Reader of field `I2C3PRIV`"]
pub type I2C3PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3PRIV`"]
pub struct I2C3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3PRIV_W<'a> {
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
#[doc = "Reader of field `CRSPRIV`"]
pub type CRSPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSPRIV`"]
pub struct CRSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSPRIV_W<'a> {
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
#[doc = "Reader of field `DACPRIV`"]
pub type DACPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACPRIV`"]
pub struct DACPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DACPRIV_W<'a> {
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
#[doc = "Reader of field `OPAMPPRIV`"]
pub type OPAMPPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAMPPRIV`"]
pub struct OPAMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPPRIV_W<'a> {
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
#[doc = "Reader of field `LPTIM1PRIV`"]
pub type LPTIM1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1PRIV`"]
pub struct LPTIM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1PRIV_W<'a> {
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
#[doc = "Reader of field `LPUART1PRIV`"]
pub type LPUART1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1PRIV`"]
pub struct LPUART1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1PRIV_W<'a> {
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
#[doc = "Reader of field `I2C4PRIV`"]
pub type I2C4PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4PRIV`"]
pub struct I2C4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4PRIV_W<'a> {
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
#[doc = "Reader of field `LPTIM2PRIV`"]
pub type LPTIM2PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2PRIV`"]
pub struct LPTIM2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2PRIV_W<'a> {
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
#[doc = "Reader of field `LPTIM3PRIV`"]
pub type LPTIM3PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3PRIV`"]
pub struct LPTIM3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3PRIV_W<'a> {
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
#[doc = "Reader of field `FDCAN1PRIV`"]
pub type FDCAN1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDCAN1PRIV`"]
pub struct FDCAN1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1PRIV_W<'a> {
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
#[doc = "Reader of field `USBFSPRIV`"]
pub type USBFSPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFSPRIV`"]
pub struct USBFSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSPRIV_W<'a> {
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
#[doc = "Reader of field `UCPD1PRIV`"]
pub type UCPD1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD1PRIV`"]
pub struct UCPD1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1PRIV_W<'a> {
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
#[doc = "Reader of field `VREFBUFPRIV`"]
pub type VREFBUFPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFBUFPRIV`"]
pub struct VREFBUFPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFPRIV_W<'a> {
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
#[doc = "Reader of field `COMPPRIV`"]
pub type COMPPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPPRIV`"]
pub struct COMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPPRIV_W<'a> {
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
#[doc = "Reader of field `TIM1PRIV`"]
pub type TIM1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1PRIV`"]
pub struct TIM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1PRIV_W<'a> {
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
#[doc = "Reader of field `SPI1PRIV`"]
pub type SPI1PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1PRIV`"]
pub struct SPI1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1PRIV_W<'a> {
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
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    pub fn dacpriv(&self) -> DACPRIV_R {
        DACPRIV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    pub fn ucpd1priv(&self) -> UCPD1PRIV_R {
        UCPD1PRIV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W {
        TIM2PRIV_W { w: self }
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W {
        TIM3PRIV_W { w: self }
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W {
        TIM4PRIV_W { w: self }
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W {
        TIM5PRIV_W { w: self }
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W {
        TIM6PRIV_W { w: self }
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W {
        TIM7PRIV_W { w: self }
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W {
        WWDGPRIV_W { w: self }
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W {
        IWDGPRIV_W { w: self }
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W {
        SPI2PRIV_W { w: self }
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W {
        SPI3PRIV_W { w: self }
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    pub fn usart2priv(&mut self) -> USART2PRIV_W {
        USART2PRIV_W { w: self }
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    pub fn usart3priv(&mut self) -> USART3PRIV_W {
        USART3PRIV_W { w: self }
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    pub fn uart4priv(&mut self) -> UART4PRIV_W {
        UART4PRIV_W { w: self }
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    pub fn uart5priv(&mut self) -> UART5PRIV_W {
        UART5PRIV_W { w: self }
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W {
        I2C1PRIV_W { w: self }
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W {
        I2C2PRIV_W { w: self }
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W {
        I2C3PRIV_W { w: self }
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    pub fn crspriv(&mut self) -> CRSPRIV_W {
        CRSPRIV_W { w: self }
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    pub fn dacpriv(&mut self) -> DACPRIV_W {
        DACPRIV_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W {
        OPAMPPRIV_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W {
        LPTIM1PRIV_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W {
        LPUART1PRIV_W { w: self }
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W {
        I2C4PRIV_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W {
        LPTIM2PRIV_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W {
        LPTIM3PRIV_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W {
        FDCAN1PRIV_W { w: self }
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W {
        USBFSPRIV_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    pub fn ucpd1priv(&mut self) -> UCPD1PRIV_W {
        UCPD1PRIV_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W {
        VREFBUFPRIV_W { w: self }
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    pub fn comppriv(&mut self) -> COMPPRIV_W {
        COMPPRIV_W { w: self }
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W {
        TIM1PRIV_W { w: self }
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W {
        SPI1PRIV_W { w: self }
    }
}
