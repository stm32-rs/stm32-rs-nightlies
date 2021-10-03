#[doc = "Register `GTPR` reader"]
pub struct R(crate::R<GTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTPR` writer"]
pub struct W(crate::W<GTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPR_SPEC>;
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
impl From<crate::W<GTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Reserved - do not program this value"]
    B_0X0 = 0,
    #[doc = "1: Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    B_0X1 = 1,
    #[doc = "2: Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    B_0X2 = 2,
    #[doc = "3: Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    B_0X3 = 3,
    #[doc = "31: Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    B_0X1F = 31,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSC` reader - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
pub struct PSC_R(crate::FieldReader<u8, PSC_A>);
impl PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSC_A> {
        match self.bits {
            0 => Some(PSC_A::B_0X0),
            1 => Some(PSC_A::B_0X1),
            2 => Some(PSC_A::B_0X2),
            3 => Some(PSC_A::B_0X3),
            31 => Some(PSC_A::B_0X1F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PSC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PSC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PSC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PSC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X1F`"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        **self == PSC_A::B_0X1F
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u8, PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PSC_A::B_0X0)
    }
    #[doc = "Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PSC_A::B_0X1)
    }
    #[doc = "Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PSC_A::B_0X2)
    }
    #[doc = "Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PSC_A::B_0X3)
    }
    #[doc = "Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(PSC_A::B_0X1F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct GT_R(crate::FieldReader<u8, u8>);
impl GT_R {
    pub(crate) fn new(bits: u8) -> Self {
        GT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct GT_W<'a> {
    w: &'a mut W,
}
impl<'a> GT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
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
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtpr](index.html) module"]
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtpr::R](R) reader structure"]
impl crate::Readable for GTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtpr::W](W) writer structure"]
impl crate::Writable for GTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
