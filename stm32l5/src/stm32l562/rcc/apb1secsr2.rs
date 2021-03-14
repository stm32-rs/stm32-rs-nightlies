#[doc = "Reader of register APB1SECSR2"]
pub type R = crate::R<u32, super::APB1SECSR2>;
#[doc = "Reader of field `UCPD1SECF`"]
pub type UCPD1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBFSSECF`"]
pub type USBFSSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FDCAN1SECF`"]
pub type FDCAN1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM3SECF`"]
pub type LPTIM3SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM2SECF`"]
pub type LPTIM2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C4SECF`"]
pub type I2C4SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPUART1SECF`"]
pub type LPUART1SECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 23 - UCPD1SECF"]
    #[inline(always)]
    pub fn ucpd1secf(&self) -> UCPD1SECF_R {
        UCPD1SECF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSSECF"]
    #[inline(always)]
    pub fn usbfssecf(&self) -> USBFSSECF_R {
        USBFSSECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1SECF"]
    #[inline(always)]
    pub fn fdcan1secf(&self) -> FDCAN1SECF_R {
        FDCAN1SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3SECF"]
    #[inline(always)]
    pub fn lptim3secf(&self) -> LPTIM3SECF_R {
        LPTIM3SECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SECF"]
    #[inline(always)]
    pub fn lptim2secf(&self) -> LPTIM2SECF_R {
        LPTIM2SECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4SECF"]
    #[inline(always)]
    pub fn i2c4secf(&self) -> I2C4SECF_R {
        I2C4SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPUART1SECF"]
    #[inline(always)]
    pub fn lpuart1secf(&self) -> LPUART1SECF_R {
        LPUART1SECF_R::new((self.bits & 0x01) != 0)
    }
}
