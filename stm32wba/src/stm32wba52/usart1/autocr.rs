///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `TDN` reader - TDN transmission data number This bitfield enables the programming of the number of data to be transmitted. It can be written only when UE is cleared in USART_CR1.
pub type TDN_R = crate::FieldReader<u16>;
///Field `TDN` writer - TDN transmission data number This bitfield enables the programming of the number of data to be transmitted. It can be written only when UE is cleared in USART_CR1.
pub type TDN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TRIGPOL` reader - Trigger polarity bit This bitfield can be written only when the UE bit is cleared in USART_CR1 register.
pub type TRIGPOL_R = crate::BitReader;
///Field `TRIGPOL` writer - Trigger polarity bit This bitfield can be written only when the UE bit is cleared in USART_CR1 register.
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGEN` reader - Trigger enable bit Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared. Note: When a trigger is detected, TE is set to 1 in USART_CR1 and the data transfer is launched.
pub type TRIGEN_R = crate::BitReader;
///Field `TRIGEN` writer - Trigger enable bit Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared. Note: When a trigger is detected, TE is set to 1 in USART_CR1 and the data transfer is launched.
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEDIS` reader - Idle frame transmission disable bit after enabling the transmitter Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
pub type IDLEDIS_R = crate::BitReader;
///Field `IDLEDIS` writer - Idle frame transmission disable bit after enabling the transmitter Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
pub type IDLEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGSEL` reader - Trigger selection bits Refer to Description of USART interconnections. This bitfield can be written only when the UE bit is cleared in USART_CR1 register. ... Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger selection bits Refer to Description of USART interconnections. This bitfield can be written only when the UE bit is cleared in USART_CR1 register. ... Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:15 - TDN transmission data number This bitfield enables the programming of the number of data to be transmitted. It can be written only when UE is cleared in USART_CR1.
    #[inline(always)]
    pub fn tdn(&self) -> TDN_R {
        TDN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Trigger polarity bit This bitfield can be written only when the UE bit is cleared in USART_CR1 register.
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Trigger enable bit Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared. Note: When a trigger is detected, TE is set to 1 in USART_CR1 and the data transfer is launched.
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Idle frame transmission disable bit after enabling the transmitter Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
    #[inline(always)]
    pub fn idledis(&self) -> IDLEDIS_R {
        IDLEDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - Trigger selection bits Refer to Description of USART interconnections. This bitfield can be written only when the UE bit is cleared in USART_CR1 register. ... Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("tdn", &self.tdn())
            .field("trigpol", &self.trigpol())
            .field("trigen", &self.trigen())
            .field("idledis", &self.idledis())
            .field("trigsel", &self.trigsel())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TDN transmission data number This bitfield enables the programming of the number of data to be transmitted. It can be written only when UE is cleared in USART_CR1.
    #[inline(always)]
    pub fn tdn(&mut self) -> TDN_W<'_, AUTOCRrs> {
        TDN_W::new(self, 0)
    }
    ///Bit 16 - Trigger polarity bit This bitfield can be written only when the UE bit is cleared in USART_CR1 register.
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<'_, AUTOCRrs> {
        TRIGPOL_W::new(self, 16)
    }
    ///Bit 17 - Trigger enable bit Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared. Note: When a trigger is detected, TE is set to 1 in USART_CR1 and the data transfer is launched.
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<'_, AUTOCRrs> {
        TRIGEN_W::new(self, 17)
    }
    ///Bit 18 - Idle frame transmission disable bit after enabling the transmitter Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
    #[inline(always)]
    pub fn idledis(&mut self) -> IDLEDIS_W<'_, AUTOCRrs> {
        IDLEDIS_W::new(self, 18)
    }
    ///Bits 19:22 - Trigger selection bits Refer to Description of USART interconnections. This bitfield can be written only when the UE bit is cleared in USART_CR1 register. ... Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<'_, AUTOCRrs> {
        TRIGSEL_W::new(self, 19)
    }
}
/**USART Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#USART1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AUTOCR to value 0x8000_0000
impl crate::Resettable for AUTOCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
