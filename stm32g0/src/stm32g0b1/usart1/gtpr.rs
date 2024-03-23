#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GTPRrs>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GTPRrs>;
#[doc = "Field `PSC` reader - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<GTPRrs> {
        PSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<GTPRrs> {
        GT_W::new(self, 8)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPRrs;
impl crate::RegisterSpec for GTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GTPRrs {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPRrs {
    const RESET_VALUE: u32 = 0;
}
