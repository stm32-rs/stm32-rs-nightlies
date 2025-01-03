///Register `GTPR` reader
pub type R = crate::R<GTPRrs>;
///Register `GTPR` writer
pub type W = crate::W<GTPRrs>;
/**Field `PSC` reader - Prescaler value In IrDA low-power and normal IrDA mode: PSC\[7:0\]
= IrDA Normal and Low-Power baud rate PSC\[7:0\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\[4:0\]
= Prescaler value PSC\[4:0\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \[7:5\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to .*/
pub type PSC_R = crate::FieldReader;
/**Field `PSC` writer - Prescaler value In IrDA low-power and normal IrDA mode: PSC\[7:0\]
= IrDA Normal and Low-Power baud rate PSC\[7:0\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\[4:0\]
= Prescaler value PSC\[4:0\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \[7:5\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to .*/
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type GT_R = crate::FieldReader;
///Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /**Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\[7:0\]
    = IrDA Normal and Low-Power baud rate PSC\[7:0\]
    is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\[4:0\]
    = Prescaler value PSC\[4:0\]
    is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \[7:5\]
    must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to .*/
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPR")
            .field("psc", &self.psc())
            .field("gt", &self.gt())
            .finish()
    }
}
impl W {
    /**Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\[7:0\]
    = IrDA Normal and Low-Power baud rate PSC\[7:0\]
    is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\[4:0\]
    = Prescaler value PSC\[4:0\]
    is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \[7:5\]
    must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to 0 when the Smartcard and IrDA modes are not supported. Refer to .*/
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<GTPRrs> {
        PSC_W::new(self, 0)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W<GTPRrs> {
        GT_W::new(self, 8)
    }
}
/**USART guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#USART1:GTPR)*/
pub struct GTPRrs;
impl crate::RegisterSpec for GTPRrs {
    type Ux = u32;
}
///`read()` method returns [`gtpr::R`](R) reader structure
impl crate::Readable for GTPRrs {}
///`write(|w| ..)` method takes [`gtpr::W`](W) writer structure
impl crate::Writable for GTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GTPR to value 0
impl crate::Resettable for GTPRrs {
    const RESET_VALUE: u32 = 0;
}
