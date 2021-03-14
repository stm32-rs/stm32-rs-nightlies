#[doc = "Reader of register IER1"]
pub type R = crate::R<u32, super::IER1>;
#[doc = "Writer for register IER1"]
pub type W = crate::W<u32, super::IER1>;
#[doc = "Register IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM2IE`"]
pub type TIM2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2IE`"]
pub struct TIM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2IE_W<'a> {
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
#[doc = "Reader of field `TIM3IE`"]
pub type TIM3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM3IE`"]
pub struct TIM3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3IE_W<'a> {
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
#[doc = "Reader of field `TIM4IE`"]
pub type TIM4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM4IE`"]
pub struct TIM4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4IE_W<'a> {
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
#[doc = "Reader of field `TIM5IE`"]
pub type TIM5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM5IE`"]
pub struct TIM5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5IE_W<'a> {
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
#[doc = "Reader of field `TIM6IE`"]
pub type TIM6IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM6IE`"]
pub struct TIM6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6IE_W<'a> {
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
#[doc = "Reader of field `TIM7IE`"]
pub type TIM7IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM7IE`"]
pub struct TIM7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7IE_W<'a> {
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
#[doc = "Reader of field `WWDGIE`"]
pub type WWDGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WWDGIE`"]
pub struct WWDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGIE_W<'a> {
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
#[doc = "Reader of field `IWDGIE`"]
pub type IWDGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDGIE`"]
pub struct IWDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGIE_W<'a> {
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
#[doc = "Reader of field `SPI2IE`"]
pub type SPI2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2IE`"]
pub struct SPI2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IE_W<'a> {
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
#[doc = "Reader of field `SPI3IE`"]
pub type SPI3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI3IE`"]
pub struct SPI3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3IE_W<'a> {
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
#[doc = "Reader of field `USART2IE`"]
pub type USART2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2IE`"]
pub struct USART2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2IE_W<'a> {
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
#[doc = "Reader of field `USART3IE`"]
pub type USART3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3IE`"]
pub struct USART3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3IE_W<'a> {
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
#[doc = "Reader of field `UART4IE`"]
pub type UART4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART4IE`"]
pub struct UART4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4IE_W<'a> {
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
#[doc = "Reader of field `UART5IE`"]
pub type UART5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART5IE`"]
pub struct UART5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5IE_W<'a> {
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
#[doc = "Reader of field `I2C1IE`"]
pub type I2C1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1IE`"]
pub struct I2C1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IE_W<'a> {
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
#[doc = "Reader of field `I2C2IE`"]
pub type I2C2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2IE`"]
pub struct I2C2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IE_W<'a> {
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
#[doc = "Reader of field `I2C3IE`"]
pub type I2C3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3IE`"]
pub struct I2C3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3IE_W<'a> {
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
#[doc = "Reader of field `CRSIE`"]
pub type CRSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSIE`"]
pub struct CRSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSIE_W<'a> {
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
#[doc = "Reader of field `DACIE`"]
pub type DACIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACIE`"]
pub struct DACIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIE_W<'a> {
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
#[doc = "Reader of field `OPAMPIE`"]
pub type OPAMPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPAMPIE`"]
pub struct OPAMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPIE_W<'a> {
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
#[doc = "Reader of field `LPTIM1IE`"]
pub type LPTIM1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1IE`"]
pub struct LPTIM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1IE_W<'a> {
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
#[doc = "Reader of field `LPUART1IE`"]
pub type LPUART1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1IE`"]
pub struct LPUART1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1IE_W<'a> {
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
#[doc = "Reader of field `I2C4IE`"]
pub type I2C4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4IE`"]
pub struct I2C4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4IE_W<'a> {
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
#[doc = "Reader of field `LPTIM2IE`"]
pub type LPTIM2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2IE`"]
pub struct LPTIM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2IE_W<'a> {
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
#[doc = "Reader of field `LPTIM3IE`"]
pub type LPTIM3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3IE`"]
pub struct LPTIM3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3IE_W<'a> {
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
#[doc = "Reader of field `FDCAN1IE`"]
pub type FDCAN1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDCAN1IE`"]
pub struct FDCAN1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1IE_W<'a> {
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
#[doc = "Reader of field `USBFSIE`"]
pub type USBFSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFSIE`"]
pub struct USBFSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSIE_W<'a> {
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
#[doc = "Reader of field `UCPD1IE`"]
pub type UCPD1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD1IE`"]
pub struct UCPD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1IE_W<'a> {
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
#[doc = "Reader of field `VREFBUFIE`"]
pub type VREFBUFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFBUFIE`"]
pub struct VREFBUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFIE_W<'a> {
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
#[doc = "Reader of field `COMPIE`"]
pub type COMPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPIE`"]
pub struct COMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPIE_W<'a> {
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
#[doc = "Reader of field `TIM1IE`"]
pub type TIM1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1IE`"]
pub struct TIM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1IE_W<'a> {
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
#[doc = "Reader of field `SPI1IE`"]
pub type SPI1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1IE`"]
pub struct SPI1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IE_W<'a> {
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
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    pub fn uart4ie(&self) -> UART4IE_R {
        UART4IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    pub fn dacie(&self) -> DACIE_R {
        DACIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    pub fn usbfsie(&self) -> USBFSIE_R {
        USBFSIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&mut self) -> TIM2IE_W {
        TIM2IE_W { w: self }
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&mut self) -> TIM3IE_W {
        TIM3IE_W { w: self }
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&mut self) -> TIM4IE_W {
        TIM4IE_W { w: self }
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&mut self) -> TIM5IE_W {
        TIM5IE_W { w: self }
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&mut self) -> TIM6IE_W {
        TIM6IE_W { w: self }
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&mut self) -> TIM7IE_W {
        TIM7IE_W { w: self }
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&mut self) -> WWDGIE_W {
        WWDGIE_W { w: self }
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&mut self) -> IWDGIE_W {
        IWDGIE_W { w: self }
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&mut self) -> SPI2IE_W {
        SPI2IE_W { w: self }
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    pub fn spi3ie(&mut self) -> SPI3IE_W {
        SPI3IE_W { w: self }
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    pub fn usart2ie(&mut self) -> USART2IE_W {
        USART2IE_W { w: self }
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    pub fn usart3ie(&mut self) -> USART3IE_W {
        USART3IE_W { w: self }
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    pub fn uart4ie(&mut self) -> UART4IE_W {
        UART4IE_W { w: self }
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    pub fn uart5ie(&mut self) -> UART5IE_W {
        UART5IE_W { w: self }
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    pub fn i2c1ie(&mut self) -> I2C1IE_W {
        I2C1IE_W { w: self }
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    pub fn i2c2ie(&mut self) -> I2C2IE_W {
        I2C2IE_W { w: self }
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    pub fn i2c3ie(&mut self) -> I2C3IE_W {
        I2C3IE_W { w: self }
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    pub fn crsie(&mut self) -> CRSIE_W {
        CRSIE_W { w: self }
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    pub fn dacie(&mut self) -> DACIE_W {
        DACIE_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    pub fn opampie(&mut self) -> OPAMPIE_W {
        OPAMPIE_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W {
        LPTIM1IE_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W {
        LPUART1IE_W { w: self }
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    pub fn i2c4ie(&mut self) -> I2C4IE_W {
        I2C4IE_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W {
        LPTIM2IE_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W {
        LPTIM3IE_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W {
        FDCAN1IE_W { w: self }
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    pub fn usbfsie(&mut self) -> USBFSIE_W {
        USBFSIE_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W {
        UCPD1IE_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W {
        VREFBUFIE_W { w: self }
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    pub fn compie(&mut self) -> COMPIE_W {
        COMPIE_W { w: self }
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W {
        TIM1IE_W { w: self }
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    pub fn spi1ie(&mut self) -> SPI1IE_W {
        SPI1IE_W { w: self }
    }
}
