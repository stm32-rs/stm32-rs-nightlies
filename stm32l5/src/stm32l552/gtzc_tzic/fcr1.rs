#[doc = "Register `FCR1` writer"]
pub struct W(crate::W<FCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2FC` writer - TIM2FC"]
pub struct TIM2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2FC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TIM3FC` writer - TIM3FC"]
pub struct TIM3FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TIM4FC` writer - TIM4FC"]
pub struct TIM4FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TIM5FC` writer - TIM5FC"]
pub struct TIM5FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TIM6FC` writer - TIM6FC"]
pub struct TIM6FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TIM7FC` writer - TIM7FC"]
pub struct TIM7FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WWDGFC` writer - WWDGFC"]
pub struct WWDGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IWDGFC` writer - IWDGFC"]
pub struct IWDGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SPI2FC` writer - SPI2FC"]
pub struct SPI2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SPI3FC` writer - SPI3FC"]
pub struct SPI3FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `USART2FC` writer - USART2FC"]
pub struct USART2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `USART3FC` writer - USART3FC"]
pub struct USART3FC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `UART4FC` writer - UART4FC"]
pub struct UART4FC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `UART5FC` writer - UART5FC"]
pub struct UART5FC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `I2C1FC` writer - I2C1FC"]
pub struct I2C1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `I2C2FC` writer - I2C2FC"]
pub struct I2C2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `I2C3FC` writer - I2C3FC"]
pub struct I2C3FC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CRSFC` writer - CRSFC"]
pub struct CRSFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DACFC` writer - DACFC"]
pub struct DACFC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OPAMPFC` writer - OPAMPFC"]
pub struct OPAMPFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `LPTIM1FC` writer - LPTIM1FC"]
pub struct LPTIM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `LPUART1FC` writer - LPUART1FC"]
pub struct LPUART1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `I2C4FC` writer - I2C4FC"]
pub struct I2C4FC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `LPTIM2FC` writer - LPTIM2FC"]
pub struct LPTIM2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `LPTIM3FC` writer - LPTIM3FC"]
pub struct LPTIM3FC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FDCAN1FC` writer - FDCAN1FC"]
pub struct FDCAN1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `USBFSFC` writer - USBFSFC"]
pub struct USBFSFC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `UCPD1FC` writer - UCPD1FC"]
pub struct UCPD1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `VREFBUFFC` writer - VREFBUFFC"]
pub struct VREFBUFFC_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `COMPFC` writer - COMPFC"]
pub struct COMPFC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TIM1FC` writer - TIM1FC"]
pub struct TIM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SPI1FC` writer - SPI1FC"]
pub struct SPI1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1FC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - TIM2FC"]
    #[inline(always)]
    pub fn tim2fc(&mut self) -> TIM2FC_W {
        TIM2FC_W { w: self }
    }
    #[doc = "Bit 1 - TIM3FC"]
    #[inline(always)]
    pub fn tim3fc(&mut self) -> TIM3FC_W {
        TIM3FC_W { w: self }
    }
    #[doc = "Bit 2 - TIM4FC"]
    #[inline(always)]
    pub fn tim4fc(&mut self) -> TIM4FC_W {
        TIM4FC_W { w: self }
    }
    #[doc = "Bit 3 - TIM5FC"]
    #[inline(always)]
    pub fn tim5fc(&mut self) -> TIM5FC_W {
        TIM5FC_W { w: self }
    }
    #[doc = "Bit 4 - TIM6FC"]
    #[inline(always)]
    pub fn tim6fc(&mut self) -> TIM6FC_W {
        TIM6FC_W { w: self }
    }
    #[doc = "Bit 5 - TIM7FC"]
    #[inline(always)]
    pub fn tim7fc(&mut self) -> TIM7FC_W {
        TIM7FC_W { w: self }
    }
    #[doc = "Bit 6 - WWDGFC"]
    #[inline(always)]
    pub fn wwdgfc(&mut self) -> WWDGFC_W {
        WWDGFC_W { w: self }
    }
    #[doc = "Bit 7 - IWDGFC"]
    #[inline(always)]
    pub fn iwdgfc(&mut self) -> IWDGFC_W {
        IWDGFC_W { w: self }
    }
    #[doc = "Bit 8 - SPI2FC"]
    #[inline(always)]
    pub fn spi2fc(&mut self) -> SPI2FC_W {
        SPI2FC_W { w: self }
    }
    #[doc = "Bit 9 - SPI3FC"]
    #[inline(always)]
    pub fn spi3fc(&mut self) -> SPI3FC_W {
        SPI3FC_W { w: self }
    }
    #[doc = "Bit 10 - USART2FC"]
    #[inline(always)]
    pub fn usart2fc(&mut self) -> USART2FC_W {
        USART2FC_W { w: self }
    }
    #[doc = "Bit 11 - USART3FC"]
    #[inline(always)]
    pub fn usart3fc(&mut self) -> USART3FC_W {
        USART3FC_W { w: self }
    }
    #[doc = "Bit 12 - UART4FC"]
    #[inline(always)]
    pub fn uart4fc(&mut self) -> UART4FC_W {
        UART4FC_W { w: self }
    }
    #[doc = "Bit 13 - UART5FC"]
    #[inline(always)]
    pub fn uart5fc(&mut self) -> UART5FC_W {
        UART5FC_W { w: self }
    }
    #[doc = "Bit 14 - I2C1FC"]
    #[inline(always)]
    pub fn i2c1fc(&mut self) -> I2C1FC_W {
        I2C1FC_W { w: self }
    }
    #[doc = "Bit 15 - I2C2FC"]
    #[inline(always)]
    pub fn i2c2fc(&mut self) -> I2C2FC_W {
        I2C2FC_W { w: self }
    }
    #[doc = "Bit 16 - I2C3FC"]
    #[inline(always)]
    pub fn i2c3fc(&mut self) -> I2C3FC_W {
        I2C3FC_W { w: self }
    }
    #[doc = "Bit 17 - CRSFC"]
    #[inline(always)]
    pub fn crsfc(&mut self) -> CRSFC_W {
        CRSFC_W { w: self }
    }
    #[doc = "Bit 18 - DACFC"]
    #[inline(always)]
    pub fn dacfc(&mut self) -> DACFC_W {
        DACFC_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPFC"]
    #[inline(always)]
    pub fn opampfc(&mut self) -> OPAMPFC_W {
        OPAMPFC_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1FC"]
    #[inline(always)]
    pub fn lptim1fc(&mut self) -> LPTIM1FC_W {
        LPTIM1FC_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1FC"]
    #[inline(always)]
    pub fn lpuart1fc(&mut self) -> LPUART1FC_W {
        LPUART1FC_W { w: self }
    }
    #[doc = "Bit 22 - I2C4FC"]
    #[inline(always)]
    pub fn i2c4fc(&mut self) -> I2C4FC_W {
        I2C4FC_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2FC"]
    #[inline(always)]
    pub fn lptim2fc(&mut self) -> LPTIM2FC_W {
        LPTIM2FC_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3FC"]
    #[inline(always)]
    pub fn lptim3fc(&mut self) -> LPTIM3FC_W {
        LPTIM3FC_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1FC"]
    #[inline(always)]
    pub fn fdcan1fc(&mut self) -> FDCAN1FC_W {
        FDCAN1FC_W { w: self }
    }
    #[doc = "Bit 26 - USBFSFC"]
    #[inline(always)]
    pub fn usbfsfc(&mut self) -> USBFSFC_W {
        USBFSFC_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1FC"]
    #[inline(always)]
    pub fn ucpd1fc(&mut self) -> UCPD1FC_W {
        UCPD1FC_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFFC"]
    #[inline(always)]
    pub fn vrefbuffc(&mut self) -> VREFBUFFC_W {
        VREFBUFFC_W { w: self }
    }
    #[doc = "Bit 29 - COMPFC"]
    #[inline(always)]
    pub fn compfc(&mut self) -> COMPFC_W {
        COMPFC_W { w: self }
    }
    #[doc = "Bit 30 - TIM1FC"]
    #[inline(always)]
    pub fn tim1fc(&mut self) -> TIM1FC_W {
        TIM1FC_W { w: self }
    }
    #[doc = "Bit 31 - SPI1FC"]
    #[inline(always)]
    pub fn spi1fc(&mut self) -> SPI1FC_W {
        SPI1FC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt clear register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr1](index.html) module"]
pub struct FCR1_SPEC;
impl crate::RegisterSpec for FCR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr1::W](W) writer structure"]
impl crate::Writable for FCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR1 to value 0"]
impl crate::Resettable for FCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
