///Register `USART_GTPR` reader
pub type R = crate::R<USART_GTPRrs>;
///Register `USART_GTPR` writer
pub type W = crate::W<USART_GTPRrs>;
/**Field `PSC` reader - Prescaler value PSC\[7:0\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\[4:0\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \[7:5\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826.*/
pub type PSC_R = crate::FieldReader;
/**Field `PSC` writer - Prescaler value PSC\[7:0\]
= IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\[4:0\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \[7:5\]
must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826.*/
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type GT_R = crate::FieldReader;
///Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /**Bits 0:7 - Prescaler value PSC\[7:0\]
    = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\[4:0\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \[7:5\]
    must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826.*/
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_GTPR")
            .field("psc", &self.psc())
            .field("gt", &self.gt())
            .finish()
    }
}
impl W {
    /**Bits 0:7 - Prescaler value PSC\[7:0\]
    = IrDA Normal and Low-power baud rate This bitfield is used for programming the prescaler for dividing the USART source clock to achieve the low-power frequency: The source clock is divided by the value given in the register (8 significant bits): ... PSC\[4:0\]: Prescaler value This bitfield is used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \[7:5\]
    must be kept cleared if Smartcard mode is used. Note: This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to Section131.4: USART implementation on page1826.*/
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<USART_GTPRrs> {
        PSC_W::new(self, 0)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W<USART_GTPRrs> {
        GT_W::new(self, 8)
    }
}
/**USART guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`usart_gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USART1:USART_GTPR)*/
pub struct USART_GTPRrs;
impl crate::RegisterSpec for USART_GTPRrs {
    type Ux = u32;
}
///`read()` method returns [`usart_gtpr::R`](R) reader structure
impl crate::Readable for USART_GTPRrs {}
///`write(|w| ..)` method takes [`usart_gtpr::W`](W) writer structure
impl crate::Writable for USART_GTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USART_GTPR to value 0
impl crate::Resettable for USART_GTPRrs {
    const RESET_VALUE: u32 = 0;
}