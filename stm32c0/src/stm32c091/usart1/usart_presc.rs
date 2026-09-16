///Register `USART_PRESC` reader
pub type R = crate::R<USART_PRESCrs>;
///Register `USART_PRESC` writer
pub type W = crate::W<USART_PRESCrs>;
/**Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER {
    ///0: input clock not divided
    B0x0 = 0,
    ///1: input clock divided by 2
    B0x1 = 1,
    ///2: input clock divided by 4
    B0x2 = 2,
    ///3: input clock divided by 6
    B0x3 = 3,
    ///4: input clock divided by 8
    B0x4 = 4,
    ///5: input clock divided by 10
    B0x5 = 5,
    ///6: input clock divided by 12
    B0x6 = 6,
    ///7: input clock divided by 16
    B0x7 = 7,
    ///8: input clock divided by 32
    B0x8 = 8,
    ///9: input clock divided by 64
    B0x9 = 9,
    ///10: input clock divided by 128
    B0xA = 10,
    ///11: input clock divided by 256
    B0xB = 11,
}
impl From<PRESCALER> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER {
    type Ux = u8;
}
impl crate::IsEnum for PRESCALER {}
///Field `PRESCALER` reader - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub type PRESCALER_R = crate::FieldReader<PRESCALER>;
impl PRESCALER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCALER> {
        match self.bits {
            0 => Some(PRESCALER::B0x0),
            1 => Some(PRESCALER::B0x1),
            2 => Some(PRESCALER::B0x2),
            3 => Some(PRESCALER::B0x3),
            4 => Some(PRESCALER::B0x4),
            5 => Some(PRESCALER::B0x5),
            6 => Some(PRESCALER::B0x6),
            7 => Some(PRESCALER::B0x7),
            8 => Some(PRESCALER::B0x8),
            9 => Some(PRESCALER::B0x9),
            10 => Some(PRESCALER::B0xA),
            11 => Some(PRESCALER::B0xB),
            _ => None,
        }
    }
    ///input clock not divided
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESCALER::B0x0
    }
    ///input clock divided by 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESCALER::B0x1
    }
    ///input clock divided by 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESCALER::B0x2
    }
    ///input clock divided by 6
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESCALER::B0x3
    }
    ///input clock divided by 8
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESCALER::B0x4
    }
    ///input clock divided by 10
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESCALER::B0x5
    }
    ///input clock divided by 12
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESCALER::B0x6
    }
    ///input clock divided by 16
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESCALER::B0x7
    }
    ///input clock divided by 32
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESCALER::B0x8
    }
    ///input clock divided by 64
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESCALER::B0x9
    }
    ///input clock divided by 128
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESCALER::B0xA
    }
    ///input clock divided by 256
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESCALER::B0xB
    }
}
///Field `PRESCALER` writer - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESCALER>;
impl<'a, REG> PRESCALER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///input clock not divided
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x0)
    }
    ///input clock divided by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x1)
    }
    ///input clock divided by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x2)
    }
    ///input clock divided by 6
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x3)
    }
    ///input clock divided by 8
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x4)
    }
    ///input clock divided by 10
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x5)
    }
    ///input clock divided by 12
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x6)
    }
    ///input clock divided by 16
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x7)
    }
    ///input clock divided by 32
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x8)
    }
    ///input clock divided by 64
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0x9)
    }
    ///input clock divided by 128
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0xA)
    }
    ///input clock divided by 256
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER::B0xB)
    }
}
impl R {
    ///Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_PRESC")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<'_, USART_PRESCrs> {
        PRESCALER_W::new(self, 0)
    }
}
/**USART prescaler register

You can [`read`](crate::Reg::read) this register and get [`usart_presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#USART1:USART_PRESC)*/
pub struct USART_PRESCrs;
impl crate::RegisterSpec for USART_PRESCrs {
    type Ux = u32;
}
///`read()` method returns [`usart_presc::R`](R) reader structure
impl crate::Readable for USART_PRESCrs {}
///`write(|w| ..)` method takes [`usart_presc::W`](W) writer structure
impl crate::Writable for USART_PRESCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_PRESC to value 0
impl crate::Resettable for USART_PRESCrs {}
